//! Per-pad capability table.
//!
//! The ESP32-P4 routes most HP peripheral signals through the GPIO matrix, so a
//! driver usually only needs a [`signal`](super::signal) index and any pad will
//! do. That is not true for everything:
//!
//! - **The GPIO matrix is not the only path.** Every pad also has up to four
//!   *direct* IO MUX functions, selected by `IO_MUX_GPIOx.MCU_SEL`. Function 1
//!   is the GPIO function (the matrix path) on every pad; 0, 2 and 3 are
//!   pad-specific — JTAG on GPIO2-5, UART0/1 on GPIO8-13 and 37-38, SPI2 on
//!   GPIO6-11 and 28-38, SD1 on GPIO39-48 and the GMAC PHY/RMII signals on
//!   GPIO28-53.
//! - **Some peripherals have no matrix signal at all.** The GMAC PHY/RMII data
//!   and clock signals exist only as the direct function 3 of GPIO28-53, and
//!   the USB data lines are hard-wired to their pads (GPIO24/25 for the USB
//!   Serial/JTAG PHY, GPIO26/27 for the OTG full-speed PHY), selected through
//!   the pad's function 0. No amount of matrix routing moves them.
//! - **Analog functions are pad-specific too**: ADC1 on GPIO16-23, ADC2 on
//!   GPIO49-54, touch on GPIO2-15, the comparators on GPIO51-54 and the 32 kHz
//!   crystal on GPIO0-1.
//! - **The low-power peripherals** (LP UART, LP I2C, ...) are only reachable
//!   from GPIO0-15, through the separate LP GPIO matrix.
//!
//! This module encodes the TRM's "HP IO MUX Pin Functions" and "GPIO Pin
//! Analog Functions" tables, so a pin reports what it can do and a driver can
//! pick a direct function instead of a matrix route:
//!
//! ```ignore
//! use esp32p4::gpio::{MuxFunction, Pins};
//!
//! let mut sclk = pins.pin::<43>();
//! sclk.set_mux_function(MuxFunction::Sd1Cclk);
//! assert_eq!(sclk.mux_function(), Some(MuxFunction::Sd1Cclk));
//! ```
//!
//! GPIO54-56 exist in the GPIO register blocks but have no `IO_MUX` register in
//! this chip's register map, so the table stops at GPIO53.
//!
//! [`signal`]: super::signal

use super::DriveStrength;

