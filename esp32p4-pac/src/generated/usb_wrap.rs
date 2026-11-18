//! USB_WRAP
//! USB_WRAP Peripheral

#![allow(dead_code)]

pub const BASE: u32 = 0x50080000;

/// OTG_CONF Register
///
/// USB wrapper configuration registers.
///
/// Address offset: 0x0000
/// Reset value: 0x00100000
///
/// ## Fields
///
/// - **CLK_EN** `[31]` (RW)
///
///   Disable auto clock gating of CSR registers.
///
/// - **DFIFO_FORCE_PU** `[22]` (RW)
///
///   Disable the dfifo to go into low power mode. The data in dfifo will not lost.
///
/// - **PHY_TX_EDGE_SEL** `[21]` (RW)
///
///   Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge.
///
/// - **PHY_CLK_FORCE_ON** `[20]` (RW)
///
///   Force phy clock always on.
///
/// - **AHB_CLK_FORCE_ON** `[19]` (RW)
///
///   Force ahb clock always on.
///
/// - **USB_PAD_ENABLE** `[18]` (RW)
///
///   Enable USB pad function.
///
/// - **PULLUP_VALUE** `[17]` (RW)
///
///   Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K.
///
/// - **DM_PULLDOWN** `[16]` (RW)
///
///   Controlle USB D+ pulldown.
///
/// - **DM_PULLUP** `[15]` (RW)
///
///   Controlle USB D+ pullup.
///
/// - **DP_PULLDOWN** `[14]` (RW)
///
///   Controlle USB D+ pulldown.
///
/// - **DP_PULLUP** `[13]` (RW)
///
///   Controlle USB D+ pullup.
///
/// - **PAD_PULL_OVERRIDE** `[12]` (RW)
///
///   Enable software controlle USB D+ D- pullup pulldown.
///
/// - **VREF_OVERRIDE** `[11]` (RW)
///
///   Enable software controlle input threshold.
///
/// - **VREFL** `[10:9]` (RW)
///
///   Control single-end input low threshold,0.8V to 1.04V, step 80mV.
///
/// - **VREFH** `[8:7]` (RW)
///
///   Control single-end input high threshold,1.76V to 2V, step 80mV.
///
/// - **EXCHG_PINS** `[6]` (RW)
///
///   USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-.
///
/// - **EXCHG_PINS_OVERRIDE** `[5]` (RW)
///
///   Enable software controlle USB D+ D- exchange
///
/// - **DBNCE_FLTR_BYPASS** `[4]` (RW)
///
///   Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals
///
/// - **DFIFO_FORCE_PD** `[3]` (RW)
///
///   Force the dfifo to go into low power mode. The data in dfifo will not lost.
///
/// - **PHY_SEL** `[2]` (RW)
///
///   Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY.
///
/// - **SRP_SESSEND_VALUE** `[1]` (RW)
///
///   Software over-ride value of srp session end signal.
///
/// - **SRP_SESSEND_OVERRIDE** `[0]` (RW)
///
///   This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software.
#[repr(transparent)]
pub struct OtgConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for OtgConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl OtgConf {
    // ======== Field Constants ========

    /// **CLK_EN**
    ///
    /// Disable auto clock gating of CSR registers.
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const CLK_EN_OFFSET: u32 = 31;
    pub const CLK_EN_MASK: u32 = 0x80000000;
    pub const CLK_EN: u32 = 1 << 31;

    /// **DFIFO_FORCE_PU**
    ///
    /// Disable the dfifo to go into low power mode. The data in dfifo will not lost.
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const DFIFO_FORCE_PU_OFFSET: u32 = 22;
    pub const DFIFO_FORCE_PU_MASK: u32 = 0x00400000;
    pub const DFIFO_FORCE_PU: u32 = 1 << 22;

    /// **PHY_TX_EDGE_SEL**
    ///
    /// Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge.
    ///
    /// Bits `[21:21]`, mask `0x00200000`
    pub const PHY_TX_EDGE_SEL_OFFSET: u32 = 21;
    pub const PHY_TX_EDGE_SEL_MASK: u32 = 0x00200000;
    pub const PHY_TX_EDGE_SEL: u32 = 1 << 21;

    /// **PHY_CLK_FORCE_ON**
    ///
    /// Force phy clock always on.
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const PHY_CLK_FORCE_ON_OFFSET: u32 = 20;
    pub const PHY_CLK_FORCE_ON_MASK: u32 = 0x00100000;
    pub const PHY_CLK_FORCE_ON: u32 = 1 << 20;

    /// **AHB_CLK_FORCE_ON**
    ///
    /// Force ahb clock always on.
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const AHB_CLK_FORCE_ON_OFFSET: u32 = 19;
    pub const AHB_CLK_FORCE_ON_MASK: u32 = 0x00080000;
    pub const AHB_CLK_FORCE_ON: u32 = 1 << 19;

    /// **USB_PAD_ENABLE**
    ///
    /// Enable USB pad function.
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const USB_PAD_ENABLE_OFFSET: u32 = 18;
    pub const USB_PAD_ENABLE_MASK: u32 = 0x00040000;
    pub const USB_PAD_ENABLE: u32 = 1 << 18;

    /// **PULLUP_VALUE**
    ///
    /// Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K.
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const PULLUP_VALUE_OFFSET: u32 = 17;
    pub const PULLUP_VALUE_MASK: u32 = 0x00020000;
    pub const PULLUP_VALUE: u32 = 1 << 17;

    /// **DM_PULLDOWN**
    ///
    /// Controlle USB D+ pulldown.
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const DM_PULLDOWN_OFFSET: u32 = 16;
    pub const DM_PULLDOWN_MASK: u32 = 0x00010000;
    pub const DM_PULLDOWN: u32 = 1 << 16;

    /// **DM_PULLUP**
    ///
    /// Controlle USB D+ pullup.
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DM_PULLUP_OFFSET: u32 = 15;
    pub const DM_PULLUP_MASK: u32 = 0x00008000;
    pub const DM_PULLUP: u32 = 1 << 15;

