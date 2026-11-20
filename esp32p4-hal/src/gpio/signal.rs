//! HP GPIO matrix signal indices.
//!
//! Each named peripheral signal maps to one index in the HP GPIO matrix. For an
//! **output**, [`Pin::set_output_signal`](super::Pin::set_output_signal) writes
//! the index into the pin's `FUNCn_OUT_SEL_CFG` register; for an **input**,
//! [`Pin::set_input_signal`](super::Pin::set_input_signal) selects the pin in the
//! signal's `FUNCn_IN_SEL_CFG` register.
//!
//! Indices and names follow the ESP32-P4 TRM Table 10.12-1 ("HP GPIO matrix
//! peripheral signals"). Signals that the matrix reserves for the LP domain are
//! not listed here; see the TRM for the LP matrix.
//!
//! ```ignore
//! use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};
//!
//! pin.set_output_signal(OutputSignal::UART0_TXD_PAD_OUT);
//! pin.set_input_signal(InputSignal::UART0_RXD_PAD_IN);
//! ```

use esp32p4_pac::GPIO;
use pastey::paste;

/// A peripheral output signal, i.e. an index written to `FUNCn_OUT_SEL_CFG`.
///
/// Index 256 is the pin's own `GPIO_OUT` bit; use [`OutputSignal::GPIO`] rather
/// than constructing it by hand.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct OutputSignal(u16);

impl OutputSignal {
    /// The pin's own GPIO output register (`OUT_SEL = 256`).
    pub const GPIO: Self = Self(256);

    /// Signals with a valid output index (0..=255).
    pub const fn new(index: u16) -> Self {
        assert!(index <= 255, "GPIO output signal index must be 0-255");
        Self(index)
    }

    /// The matrix index.
    #[inline]
    pub const fn index(self) -> u16 {
        self.0
    }

    /// Whether this is the pin's `GPIO_OUT` register rather than a peripheral.
    #[inline]
    pub const fn is_gpio(self) -> bool {
        self.0 == 256
    }

    /// Rebuild a signal from a register read (0..=256).
    #[inline]
    pub(super) const fn from_raw(index: u16) -> Self {
        Self(index)
    }