/// A direct IO MUX function of a pad.
///
/// The variants name the TRM's "HP IO MUX Pin Functions" entries. One variant
/// maps to a different `MCU_SEL` value on every pad, which is why
/// [`Pin::set_mux_function`](super::Pin::set_mux_function) looks the pad up in
/// [`PAD_TABLE`] instead of taking a number.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MuxFunction {
    /// The GPIO function: the pad is driven by the GPIO matrix (`MCU_SEL = 1`
    /// on every pad).
    Gpio,
    /// JTAG TCK.
    JtagTck,
    /// JTAG TDI.
    JtagTdi,
    /// JTAG TMS.
    JtagTms,
    /// JTAG TDO.
    JtagTdo,
    /// `UART0_TXD_PAD`.
    Uart0Txd,
    /// `UART0_RXD_PAD`.
    Uart0Rxd,
    /// `UART0_RTS_PAD`.
    Uart0Rts,
    /// `UART0_CTS_PAD`.
    Uart0Cts,
    /// `UART1_TXD_PAD`.
    Uart1Txd,
    /// `UART1_RXD_PAD`.
    Uart1Rxd,
    /// `UART1_RTS_PAD`.
    Uart1Rts,
    /// `UART1_CTS_PAD`.
    Uart1Cts,
    /// `SPI2_CK_PAD`.
    Spi2Ck,
    /// `SPI2_CS_PAD`.
    Spi2Cs,
    /// `SPI2_D_PAD`.
    Spi2D,
    /// `SPI2_Q_PAD`.
    Spi2Q,
    /// `SPI2_HOLD_PAD`.
    Spi2Hold,
    /// `SPI2_WP_PAD`.
    Spi2Wp,
    /// `SPI2_IO4_PAD`.
    Spi2Io4,
    /// `SPI2_IO5_PAD`.
    Spi2Io5,
    /// `SPI2_IO6_PAD`.
    Spi2Io6,
    /// `SPI2_IO7_PAD`.
    Spi2Io7,
    /// `SPI2_DQS_PAD`.
    Spi2Dqs,
    /// `SD1_CCLK_PAD`.
    Sd1Cclk,
    /// `SD1_CCMD_PAD`.
    Sd1Ccmd,
    /// `SD1_CDATA0_PAD`.
    Sd1Cdata0,
    /// `SD1_CDATA1_PAD`.
    Sd1Cdata1,
    /// `SD1_CDATA2_PAD`.
    Sd1Cdata2,
    /// `SD1_CDATA3_PAD`.
    Sd1Cdata3,
    /// `SD1_CDATA4_PAD`.
    Sd1Cdata4,
    /// `SD1_CDATA5_PAD`.
    Sd1Cdata5,
    /// `SD1_CDATA6_PAD`.
    Sd1Cdata6,
    /// `SD1_CDATA7_PAD`.
    Sd1Cdata7,
    /// `GMAC_PHY_RXDV_PAD`.
    GmacPhyRxdv,
    /// `GMAC_PHY_RXD0_PAD`.
    GmacPhyRxd0,
    /// `GMAC_PHY_RXD1_PAD`.
    GmacPhyRxd1,
    /// `GMAC_PHY_RXER_PAD`.
    GmacPhyRxer,
    /// `GMAC_RMII_CLK_PAD`.
    GmacRmiiClk,
    /// `GMAC_PHY_TXEN_PAD`.
    GmacPhyTxen,
    /// `GMAC_PHY_TXD0_PAD`.
    GmacPhyTxd0,
    /// `GMAC_PHY_TXD1_PAD`.
    GmacPhyTxd1,
    /// `GMAC_PHY_TXER_PAD`.
    GmacPhyTxer,
    /// `REF_50M_CLK_PAD`.
    Ref50mClk,
}

impl MuxFunction {
    /// The name the TRM uses for this function.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Gpio => "GPIO",
            Self::JtagTck => "MTCK",
            Self::JtagTdi => "MTDI",
            Self::JtagTms => "MTMS",
            Self::JtagTdo => "MTDO",
            Self::Uart0Txd => "UART0_TXD_PAD",
            Self::Uart0Rxd => "UART0_RXD_PAD",
            Self::Uart0Rts => "UART0_RTS_PAD",
            Self::Uart0Cts => "UART0_CTS_PAD",
            Self::Uart1Txd => "UART1_TXD_PAD",
            Self::Uart1Rxd => "UART1_RXD_PAD",
            Self::Uart1Rts => "UART1_RTS_PAD",
            Self::Uart1Cts => "UART1_CTS_PAD",
            Self::Spi2Ck => "SPI2_CK_PAD",
            Self::Spi2Cs => "SPI2_CS_PAD",
            Self::Spi2D => "SPI2_D_PAD",
            Self::Spi2Q => "SPI2_Q_PAD",
            Self::Spi2Hold => "SPI2_HOLD_PAD",
            Self::Spi2Wp => "SPI2_WP_PAD",
            Self::Spi2Io4 => "SPI2_IO4_PAD",
            Self::Spi2Io5 => "SPI2_IO5_PAD",
            Self::Spi2Io6 => "SPI2_IO6_PAD",
            Self::Spi2Io7 => "SPI2_IO7_PAD",
            Self::Spi2Dqs => "SPI2_DQS_PAD",
            Self::Sd1Cclk => "SD1_CCLK_PAD",
            Self::Sd1Ccmd => "SD1_CCMD_PAD",
            Self::Sd1Cdata0 => "SD1_CDATA0_PAD",
            Self::Sd1Cdata1 => "SD1_CDATA1_PAD",
            Self::Sd1Cdata2 => "SD1_CDATA2_PAD",
            Self::Sd1Cdata3 => "SD1_CDATA3_PAD",
            Self::Sd1Cdata4 => "SD1_CDATA4_PAD",
            Self::Sd1Cdata5 => "SD1_CDATA5_PAD",
            Self::Sd1Cdata6 => "SD1_CDATA6_PAD",
            Self::Sd1Cdata7 => "SD1_CDATA7_PAD",
            Self::GmacPhyRxdv => "GMAC_PHY_RXDV_PAD",
            Self::GmacPhyRxd0 => "GMAC_PHY_RXD0_PAD",
            Self::GmacPhyRxd1 => "GMAC_PHY_RXD1_PAD",
            Self::GmacPhyRxer => "GMAC_PHY_RXER_PAD",
            Self::GmacRmiiClk => "GMAC_RMII_CLK_PAD",
            Self::GmacPhyTxen => "GMAC_PHY_TXEN_PAD",
            Self::GmacPhyTxd0 => "GMAC_PHY_TXD0_PAD",
            Self::GmacPhyTxd1 => "GMAC_PHY_TXD1_PAD",
            Self::GmacPhyTxer => "GMAC_PHY_TXER_PAD",
            Self::Ref50mClk => "REF_50M_CLK_PAD",
        }
    }
}