    /// **DP_PULLDOWN**
    ///
    /// Controlle USB D+ pulldown.
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DP_PULLDOWN_OFFSET: u32 = 14;
    pub const DP_PULLDOWN_MASK: u32 = 0x00004000;
    pub const DP_PULLDOWN: u32 = 1 << 14;

    /// **DP_PULLUP**
    ///
    /// Controlle USB D+ pullup.
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DP_PULLUP_OFFSET: u32 = 13;
    pub const DP_PULLUP_MASK: u32 = 0x00002000;
    pub const DP_PULLUP: u32 = 1 << 13;

    /// **PAD_PULL_OVERRIDE**
    ///
    /// Enable software controlle USB D+ D- pullup pulldown.
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const PAD_PULL_OVERRIDE_OFFSET: u32 = 12;
    pub const PAD_PULL_OVERRIDE_MASK: u32 = 0x00001000;
    pub const PAD_PULL_OVERRIDE: u32 = 1 << 12;

    /// **VREF_OVERRIDE**
    ///
    /// Enable software controlle input threshold.
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const VREF_OVERRIDE_OFFSET: u32 = 11;
    pub const VREF_OVERRIDE_MASK: u32 = 0x00000800;
    pub const VREF_OVERRIDE: u32 = 1 << 11;

    /// **VREFL**
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV.
    ///
    /// Bits `[10:9]`, mask `0x00000600`
    pub const VREFL_OFFSET: u32 = 9;
    pub const VREFL_MASK: u32 = 0x00000600;

    /// **VREFH**
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV.
    ///
    /// Bits `[8:7]`, mask `0x00000180`
    pub const VREFH_OFFSET: u32 = 7;
    pub const VREFH_MASK: u32 = 0x00000180;

    /// **EXCHG_PINS**
    ///
    /// USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const EXCHG_PINS_OFFSET: u32 = 6;
    pub const EXCHG_PINS_MASK: u32 = 0x00000040;
    pub const EXCHG_PINS: u32 = 1 << 6;

    /// **EXCHG_PINS_OVERRIDE**
    ///
    /// Enable software controlle USB D+ D- exchange
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const EXCHG_PINS_OVERRIDE_OFFSET: u32 = 5;
    pub const EXCHG_PINS_OVERRIDE_MASK: u32 = 0x00000020;
    pub const EXCHG_PINS_OVERRIDE: u32 = 1 << 5;

    /// **DBNCE_FLTR_BYPASS**
    ///
    /// Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DBNCE_FLTR_BYPASS_OFFSET: u32 = 4;
    pub const DBNCE_FLTR_BYPASS_MASK: u32 = 0x00000010;
    pub const DBNCE_FLTR_BYPASS: u32 = 1 << 4;

    /// **DFIFO_FORCE_PD**
    ///
    /// Force the dfifo to go into low power mode. The data in dfifo will not lost.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DFIFO_FORCE_PD_OFFSET: u32 = 3;
    pub const DFIFO_FORCE_PD_MASK: u32 = 0x00000008;
    pub const DFIFO_FORCE_PD: u32 = 1 << 3;

    /// **PHY_SEL**
    ///
    /// Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const PHY_SEL_OFFSET: u32 = 2;
    pub const PHY_SEL_MASK: u32 = 0x00000004;
    pub const PHY_SEL: u32 = 1 << 2;

    /// **SRP_SESSEND_VALUE**
    ///
    /// Software over-ride value of srp session end signal.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const SRP_SESSEND_VALUE_OFFSET: u32 = 1;
    pub const SRP_SESSEND_VALUE_MASK: u32 = 0x00000002;
    pub const SRP_SESSEND_VALUE: u32 = 1 << 1;

    /// **SRP_SESSEND_OVERRIDE**
    ///
    /// This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SRP_SESSEND_OVERRIDE_OFFSET: u32 = 0;
    pub const SRP_SESSEND_OVERRIDE_MASK: u32 = 0x00000001;
    pub const SRP_SESSEND_OVERRIDE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **CLK_EN** `[31]` (RW)
    ///
    /// Disable auto clock gating of CSR registers.
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

    /// **DFIFO_FORCE_PU** `[22]` (RW)
    ///
    /// Disable the dfifo to go into low power mode. The data in dfifo will not lost.
    #[inline(always)]
    pub fn dfifo_force_pu(&self) -> bool {
        (self.value.read_bits() & Self::DFIFO_FORCE_PU_MASK) != 0
    }