    // Peripheral outputs, generated from the TRM signal table.
    pub const SD_CARD_CCLK_2_PAD_OUT: Self = Self(0);
    pub const SD_CARD_CCMD_2_PAD_OUT: Self = Self(1);
    pub const SD_CARD_CDATA0_2_PAD_OUT: Self = Self(2);
    pub const SD_CARD_CDATA1_2_PAD_OUT: Self = Self(3);
    pub const SD_CARD_CDATA2_2_PAD_OUT: Self = Self(4);
    pub const SD_CARD_CDATA3_2_PAD_OUT: Self = Self(5);
    pub const SD_CARD_CDATA4_2_PAD_OUT: Self = Self(6);
    pub const SD_CARD_CDATA5_2_PAD_OUT: Self = Self(7);
    pub const SD_CARD_CDATA6_2_PAD_OUT: Self = Self(8);
    pub const SD_CARD_CDATA7_2_PAD_OUT: Self = Self(9);
    pub const UART0_TXD_PAD_OUT: Self = Self(10);
    pub const UART0_RTS_PAD_OUT: Self = Self(11);
    pub const UART0_DTR_PAD_OUT: Self = Self(12);
    pub const UART1_TXD_PAD_OUT: Self = Self(13);
    pub const UART1_RTS_PAD_OUT: Self = Self(14);
    pub const UART1_DTR_PAD_OUT: Self = Self(15);
    pub const UART2_TXD_PAD_OUT: Self = Self(16);
    pub const UART2_RTS_PAD_OUT: Self = Self(17);
    pub const UART2_DTR_PAD_OUT: Self = Self(18);
    pub const UART3_TXD_PAD_OUT: Self = Self(19);
    pub const UART3_RTS_PAD_OUT: Self = Self(20);
    pub const UART3_DTR_PAD_OUT: Self = Self(21);
    pub const UART4_TXD_PAD_OUT: Self = Self(22);
    pub const UART4_RTS_PAD_OUT: Self = Self(23);
    pub const UART4_DTR_PAD_OUT: Self = Self(24);
    pub const I2S0_O_BCK_PAD_OUT: Self = Self(25);
    pub const I2S0_MCLK_PAD_OUT: Self = Self(26);
    pub const I2S0_O_WS_PAD_OUT: Self = Self(27);
    pub const I2S0_O_SD_PAD_OUT: Self = Self(28);
    pub const I2S0_I_BCK_PAD_OUT: Self = Self(29);
    pub const I2S0_I_WS_PAD_OUT: Self = Self(30);
    pub const I2S1_O_BCK_PAD_OUT: Self = Self(31);
    pub const I2S1_MCLK_PAD_OUT: Self = Self(32);
    pub const I2S1_O_WS_PAD_OUT: Self = Self(33);
    pub const I2S1_O_SD_PAD_OUT: Self = Self(34);
    pub const I2S1_I_BCK_PAD_OUT: Self = Self(35);
    pub const I2S1_I_WS_PAD_OUT: Self = Self(36);
    pub const I2S2_O_BCK_PAD_OUT: Self = Self(37);
    pub const I2S2_MCLK_PAD_OUT: Self = Self(38);
    pub const I2S2_O_WS_PAD_OUT: Self = Self(39);
    pub const I2S2_O_SD_PAD_OUT: Self = Self(40);
    pub const I2S2_I_BCK_PAD_OUT: Self = Self(41);
    pub const I2S2_I_WS_PAD_OUT: Self = Self(42);
    pub const I2S0_O_SD1_PAD_OUT: Self = Self(43);
    pub const SPI2_DQS_PAD_OUT: Self = Self(44);
    pub const SPI3_CS2_PAD_OUT: Self = Self(45);
    pub const SPI3_CS1_PAD_OUT: Self = Self(46);
    pub const SPI3_CK_PAD_OUT: Self = Self(47);
    pub const SPI3_QO_PAD_OUT: Self = Self(48);
    pub const SPI3_D_PAD_OUT: Self = Self(49);
    pub const SPI3_HOLD_PAD_OUT: Self = Self(50);
    pub const SPI3_WP_PAD_OUT: Self = Self(51);
    pub const SPI3_CS_PAD_OUT: Self = Self(52);
    pub const SPI2_CK_PAD_OUT: Self = Self(53);
    pub const SPI2_Q_PAD_OUT: Self = Self(54);
    pub const SPI2_D_PAD_OUT: Self = Self(55);
    pub const SPI2_HOLD_PAD_OUT: Self = Self(56);
    pub const SPI2_WP_PAD_OUT: Self = Self(57);
    pub const SPI2_IO4_PAD_OUT: Self = Self(58);
    pub const SPI2_IO5_PAD_OUT: Self = Self(59);
    pub const SPI2_IO6_PAD_OUT: Self = Self(60);
    pub const SPI2_IO7_PAD_OUT: Self = Self(61);
    pub const SPI2_CS_PAD_OUT: Self = Self(62);
    pub const SPI2_CS1_PAD_OUT: Self = Self(63);
    pub const SPI2_CS2_PAD_OUT: Self = Self(64);
    pub const SPI2_CS3_PAD_OUT: Self = Self(65);
    pub const SPI2_CS4_PAD_OUT: Self = Self(66);
    pub const SPI2_CS5_PAD_OUT: Self = Self(67);
    pub const I2C0_SCL_PAD_OUT: Self = Self(68);
    pub const I2C0_SDA_PAD_OUT: Self = Self(69);
    pub const I2C1_SCL_PAD_OUT: Self = Self(70);
    pub const I2C1_SDA_PAD_OUT: Self = Self(71);
    pub const GPIO_SD0_OUT: Self = Self(72);
    pub const GPIO_SD1_OUT: Self = Self(73);
    pub const GPIO_SD2_OUT: Self = Self(74);
    pub const GPIO_SD3_OUT: Self = Self(75);
    pub const GPIO_SD4_OUT: Self = Self(76);
    pub const GPIO_SD5_OUT: Self = Self(77);
    pub const GPIO_SD6_OUT: Self = Self(78);
    pub const GPIO_SD7_OUT: Self = Self(79);
    pub const TWAI0_TX_PAD_OUT: Self = Self(80);
    pub const TWAI0_BUS_OFF_ON_PAD_OUT: Self = Self(81);
    pub const TWAI0_CLKOUT_PAD_OUT: Self = Self(82);
    pub const TWAI1_TX_PAD_OUT: Self = Self(83);
    pub const TWAI1_BUS_OFF_ON_PAD_OUT: Self = Self(84);
    pub const TWAI1_CLKOUT_PAD_OUT: Self = Self(85);
    pub const TWAI2_TX_PAD_OUT: Self = Self(86);
    pub const TWAI2_BUS_OFF_ON_PAD_OUT: Self = Self(87);
    pub const TWAI2_CLKOUT_PAD_OUT: Self = Self(88);
    pub const PWM0_CH0_A_PAD_OUT: Self = Self(89);
    pub const PWM0_CH0_B_PAD_OUT: Self = Self(90);
    pub const PWM0_CH1_A_PAD_OUT: Self = Self(91);
    pub const PWM0_CH1_B_PAD_OUT: Self = Self(92);
    pub const PWM0_CH2_A_PAD_OUT: Self = Self(93);
    pub const PWM0_CH2_B_PAD_OUT: Self = Self(94);
    pub const PWM1_CH0_A_PAD_OUT: Self = Self(95);
    pub const PWM1_CH0_B_PAD_OUT: Self = Self(96);
    pub const PWM1_CH1_A_PAD_OUT: Self = Self(97);
    pub const PWM1_CH1_B_PAD_OUT: Self = Self(98);
    pub const PWM1_CH2_A_PAD_OUT: Self = Self(99);
    pub const PWM1_CH2_B_PAD_OUT: Self = Self(100);
    pub const TWAI0_STANDBY_PAD_OUT: Self = Self(105);
    pub const TWAI1_STANDBY_PAD_OUT: Self = Self(106);
    pub const TWAI2_STANDBY_PAD_OUT: Self = Self(107);
    pub const MII_MDC_PAD_OUT: Self = Self(108);
    pub const MII_MDO_PAD_OUT: Self = Self(109);
    pub const USB_SRP_DISCHRGVBUS_PAD_OUT: Self = Self(110);
    pub const USB_OTG11_IDPULLUP_PAD_OUT: Self = Self(111);
    pub const USB_OTG11_DPPULLDOWN_PAD_OUT: Self = Self(112);
    pub const USB_OTG11_DMPULLDOWN_PAD_OUT: Self = Self(113);
    pub const USB_OTG11_DRVVBUS_PAD_OUT: Self = Self(114);
    pub const USB_SRP_CHRGVBUS_PAD_OUT: Self = Self(115);
    pub const RNG_CHAIN_CLK_PAD_OUT: Self = Self(117);
    pub const HP_PROBE_TOP_OUT0: Self = Self(118);
    pub const HP_PROBE_TOP_OUT1: Self = Self(119);
    pub const HP_PROBE_TOP_OUT2: Self = Self(120);
    pub const HP_PROBE_TOP_OUT3: Self = Self(121);
    pub const HP_PROBE_TOP_OUT4: Self = Self(122);
    pub const HP_PROBE_TOP_OUT5: Self = Self(123);
    pub const HP_PROBE_TOP_OUT6: Self = Self(124);
    pub const HP_PROBE_TOP_OUT7: Self = Self(125);
    pub const LEDC_LS_SIG_OUT_PAD_OUT0: Self = Self(126);
    pub const LEDC_LS_SIG_OUT_PAD_OUT1: Self = Self(127);
    pub const LEDC_LS_SIG_OUT_PAD_OUT2: Self = Self(128);
    pub const LEDC_LS_SIG_OUT_PAD_OUT3: Self = Self(129);
    pub const LEDC_LS_SIG_OUT_PAD_OUT4: Self = Self(130);
    pub const LEDC_LS_SIG_OUT_PAD_OUT5: Self = Self(131);
    pub const LEDC_LS_SIG_OUT_PAD_OUT6: Self = Self(132);
    pub const LEDC_LS_SIG_OUT_PAD_OUT7: Self = Self(133);
    pub const I3C_MST_SCL_PAD_OUT: Self = Self(134);
    pub const I3C_MST_SDA_PAD_OUT: Self = Self(135);
    pub const I3C_SLV_SCL_PAD_OUT: Self = Self(136);
    pub const I3C_SLV_SDA_PAD_OUT: Self = Self(137);
    pub const I3C_MST_SCL_PULLUP_EN_PAD_OUT: Self = Self(138);
    pub const I3C_MST_SDA_PULLUP_EN_PAD_OUT: Self = Self(139);
    pub const USB_JTAG_TDI_BRIDGE_PAD_OUT: Self = Self(140);
    pub const USB_JTAG_TMS_BRIDGE_PAD_OUT: Self = Self(141);
    pub const USB_JTAG_TCK_BRIDGE_PAD_OUT: Self = Self(142);
    pub const USB_JTAG_TRST_BRIDGE_PAD_OUT: Self = Self(143);
    pub const LCD_CS_PAD_OUT: Self = Self(144);
    pub const LCD_DC_PAD_OUT: Self = Self(145);
    pub const SD_RST_N_1_PAD_OUT: Self = Self(146);
    pub const SD_RST_N_2_PAD_OUT: Self = Self(147);
    pub const SD_CCMD_OD_PULLUP_EN_N_PAD_OUT: Self = Self(148);
    pub const LCD_PCLK_PAD_OUT: Self = Self(149);
    pub const CAM_CLK_PAD_OUT: Self = Self(150);
    pub const LCD_H_ENABLE_PAD_OUT: Self = Self(151);
    pub const LCD_H_SYNC_PAD_OUT: Self = Self(152);
    pub const LCD_V_SYNC_PAD_OUT: Self = Self(153);
    pub const LCD_DATA_OUT_PAD_OUT0: Self = Self(154);
    pub const LCD_DATA_OUT_PAD_OUT1: Self = Self(155);
    pub const LCD_DATA_OUT_PAD_OUT2: Self = Self(156);
    pub const LCD_DATA_OUT_PAD_OUT3: Self = Self(157);
    pub const LCD_DATA_OUT_PAD_OUT4: Self = Self(158);
    pub const LCD_DATA_OUT_PAD_OUT5: Self = Self(159);
    pub const LCD_DATA_OUT_PAD_OUT6: Self = Self(160);
    pub const LCD_DATA_OUT_PAD_OUT7: Self = Self(161);
    pub const LCD_DATA_OUT_PAD_OUT8: Self = Self(162);
    pub const LCD_DATA_OUT_PAD_OUT9: Self = Self(163);
    pub const LCD_DATA_OUT_PAD_OUT10: Self = Self(164);
    pub const LCD_DATA_OUT_PAD_OUT11: Self = Self(165);
    pub const LCD_DATA_OUT_PAD_OUT12: Self = Self(166);
    pub const LCD_DATA_OUT_PAD_OUT13: Self = Self(167);
    pub const LCD_DATA_OUT_PAD_OUT14: Self = Self(168);
    pub const LCD_DATA_OUT_PAD_OUT15: Self = Self(169);
    pub const LCD_DATA_OUT_PAD_OUT16: Self = Self(170);
    pub const LCD_DATA_OUT_PAD_OUT17: Self = Self(171);
    pub const LCD_DATA_OUT_PAD_OUT18: Self = Self(172);
    pub const LCD_DATA_OUT_PAD_OUT19: Self = Self(173);
    pub const LCD_DATA_OUT_PAD_OUT20: Self = Self(174);
    pub const LCD_DATA_OUT_PAD_OUT21: Self = Self(175);
    pub const LCD_DATA_OUT_PAD_OUT22: Self = Self(176);
    pub const LCD_DATA_OUT_PAD_OUT23: Self = Self(177);
    pub const EMAC_PHY_TXEN_PAD_OUT: Self = Self(178);
    pub const EMAC_PHY_TXD0_PAD_OUT: Self = Self(179);
    pub const EMAC_PHY_TXD1_PAD_OUT: Self = Self(180);
    pub const EMAC_PHY_TXD2_PAD_OUT: Self = Self(181);
    pub const EMAC_PHY_TXD3_PAD_OUT: Self = Self(182);
    pub const EMAC_PHY_TXER_PAD_OUT: Self = Self(183);
    pub const DBG_CH0_CLK: Self = Self(184);
    pub const DBG_CH1_CLK: Self = Self(185);
    pub const PARLIO_RX_CLK_PAD_OUT: Self = Self(186);
    pub const PARLIO_TX_CLK_PAD_OUT: Self = Self(187);
    pub const PARLIO_TX_DATA0_PAD_OUT: Self = Self(188);
    pub const PARLIO_TX_DATA1_PAD_OUT: Self = Self(189);
    pub const PARLIO_TX_DATA2_PAD_OUT: Self = Self(190);
    pub const PARLIO_TX_DATA3_PAD_OUT: Self = Self(191);
    pub const PARLIO_TX_DATA4_PAD_OUT: Self = Self(192);
    pub const PARLIO_TX_DATA5_PAD_OUT: Self = Self(193);
    pub const PARLIO_TX_DATA6_PAD_OUT: Self = Self(194);
    pub const PARLIO_TX_DATA7_PAD_OUT: Self = Self(195);
    pub const PARLIO_TX_DATA8_PAD_OUT: Self = Self(196);
    pub const PARLIO_TX_DATA9_PAD_OUT: Self = Self(197);
    pub const PARLIO_TX_DATA10_PAD_OUT: Self = Self(198);
    pub const PARLIO_TX_DATA11_PAD_OUT: Self = Self(199);
    pub const PARLIO_TX_DATA12_PAD_OUT: Self = Self(200);
    pub const PARLIO_TX_DATA13_PAD_OUT: Self = Self(201);
    pub const PARLIO_TX_DATA14_PAD_OUT: Self = Self(202);
    pub const PARLIO_TX_DATA15_PAD_OUT: Self = Self(203);
    pub const HP_PROBE_TOP_OUT8: Self = Self(204);
    pub const HP_PROBE_TOP_OUT9: Self = Self(205);
    pub const HP_PROBE_TOP_OUT10: Self = Self(206);
    pub const HP_PROBE_TOP_OUT11: Self = Self(207);
    pub const HP_PROBE_TOP_OUT12: Self = Self(208);
    pub const HP_PROBE_TOP_OUT13: Self = Self(209);
    pub const HP_PROBE_TOP_OUT14: Self = Self(210);
    pub const HP_PROBE_TOP_OUT15: Self = Self(211);
    pub const CONSTANT0_PAD_OUT: Self = Self(212);
    pub const CONSTANT1_PAD_OUT: Self = Self(213);
    pub const CORE_GPIO_OUT_PAD_OUT0: Self = Self(214);
    pub const CORE_GPIO_OUT_PAD_OUT1: Self = Self(215);
    pub const CORE_GPIO_OUT_PAD_OUT2: Self = Self(216);
    pub const CORE_GPIO_OUT_PAD_OUT3: Self = Self(217);
    pub const CORE_GPIO_OUT_PAD_OUT4: Self = Self(218);
    pub const CORE_GPIO_OUT_PAD_OUT5: Self = Self(219);
    pub const CORE_GPIO_OUT_PAD_OUT6: Self = Self(220);
    pub const CORE_GPIO_OUT_PAD_OUT7: Self = Self(221);
    pub const CORE_GPIO_OUT_PAD_OUT8: Self = Self(222);
    pub const CORE_GPIO_OUT_PAD_OUT9: Self = Self(223);
    pub const CORE_GPIO_OUT_PAD_OUT10: Self = Self(224);
    pub const CORE_GPIO_OUT_PAD_OUT11: Self = Self(225);
    pub const CORE_GPIO_OUT_PAD_OUT12: Self = Self(226);
    pub const CORE_GPIO_OUT_PAD_OUT13: Self = Self(227);
    pub const CORE_GPIO_OUT_PAD_OUT14: Self = Self(228);
    pub const CORE_GPIO_OUT_PAD_OUT15: Self = Self(229);
    pub const CORE_GPIO_OUT_PAD_OUT16: Self = Self(230);
    pub const CORE_GPIO_OUT_PAD_OUT17: Self = Self(231);
    pub const CORE_GPIO_OUT_PAD_OUT18: Self = Self(232);
    pub const CORE_GPIO_OUT_PAD_OUT19: Self = Self(233);
    pub const CORE_GPIO_OUT_PAD_OUT20: Self = Self(234);
    pub const CORE_GPIO_OUT_PAD_OUT21: Self = Self(235);
    pub const CORE_GPIO_OUT_PAD_OUT22: Self = Self(236);
    pub const CORE_GPIO_OUT_PAD_OUT23: Self = Self(237);
    pub const CORE_GPIO_OUT_PAD_OUT24: Self = Self(238);
    pub const CORE_GPIO_OUT_PAD_OUT25: Self = Self(239);
    pub const CORE_GPIO_OUT_PAD_OUT26: Self = Self(240);
    pub const CORE_GPIO_OUT_PAD_OUT27: Self = Self(241);
    pub const PARLIO_TX_CS_PAD_OUT: Self = Self(242);
    pub const EMAC_PTP_PPS_PAD_OUT: Self = Self(243);
    pub const ANA_COMP0_OUT: Self = Self(244);
    pub const ANA_COMP1_OUT: Self = Self(245);
    pub const RMT_SIG_PAD_OUT0: Self = Self(246);
    pub const RMT_SIG_PAD_OUT1: Self = Self(247);
    pub const RMT_SIG_PAD_OUT2: Self = Self(248);
    pub const RMT_SIG_PAD_OUT3: Self = Self(249);
}