/// A pad's analog function, from the TRM's "GPIO Pin Analog Functions" table.
///
/// These do not use `MCU_SEL`: the pad is handed to the analog block by making
/// it floating (input buffer, pulls and output enable off) and leaving the
/// analog block selected.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AnalogFunction {
    /// 32 kHz crystal, negative terminal (GPIO0).
    Xtal32kN,
    /// 32 kHz crystal, positive terminal (GPIO1).
    Xtal32kP,
    /// Capacitive touch channel `n` (GPIO2-15, channels 0-13).
    Touch(u8),
    /// ADC1 channel `n` (GPIO16-23, channels 0-7).
    Adc1(u8),
    /// ADC2 channel `n` (GPIO49-54, channels 0-5).
    Adc2(u8),
    /// Analog comparator 0 input (GPIO51/52).
    AnaComp0,
    /// Analog comparator 1 input (GPIO53/54).
    AnaComp1,
    /// USB Serial/JTAG PHY, D- (GPIO24).
    Usb1p1N0,
    /// USB Serial/JTAG PHY, D+ (GPIO25).
    Usb1p1P0,
    /// USB OTG full-speed PHY, D- (GPIO26).
    Usb1p1N1,
    /// USB OTG full-speed PHY, D+ (GPIO27).
    Usb1p1P1,
}

impl AnalogFunction {
    /// The channel number of a touch/ADC function.
    pub const fn channel(self) -> Option<u8> {
        match self {
            Self::Touch(channel) | Self::Adc1(channel) | Self::Adc2(channel) => Some(channel),
            _ => None,
        }
    }

    /// The name the TRM uses for this function.
    pub const fn name(self) -> &'static str {
        match self {
            Self::Xtal32kN => "XTAL_32K_N",
            Self::Xtal32kP => "XTAL_32K_P",
            Self::Touch(_) => "TOUCH_CHANNEL",
            Self::Adc1(_) => "ADC1_CHANNEL",
            Self::Adc2(_) => "ADC2_CHANNEL",
            Self::AnaComp0 => "ANA_COMP0",
            Self::AnaComp1 => "ANA_COMP1",
            Self::Usb1p1N0 => "USB1P1_N0",
            Self::Usb1p1P0 => "USB1P1_P0",
            Self::Usb1p1N1 => "USB1P1_N1",
            Self::Usb1p1P1 => "USB1P1_P1",
        }
    }
}

/// The pads of the USB Serial/JTAG PHY: D- on GPIO24, D+ on GPIO25.
///
/// The USB data lines are not GPIO-matrix signals; the pads are switched to the
/// PHY with IO MUX function 0.
pub const USB_SERIAL_JTAG_PADS: (u32, u32) = (24, 25);

/// The pads of the USB OTG full-speed PHY: D- on GPIO26, D+ on GPIO27.
///
/// Like the Serial/JTAG pair, these are fixed pads reached through IO MUX
/// function 0, not matrix signals.
pub const USB_OTG_FS_PADS: (u32, u32) = (26, 27);

/// Everything the HAL knows about one pad.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PadDesc {
    /// GPIO number.
    pub number: u32,
    /// Whether the low-power domain can reach this pad, i.e. whether it is part
    /// of the LP GPIO matrix (GPIO0-15).
    pub lp: bool,
    /// Drive strength the pad resets to, from the TRM's `DRV` column.
    pub reset_drive_strength: DriveStrength,
    /// Direct IO MUX functions 0-3, indexed by [`Function`](super::Function).
    ///
    /// Index 1 is always [`MuxFunction::Gpio`]; a `None` entry means the pad
    /// has no function in that slot.
    functions: [Option<MuxFunction>; 4],
    /// Analog functions, in the TRM's column order.
    analog: [Option<AnalogFunction>; 2],
}