    /// Set DFIFO_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dfifo_force_pu(&self) {
        self.value.modify_bits(|r| r | Self::DFIFO_FORCE_PU_MASK);
    }

    /// Clear DFIFO_FORCE_PU bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dfifo_force_pu(&self) {
        self.value.modify_bits(|r| r & !Self::DFIFO_FORCE_PU_MASK);
    }

    /// **PHY_TX_EDGE_SEL** `[21]` (RW)
    ///
    /// Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge.
    #[inline(always)]
    pub fn phy_tx_edge_sel(&self) -> bool {
        (self.value.read_bits() & Self::PHY_TX_EDGE_SEL_MASK) != 0
    }

    /// Set PHY_TX_EDGE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_tx_edge_sel(&self) {
        self.value.modify_bits(|r| r | Self::PHY_TX_EDGE_SEL_MASK);
    }

    /// Clear PHY_TX_EDGE_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_tx_edge_sel(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_TX_EDGE_SEL_MASK);
    }

    /// **PHY_CLK_FORCE_ON** `[20]` (RW)
    ///
    /// Force phy clock always on.
    #[inline(always)]
    pub fn phy_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::PHY_CLK_FORCE_ON_MASK) != 0
    }

    /// Set PHY_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_clk_force_on(&self) {
        self.value.modify_bits(|r| r | Self::PHY_CLK_FORCE_ON_MASK);
    }

    /// Clear PHY_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_clk_force_on(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_CLK_FORCE_ON_MASK);
    }

    /// **AHB_CLK_FORCE_ON** `[19]` (RW)
    ///
    /// Force ahb clock always on.
    #[inline(always)]
    pub fn ahb_clk_force_on(&self) -> bool {
        (self.value.read_bits() & Self::AHB_CLK_FORCE_ON_MASK) != 0
    }

    /// Set AHB_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ahb_clk_force_on(&self) {
        self.value.modify_bits(|r| r | Self::AHB_CLK_FORCE_ON_MASK);
    }

    /// Clear AHB_CLK_FORCE_ON bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ahb_clk_force_on(&self) {
        self.value.modify_bits(|r| r & !Self::AHB_CLK_FORCE_ON_MASK);
    }

    /// **USB_PAD_ENABLE** `[18]` (RW)
    ///
    /// Enable USB pad function.
    #[inline(always)]
    pub fn usb_pad_enable(&self) -> bool {
        (self.value.read_bits() & Self::USB_PAD_ENABLE_MASK) != 0
    }

    /// Set USB_PAD_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_usb_pad_enable(&self) {
        self.value.modify_bits(|r| r | Self::USB_PAD_ENABLE_MASK);
    }

    /// Clear USB_PAD_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_usb_pad_enable(&self) {
        self.value.modify_bits(|r| r & !Self::USB_PAD_ENABLE_MASK);
    }

    /// **PULLUP_VALUE** `[17]` (RW)
    ///
    /// Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K.
    #[inline(always)]
    pub fn pullup_value(&self) -> bool {
        (self.value.read_bits() & Self::PULLUP_VALUE_MASK) != 0
    }

    /// Set PULLUP_VALUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pullup_value(&self) {
        self.value.modify_bits(|r| r | Self::PULLUP_VALUE_MASK);
    }

    /// Clear PULLUP_VALUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pullup_value(&self) {
        self.value.modify_bits(|r| r & !Self::PULLUP_VALUE_MASK);
    }

    /// **DM_PULLDOWN** `[16]` (RW)
    ///
    /// Controlle USB D+ pulldown.
    #[inline(always)]
    pub fn dm_pulldown(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLDOWN_MASK) != 0
    }

    /// Set DM_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dm_pulldown(&self) {
        self.value.modify_bits(|r| r | Self::DM_PULLDOWN_MASK);
    }

    /// Clear DM_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dm_pulldown(&self) {
        self.value.modify_bits(|r| r & !Self::DM_PULLDOWN_MASK);
    }

    /// **DM_PULLUP** `[15]` (RW)
    ///
    /// Controlle USB D+ pullup.
    #[inline(always)]
    pub fn dm_pullup(&self) -> bool {
        (self.value.read_bits() & Self::DM_PULLUP_MASK) != 0
    }

    /// Set DM_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dm_pullup(&self) {
        self.value.modify_bits(|r| r | Self::DM_PULLUP_MASK);
    }

    /// Clear DM_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dm_pullup(&self) {
        self.value.modify_bits(|r| r & !Self::DM_PULLUP_MASK);
    }

    /// **DP_PULLDOWN** `[14]` (RW)
    ///
    /// Controlle USB D+ pulldown.
    #[inline(always)]
    pub fn dp_pulldown(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLDOWN_MASK) != 0
    }

    /// Set DP_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dp_pulldown(&self) {
        self.value.modify_bits(|r| r | Self::DP_PULLDOWN_MASK);
    }

    /// Clear DP_PULLDOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dp_pulldown(&self) {
        self.value.modify_bits(|r| r & !Self::DP_PULLDOWN_MASK);
    }

    /// **DP_PULLUP** `[13]` (RW)
    ///
    /// Controlle USB D+ pullup.
    #[inline(always)]
    pub fn dp_pullup(&self) -> bool {
        (self.value.read_bits() & Self::DP_PULLUP_MASK) != 0
    }

    /// Set DP_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dp_pullup(&self) {
        self.value.modify_bits(|r| r | Self::DP_PULLUP_MASK);
    }

    /// Clear DP_PULLUP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dp_pullup(&self) {
        self.value.modify_bits(|r| r & !Self::DP_PULLUP_MASK);
    }

    /// **PAD_PULL_OVERRIDE** `[12]` (RW)
    ///
    /// Enable software controlle USB D+ D- pullup pulldown.
    #[inline(always)]
    pub fn pad_pull_override(&self) -> bool {
        (self.value.read_bits() & Self::PAD_PULL_OVERRIDE_MASK) != 0
    }

    /// Set PAD_PULL_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pad_pull_override(&self) {
        self.value.modify_bits(|r| r | Self::PAD_PULL_OVERRIDE_MASK);
    }

    /// Clear PAD_PULL_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pad_pull_override(&self) {
        self.value
            .modify_bits(|r| r & !Self::PAD_PULL_OVERRIDE_MASK);
    }

    /// **VREF_OVERRIDE** `[11]` (RW)
    ///
    /// Enable software controlle input threshold.
    #[inline(always)]
    pub fn vref_override(&self) -> bool {
        (self.value.read_bits() & Self::VREF_OVERRIDE_MASK) != 0
    }

    /// Set VREF_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vref_override(&self) {
        self.value.modify_bits(|r| r | Self::VREF_OVERRIDE_MASK);
    }

    /// Clear VREF_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vref_override(&self) {
        self.value.modify_bits(|r| r & !Self::VREF_OVERRIDE_MASK);
    }

    /// **VREFL** `[10:9]` (RW)
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV.
    #[inline(always)]
    pub fn vrefl(&self) -> u32 {
        (self.value.read_bits() & Self::VREFL_MASK) >> Self::VREFL_OFFSET
    }

    /// Write **VREFL** `[10:9]` (RW)
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV.
    #[inline(always)]
    pub fn write_vrefl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VREFL_MASK) | ((value << Self::VREFL_OFFSET) & Self::VREFL_MASK)
        });
    }

    /// **VREFH** `[8:7]` (RW)
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV.
    #[inline(always)]
    pub fn vrefh(&self) -> u32 {
        (self.value.read_bits() & Self::VREFH_MASK) >> Self::VREFH_OFFSET
    }

    /// Write **VREFH** `[8:7]` (RW)
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV.
    #[inline(always)]
    pub fn write_vrefh(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VREFH_MASK) | ((value << Self::VREFH_OFFSET) & Self::VREFH_MASK)
        });
    }

    /// **EXCHG_PINS** `[6]` (RW)
    ///
    /// USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-.
    #[inline(always)]
    pub fn exchg_pins(&self) -> bool {
        (self.value.read_bits() & Self::EXCHG_PINS_MASK) != 0
    }

    /// Set EXCHG_PINS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_exchg_pins(&self) {
        self.value.modify_bits(|r| r | Self::EXCHG_PINS_MASK);
    }

    /// Clear EXCHG_PINS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_exchg_pins(&self) {
        self.value.modify_bits(|r| r & !Self::EXCHG_PINS_MASK);
    }

    /// **EXCHG_PINS_OVERRIDE** `[5]` (RW)
    ///
    /// Enable software controlle USB D+ D- exchange
    #[inline(always)]
    pub fn exchg_pins_override(&self) -> bool {
        (self.value.read_bits() & Self::EXCHG_PINS_OVERRIDE_MASK) != 0
    }

    /// Set EXCHG_PINS_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_exchg_pins_override(&self) {
        self.value
            .modify_bits(|r| r | Self::EXCHG_PINS_OVERRIDE_MASK);
    }

    /// Clear EXCHG_PINS_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_exchg_pins_override(&self) {
        self.value
            .modify_bits(|r| r & !Self::EXCHG_PINS_OVERRIDE_MASK);
    }

    /// **DBNCE_FLTR_BYPASS** `[4]` (RW)
    ///
    /// Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals
    #[inline(always)]
    pub fn dbnce_fltr_bypass(&self) -> bool {
        (self.value.read_bits() & Self::DBNCE_FLTR_BYPASS_MASK) != 0
    }

    /// Set DBNCE_FLTR_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dbnce_fltr_bypass(&self) {
        self.value.modify_bits(|r| r | Self::DBNCE_FLTR_BYPASS_MASK);
    }

    /// Clear DBNCE_FLTR_BYPASS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dbnce_fltr_bypass(&self) {
        self.value
            .modify_bits(|r| r & !Self::DBNCE_FLTR_BYPASS_MASK);
    }

    /// **DFIFO_FORCE_PD** `[3]` (RW)
    ///
    /// Force the dfifo to go into low power mode. The data in dfifo will not lost.
    #[inline(always)]
    pub fn dfifo_force_pd(&self) -> bool {
        (self.value.read_bits() & Self::DFIFO_FORCE_PD_MASK) != 0
    }

    /// Set DFIFO_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dfifo_force_pd(&self) {
        self.value.modify_bits(|r| r | Self::DFIFO_FORCE_PD_MASK);
    }

    /// Clear DFIFO_FORCE_PD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dfifo_force_pd(&self) {
        self.value.modify_bits(|r| r & !Self::DFIFO_FORCE_PD_MASK);
    }

    /// **PHY_SEL** `[2]` (RW)
    ///
    /// Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY.
    #[inline(always)]
    pub fn phy_sel(&self) -> bool {
        (self.value.read_bits() & Self::PHY_SEL_MASK) != 0
    }

    /// Set PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_phy_sel(&self) {
        self.value.modify_bits(|r| r | Self::PHY_SEL_MASK);
    }

    /// Clear PHY_SEL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_phy_sel(&self) {
        self.value.modify_bits(|r| r & !Self::PHY_SEL_MASK);
    }

    /// **SRP_SESSEND_VALUE** `[1]` (RW)
    ///
    /// Software over-ride value of srp session end signal.
    #[inline(always)]
    pub fn srp_sessend_value(&self) -> bool {
        (self.value.read_bits() & Self::SRP_SESSEND_VALUE_MASK) != 0
    }

    /// Set SRP_SESSEND_VALUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_srp_sessend_value(&self) {
        self.value.modify_bits(|r| r | Self::SRP_SESSEND_VALUE_MASK);
    }

    /// Clear SRP_SESSEND_VALUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_srp_sessend_value(&self) {
        self.value
            .modify_bits(|r| r & !Self::SRP_SESSEND_VALUE_MASK);
    }

    /// **SRP_SESSEND_OVERRIDE** `[0]` (RW)
    ///
    /// This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software.
    #[inline(always)]
    pub fn srp_sessend_override(&self) -> bool {
        (self.value.read_bits() & Self::SRP_SESSEND_OVERRIDE_MASK) != 0
    }

    /// Set SRP_SESSEND_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_srp_sessend_override(&self) {
        self.value
            .modify_bits(|r| r | Self::SRP_SESSEND_OVERRIDE_MASK);
    }

    /// Clear SRP_SESSEND_OVERRIDE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_srp_sessend_override(&self) {
        self.value
            .modify_bits(|r| r & !Self::SRP_SESSEND_OVERRIDE_MASK);
    }
}