/// A peripheral input signal, i.e. one `FUNCn_IN_SEL_CFG` register.
///
/// The matrix has 254 usable input signals (indices 1..=254); index 255 is
/// reserved and has no configuration register.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct InputSignal(u8);

impl InputSignal {
    /// The input signal with matrix index `index` (1..=254).
    pub const fn new(index: u8) -> Self {
        assert!(
            index >= 1 && index <= 254,
            "GPIO input signal index must be 1-254"
        );
        Self(index)
    }

    /// The matrix index.
    #[inline]
    pub const fn index(self) -> u8 {
        self.0
    }

    // Peripheral inputs, generated from the TRM signal table.

    pub const SD_CARD_CCMD_2_PAD_IN: Self = Self(1);
    pub const SD_CARD_CDATA0_2_PAD_IN: Self = Self(2);
    pub const SD_CARD_CDATA1_2_PAD_IN: Self = Self(3);
    pub const SD_CARD_CDATA2_2_PAD_IN: Self = Self(4);
    pub const SD_CARD_CDATA3_2_PAD_IN: Self = Self(5);
    pub const SD_CARD_CDATA4_2_PAD_IN: Self = Self(6);
    pub const SD_CARD_CDATA5_2_PAD_IN: Self = Self(7);
    pub const SD_CARD_CDATA6_2_PAD_IN: Self = Self(8);
    pub const SD_CARD_CDATA7_2_PAD_IN: Self = Self(9);
    pub const UART0_RXD_PAD_IN: Self = Self(10);
    pub const UART0_CTS_PAD_IN: Self = Self(11);
    pub const UART0_DSR_PAD_IN: Self = Self(12);
    pub const UART1_RXD_PAD_IN: Self = Self(13);
    pub const UART1_CTS_PAD_IN: Self = Self(14);
    pub const UART1_DSR_PAD_IN: Self = Self(15);
    pub const UART2_RXD_PAD_IN: Self = Self(16);
    pub const UART2_CTS_PAD_IN: Self = Self(17);
    pub const UART2_DSR_PAD_IN: Self = Self(18);
    pub const UART3_RXD_PAD_IN: Self = Self(19);
    pub const UART3_CTS_PAD_IN: Self = Self(20);
    pub const UART3_DSR_PAD_IN: Self = Self(21);
    pub const UART4_RXD_PAD_IN: Self = Self(22);
    pub const UART4_CTS_PAD_IN: Self = Self(23);
    pub const UART4_DSR_PAD_IN: Self = Self(24);
    pub const I2S0_O_BCK_PAD_IN: Self = Self(25);
    pub const I2S0_MCLK_PAD_IN: Self = Self(26);
    pub const I2S0_O_WS_PAD_IN: Self = Self(27);
    pub const I2S0_I_SD_PAD_IN: Self = Self(28);
    pub const I2S0_I_BCK_PAD_IN: Self = Self(29);
    pub const I2S0_I_WS_PAD_IN: Self = Self(30);
    pub const I2S1_O_BCK_PAD_IN: Self = Self(31);
    pub const I2S1_MCLK_PAD_IN: Self = Self(32);
    pub const I2S1_O_WS_PAD_IN: Self = Self(33);
    pub const I2S1_I_SD_PAD_IN: Self = Self(34);
    pub const I2S1_I_BCK_PAD_IN: Self = Self(35);
    pub const I2S1_I_WS_PAD_IN: Self = Self(36);
    pub const I2S2_O_BCK_PAD_IN: Self = Self(37);
    pub const I2S2_MCLK_PAD_IN: Self = Self(38);
    pub const I2S2_O_WS_PAD_IN: Self = Self(39);
    pub const I2S2_I_SD_PAD_IN: Self = Self(40);
    pub const I2S2_I_BCK_PAD_IN: Self = Self(41);
    pub const I2S2_I_WS_PAD_IN: Self = Self(42);
    pub const I2S0_I_SD1_PAD_IN: Self = Self(43);
    pub const I2S0_I_SD2_PAD_IN: Self = Self(44);
    pub const I2S0_I_SD3_PAD_IN: Self = Self(45);
    pub const SPI3_CK_PAD_IN: Self = Self(47);
    pub const SPI3_Q_PAD_IN: Self = Self(48);
    pub const SPI3_D_PAD_IN: Self = Self(49);
    pub const SPI3_HOLD_PAD_IN: Self = Self(50);
    pub const SPI3_WP_PAD_IN: Self = Self(51);
    pub const SPI3_CS_PAD_IN: Self = Self(52);
    pub const SPI2_CK_PAD_IN: Self = Self(53);
    pub const SPI2_Q_PAD_IN: Self = Self(54);
    pub const SPI2_D_PAD_IN: Self = Self(55);
    pub const SPI2_HOLD_PAD_IN: Self = Self(56);
    pub const SPI2_WP_PAD_IN: Self = Self(57);
    pub const SPI2_IO4_PAD_IN: Self = Self(58);
    pub const SPI2_IO5_PAD_IN: Self = Self(59);
    pub const SPI2_IO6_PAD_IN: Self = Self(60);
    pub const SPI2_IO7_PAD_IN: Self = Self(61);
    pub const SPI2_CS_PAD_IN: Self = Self(62);
    pub const PCNT_RST_PAD_IN0: Self = Self(63);
    pub const PCNT_RST_PAD_IN1: Self = Self(64);
    pub const PCNT_RST_PAD_IN2: Self = Self(65);
    pub const PCNT_RST_PAD_IN3: Self = Self(66);
    pub const I2C0_SCL_PAD_IN: Self = Self(68);
    pub const I2C0_SDA_PAD_IN: Self = Self(69);
    pub const I2C1_SCL_PAD_IN: Self = Self(70);
    pub const I2C1_SDA_PAD_IN: Self = Self(71);
    pub const UART0_SLP_CLK_PAD_IN: Self = Self(74);
    pub const UART1_SLP_CLK_PAD_IN: Self = Self(75);
    pub const UART2_SLP_CLK_PAD_IN: Self = Self(76);
    pub const UART3_SLP_CLK_PAD_IN: Self = Self(77);
    pub const UART4_SLP_CLK_PAD_IN: Self = Self(78);
    pub const TWAI0_RX_PAD_IN: Self = Self(80);
    pub const TWAI1_RX_PAD_IN: Self = Self(83);
    pub const TWAI2_RX_PAD_IN: Self = Self(86);
    pub const PWM0_SYNC0_PAD_IN: Self = Self(89);
    pub const PWM0_SYNC1_PAD_IN: Self = Self(90);
    pub const PWM0_SYNC2_PAD_IN: Self = Self(91);
    pub const PWM0_F0_PAD_IN: Self = Self(92);
    pub const PWM0_F1_PAD_IN: Self = Self(93);
    pub const PWM0_F2_PAD_IN: Self = Self(94);
    pub const PWM0_CAP0_PAD_IN: Self = Self(95);
    pub const PWM0_CAP1_PAD_IN: Self = Self(96);
    pub const PWM0_CAP2_PAD_IN: Self = Self(97);
    pub const PWM1_SYNC0_PAD_IN: Self = Self(98);
    pub const PWM1_SYNC1_PAD_IN: Self = Self(99);
    pub const PWM1_SYNC2_PAD_IN: Self = Self(100);
    pub const PWM1_F0_PAD_IN: Self = Self(101);
    pub const PWM1_F1_PAD_IN: Self = Self(102);
    pub const PWM1_F2_PAD_IN: Self = Self(103);
    pub const PWM1_CAP0_PAD_IN: Self = Self(104);
    pub const PWM1_CAP1_PAD_IN: Self = Self(105);
    pub const PWM1_CAP2_PAD_IN: Self = Self(106);
    pub const MII_MDI_PAD_IN: Self = Self(107);
    pub const EMAC_PHY_COL_PAD_IN: Self = Self(108);
    pub const EMAC_PHY_CRS_PAD_IN: Self = Self(109);
    pub const USB_OTG11_IDDIG_PAD_IN: Self = Self(110);
    pub const USB_OTG11_AVALID_PAD_IN: Self = Self(111);
    pub const USB_SRP_BVALID_PAD_IN: Self = Self(112);
    pub const USB_OTG11_VBUSVALID_PAD_IN: Self = Self(113);
    pub const USB_SRP_SESSEND_PAD_IN: Self = Self(114);
    pub const ULPI_CLK_PAD_IN: Self = Self(117);
    pub const USB_HSPHY_REFCLK_IN: Self = Self(118);
    pub const SD_CARD_DETECT_N_1_PAD_IN: Self = Self(126);
    pub const SD_CARD_DETECT_N_2_PAD_IN: Self = Self(127);
    pub const SD_CARD_INT_N_1_PAD_IN: Self = Self(128);
    pub const SD_CARD_INT_N_2_PAD_IN: Self = Self(129);
    pub const SD_CARD_WRITE_PRT_1_PAD_IN: Self = Self(130);
    pub const SD_CARD_WRITE_PRT_2_PAD_IN: Self = Self(131);
    pub const SD_DATA_STROBE_1_PAD_IN: Self = Self(132);
    pub const SD_DATA_STROBE_2_PAD_IN: Self = Self(133);
    pub const I3C_MST_SCL_PAD_IN: Self = Self(134);
    pub const I3C_MST_SDA_PAD_IN: Self = Self(135);
    pub const I3C_SLV_SCL_PAD_IN: Self = Self(136);
    pub const I3C_SLV_SDA_PAD_IN: Self = Self(137);
    pub const USB_JTAG_TDO_BRIDGE_PAD_IN: Self = Self(140);
    pub const PCNT_SIG_CH0_PAD_IN0: Self = Self(141);
    pub const PCNT_SIG_CH0_PAD_IN1: Self = Self(142);
    pub const PCNT_SIG_CH0_PAD_IN2: Self = Self(143);
    pub const PCNT_SIG_CH0_PAD_IN3: Self = Self(144);
    pub const PCNT_SIG_CH1_PAD_IN0: Self = Self(145);
    pub const PCNT_SIG_CH1_PAD_IN1: Self = Self(146);
    pub const PCNT_SIG_CH1_PAD_IN2: Self = Self(147);
    pub const PCNT_SIG_CH1_PAD_IN3: Self = Self(148);
    pub const PCNT_CTRL_CH0_PAD_IN0: Self = Self(149);
    pub const PCNT_CTRL_CH0_PAD_IN1: Self = Self(150);
    pub const PCNT_CTRL_CH0_PAD_IN2: Self = Self(151);
    pub const PCNT_CTRL_CH0_PAD_IN3: Self = Self(152);
    pub const PCNT_CTRL_CH1_PAD_IN0: Self = Self(153);
    pub const PCNT_CTRL_CH1_PAD_IN1: Self = Self(154);
    pub const PCNT_CTRL_CH1_PAD_IN2: Self = Self(155);
    pub const PCNT_CTRL_CH1_PAD_IN3: Self = Self(156);
    pub const CAM_PCLK_PAD_IN: Self = Self(158);
    pub const CAM_H_ENABLE_PAD_IN: Self = Self(159);
    pub const CAM_H_SYNC_PAD_IN: Self = Self(160);
    pub const CAM_V_SYNC_PAD_IN: Self = Self(161);
    pub const CAM_DATA_IN_PAD_IN0: Self = Self(162);
    pub const CAM_DATA_IN_PAD_IN1: Self = Self(163);
    pub const CAM_DATA_IN_PAD_IN2: Self = Self(164);
    pub const CAM_DATA_IN_PAD_IN3: Self = Self(165);
    pub const CAM_DATA_IN_PAD_IN4: Self = Self(166);
    pub const CAM_DATA_IN_PAD_IN5: Self = Self(167);
    pub const CAM_DATA_IN_PAD_IN6: Self = Self(168);
    pub const CAM_DATA_IN_PAD_IN7: Self = Self(169);
    pub const CAM_DATA_IN_PAD_IN8: Self = Self(170);
    pub const CAM_DATA_IN_PAD_IN9: Self = Self(171);
    pub const CAM_DATA_IN_PAD_IN10: Self = Self(172);
    pub const CAM_DATA_IN_PAD_IN11: Self = Self(173);
    pub const CAM_DATA_IN_PAD_IN12: Self = Self(174);
    pub const CAM_DATA_IN_PAD_IN13: Self = Self(175);
    pub const CAM_DATA_IN_PAD_IN14: Self = Self(176);
    pub const CAM_DATA_IN_PAD_IN15: Self = Self(177);
    pub const EMAC_PHY_RXDV_PAD_IN: Self = Self(178);
    pub const EMAC_PHY_RXD0_PAD_IN: Self = Self(179);
    pub const EMAC_PHY_RXD1_PAD_IN: Self = Self(180);
    pub const EMAC_PHY_RXD2_PAD_IN: Self = Self(181);
    pub const EMAC_PHY_RXD3_PAD_IN: Self = Self(182);
    pub const EMAC_PHY_RXER_PAD_IN: Self = Self(183);
    pub const EMAC_RX_CLK_PAD_IN: Self = Self(184);
    pub const EMAC_TX_CLK_PAD_IN: Self = Self(185);
    pub const PARLIO_RX_CLK_PAD_IN: Self = Self(186);
    pub const PARLIO_TX_CLK_PAD_IN: Self = Self(187);
    pub const PARLIO_RX_DATA0_PAD_IN: Self = Self(188);
    pub const PARLIO_RX_DATA1_PAD_IN: Self = Self(189);
    pub const PARLIO_RX_DATA2_PAD_IN: Self = Self(190);
    pub const PARLIO_RX_DATA3_PAD_IN: Self = Self(191);
    pub const PARLIO_RX_DATA4_PAD_IN: Self = Self(192);
    pub const PARLIO_RX_DATA5_PAD_IN: Self = Self(193);
    pub const PARLIO_RX_DATA6_PAD_IN: Self = Self(194);
    pub const PARLIO_RX_DATA7_PAD_IN: Self = Self(195);
    pub const PARLIO_RX_DATA8_PAD_IN: Self = Self(196);
    pub const PARLIO_RX_DATA9_PAD_IN: Self = Self(197);
    pub const PARLIO_RX_DATA10_PAD_IN: Self = Self(198);
    pub const PARLIO_RX_DATA11_PAD_IN: Self = Self(199);
    pub const PARLIO_RX_DATA12_PAD_IN: Self = Self(200);
    pub const PARLIO_RX_DATA13_PAD_IN: Self = Self(201);
    pub const PARLIO_RX_DATA14_PAD_IN: Self = Self(202);
    pub const PARLIO_RX_DATA15_PAD_IN: Self = Self(203);
    pub const CORE_GPIO_IN_PAD_IN0: Self = Self(214);
    pub const CORE_GPIO_IN_PAD_IN1: Self = Self(215);
    pub const CORE_GPIO_IN_PAD_IN2: Self = Self(216);
    pub const CORE_GPIO_IN_PAD_IN3: Self = Self(217);
    pub const CORE_GPIO_IN_PAD_IN4: Self = Self(218);
    pub const CORE_GPIO_IN_PAD_IN5: Self = Self(219);
    pub const CORE_GPIO_IN_PAD_IN6: Self = Self(220);
    pub const CORE_GPIO_IN_PAD_IN7: Self = Self(221);
    pub const CORE_GPIO_IN_PAD_IN8: Self = Self(222);
    pub const CORE_GPIO_IN_PAD_IN9: Self = Self(223);
    pub const CORE_GPIO_IN_PAD_IN10: Self = Self(224);
    pub const CORE_GPIO_IN_PAD_IN11: Self = Self(225);
    pub const CORE_GPIO_IN_PAD_IN12: Self = Self(226);
    pub const CORE_GPIO_IN_PAD_IN13: Self = Self(227);
    pub const CORE_GPIO_IN_PAD_IN14: Self = Self(228);
    pub const CORE_GPIO_IN_PAD_IN15: Self = Self(229);
    pub const CORE_GPIO_IN_PAD_IN16: Self = Self(230);
    pub const CORE_GPIO_IN_PAD_IN17: Self = Self(231);
    pub const CORE_GPIO_IN_PAD_IN18: Self = Self(232);
    pub const CORE_GPIO_IN_PAD_IN19: Self = Self(233);
    pub const CORE_GPIO_IN_PAD_IN20: Self = Self(234);
    pub const CORE_GPIO_IN_PAD_IN21: Self = Self(235);
    pub const CORE_GPIO_IN_PAD_IN22: Self = Self(236);
    pub const CORE_GPIO_IN_PAD_IN23: Self = Self(237);
    pub const CORE_GPIO_IN_PAD_IN24: Self = Self(238);
    pub const CORE_GPIO_IN_PAD_IN25: Self = Self(239);
    pub const CORE_GPIO_IN_PAD_IN26: Self = Self(240);
    pub const CORE_GPIO_IN_PAD_IN27: Self = Self(241);
    pub const CORE_GPIO_IN_PAD_IN28: Self = Self(242);
    pub const CORE_GPIO_IN_PAD_IN29: Self = Self(243);
    pub const CORE_GPIO_IN_PAD_IN30: Self = Self(244);
    pub const CORE_GPIO_IN_PAD_IN31: Self = Self(245);
    pub const RMT_SIG_PAD_IN0: Self = Self(246);
    pub const RMT_SIG_PAD_IN1: Self = Self(247);
    pub const RMT_SIG_PAD_IN2: Self = Self(248);
    pub const RMT_SIG_PAD_IN3: Self = Self(249);
    pub const SIG_IN_FUNC250: Self = Self(250);
    pub const SIG_IN_FUNC251: Self = Self(251);
    pub const SIG_IN_FUNC252: Self = Self(252);
    pub const SIG_IN_FUNC253: Self = Self(253);
    pub const SIG_IN_FUNC254: Self = Self(254);
}