impl PadDesc {
    /// The direct function in slot `function`.
    #[inline]
    pub const fn function(&self, function: super::Function) -> Option<MuxFunction> {
        self.functions[function as usize]
    }

    /// The `MCU_SEL` value of `function` on this pad, or `None` if the pad does
    /// not expose it.
    pub const fn function_index(&self, function: MuxFunction) -> Option<u32> {
        // The GPIO function is always slot 1, even on the pads whose slot 0 is
        // also labelled GPIO.
        if matches!(function, MuxFunction::Gpio) {
            return Some(1);
        }
        let mut index = 0;
        while index < self.functions.len() {
            match self.functions[index] {
                Some(candidate) if candidate as u32 == function as u32 => {
                    return Some(index as u32);
                }
                _ => index += 1,
            }
        }
        None
    }

    /// The direct functions available on this pad.
    #[inline]
    pub const fn functions(&self) -> &[Option<MuxFunction>; 4] {
        &self.functions
    }

    /// The analog functions available on this pad.
    #[inline]
    pub const fn analog(&self) -> &[Option<AnalogFunction>; 2] {
        &self.analog
    }

    /// Whether this pad belongs to a USB PHY.
    ///
    /// USB data lines are not GPIO-matrix signals: the pad must be switched to
    /// its analog function.
    pub const fn is_usb_phy(&self) -> bool {
        matches!(
            self.analog[0],
            Some(
                AnalogFunction::Usb1p1N0
                    | AnalogFunction::Usb1p1P0
                    | AnalogFunction::Usb1p1N1
                    | AnalogFunction::Usb1p1P1
            )
        )
    }

    /// Whether this pad carries one of the dedicated GMAC PHY/RMII signals.
    ///
    /// Those signals have no GPIO-matrix entry at all, so the MAC can only use
    /// the pads whose IO MUX exposes the function.
    pub const fn is_gmac_phy(&self) -> bool {
        let mut index = 0;
        while index < self.functions.len() {
            match self.functions[index] {
                Some(function) if is_gmac_function(function) => return true,
                _ => index += 1,
            }
        }
        false
    }
}

/// Whether `function` is one of the GMAC PHY/RMII pads.
const fn is_gmac_function(function: MuxFunction) -> bool {
    matches!(
        function,
        MuxFunction::GmacPhyRxdv
            | MuxFunction::GmacPhyRxd0
            | MuxFunction::GmacPhyRxd1
            | MuxFunction::GmacPhyRxer
            | MuxFunction::GmacRmiiClk
            | MuxFunction::GmacPhyTxen
            | MuxFunction::GmacPhyTxd0
            | MuxFunction::GmacPhyTxd1
            | MuxFunction::GmacPhyTxer
    )
}