/// OTG_CONF value type for batch read/modify/write operations
///
/// Use `otgconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `otgconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct OtgConfVal {
    bits: u32,
}

impl OtgConfVal {
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
        Self { bits: 0x00100000 }
    }

    /// Read **CLK_EN** `[31:31]`
    ///
    /// Disable auto clock gating of CSR registers.
    #[inline(always)]
    pub const fn clk_en(&self) -> bool {
        (self.bits & OtgConf::CLK_EN_MASK) != 0
    }

    /// Read **DFIFO_FORCE_PU** `[22:22]`
    ///
    /// Disable the dfifo to go into low power mode. The data in dfifo will not lost.
    #[inline(always)]
    pub const fn dfifo_force_pu(&self) -> bool {
        (self.bits & OtgConf::DFIFO_FORCE_PU_MASK) != 0
    }

    /// Read **PHY_TX_EDGE_SEL** `[21:21]`
    ///
    /// Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge.
    #[inline(always)]
    pub const fn phy_tx_edge_sel(&self) -> bool {
        (self.bits & OtgConf::PHY_TX_EDGE_SEL_MASK) != 0
    }

    /// Read **PHY_CLK_FORCE_ON** `[20:20]`
    ///
    /// Force phy clock always on.
    #[inline(always)]
    pub const fn phy_clk_force_on(&self) -> bool {
        (self.bits & OtgConf::PHY_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **AHB_CLK_FORCE_ON** `[19:19]`
    ///
    /// Force ahb clock always on.
    #[inline(always)]
    pub const fn ahb_clk_force_on(&self) -> bool {
        (self.bits & OtgConf::AHB_CLK_FORCE_ON_MASK) != 0
    }

    /// Read **USB_PAD_ENABLE** `[18:18]`
    ///
    /// Enable USB pad function.
    #[inline(always)]
    pub const fn usb_pad_enable(&self) -> bool {
        (self.bits & OtgConf::USB_PAD_ENABLE_MASK) != 0
    }

    /// Read **PULLUP_VALUE** `[17:17]`
    ///
    /// Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K.
    #[inline(always)]
    pub const fn pullup_value(&self) -> bool {
        (self.bits & OtgConf::PULLUP_VALUE_MASK) != 0
    }

    /// Read **DM_PULLDOWN** `[16:16]`
    ///
    /// Controlle USB D+ pulldown.
    #[inline(always)]
    pub const fn dm_pulldown(&self) -> bool {
        (self.bits & OtgConf::DM_PULLDOWN_MASK) != 0
    }

    /// Read **DM_PULLUP** `[15:15]`
    ///
    /// Controlle USB D+ pullup.
    #[inline(always)]
    pub const fn dm_pullup(&self) -> bool {
        (self.bits & OtgConf::DM_PULLUP_MASK) != 0
    }

    /// Read **DP_PULLDOWN** `[14:14]`
    ///
    /// Controlle USB D+ pulldown.
    #[inline(always)]
    pub const fn dp_pulldown(&self) -> bool {
        (self.bits & OtgConf::DP_PULLDOWN_MASK) != 0
    }

    /// Read **DP_PULLUP** `[13:13]`
    ///
    /// Controlle USB D+ pullup.
    #[inline(always)]
    pub const fn dp_pullup(&self) -> bool {
        (self.bits & OtgConf::DP_PULLUP_MASK) != 0
    }

    /// Read **PAD_PULL_OVERRIDE** `[12:12]`
    ///
    /// Enable software controlle USB D+ D- pullup pulldown.
    #[inline(always)]
    pub const fn pad_pull_override(&self) -> bool {
        (self.bits & OtgConf::PAD_PULL_OVERRIDE_MASK) != 0
    }

    /// Read **VREF_OVERRIDE** `[11:11]`
    ///
    /// Enable software controlle input threshold.
    #[inline(always)]
    pub const fn vref_override(&self) -> bool {
        (self.bits & OtgConf::VREF_OVERRIDE_MASK) != 0
    }

    /// Read **VREFL** `[10:9]`
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV.
    #[inline(always)]
    pub const fn vrefl(&self) -> u32 {
        (self.bits & OtgConf::VREFL_MASK) >> OtgConf::VREFL_OFFSET
    }

    /// Read **VREFH** `[8:7]`
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV.
    #[inline(always)]
    pub const fn vrefh(&self) -> u32 {
        (self.bits & OtgConf::VREFH_MASK) >> OtgConf::VREFH_OFFSET
    }

    /// Read **EXCHG_PINS** `[6:6]`
    ///
    /// USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-.
    #[inline(always)]
    pub const fn exchg_pins(&self) -> bool {
        (self.bits & OtgConf::EXCHG_PINS_MASK) != 0
    }

    /// Read **EXCHG_PINS_OVERRIDE** `[5:5]`
    ///
    /// Enable software controlle USB D+ D- exchange
    #[inline(always)]
    pub const fn exchg_pins_override(&self) -> bool {
        (self.bits & OtgConf::EXCHG_PINS_OVERRIDE_MASK) != 0
    }

    /// Read **DBNCE_FLTR_BYPASS** `[4:4]`
    ///
    /// Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals
    #[inline(always)]
    pub const fn dbnce_fltr_bypass(&self) -> bool {
        (self.bits & OtgConf::DBNCE_FLTR_BYPASS_MASK) != 0
    }

    /// Read **DFIFO_FORCE_PD** `[3:3]`
    ///
    /// Force the dfifo to go into low power mode. The data in dfifo will not lost.
    #[inline(always)]
    pub const fn dfifo_force_pd(&self) -> bool {
        (self.bits & OtgConf::DFIFO_FORCE_PD_MASK) != 0
    }

    /// Read **PHY_SEL** `[2:2]`
    ///
    /// Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY.
    #[inline(always)]
    pub const fn phy_sel(&self) -> bool {
        (self.bits & OtgConf::PHY_SEL_MASK) != 0
    }

    /// Read **SRP_SESSEND_VALUE** `[1:1]`
    ///
    /// Software over-ride value of srp session end signal.
    #[inline(always)]
    pub const fn srp_sessend_value(&self) -> bool {
        (self.bits & OtgConf::SRP_SESSEND_VALUE_MASK) != 0
    }

    /// Read **SRP_SESSEND_OVERRIDE** `[0:0]`
    ///
    /// This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software.
    #[inline(always)]
    pub const fn srp_sessend_override(&self) -> bool {
        (self.bits & OtgConf::SRP_SESSEND_OVERRIDE_MASK) != 0
    }

    /// Set **CLK_EN** `[31:31]`
    ///
    /// Disable auto clock gating of CSR registers.
    #[inline(always)]
    pub const fn with_clk_en(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::CLK_EN_MASK
            } else {
                self.bits & !OtgConf::CLK_EN_MASK
            },
        }
    }

    /// Set **DFIFO_FORCE_PU** `[22:22]`
    ///
    /// Disable the dfifo to go into low power mode. The data in dfifo will not lost.
    #[inline(always)]
    pub const fn with_dfifo_force_pu(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::DFIFO_FORCE_PU_MASK
            } else {
                self.bits & !OtgConf::DFIFO_FORCE_PU_MASK
            },
        }
    }

    /// Set **PHY_TX_EDGE_SEL** `[21:21]`
    ///
    /// Select phy tx signal output clock edge. 1'b0: negedge, 1'b1: posedge.
    #[inline(always)]
    pub const fn with_phy_tx_edge_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::PHY_TX_EDGE_SEL_MASK
            } else {
                self.bits & !OtgConf::PHY_TX_EDGE_SEL_MASK
            },
        }
    }

    /// Set **PHY_CLK_FORCE_ON** `[20:20]`
    ///
    /// Force phy clock always on.
    #[inline(always)]
    pub const fn with_phy_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::PHY_CLK_FORCE_ON_MASK
            } else {
                self.bits & !OtgConf::PHY_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **AHB_CLK_FORCE_ON** `[19:19]`
    ///
    /// Force ahb clock always on.
    #[inline(always)]
    pub const fn with_ahb_clk_force_on(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::AHB_CLK_FORCE_ON_MASK
            } else {
                self.bits & !OtgConf::AHB_CLK_FORCE_ON_MASK
            },
        }
    }

    /// Set **USB_PAD_ENABLE** `[18:18]`
    ///
    /// Enable USB pad function.
    #[inline(always)]
    pub const fn with_usb_pad_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::USB_PAD_ENABLE_MASK
            } else {
                self.bits & !OtgConf::USB_PAD_ENABLE_MASK
            },
        }
    }

    /// Set **PULLUP_VALUE** `[17:17]`
    ///
    /// Controlle pullup value. 1'b0: typical value is 2.4K, 1'b1: typical value is 1.2K.
    #[inline(always)]
    pub const fn with_pullup_value(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::PULLUP_VALUE_MASK
            } else {
                self.bits & !OtgConf::PULLUP_VALUE_MASK
            },
        }
    }

    /// Set **DM_PULLDOWN** `[16:16]`
    ///
    /// Controlle USB D+ pulldown.
    #[inline(always)]
    pub const fn with_dm_pulldown(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::DM_PULLDOWN_MASK
            } else {
                self.bits & !OtgConf::DM_PULLDOWN_MASK
            },
        }
    }

    /// Set **DM_PULLUP** `[15:15]`
    ///
    /// Controlle USB D+ pullup.
    #[inline(always)]
    pub const fn with_dm_pullup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::DM_PULLUP_MASK
            } else {
                self.bits & !OtgConf::DM_PULLUP_MASK
            },
        }
    }

    /// Set **DP_PULLDOWN** `[14:14]`
    ///
    /// Controlle USB D+ pulldown.
    #[inline(always)]
    pub const fn with_dp_pulldown(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::DP_PULLDOWN_MASK
            } else {
                self.bits & !OtgConf::DP_PULLDOWN_MASK
            },
        }
    }

    /// Set **DP_PULLUP** `[13:13]`
    ///
    /// Controlle USB D+ pullup.
    #[inline(always)]
    pub const fn with_dp_pullup(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::DP_PULLUP_MASK
            } else {
                self.bits & !OtgConf::DP_PULLUP_MASK
            },
        }
    }

    /// Set **PAD_PULL_OVERRIDE** `[12:12]`
    ///
    /// Enable software controlle USB D+ D- pullup pulldown.
    #[inline(always)]
    pub const fn with_pad_pull_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::PAD_PULL_OVERRIDE_MASK
            } else {
                self.bits & !OtgConf::PAD_PULL_OVERRIDE_MASK
            },
        }
    }

    /// Set **VREF_OVERRIDE** `[11:11]`
    ///
    /// Enable software controlle input threshold.
    #[inline(always)]
    pub const fn with_vref_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::VREF_OVERRIDE_MASK
            } else {
                self.bits & !OtgConf::VREF_OVERRIDE_MASK
            },
        }
    }

    /// Set **VREFL** `[10:9]`
    ///
    /// Control single-end input low threshold,0.8V to 1.04V, step 80mV.
    #[inline(always)]
    pub const fn with_vrefl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OtgConf::VREFL_MASK)
                | ((value << OtgConf::VREFL_OFFSET) & OtgConf::VREFL_MASK),
        }
    }

    /// Set **VREFH** `[8:7]`
    ///
    /// Control single-end input high threshold,1.76V to 2V, step 80mV.
    #[inline(always)]
    pub const fn with_vrefh(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !OtgConf::VREFH_MASK)
                | ((value << OtgConf::VREFH_OFFSET) & OtgConf::VREFH_MASK),
        }
    }

    /// Set **EXCHG_PINS** `[6:6]`
    ///
    /// USB D+ D- exchange. 1'b0: don't change, 1'b1: exchange D+ D-.
    #[inline(always)]
    pub const fn with_exchg_pins(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::EXCHG_PINS_MASK
            } else {
                self.bits & !OtgConf::EXCHG_PINS_MASK
            },
        }
    }

    /// Set **EXCHG_PINS_OVERRIDE** `[5:5]`
    ///
    /// Enable software controlle USB D+ D- exchange
    #[inline(always)]
    pub const fn with_exchg_pins_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::EXCHG_PINS_OVERRIDE_MASK
            } else {
                self.bits & !OtgConf::EXCHG_PINS_OVERRIDE_MASK
            },
        }
    }

    /// Set **DBNCE_FLTR_BYPASS** `[4:4]`
    ///
    /// Bypass Debounce filters for avalid,bvalid,vbusvalid,session end, id signals
    #[inline(always)]
    pub const fn with_dbnce_fltr_bypass(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::DBNCE_FLTR_BYPASS_MASK
            } else {
                self.bits & !OtgConf::DBNCE_FLTR_BYPASS_MASK
            },
        }
    }

    /// Set **DFIFO_FORCE_PD** `[3:3]`
    ///
    /// Force the dfifo to go into low power mode. The data in dfifo will not lost.
    #[inline(always)]
    pub const fn with_dfifo_force_pd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::DFIFO_FORCE_PD_MASK
            } else {
                self.bits & !OtgConf::DFIFO_FORCE_PD_MASK
            },
        }
    }

    /// Set **PHY_SEL** `[2:2]`
    ///
    /// Select internal external PHY. 1'b0: Select internal PHY, 1'b1: Select external PHY.
    #[inline(always)]
    pub const fn with_phy_sel(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::PHY_SEL_MASK
            } else {
                self.bits & !OtgConf::PHY_SEL_MASK
            },
        }
    }

    /// Set **SRP_SESSEND_VALUE** `[1:1]`
    ///
    /// Software over-ride value of srp session end signal.
    #[inline(always)]
    pub const fn with_srp_sessend_value(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::SRP_SESSEND_VALUE_MASK
            } else {
                self.bits & !OtgConf::SRP_SESSEND_VALUE_MASK
            },
        }
    }

    /// Set **SRP_SESSEND_OVERRIDE** `[0:0]`
    ///
    /// This bit is used to enable the software over-ride of srp session end signal. 1'b0: the signal is controlled by the chip input, 1'b1: the signal is controlled by the software.
    #[inline(always)]
    pub const fn with_srp_sessend_override(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | OtgConf::SRP_SESSEND_OVERRIDE_MASK
            } else {
                self.bits & !OtgConf::SRP_SESSEND_OVERRIDE_MASK
            },
        }
    }
}