// ============================================================================
// Register access for input signals
// ============================================================================

/// The `FUNCn_IN_SEL_CFG` writes need the PAC's per-register types, but the
/// signal index is a runtime value. This trait is the common surface; the
/// implementations are generated for all 254 registers below.
trait InSelCfg {
    /// Connect `pin` to this signal and stop bypassing the GPIO matrix.
    fn select_pin(&self, pin: u32);
    /// Disconnect the signal from any GPIO.
    fn bypass(&self);
    /// Invert the signal on its way to the peripheral.
    fn set_inverted(&self, inverted: bool);
}

macro_rules! impl_in_sel_cfg {
    ($($n:literal),* $(,)?) => {
        paste! {
            $(
                impl InSelCfg for crate::pac::gpio::[<Func $n InSelCfg>] {
                    #[inline]
                    fn select_pin(&self, pin: u32) {
                        self.write_in_sel(pin);
                        self.set_sel();
                    }

                    #[inline]
                    fn bypass(&self) {
                        self.clr_sel();
                    }

                    #[inline]
                    fn set_inverted(&self, inverted: bool) {
                        if inverted {
                            self.set_in_inv_sel();
                        } else {
                            self.clr_in_inv_sel();
                        }
                    }
                }
            )*
        }
    };
}

impl_in_sel_cfg!(
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74,
    75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98,
    99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117,
    118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136,
    137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155,
    156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174,
    175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193,
    194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212,
    213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231,
    232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250,
    251, 252, 253, 254
);