/// Every pad with an IO MUX register, indexed by GPIO number.
pub const PAD_TABLE: [PadDesc; 54] = [
    // 0
    PadDesc {
        number: 0,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [Some(MuxFunction::Gpio), Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Xtal32kN), None],
    },
    // 1
    PadDesc {
        number: 1,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [Some(MuxFunction::Gpio), Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Xtal32kP), None],
    },
    // 2
    PadDesc {
        number: 2,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::JtagTck),
            Some(MuxFunction::Gpio),
            None,
            None,
        ],
        analog: [Some(AnalogFunction::Touch(0)), None],
    },
    // 3
    PadDesc {
        number: 3,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::JtagTdi),
            Some(MuxFunction::Gpio),
            None,
            None,
        ],
        analog: [Some(AnalogFunction::Touch(1)), None],
    },
    // 4
    PadDesc {
        number: 4,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::JtagTms),
            Some(MuxFunction::Gpio),
            None,
            None,
        ],
        analog: [Some(AnalogFunction::Touch(2)), None],
    },
    // 5
    PadDesc {
        number: 5,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::JtagTdo),
            Some(MuxFunction::Gpio),
            None,
            None,
        ],
        analog: [Some(AnalogFunction::Touch(3)), None],
    },
    // 6
    PadDesc {
        number: 6,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2Hold),
        ],
        analog: [Some(AnalogFunction::Touch(4)), None],
    },
    // 7
    PadDesc {
        number: 7,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2Cs),
        ],
        analog: [Some(AnalogFunction::Touch(5)), None],
    },
    // 8
    PadDesc {
        number: 8,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart0Rts),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2D),
        ],
        analog: [Some(AnalogFunction::Touch(6)), None],
    },
    // 9
    PadDesc {
        number: 9,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart0Cts),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2Ck),
        ],
        analog: [Some(AnalogFunction::Touch(7)), None],
    },
    // 10
    PadDesc {
        number: 10,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart1Txd),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2Q),
        ],
        analog: [Some(AnalogFunction::Touch(8)), None],
    },
    // 11
    PadDesc {
        number: 11,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart1Rxd),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2Wp),
        ],
        analog: [Some(AnalogFunction::Touch(9)), None],
    },
    // 12
    PadDesc {
        number: 12,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart1Rts),
            Some(MuxFunction::Gpio),
            None,
            None,
        ],
        analog: [Some(AnalogFunction::Touch(10)), None],
    },
    // 13
    PadDesc {
        number: 13,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart1Cts),
            Some(MuxFunction::Gpio),
            None,
            None,
        ],
        analog: [Some(AnalogFunction::Touch(11)), None],
    },
    // 14
    PadDesc {
        number: 14,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Touch(12)), None],
    },
    // 15
    PadDesc {
        number: 15,
        lp: true,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Touch(13)), None],
    },
    // 16
    PadDesc {
        number: 16,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Adc1(0)), None],
    },
    // 17
    PadDesc {
        number: 17,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Adc1(1)), None],
    },
    // 18
    PadDesc {
        number: 18,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Adc1(2)), None],
    },
    // 19
    PadDesc {
        number: 19,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Adc1(3)), None],
    },
    // 20
    PadDesc {
        number: 20,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Adc1(4)), None],
    },
    // 21
    PadDesc {
        number: 21,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Adc1(5)), None],
    },
    // 22
    PadDesc {
        number: 22,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Adc1(6)), None],
    },
    // 23
    PadDesc {
        number: 23,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Ref50mClk),
        ],
        analog: [Some(AnalogFunction::Adc1(7)), None],
    },
    // 24
    PadDesc {
        number: 24,
        lp: false,
        reset_drive_strength: DriveStrength::_40mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Usb1p1N0), None],
    },
    // 25
    PadDesc {
        number: 25,
        lp: false,
        reset_drive_strength: DriveStrength::_40mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Usb1p1P0), None],
    },
    // 26
    PadDesc {
        number: 26,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Usb1p1N1), None],
    },
    // 27
    PadDesc {
        number: 27,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [None, Some(MuxFunction::Gpio), None, None],
        analog: [Some(AnalogFunction::Usb1p1P1), None],
    },
    // 28
    PadDesc {
        number: 28,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Cs),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxdv),
        ],
        analog: [None, None],
    },
    // 29
    PadDesc {
        number: 29,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2D),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxd0),
        ],
        analog: [None, None],
    },
    // 30
    PadDesc {
        number: 30,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Ck),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxd1),
        ],
        analog: [None, None],
    },
    // 31
    PadDesc {
        number: 31,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Q),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxer),
        ],
        analog: [None, None],
    },
    // 32
    PadDesc {
        number: 32,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Hold),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacRmiiClk),
        ],
        analog: [None, None],
    },
    // 33
    PadDesc {
        number: 33,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Wp),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxen),
        ],
        analog: [None, None],
    },
    // 34
    PadDesc {
        number: 34,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Io4),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxd0),
        ],
        analog: [None, None],
    },
    // 35
    PadDesc {
        number: 35,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Io5),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxd1),
        ],
        analog: [None, None],
    },
    // 36
    PadDesc {
        number: 36,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Spi2Io6),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxer),
        ],
        analog: [None, None],
    },
    // 37
    PadDesc {
        number: 37,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart0Txd),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2Io7),
        ],
        analog: [None, None],
    },
    // 38
    PadDesc {
        number: 38,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Uart0Rxd),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Spi2Dqs),
        ],
        analog: [None, None],
    },
    // 39
    PadDesc {
        number: 39,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata0),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::Ref50mClk),
        ],
        analog: [None, None],
    },
    // 40
    PadDesc {
        number: 40,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata1),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxen),
        ],
        analog: [None, None],
    },
    // 41
    PadDesc {
        number: 41,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata2),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxd0),
        ],
        analog: [None, None],
    },
    // 42
    PadDesc {
        number: 42,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata3),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxd1),
        ],
        analog: [None, None],
    },
    // 43
    PadDesc {
        number: 43,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cclk),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxer),
        ],
        analog: [None, None],
    },
    // 44
    PadDesc {
        number: 44,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Ccmd),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacRmiiClk),
        ],
        analog: [None, None],
    },
    // 45
    PadDesc {
        number: 45,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata4),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxdv),
        ],
        analog: [None, None],
    },
    // 46
    PadDesc {
        number: 46,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata5),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxd0),
        ],
        analog: [None, None],
    },
    // 47
    PadDesc {
        number: 47,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata6),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxd1),
        ],
        analog: [None, None],
    },
    // 48
    PadDesc {
        number: 48,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            Some(MuxFunction::Sd1Cdata7),
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxer),
        ],
        analog: [None, None],
    },
    // 49
    PadDesc {
        number: 49,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyTxen),
        ],
        analog: [Some(AnalogFunction::Adc2(0)), None],
    },
    // 50
    PadDesc {
        number: 50,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacRmiiClk),
        ],
        analog: [Some(AnalogFunction::Adc2(1)), None],
    },
    // 51
    PadDesc {
        number: 51,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxdv),
        ],
        analog: [
            Some(AnalogFunction::Adc2(2)),
            Some(AnalogFunction::AnaComp0),
        ],
    },
    // 52
    PadDesc {
        number: 52,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxd0),
        ],
        analog: [
            Some(AnalogFunction::Adc2(3)),
            Some(AnalogFunction::AnaComp0),
        ],
    },
    // 53
    PadDesc {
        number: 53,
        lp: false,
        reset_drive_strength: DriveStrength::_20mA,
        functions: [
            None,
            Some(MuxFunction::Gpio),
            None,
            Some(MuxFunction::GmacPhyRxd1),
        ],
        analog: [
            Some(AnalogFunction::Adc2(4)),
            Some(AnalogFunction::AnaComp1),
        ],
    },
];