impl Default for OtgConfVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl OtgConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> OtgConfVal {
        OtgConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: OtgConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(OtgConfVal) -> OtgConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// TEST_CONF Register
///
/// USB wrapper test configuration registers.
///
/// Address offset: 0x0004
///
/// ## Fields
///
/// - **TEST_RX_DM** `[6]` (RO)
///
///   USB D- rx value in test.
///
/// - **TEST_RX_DP** `[5]` (RO)
///
///   USB D+ rx value in test.
///
/// - **TEST_RX_RCV** `[4]` (RO)
///
///   USB differential rx value in test.
///
/// - **TEST_TX_DM** `[3]` (RW)
///
///   USB D- tx value in test.
///
/// - **TEST_TX_DP** `[2]` (RW)
///
///   USB D+ tx value in test.
///
/// - **TEST_USB_OE** `[1]` (RW)
///
///   USB pad oen in test.
///
/// - **TEST_ENABLE** `[0]` (RW)
///
///   Enable test of the USB pad.
#[repr(transparent)]
pub struct TestConf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for TestConf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl TestConf {
    // ======== Field Constants ========

    /// **TEST_RX_DM**
    ///
    /// USB D- rx value in test.
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const TEST_RX_DM_OFFSET: u32 = 6;
    pub const TEST_RX_DM_MASK: u32 = 0x00000040;
    pub const TEST_RX_DM: u32 = 1 << 6;