#[inline]
fn in_sel_cfg(signal: InputSignal) -> &'static dyn InSelCfg {
    // SAFETY: `Pins::init` owns the GPIO block; a reference to its static
    // register block is only used to program the matrix.
    let gpio = unsafe { &*GPIO::PTR };
    match signal.index() {
        1 => &gpio.func1_in_sel_cfg,
        2 => &gpio.func2_in_sel_cfg,
        3 => &gpio.func3_in_sel_cfg,
        4 => &gpio.func4_in_sel_cfg,
        5 => &gpio.func5_in_sel_cfg,
        6 => &gpio.func6_in_sel_cfg,
        7 => &gpio.func7_in_sel_cfg,
        8 => &gpio.func8_in_sel_cfg,
        9 => &gpio.func9_in_sel_cfg,
        10 => &gpio.func10_in_sel_cfg,
        11 => &gpio.func11_in_sel_cfg,
        12 => &gpio.func12_in_sel_cfg,
        13 => &gpio.func13_in_sel_cfg,
        14 => &gpio.func14_in_sel_cfg,
        15 => &gpio.func15_in_sel_cfg,
        16 => &gpio.func16_in_sel_cfg,
        17 => &gpio.func17_in_sel_cfg,
        18 => &gpio.func18_in_sel_cfg,
        19 => &gpio.func19_in_sel_cfg,
        20 => &gpio.func20_in_sel_cfg,
        21 => &gpio.func21_in_sel_cfg,
        22 => &gpio.func22_in_sel_cfg,
        23 => &gpio.func23_in_sel_cfg,
        24 => &gpio.func24_in_sel_cfg,
        25 => &gpio.func25_in_sel_cfg,
        26 => &gpio.func26_in_sel_cfg,
        27 => &gpio.func27_in_sel_cfg,
        28 => &gpio.func28_in_sel_cfg,
        29 => &gpio.func29_in_sel_cfg,
        30 => &gpio.func30_in_sel_cfg,
        31 => &gpio.func31_in_sel_cfg,
        32 => &gpio.func32_in_sel_cfg,
        33 => &gpio.func33_in_sel_cfg,
        34 => &gpio.func34_in_sel_cfg,
        35 => &gpio.func35_in_sel_cfg,
        36 => &gpio.func36_in_sel_cfg,
        37 => &gpio.func37_in_sel_cfg,
        38 => &gpio.func38_in_sel_cfg,
        39 => &gpio.func39_in_sel_cfg,
        40 => &gpio.func40_in_sel_cfg,
        41 => &gpio.func41_in_sel_cfg,
        42 => &gpio.func42_in_sel_cfg,
        43 => &gpio.func43_in_sel_cfg,
        44 => &gpio.func44_in_sel_cfg,
        45 => &gpio.func45_in_sel_cfg,
        46 => &gpio.func46_in_sel_cfg,
        47 => &gpio.func47_in_sel_cfg,
        48 => &gpio.func48_in_sel_cfg,
        49 => &gpio.func49_in_sel_cfg,
        50 => &gpio.func50_in_sel_cfg,
        51 => &gpio.func51_in_sel_cfg,
        52 => &gpio.func52_in_sel_cfg,
        53 => &gpio.func53_in_sel_cfg,
        54 => &gpio.func54_in_sel_cfg,
        55 => &gpio.func55_in_sel_cfg,
        56 => &gpio.func56_in_sel_cfg,
        57 => &gpio.func57_in_sel_cfg,
        58 => &gpio.func58_in_sel_cfg,
        59 => &gpio.func59_in_sel_cfg,
        60 => &gpio.func60_in_sel_cfg,
        61 => &gpio.func61_in_sel_cfg,
        62 => &gpio.func62_in_sel_cfg,
        63 => &gpio.func63_in_sel_cfg,
        64 => &gpio.func64_in_sel_cfg,
        65 => &gpio.func65_in_sel_cfg,
        66 => &gpio.func66_in_sel_cfg,
        67 => &gpio.func67_in_sel_cfg,
        68 => &gpio.func68_in_sel_cfg,
        69 => &gpio.func69_in_sel_cfg,
        70 => &gpio.func70_in_sel_cfg,
        71 => &gpio.func71_in_sel_cfg,
        72 => &gpio.func72_in_sel_cfg,
        73 => &gpio.func73_in_sel_cfg,
        74 => &gpio.func74_in_sel_cfg,
        75 => &gpio.func75_in_sel_cfg,
        76 => &gpio.func76_in_sel_cfg,
        77 => &gpio.func77_in_sel_cfg,
        78 => &gpio.func78_in_sel_cfg,
        79 => &gpio.func79_in_sel_cfg,
        80 => &gpio.func80_in_sel_cfg,
        81 => &gpio.func81_in_sel_cfg,
        82 => &gpio.func82_in_sel_cfg,
        83 => &gpio.func83_in_sel_cfg,
        84 => &gpio.func84_in_sel_cfg,
        85 => &gpio.func85_in_sel_cfg,
        86 => &gpio.func86_in_sel_cfg,
        87 => &gpio.func87_in_sel_cfg,
        88 => &gpio.func88_in_sel_cfg,
        89 => &gpio.func89_in_sel_cfg,
        90 => &gpio.func90_in_sel_cfg,
        91 => &gpio.func91_in_sel_cfg,
        92 => &gpio.func92_in_sel_cfg,
        93 => &gpio.func93_in_sel_cfg,
        94 => &gpio.func94_in_sel_cfg,
        95 => &gpio.func95_in_sel_cfg,
        96 => &gpio.func96_in_sel_cfg,
        97 => &gpio.func97_in_sel_cfg,
        98 => &gpio.func98_in_sel_cfg,
        99 => &gpio.func99_in_sel_cfg,
        100 => &gpio.func100_in_sel_cfg,
        101 => &gpio.func101_in_sel_cfg,
        102 => &gpio.func102_in_sel_cfg,
        103 => &gpio.func103_in_sel_cfg,
        104 => &gpio.func104_in_sel_cfg,
        105 => &gpio.func105_in_sel_cfg,
        106 => &gpio.func106_in_sel_cfg,
        107 => &gpio.func107_in_sel_cfg,
        108 => &gpio.func108_in_sel_cfg,
        109 => &gpio.func109_in_sel_cfg,
        110 => &gpio.func110_in_sel_cfg,
        111 => &gpio.func111_in_sel_cfg,
        112 => &gpio.func112_in_sel_cfg,
        113 => &gpio.func113_in_sel_cfg,
        114 => &gpio.func114_in_sel_cfg,
        115 => &gpio.func115_in_sel_cfg,
        116 => &gpio.func116_in_sel_cfg,
        117 => &gpio.func117_in_sel_cfg,
        118 => &gpio.func118_in_sel_cfg,
        119 => &gpio.func119_in_sel_cfg,
        120 => &gpio.func120_in_sel_cfg,
        121 => &gpio.func121_in_sel_cfg,
        122 => &gpio.func122_in_sel_cfg,
        123 => &gpio.func123_in_sel_cfg,
        124 => &gpio.func124_in_sel_cfg,
        125 => &gpio.func125_in_sel_cfg,
        126 => &gpio.func126_in_sel_cfg,
        127 => &gpio.func127_in_sel_cfg,
        128 => &gpio.func128_in_sel_cfg,
        129 => &gpio.func129_in_sel_cfg,
        130 => &gpio.func130_in_sel_cfg,
        131 => &gpio.func131_in_sel_cfg,
        132 => &gpio.func132_in_sel_cfg,
        133 => &gpio.func133_in_sel_cfg,
        134 => &gpio.func134_in_sel_cfg,
        135 => &gpio.func135_in_sel_cfg,
        136 => &gpio.func136_in_sel_cfg,
        137 => &gpio.func137_in_sel_cfg,
        138 => &gpio.func138_in_sel_cfg,
        139 => &gpio.func139_in_sel_cfg,
        140 => &gpio.func140_in_sel_cfg,
        141 => &gpio.func141_in_sel_cfg,
        142 => &gpio.func142_in_sel_cfg,
        143 => &gpio.func143_in_sel_cfg,
        144 => &gpio.func144_in_sel_cfg,
        145 => &gpio.func145_in_sel_cfg,
        146 => &gpio.func146_in_sel_cfg,
        147 => &gpio.func147_in_sel_cfg,
        148 => &gpio.func148_in_sel_cfg,
        149 => &gpio.func149_in_sel_cfg,
        150 => &gpio.func150_in_sel_cfg,
        151 => &gpio.func151_in_sel_cfg,
        152 => &gpio.func152_in_sel_cfg,
        153 => &gpio.func153_in_sel_cfg,
        154 => &gpio.func154_in_sel_cfg,
        155 => &gpio.func155_in_sel_cfg,
        156 => &gpio.func156_in_sel_cfg,
        157 => &gpio.func157_in_sel_cfg,
        158 => &gpio.func158_in_sel_cfg,
        159 => &gpio.func159_in_sel_cfg,
        160 => &gpio.func160_in_sel_cfg,
        161 => &gpio.func161_in_sel_cfg,
        162 => &gpio.func162_in_sel_cfg,
        163 => &gpio.func163_in_sel_cfg,
        164 => &gpio.func164_in_sel_cfg,
        165 => &gpio.func165_in_sel_cfg,
        166 => &gpio.func166_in_sel_cfg,
        167 => &gpio.func167_in_sel_cfg,
        168 => &gpio.func168_in_sel_cfg,
        169 => &gpio.func169_in_sel_cfg,
        170 => &gpio.func170_in_sel_cfg,
        171 => &gpio.func171_in_sel_cfg,
        172 => &gpio.func172_in_sel_cfg,
        173 => &gpio.func173_in_sel_cfg,
        174 => &gpio.func174_in_sel_cfg,
        175 => &gpio.func175_in_sel_cfg,
        176 => &gpio.func176_in_sel_cfg,
        177 => &gpio.func177_in_sel_cfg,
        178 => &gpio.func178_in_sel_cfg,
        179 => &gpio.func179_in_sel_cfg,
        180 => &gpio.func180_in_sel_cfg,
        181 => &gpio.func181_in_sel_cfg,
        182 => &gpio.func182_in_sel_cfg,
        183 => &gpio.func183_in_sel_cfg,
        184 => &gpio.func184_in_sel_cfg,
        185 => &gpio.func185_in_sel_cfg,
        186 => &gpio.func186_in_sel_cfg,
        187 => &gpio.func187_in_sel_cfg,
        188 => &gpio.func188_in_sel_cfg,
        189 => &gpio.func189_in_sel_cfg,
        190 => &gpio.func190_in_sel_cfg,
        191 => &gpio.func191_in_sel_cfg,
        192 => &gpio.func192_in_sel_cfg,
        193 => &gpio.func193_in_sel_cfg,
        194 => &gpio.func194_in_sel_cfg,
        195 => &gpio.func195_in_sel_cfg,
        196 => &gpio.func196_in_sel_cfg,
        197 => &gpio.func197_in_sel_cfg,
        198 => &gpio.func198_in_sel_cfg,
        199 => &gpio.func199_in_sel_cfg,
        200 => &gpio.func200_in_sel_cfg,
        201 => &gpio.func201_in_sel_cfg,
        202 => &gpio.func202_in_sel_cfg,
        203 => &gpio.func203_in_sel_cfg,
        204 => &gpio.func204_in_sel_cfg,
        205 => &gpio.func205_in_sel_cfg,
        206 => &gpio.func206_in_sel_cfg,
        207 => &gpio.func207_in_sel_cfg,
        208 => &gpio.func208_in_sel_cfg,
        209 => &gpio.func209_in_sel_cfg,
        210 => &gpio.func210_in_sel_cfg,
        211 => &gpio.func211_in_sel_cfg,
        212 => &gpio.func212_in_sel_cfg,
        213 => &gpio.func213_in_sel_cfg,
        214 => &gpio.func214_in_sel_cfg,
        215 => &gpio.func215_in_sel_cfg,
        216 => &gpio.func216_in_sel_cfg,
        217 => &gpio.func217_in_sel_cfg,
        218 => &gpio.func218_in_sel_cfg,
        219 => &gpio.func219_in_sel_cfg,
        220 => &gpio.func220_in_sel_cfg,
        221 => &gpio.func221_in_sel_cfg,
        222 => &gpio.func222_in_sel_cfg,
        223 => &gpio.func223_in_sel_cfg,
        224 => &gpio.func224_in_sel_cfg,
        225 => &gpio.func225_in_sel_cfg,
        226 => &gpio.func226_in_sel_cfg,
        227 => &gpio.func227_in_sel_cfg,
        228 => &gpio.func228_in_sel_cfg,
        229 => &gpio.func229_in_sel_cfg,
        230 => &gpio.func230_in_sel_cfg,
        231 => &gpio.func231_in_sel_cfg,
        232 => &gpio.func232_in_sel_cfg,
        233 => &gpio.func233_in_sel_cfg,
        234 => &gpio.func234_in_sel_cfg,
        235 => &gpio.func235_in_sel_cfg,
        236 => &gpio.func236_in_sel_cfg,
        237 => &gpio.func237_in_sel_cfg,
        238 => &gpio.func238_in_sel_cfg,
        239 => &gpio.func239_in_sel_cfg,
        240 => &gpio.func240_in_sel_cfg,
        241 => &gpio.func241_in_sel_cfg,
        242 => &gpio.func242_in_sel_cfg,
        243 => &gpio.func243_in_sel_cfg,
        244 => &gpio.func244_in_sel_cfg,
        245 => &gpio.func245_in_sel_cfg,
        246 => &gpio.func246_in_sel_cfg,
        247 => &gpio.func247_in_sel_cfg,
        248 => &gpio.func248_in_sel_cfg,
        249 => &gpio.func249_in_sel_cfg,
        250 => &gpio.func250_in_sel_cfg,
        251 => &gpio.func251_in_sel_cfg,
        252 => &gpio.func252_in_sel_cfg,
        253 => &gpio.func253_in_sel_cfg,
        254 => &gpio.func254_in_sel_cfg,
        _ => unreachable!("InputSignal is range-checked"),
    }
}

pub(super) fn select_input_pin(signal: InputSignal, pin: u32) {
    in_sel_cfg(signal).select_pin(pin);
}

pub(super) fn bypass_input_signal(signal: InputSignal) {
    in_sel_cfg(signal).bypass();
}

pub(super) fn invert_input_signal(signal: InputSignal, inverted: bool) {
    in_sel_cfg(signal).set_inverted(inverted);
}