/// The pads the low-power domain can reach.
pub const LP_PINS: [u32; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

/// Describes pad `number`, or returns `None` for GPIO54 and above.
#[inline]
pub const fn pad(number: u32) -> Option<&'static PadDesc> {
    if number < PAD_TABLE.len() as u32 {
        Some(&PAD_TABLE[number as usize])
    } else {
        None
    }
}

/// Whether pad `number` exposes `function` directly through the IO MUX.
///
/// Returns `false` for pads without an `IO_MUX` register (GPIO54 and above).
/// Usable in a `const` block, so a driver can reject a wrong pin at compile
/// time:
///
/// ```ignore
/// const { assert!(pad_has(33, MuxFunction::GmacPhyTxen)) };
/// ```
#[inline]
pub const fn pad_has(number: u32, function: MuxFunction) -> bool {
    match pad(number) {
        Some(desc) => desc.function_index(function).is_some(),
        None => false,
    }
}

/// Iterator over the pads that expose `function` directly.
pub struct PadsWith(MuxFunction, usize);

impl Iterator for PadsWith {
    type Item = &'static PadDesc;

    fn next(&mut self) -> Option<Self::Item> {
        while self.1 < PAD_TABLE.len() {
            let desc = &PAD_TABLE[self.1];
            self.1 += 1;
            if desc.function_index(self.0).is_some() {
                return Some(desc);
            }
        }
        None
    }
}

/// The pads that expose `function` directly through the IO MUX.
///
/// ```ignore
/// for pad in esp32p4::gpio::pins::pads_with(MuxFunction::Uart0Txd) {
///     // GPIO37 is the only pad with a direct UART0 TXD function
/// }
/// ```
#[inline]
pub const fn pads_with(function: MuxFunction) -> PadsWith {
    PadsWith(function, 0)
}