    /// **TEST_RX_DP**
    ///
    /// USB D+ rx value in test.
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TEST_RX_DP_OFFSET: u32 = 5;
    pub const TEST_RX_DP_MASK: u32 = 0x00000020;
    pub const TEST_RX_DP: u32 = 1 << 5;

    /// **TEST_RX_RCV**
    ///
    /// USB differential rx value in test.
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TEST_RX_RCV_OFFSET: u32 = 4;
    pub const TEST_RX_RCV_MASK: u32 = 0x00000010;
    pub const TEST_RX_RCV: u32 = 1 << 4;

    /// **TEST_TX_DM**
    ///
    /// USB D- tx value in test.
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TEST_TX_DM_OFFSET: u32 = 3;
    pub const TEST_TX_DM_MASK: u32 = 0x00000008;
    pub const TEST_TX_DM: u32 = 1 << 3;

    /// **TEST_TX_DP**
    ///
    /// USB D+ tx value in test.
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TEST_TX_DP_OFFSET: u32 = 2;
    pub const TEST_TX_DP_MASK: u32 = 0x00000004;
    pub const TEST_TX_DP: u32 = 1 << 2;

    /// **TEST_USB_OE**
    ///
    /// USB pad oen in test.
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TEST_USB_OE_OFFSET: u32 = 1;
    pub const TEST_USB_OE_MASK: u32 = 0x00000002;
    pub const TEST_USB_OE: u32 = 1 << 1;

    /// **TEST_ENABLE**
    ///
    /// Enable test of the USB pad.
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TEST_ENABLE_OFFSET: u32 = 0;
    pub const TEST_ENABLE_MASK: u32 = 0x00000001;
    pub const TEST_ENABLE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TEST_RX_DM** `[6]` (RO)
    ///
    /// USB D- rx value in test.
    #[inline(always)]
    pub fn test_rx_dm(&self) -> bool {
        (self.value.read_bits() & Self::TEST_RX_DM_MASK) != 0
    }

    /// **TEST_RX_DP** `[5]` (RO)
    ///
    /// USB D+ rx value in test.
    #[inline(always)]
    pub fn test_rx_dp(&self) -> bool {
        (self.value.read_bits() & Self::TEST_RX_DP_MASK) != 0
    }

    /// **TEST_RX_RCV** `[4]` (RO)
    ///
    /// USB differential rx value in test.
    #[inline(always)]
    pub fn test_rx_rcv(&self) -> bool {
        (self.value.read_bits() & Self::TEST_RX_RCV_MASK) != 0
    }

    /// **TEST_TX_DM** `[3]` (RW)
    ///
    /// USB D- tx value in test.
    #[inline(always)]
    pub fn test_tx_dm(&self) -> bool {
        (self.value.read_bits() & Self::TEST_TX_DM_MASK) != 0
    }

    /// Set TEST_TX_DM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_test_tx_dm(&self) {
        self.value.modify_bits(|r| r | Self::TEST_TX_DM_MASK);
    }

    /// Clear TEST_TX_DM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_test_tx_dm(&self) {
        self.value.modify_bits(|r| r & !Self::TEST_TX_DM_MASK);
    }

    /// **TEST_TX_DP** `[2]` (RW)
    ///
    /// USB D+ tx value in test.
    #[inline(always)]
    pub fn test_tx_dp(&self) -> bool {
        (self.value.read_bits() & Self::TEST_TX_DP_MASK) != 0
    }

    /// Set TEST_TX_DP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_test_tx_dp(&self) {
        self.value.modify_bits(|r| r | Self::TEST_TX_DP_MASK);
    }

    /// Clear TEST_TX_DP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_test_tx_dp(&self) {
        self.value.modify_bits(|r| r & !Self::TEST_TX_DP_MASK);
    }

    /// **TEST_USB_OE** `[1]` (RW)
    ///
    /// USB pad oen in test.
    #[inline(always)]
    pub fn test_usb_oe(&self) -> bool {
        (self.value.read_bits() & Self::TEST_USB_OE_MASK) != 0
    }

    /// Set TEST_USB_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_test_usb_oe(&self) {
        self.value.modify_bits(|r| r | Self::TEST_USB_OE_MASK);
    }

    /// Clear TEST_USB_OE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_test_usb_oe(&self) {
        self.value.modify_bits(|r| r & !Self::TEST_USB_OE_MASK);
    }

    /// **TEST_ENABLE** `[0]` (RW)
    ///
    /// Enable test of the USB pad.
    #[inline(always)]
    pub fn test_enable(&self) -> bool {
        (self.value.read_bits() & Self::TEST_ENABLE_MASK) != 0
    }

    /// Set TEST_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_test_enable(&self) {
        self.value.modify_bits(|r| r | Self::TEST_ENABLE_MASK);
    }

    /// Clear TEST_ENABLE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_test_enable(&self) {
        self.value.modify_bits(|r| r & !Self::TEST_ENABLE_MASK);
    }
}

/// TEST_CONF value type for batch read/modify/write operations
///
/// Use `testconf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `testconf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct TestConfVal {
    bits: u32,
}

impl TestConfVal {
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

    /// Read **TEST_RX_DM** `[6:6]`
    ///
    /// USB D- rx value in test.
    #[inline(always)]
    pub const fn test_rx_dm(&self) -> bool {
        (self.bits & TestConf::TEST_RX_DM_MASK) != 0
    }

    /// Read **TEST_RX_DP** `[5:5]`
    ///
    /// USB D+ rx value in test.
    #[inline(always)]
    pub const fn test_rx_dp(&self) -> bool {
        (self.bits & TestConf::TEST_RX_DP_MASK) != 0
    }

    /// Read **TEST_RX_RCV** `[4:4]`
    ///
    /// USB differential rx value in test.
    #[inline(always)]
    pub const fn test_rx_rcv(&self) -> bool {
        (self.bits & TestConf::TEST_RX_RCV_MASK) != 0
    }

    /// Read **TEST_TX_DM** `[3:3]`
    ///
    /// USB D- tx value in test.
    #[inline(always)]
    pub const fn test_tx_dm(&self) -> bool {
        (self.bits & TestConf::TEST_TX_DM_MASK) != 0
    }

    /// Read **TEST_TX_DP** `[2:2]`
    ///
    /// USB D+ tx value in test.
    #[inline(always)]
    pub const fn test_tx_dp(&self) -> bool {
        (self.bits & TestConf::TEST_TX_DP_MASK) != 0
    }

    /// Read **TEST_USB_OE** `[1:1]`
    ///
    /// USB pad oen in test.
    #[inline(always)]
    pub const fn test_usb_oe(&self) -> bool {
        (self.bits & TestConf::TEST_USB_OE_MASK) != 0
    }

    /// Read **TEST_ENABLE** `[0:0]`
    ///
    /// Enable test of the USB pad.
    #[inline(always)]
    pub const fn test_enable(&self) -> bool {
        (self.bits & TestConf::TEST_ENABLE_MASK) != 0
    }

    /// Set **TEST_TX_DM** `[3:3]`
    ///
    /// USB D- tx value in test.
    #[inline(always)]
    pub const fn with_test_tx_dm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TestConf::TEST_TX_DM_MASK
            } else {
                self.bits & !TestConf::TEST_TX_DM_MASK
            },
        }
    }

    /// Set **TEST_TX_DP** `[2:2]`
    ///
    /// USB D+ tx value in test.
    #[inline(always)]
    pub const fn with_test_tx_dp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TestConf::TEST_TX_DP_MASK
            } else {
                self.bits & !TestConf::TEST_TX_DP_MASK
            },
        }
    }

    /// Set **TEST_USB_OE** `[1:1]`
    ///
    /// USB pad oen in test.
    #[inline(always)]
    pub const fn with_test_usb_oe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TestConf::TEST_USB_OE_MASK
            } else {
                self.bits & !TestConf::TEST_USB_OE_MASK
            },
        }
    }

    /// Set **TEST_ENABLE** `[0:0]`
    ///
    /// Enable test of the USB pad.
    #[inline(always)]
    pub const fn with_test_enable(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | TestConf::TEST_ENABLE_MASK
            } else {
                self.bits & !TestConf::TEST_ENABLE_MASK
            },
        }
    }
}

impl Default for TestConfVal {
    fn default() -> Self {
        Self { bits: 0 }
    }
}

impl TestConf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> TestConfVal {
        TestConfVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: TestConfVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(TestConfVal) -> TestConfVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DATE Register
///
/// Date register.
///
/// Address offset: 0x03fc
/// Reset value: 0x23030504
///
/// ## Fields
///
/// - **USB_WRAP_DATE** `[31:0]` (RO)
///
///   Date register.
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

    /// **USB_WRAP_DATE**
    ///
    /// Date register.
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const USB_WRAP_DATE_OFFSET: u32 = 0;
    pub const USB_WRAP_DATE_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **USB_WRAP_DATE** `[31:0]` (RO)
    ///
    /// Date register.
    #[inline(always)]
    pub fn usb_wrap_date(&self) -> u32 {
        (self.value.read_bits() & Self::USB_WRAP_DATE_MASK) >> Self::USB_WRAP_DATE_OFFSET
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
        Self { bits: 0x23030504 }
    }

    /// Read **USB_WRAP_DATE** `[31:0]`
    ///
    /// Date register.
    #[inline(always)]
    pub const fn usb_wrap_date(&self) -> u32 {
        (self.bits & Date::USB_WRAP_DATE_MASK) >> Date::USB_WRAP_DATE_OFFSET
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
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// USB wrapper configuration registers.
    pub otg_conf: OtgConf,
    /// 0x0004
    /// USB wrapper test configuration registers.
    pub test_conf: TestConf,
    _reserved0: [u8; 1012],
    /// 0x03fc
    /// Date register.
    pub date: Date,
}
