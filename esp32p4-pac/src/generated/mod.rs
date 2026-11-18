//! ESP32-P4 Hardware Register Definitions
//!
//! Auto-generated from esp32p4.svd - DO NOT EDIT MANUALLY

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(clippy::identity_op)]

use core::marker::PhantomData;
use core::ops::Deref;

pub mod common;

/// ESP32-P4 interrupt sources.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    /// 1 - interrupt source
    LP_WDT = 1,
    /// 2 - interrupt source
    LP_TIMER0 = 2,
    /// 3 - interrupt source
    LP_TIMER1 = 3,
    /// 6 - interrupt source
    PMU0 = 6,
    /// 7 - interrupt source
    PMU1 = 7,
    /// 8 - interrupt source
    LP_ANA = 8,
    /// 9 - interrupt source
    LP_ADC = 9,
    /// 10 - interrupt source
    LP_GPIO = 10,
    /// 11 - interrupt source
    LP_I2C0 = 11,
    /// 12 - interrupt source
    LP_I2S0 = 12,
    /// 14 - interrupt source
    LP_TOUCH = 14,
    /// 15 - interrupt source
    LP_TSENS = 15,
    /// 16 - interrupt source
    LP_UART = 16,
    /// 19 - interrupt source
    LP_SYS = 19,
    /// 20 - interrupt source
    LP_HUK = 20,
    /// 22 - interrupt source
    USB_DEVICE = 22,
    /// 24 - interrupt source
    DMA = 24,
    /// 25 - interrupt source
    SPI2 = 25,
    /// 26 - interrupt source
    SPI3 = 26,
    /// 27 - interrupt source
    I2S0 = 27,
    /// 28 - interrupt source
    I2S1 = 28,
    /// 29 - interrupt source
    I2S2 = 29,
    /// 30 - interrupt source
    UHCI0 = 30,
    /// 31 - interrupt source
    UART0 = 31,
    /// 32 - interrupt source
    UART1 = 32,
    /// 33 - interrupt source
    UART2 = 33,
    /// 34 - interrupt source
    UART3 = 34,
    /// 35 - interrupt source
    UART4 = 35,
    /// 38 - interrupt source
    PWM0 = 38,
    /// 39 - interrupt source
    PWM1 = 39,
    /// 40 - interrupt source
    TWAI0 = 40,
    /// 41 - interrupt source
    TWAI1 = 41,
    /// 42 - interrupt source
    TWAI2 = 42,
    /// 43 - interrupt source
    RMT = 43,
    /// 44 - interrupt source
    I2C0 = 44,
    /// 45 - interrupt source
    I2C1 = 45,
    /// 46 - interrupt source
    TG0_T0 = 46,
    /// 47 - interrupt source
    TG0_T1 = 47,
    /// 48 - interrupt source
    TG0_WDT = 48,
    /// 49 - interrupt source
    TG1_T0 = 49,
    /// 50 - interrupt source
    TG1_T1 = 50,
    /// 51 - interrupt source
    TG1_WDT = 51,
    /// 52 - interrupt source
    LEDC = 52,
    /// 53 - interrupt source
    SYSTIMER_TARGET0 = 53,
    /// 54 - interrupt source
    SYSTIMER_TARGET1 = 54,
    /// 55 - interrupt source
    SYSTIMER_TARGET2 = 55,
    /// 56 - interrupt source
    AHB_PDMA_IN_CH0 = 56,
    /// 57 - interrupt source
    AHB_PDMA_IN_CH1 = 57,
    /// 58 - interrupt source
    AHB_PDMA_IN_CH2 = 58,
    /// 59 - interrupt source
    AHB_PDMA_OUT_CH0 = 59,
    /// 60 - interrupt source
    AHB_PDMA_OUT_CH1 = 60,
    /// 61 - interrupt source
    AHB_PDMA_OUT_CH2 = 61,
    /// 62 - interrupt source
    AXI_PDMA_IN_CH0 = 62,
    /// 63 - interrupt source
    AXI_PDMA_IN_CH1 = 63,
    /// 64 - interrupt source
    AXI_PDMA_IN_CH2 = 64,
    /// 65 - interrupt source
    AXI_PDMA_OUT_CH0 = 65,
    /// 66 - interrupt source
    AXI_PDMA_OUT_CH1 = 66,
    /// 67 - interrupt source
    AXI_PDMA_OUT_CH2 = 67,
    /// 68 - interrupt source
    RSA = 68,
    /// 69 - interrupt source
    AES = 69,
    /// 70 - interrupt source
    SHA = 70,
    /// 71 - interrupt source
    ECC = 71,
    /// 74 - interrupt source
    GPIO_INT0 = 74,
    /// 75 - interrupt source
    GPIO_INT1 = 75,
    /// 76 - interrupt source
    GPIO_INT2 = 76,
    /// 77 - interrupt source
    GPIO_INT3 = 77,
    /// 78 - interrupt source
    GPIO_PAD_COMP = 78,
    /// 83 - interrupt source
    CACHE = 83,
    /// 85 - interrupt source
    CSI_BRIDGE = 85,
    /// 86 - interrupt source
    DSI_BRIDGE = 86,
    /// 87 - interrupt source
    CSI = 87,
    /// 88 - interrupt source
    DSI = 88,
    /// 95 - interrupt source
    JPEG = 95,
    /// 96 - interrupt source
    PPA = 96,
    /// 100 - interrupt source
    ISP = 100,
    /// 101 - interrupt source
    I3C = 101,
    /// 102 - interrupt source
    I3C_SLV = 102,
    /// 110 - interrupt source
    HP_SYS = 110,
    /// 111 - interrupt source
    PCNT = 111,
    /// 112 - interrupt source
    PAU = 112,
    /// 113 - interrupt source
    PARLIO_RX = 113,
    /// 114 - interrupt source
    PARLIO_TX = 114,
    /// 115 - interrupt source
    H264_DMA2D_OUT_CH0 = 115,
    /// 116 - interrupt source
    H264_DMA2D_OUT_CH1 = 116,
    /// 117 - interrupt source
    H264_DMA2D_OUT_CH2 = 117,
    /// 118 - interrupt source
    H264_DMA2D_OUT_CH3 = 118,
    /// 119 - interrupt source
    H264_DMA2D_OUT_CH4 = 119,
    /// 120 - interrupt source
    H264_DMA2D_IN_CH0 = 120,
    /// 121 - interrupt source
    H264_DMA2D_IN_CH1 = 121,
    /// 122 - interrupt source
    H264_DMA2D_IN_CH2 = 122,
    /// 123 - interrupt source
    H264_DMA2D_IN_CH3 = 123,
    /// 124 - interrupt source
    H264_DMA2D_IN_CH4 = 124,
    /// 125 - interrupt source
    H264_DMA2D_IN_CH5 = 125,
    /// 126 - interrupt source
    H264_REG = 126,
    /// 127 - interrupt source
    ASSIST_DEBUG = 127,
}

impl Interrupt {
    /// Raw interrupt-matrix source number.
    #[inline(always)]
    pub const fn number(self) -> u16 {
        self as u16
    }
}

// ============================================================================
// Peripheral Wrapper Macro
// ============================================================================

/// Helper macro to create peripheral wrappers
macro_rules! peripheral {
    ($name:ident, $reg:ty, $base:expr) => {
        impl $name {
            /// Pointer to the peripheral's register block
            pub const PTR: *const $reg = $base as *const $reg;

            /// Steal an instance of this peripheral
            ///
            /// # Safety
            ///
            /// Ensure that the new instance of the peripheral cannot be used in a way
            /// that may race with any existing instances.
            #[inline]
            pub const unsafe fn steal() -> Self {
                Self {
                    _marker: PhantomData,
                }
            }
        }

        impl Deref for $name {
            type Target = $reg;

            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                unsafe { &*Self::PTR }
            }
        }

        impl core::fmt::Debug for $name {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.debug_struct(stringify!($name)).finish()
            }
        }
    };
}

// ============================================================================
// Peripheral Definitions
// ============================================================================

/// ADC (Analog to Digital Converter)
pub struct ADC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ADC {}

peripheral!(ADC, adc::RegisterBlock, 0x500de000);

/// ADC (Analog to Digital Converter)
pub mod adc;

/// AES (Advanced Encryption Standard) Accelerator
pub struct AES {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for AES {}

peripheral!(AES, aes::RegisterBlock, 0x50090000);

/// AES (Advanced Encryption Standard) Accelerator
pub mod aes;

/// AHB_DMA Peripheral
pub struct AHB_DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for AHB_DMA {}

peripheral!(AHB_DMA, ahb_dma::RegisterBlock, 0x50085000);

/// AHB_DMA Peripheral
pub mod ahb_dma;

/// LP_I2C_ANA_MST Peripheral
pub struct LP_I2C_ANA_MST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_I2C_ANA_MST {}

peripheral!(LP_I2C_ANA_MST, lp_i2c_ana_mst::RegisterBlock, 0x50124000);

/// LP_I2C_ANA_MST Peripheral
pub mod lp_i2c_ana_mst;

/// Debug Assist
pub struct ASSIST_DEBUG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ASSIST_DEBUG {}

peripheral!(ASSIST_DEBUG, assist_debug::RegisterBlock, 0x3ff06000);

/// Debug Assist
pub mod assist_debug;

/// AXI_DMA Peripheral
pub struct AXI_DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for AXI_DMA {}

peripheral!(AXI_DMA, axi_dma::RegisterBlock, 0x5008a000);

/// AXI_DMA Peripheral
pub mod axi_dma;

/// BITSCRAMBLER Peripheral
pub struct BITSCRAMBLER {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for BITSCRAMBLER {}

peripheral!(BITSCRAMBLER, bitscrambler::RegisterBlock, 0x500a3000);

/// BITSCRAMBLER Peripheral
pub mod bitscrambler;

/// CACHE Peripheral
pub struct CACHE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for CACHE {}

peripheral!(CACHE, cache::RegisterBlock, 0x3ff10000);

/// CACHE Peripheral
pub mod cache;

/// Interrupt Controller (Core 0)
pub struct INTERRUPT_CORE0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for INTERRUPT_CORE0 {}

peripheral!(INTERRUPT_CORE0, interrupt_core0::RegisterBlock, 0x500d6000);

/// Interrupt Controller (Core 0)
pub mod interrupt_core0;

/// Interrupt Controller (Core 1)
pub struct INTERRUPT_CORE1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for INTERRUPT_CORE1 {}

peripheral!(INTERRUPT_CORE1, interrupt_core1::RegisterBlock, 0x500d6800);

/// Interrupt Controller (Core 1)
pub mod interrupt_core1;

/// MIPI Camera Interface Bridge
pub struct MIPI_CSI_BRIDGE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MIPI_CSI_BRIDGE {}

peripheral!(MIPI_CSI_BRIDGE, mipi_csi_bridge::RegisterBlock, 0x5009f800);

/// MIPI Camera Interface Bridge
pub mod mipi_csi_bridge;

/// MIPI Camera Interface Host
pub struct MIPI_CSI_HOST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MIPI_CSI_HOST {}

peripheral!(MIPI_CSI_HOST, mipi_csi_host::RegisterBlock, 0x5009f000);

/// MIPI Camera Interface Host
pub mod mipi_csi_host;

/// DMA (Direct Memory Access) Controller
pub struct DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DMA {}

peripheral!(DMA, dma::RegisterBlock, 0x50081000);

/// DMA (Direct Memory Access) Controller
pub mod dma;

/// Digital Signature
pub struct DS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for DS {}

peripheral!(DS, ds::RegisterBlock, 0x50094000);

/// Digital Signature
pub mod ds;

/// MIPI Camera Interface Bridge
pub struct MIPI_DSI_BRIDGE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MIPI_DSI_BRIDGE {}

peripheral!(MIPI_DSI_BRIDGE, mipi_dsi_bridge::RegisterBlock, 0x500a0800);

/// MIPI Camera Interface Bridge
pub mod mipi_dsi_bridge;

/// MIPI Display Interface Host
pub struct MIPI_DSI_HOST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MIPI_DSI_HOST {}

peripheral!(MIPI_DSI_HOST, mipi_dsi_host::RegisterBlock, 0x500a0000);

/// MIPI Display Interface Host
pub mod mipi_dsi_host;

/// ECC (ECC Hardware Accelerator)
pub struct ECC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ECC {}

peripheral!(ECC, ecc::RegisterBlock, 0x50093000);

/// ECC (ECC Hardware Accelerator)
pub mod ecc;

/// ECDSA (Elliptic Curve Digital Signature Algorithm) Accelerator
pub struct ECDSA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ECDSA {}

peripheral!(ECDSA, ecdsa::RegisterBlock, 0x50096000);

/// ECDSA (Elliptic Curve Digital Signature Algorithm) Accelerator
pub mod ecdsa;

/// eFuse Controller
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for EFUSE {}

peripheral!(EFUSE, efuse::RegisterBlock, 0x5012d000);

/// eFuse Controller
pub mod efuse;

/// General Purpose Input/Output
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for GPIO {}

peripheral!(GPIO, gpio::RegisterBlock, 0x500e0000);

/// General Purpose Input/Output
pub mod gpio;

/// Sigma-Delta Modulation
pub struct GPIO_SD {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for GPIO_SD {}

peripheral!(GPIO_SD, gpio_sd::RegisterBlock, 0x500e0f00);

/// Sigma-Delta Modulation
pub mod gpio_sd;

/// H264 Encoder (Core)
pub struct H264 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for H264 {}

peripheral!(H264, h264::RegisterBlock, 0x50084000);

/// H264 Encoder (Core)
pub mod h264;

/// H264 Encoder (DMA)
pub struct H264_DMA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for H264_DMA {}

peripheral!(H264_DMA, h264_dma::RegisterBlock, 0x500a7000);

/// H264 Encoder (DMA)
pub mod h264_dma;

/// HMAC (Hash-based Message Authentication Code) Accelerator
pub struct HMAC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HMAC {}

peripheral!(HMAC, hmac::RegisterBlock, 0x50095000);

/// HMAC (Hash-based Message Authentication Code) Accelerator
pub mod hmac;

/// High-Power System
pub struct HP_SYS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HP_SYS {}

peripheral!(HP_SYS, hp_sys::RegisterBlock, 0x500e5000);

/// High-Power System
pub mod hp_sys;

/// HP_SYS_CLKRST Peripheral
pub struct HP_SYS_CLKRST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for HP_SYS_CLKRST {}

peripheral!(HP_SYS_CLKRST, hp_sys_clkrst::RegisterBlock, 0x500e6000);

/// HP_SYS_CLKRST Peripheral
pub mod hp_sys_clkrst;

/// LP_HUK Peripheral
pub struct LP_HUK {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_HUK {}

peripheral!(LP_HUK, lp_huk::RegisterBlock, 0x50114000);

/// LP_HUK Peripheral
pub mod lp_huk;

/// I2C (Inter-Integrated Circuit) Controller 0
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2C0 {}

peripheral!(I2C0, i2c0::RegisterBlock, 0x500c4000);

/// I2C (Inter-Integrated Circuit) Controller 0
pub mod i2c0;

/// I2C (Inter-Integrated Circuit) Controller 1
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2C1 {}

peripheral!(I2C1, i2c1::RegisterBlock, 0x500c5000);

/// I2C (Inter-Integrated Circuit) Controller 1
pub mod i2c1;

/// I2S (Inter-IC Sound) Controller 0
pub struct I2S0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2S0 {}

peripheral!(I2S0, i2s0::RegisterBlock, 0x500c6000);

/// I2S (Inter-IC Sound) Controller 0
pub mod i2s0;

/// I2S (Inter-IC Sound) Controller 1
pub struct I2S1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2S1 {}

peripheral!(I2S1, i2s1::RegisterBlock, 0x500c7000);

/// I2S (Inter-IC Sound) Controller 1
pub mod i2s1;

/// I2S (Inter-IC Sound) Controller 2
pub struct I2S2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I2S2 {}

peripheral!(I2S2, i2s2::RegisterBlock, 0x500c8000);

/// I2S (Inter-IC Sound) Controller 2
pub mod i2s2;

/// I3C Controller (Master)
pub struct I3C_MST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I3C_MST {}

peripheral!(I3C_MST, i3c_mst::RegisterBlock, 0x500da000);

/// I3C Controller (Master)
pub mod i3c_mst;

/// I3C_MST_MEM Peripheral
pub struct I3C_MST_MEM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I3C_MST_MEM {}

peripheral!(I3C_MST_MEM, i3c_mst_mem::RegisterBlock, 0x500da000);

/// I3C_MST_MEM Peripheral
pub mod i3c_mst_mem;

/// I3C Controller (Slave)
pub struct I3C_SLV {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for I3C_SLV {}

peripheral!(I3C_SLV, i3c_slv::RegisterBlock, 0x500db000);

/// I3C Controller (Slave)
pub mod i3c_slv;

/// AXI_ICM Peripheral
pub struct AXI_ICM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for AXI_ICM {}

peripheral!(AXI_ICM, axi_icm::RegisterBlock, 0x500a4000);

/// AXI_ICM Peripheral
pub mod axi_icm;

/// Input/Output Multiplexer
pub struct IO_MUX {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for IO_MUX {}

peripheral!(IO_MUX, io_mux::RegisterBlock, 0x500e1000);

/// Input/Output Multiplexer
pub mod io_mux;

/// ISP Peripheral
pub struct ISP {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for ISP {}

peripheral!(ISP, isp::RegisterBlock, 0x500a1000);

/// ISP Peripheral
pub mod isp;

/// JPEG Codec
pub struct JPEG {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for JPEG {}

peripheral!(JPEG, jpeg::RegisterBlock, 0x50086000);

/// JPEG Codec
pub mod jpeg;

/// Camera/LCD Controller
pub struct LCD_CAM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LCD_CAM {}

peripheral!(LCD_CAM, lcd_cam::RegisterBlock, 0x500dc000);

/// Camera/LCD Controller
pub mod lcd_cam;

/// LED Control PWM (Pulse Width Modulation)
pub struct LEDC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LEDC {}

peripheral!(LEDC, ledc::RegisterBlock, 0x500d3000);

/// LED Control PWM (Pulse Width Modulation)
pub mod ledc;

/// Low-power Interrupt Controller
pub struct LP_INTR {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_INTR {}

peripheral!(LP_INTR, lp_intr::RegisterBlock, 0x5012c000);

/// Low-power Interrupt Controller
pub mod lp_intr;

/// LP_PERI Peripheral
pub struct LP_PERI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_PERI {}

peripheral!(LP_PERI, lp_peri::RegisterBlock, 0x50120000);

/// LP_PERI Peripheral
pub mod lp_peri;

/// LP_SYS Peripheral
pub struct LP_SYS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_SYS {}

peripheral!(LP_SYS, lp_sys::RegisterBlock, 0x50110000);

/// LP_SYS Peripheral
pub mod lp_sys;

/// LP_ANA_PERI Peripheral
pub struct LP_ANA_PERI {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_ANA_PERI {}

peripheral!(LP_ANA_PERI, lp_ana_peri::RegisterBlock, 0x50113000);

/// LP_ANA_PERI Peripheral
pub mod lp_ana_peri;

/// LP_AON_CLKRST Peripheral
pub struct LP_AON_CLKRST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_AON_CLKRST {}

peripheral!(LP_AON_CLKRST, lp_aon_clkrst::RegisterBlock, 0x50111000);

/// LP_AON_CLKRST Peripheral
pub mod lp_aon_clkrst;

/// Low-power General Purpose Input/Output
pub struct LP_GPIO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_GPIO {}

peripheral!(LP_GPIO, lp_gpio::RegisterBlock, 0x5012a000);

/// Low-power General Purpose Input/Output
pub mod lp_gpio;

/// Low-power I2C (Inter-Integrated Circuit) Controller 0
pub struct LP_I2C0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_I2C0 {}

peripheral!(LP_I2C0, lp_i2c0::RegisterBlock, 0x50122000);

/// Low-power I2C (Inter-Integrated Circuit) Controller 0
pub mod lp_i2c0;

/// Low-power I2S (Inter-IC Sound) Controller 0
pub struct LP_I2S0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_I2S0 {}

peripheral!(LP_I2S0, lp_i2s0::RegisterBlock, 0x50125000);

/// Low-power I2S (Inter-IC Sound) Controller 0
pub mod lp_i2s0;

/// Low-power Input/Output Multiplexer
pub struct LP_IO_MUX {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_IO_MUX {}

peripheral!(LP_IO_MUX, lp_io_mux::RegisterBlock, 0x5012b000);

/// Low-power Input/Output Multiplexer
pub mod lp_io_mux;

/// Low-power UART (Universal Asynchronous Receiver-Transmitter) Controller
pub struct LP_UART {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_UART {}

peripheral!(LP_UART, lp_uart::RegisterBlock, 0x50121000);

/// Low-power UART (Universal Asynchronous Receiver-Transmitter) Controller
pub mod lp_uart;

/// Motor Control Pulse-Width Modulation 0
pub struct MCPWM0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MCPWM0 {}

peripheral!(MCPWM0, mcpwm0::RegisterBlock, 0x500c0000);

/// Motor Control Pulse-Width Modulation 0
pub mod mcpwm0;

/// Motor Control Pulse-Width Modulation 1
pub struct MCPWM1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for MCPWM1 {}

peripheral!(MCPWM1, mcpwm1::RegisterBlock, 0x500c1000);

/// Motor Control Pulse-Width Modulation 1
pub mod mcpwm1;

/// Parallel IO Controller
pub struct PARL_IO {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PARL_IO {}

peripheral!(PARL_IO, parl_io::RegisterBlock, 0x500cf000);

/// Parallel IO Controller
pub mod parl_io;

/// PAU Peripheral
pub struct PAU {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PAU {}

peripheral!(PAU, pau::RegisterBlock, 0x60093000);

/// PAU Peripheral
pub mod pau;

/// Pulse Count Controller
pub struct PCNT {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PCNT {}

peripheral!(PCNT, pcnt::RegisterBlock, 0x500c9000);

/// Pulse Count Controller
pub mod pcnt;

/// PMU Peripheral
pub struct PMU {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PMU {}

peripheral!(PMU, pmu::RegisterBlock, 0x50115000);

/// PMU Peripheral
pub mod pmu;

/// PPA Peripheral
pub struct PPA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PPA {}

peripheral!(PPA, ppa::RegisterBlock, 0x50087000);

/// PPA Peripheral
pub mod ppa;

/// PVT Peripheral
pub struct PVT {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for PVT {}

peripheral!(PVT, pvt::RegisterBlock, 0x5009e000);

/// PVT Peripheral
pub mod pvt;

/// Remote Control
pub struct RMT {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for RMT {}

peripheral!(RMT, rmt::RegisterBlock, 0x500d4000);

/// Remote Control
pub mod rmt;

/// RSA (Rivest Shamir Adleman) Accelerator
pub struct RSA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for RSA {}

peripheral!(RSA, rsa::RegisterBlock, 0x50092000);

/// RSA (Rivest Shamir Adleman) Accelerator
pub mod rsa;

/// Low-power Analog to Digital Converter
pub struct LP_ADC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_ADC {}

peripheral!(LP_ADC, lp_adc::RegisterBlock, 0x50127000);

/// Low-power Analog to Digital Converter
pub mod lp_adc;

/// Low-power Timer
pub struct LP_TIMER {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_TIMER {}

peripheral!(LP_TIMER, lp_timer::RegisterBlock, 0x50112000);

/// Low-power Timer
pub mod lp_timer;

/// LP_TOUCH Peripheral
pub struct LP_TOUCH {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_TOUCH {}

peripheral!(LP_TOUCH, lp_touch::RegisterBlock, 0x50128000);

/// LP_TOUCH Peripheral
pub mod lp_touch;

/// Low-power Watchdog Timer
pub struct LP_WDT {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_WDT {}

peripheral!(LP_WDT, lp_wdt::RegisterBlock, 0x50116000);

/// Low-power Watchdog Timer
pub mod lp_wdt;

/// SD/MMC Host Controller
pub struct SDHOST {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SDHOST {}

peripheral!(SDHOST, sdhost::RegisterBlock, 0x50083000);

/// SD/MMC Host Controller
pub mod sdhost;

/// SHA (Secure Hash Algorithm) Accelerator
pub struct SHA {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SHA {}

peripheral!(SHA, sha::RegisterBlock, 0x50091000);

/// SHA (Secure Hash Algorithm) Accelerator
pub mod sha;

/// Event Task Matrix
pub struct SOC_ETM {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SOC_ETM {}

peripheral!(SOC_ETM, soc_etm::RegisterBlock, 0x500d5000);

/// Event Task Matrix
pub mod soc_etm;

/// SPI (Serial Peripheral Interface) Controller 0
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI0 {}

peripheral!(SPI0, spi0::RegisterBlock, 0x5008c000);

/// SPI (Serial Peripheral Interface) Controller 0
pub mod spi0;

/// SPI (Serial Peripheral Interface) Controller 1
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI1 {}

peripheral!(SPI1, spi1::RegisterBlock, 0x5008d000);

/// SPI (Serial Peripheral Interface) Controller 1
pub mod spi1;

/// SPI (Serial Peripheral Interface) Controller 2
pub struct SPI2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI2 {}

peripheral!(SPI2, spi2::RegisterBlock, 0x500d0000);

/// SPI (Serial Peripheral Interface) Controller 2
pub mod spi2;

/// SPI (Serial Peripheral Interface) Controller 3
pub struct SPI3 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SPI3 {}

peripheral!(SPI3, spi3::RegisterBlock, 0x500d1000);

/// SPI (Serial Peripheral Interface) Controller 3
pub mod spi3;

/// System Timer
pub struct SYSTIMER {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for SYSTIMER {}

peripheral!(SYSTIMER, systimer::RegisterBlock, 0x500e2000);

/// System Timer
pub mod systimer;

/// Timer Group 0
pub struct TIMG0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIMG0 {}

peripheral!(TIMG0, timg0::RegisterBlock, 0x500c2000);

/// Timer Group 0
pub mod timg0;

/// Timer Group 1
pub struct TIMG1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TIMG1 {}

peripheral!(TIMG1, timg1::RegisterBlock, 0x500c3000);

/// Timer Group 1
pub mod timg1;

/// TRACE0 Peripheral
pub struct TRACE0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TRACE0 {}

peripheral!(TRACE0, trace0::RegisterBlock, 0x3ff04000);

/// TRACE0 Peripheral
pub mod trace0;

/// TRACE1 Peripheral
pub struct TRACE1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TRACE1 {}

peripheral!(TRACE1, trace1::RegisterBlock, 0x3ff05000);

/// TRACE1 Peripheral
pub mod trace1;

/// Low-power Temperature Sensor
pub struct LP_TSENS {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for LP_TSENS {}

peripheral!(LP_TSENS, lp_tsens::RegisterBlock, 0x5012f000);

/// Low-power Temperature Sensor
pub mod lp_tsens;

/// Two-Wire Automotive Interface
pub struct TWAI0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TWAI0 {}

peripheral!(TWAI0, twai0::RegisterBlock, 0x500d7000);

/// Two-Wire Automotive Interface
pub mod twai0;

/// Two-Wire Automotive Interface
pub struct TWAI1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TWAI1 {}

peripheral!(TWAI1, twai1::RegisterBlock, 0x500d8000);

/// Two-Wire Automotive Interface
pub mod twai1;

/// Two-Wire Automotive Interface
pub struct TWAI2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for TWAI2 {}

peripheral!(TWAI2, twai2::RegisterBlock, 0x500d9000);

/// Two-Wire Automotive Interface
pub mod twai2;

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 0
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART0 {}

peripheral!(UART0, uart0::RegisterBlock, 0x500ca000);

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 0
pub mod uart0;

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 1
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART1 {}

peripheral!(UART1, uart1::RegisterBlock, 0x500cb000);

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 1
pub mod uart1;

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 2
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART2 {}

peripheral!(UART2, uart2::RegisterBlock, 0x500cc000);

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 2
pub mod uart2;

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 3
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART3 {}

peripheral!(UART3, uart3::RegisterBlock, 0x500cd000);

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 3
pub mod uart3;

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 4
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UART4 {}

peripheral!(UART4, uart4::RegisterBlock, 0x500ce000);

/// UART (Universal Asynchronous Receiver-Transmitter) Controller 4
pub mod uart4;

/// Universal Host Controller Interface 0
pub struct UHCI0 {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for UHCI0 {}

peripheral!(UHCI0, uhci0::RegisterBlock, 0x500df000);

/// Universal Host Controller Interface 0
pub mod uhci0;

/// Full-speed USB Serial/JTAG Controller
pub struct USB_DEVICE {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USB_DEVICE {}

peripheral!(USB_DEVICE, usb_device::RegisterBlock, 0x500d2000);

/// Full-speed USB Serial/JTAG Controller
pub mod usb_device;

/// USB_WRAP Peripheral
pub struct USB_WRAP {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for USB_WRAP {}

peripheral!(USB_WRAP, usb_wrap::RegisterBlock, 0x50080000);

/// USB_WRAP Peripheral
pub mod usb_wrap;

/// Ethernet MAC (DesignWare 10/100 MAC + DMA)
pub struct EMAC {
    _marker: PhantomData<*const ()>,
}

unsafe impl Send for EMAC {}

peripheral!(EMAC, emac::RegisterBlock, 0x50098000);

/// Ethernet MAC (DesignWare 10/100 MAC + DMA)
pub mod emac;

// ============================================================================
// Peripherals - All device peripherals
// ============================================================================

use core::sync::atomic::{AtomicBool, Ordering};

/// Guard to ensure peripherals are taken only once
static TAKEN: AtomicBool = AtomicBool::new(false);

/// All the peripherals.
#[allow(non_snake_case)]
pub struct Peripherals {
    /// ADC (Analog to Digital Converter)
    pub ADC: ADC,
    /// AES (Advanced Encryption Standard) Accelerator
    pub AES: AES,
    /// AHB_DMA Peripheral
    pub AHB_DMA: AHB_DMA,
    /// LP_I2C_ANA_MST Peripheral
    pub LP_I2C_ANA_MST: LP_I2C_ANA_MST,
    /// Debug Assist
    pub ASSIST_DEBUG: ASSIST_DEBUG,
    /// AXI_DMA Peripheral
    pub AXI_DMA: AXI_DMA,
    /// BITSCRAMBLER Peripheral
    pub BITSCRAMBLER: BITSCRAMBLER,
    /// CACHE Peripheral
    pub CACHE: CACHE,
    /// Interrupt Controller (Core 0)
    pub INTERRUPT_CORE0: INTERRUPT_CORE0,
    /// Interrupt Controller (Core 1)
    pub INTERRUPT_CORE1: INTERRUPT_CORE1,
    /// MIPI Camera Interface Bridge
    pub MIPI_CSI_BRIDGE: MIPI_CSI_BRIDGE,
    /// MIPI Camera Interface Host
    pub MIPI_CSI_HOST: MIPI_CSI_HOST,
    /// DMA (Direct Memory Access) Controller
    pub DMA: DMA,
    /// Digital Signature
    pub DS: DS,
    /// MIPI Camera Interface Bridge
    pub MIPI_DSI_BRIDGE: MIPI_DSI_BRIDGE,
    /// MIPI Display Interface Host
    pub MIPI_DSI_HOST: MIPI_DSI_HOST,
    /// ECC (ECC Hardware Accelerator)
    pub ECC: ECC,
    /// ECDSA (Elliptic Curve Digital Signature Algorithm) Accelerator
    pub ECDSA: ECDSA,
    /// eFuse Controller
    pub EFUSE: EFUSE,
    /// General Purpose Input/Output
    pub GPIO: GPIO,
    /// Sigma-Delta Modulation
    pub GPIO_SD: GPIO_SD,
    /// H264 Encoder (Core)
    pub H264: H264,
    /// H264 Encoder (DMA)
    pub H264_DMA: H264_DMA,
    /// HMAC (Hash-based Message Authentication Code) Accelerator
    pub HMAC: HMAC,
    /// High-Power System
    pub HP_SYS: HP_SYS,
    /// HP_SYS_CLKRST Peripheral
    pub HP_SYS_CLKRST: HP_SYS_CLKRST,
    /// LP_HUK Peripheral
    pub LP_HUK: LP_HUK,
    /// I2C (Inter-Integrated Circuit) Controller 0
    pub I2C0: I2C0,
    /// I2C (Inter-Integrated Circuit) Controller 1
    pub I2C1: I2C1,
    /// I2S (Inter-IC Sound) Controller 0
    pub I2S0: I2S0,
    /// I2S (Inter-IC Sound) Controller 1
    pub I2S1: I2S1,
    /// I2S (Inter-IC Sound) Controller 2
    pub I2S2: I2S2,
    /// I3C Controller (Master)
    pub I3C_MST: I3C_MST,
    /// I3C_MST_MEM Peripheral
    pub I3C_MST_MEM: I3C_MST_MEM,
    /// I3C Controller (Slave)
    pub I3C_SLV: I3C_SLV,
    /// AXI_ICM Peripheral
    pub AXI_ICM: AXI_ICM,
    /// Input/Output Multiplexer
    pub IO_MUX: IO_MUX,
    /// ISP Peripheral
    pub ISP: ISP,
    /// JPEG Codec
    pub JPEG: JPEG,
    /// Camera/LCD Controller
    pub LCD_CAM: LCD_CAM,
    /// LED Control PWM (Pulse Width Modulation)
    pub LEDC: LEDC,
    /// Low-power Interrupt Controller
    pub LP_INTR: LP_INTR,
    /// LP_PERI Peripheral
    pub LP_PERI: LP_PERI,
    /// LP_SYS Peripheral
    pub LP_SYS: LP_SYS,
    /// LP_ANA_PERI Peripheral
    pub LP_ANA_PERI: LP_ANA_PERI,
    /// LP_AON_CLKRST Peripheral
    pub LP_AON_CLKRST: LP_AON_CLKRST,
    /// Low-power General Purpose Input/Output
    pub LP_GPIO: LP_GPIO,
    /// Low-power I2C (Inter-Integrated Circuit) Controller 0
    pub LP_I2C0: LP_I2C0,
    /// Low-power I2S (Inter-IC Sound) Controller 0
    pub LP_I2S0: LP_I2S0,
    /// Low-power Input/Output Multiplexer
    pub LP_IO_MUX: LP_IO_MUX,
    /// Low-power UART (Universal Asynchronous Receiver-Transmitter) Controller
    pub LP_UART: LP_UART,
    /// Motor Control Pulse-Width Modulation 0
    pub MCPWM0: MCPWM0,
    /// Motor Control Pulse-Width Modulation 1
    pub MCPWM1: MCPWM1,
    /// Parallel IO Controller
    pub PARL_IO: PARL_IO,
    /// PAU Peripheral
    pub PAU: PAU,
    /// Pulse Count Controller
    pub PCNT: PCNT,
    /// PMU Peripheral
    pub PMU: PMU,
    /// PPA Peripheral
    pub PPA: PPA,
    /// PVT Peripheral
    pub PVT: PVT,
    /// Remote Control
    pub RMT: RMT,
    /// RSA (Rivest Shamir Adleman) Accelerator
    pub RSA: RSA,
    /// Low-power Analog to Digital Converter
    pub LP_ADC: LP_ADC,
    /// Low-power Timer
    pub LP_TIMER: LP_TIMER,
    /// LP_TOUCH Peripheral
    pub LP_TOUCH: LP_TOUCH,
    /// Low-power Watchdog Timer
    pub LP_WDT: LP_WDT,
    /// SD/MMC Host Controller
    pub SDHOST: SDHOST,
    /// SHA (Secure Hash Algorithm) Accelerator
    pub SHA: SHA,
    /// Event Task Matrix
    pub SOC_ETM: SOC_ETM,
    /// SPI (Serial Peripheral Interface) Controller 0
    pub SPI0: SPI0,
    /// SPI (Serial Peripheral Interface) Controller 1
    pub SPI1: SPI1,
    /// SPI (Serial Peripheral Interface) Controller 2
    pub SPI2: SPI2,
    /// SPI (Serial Peripheral Interface) Controller 3
    pub SPI3: SPI3,
    /// System Timer
    pub SYSTIMER: SYSTIMER,
    /// Timer Group 0
    pub TIMG0: TIMG0,
    /// Timer Group 1
    pub TIMG1: TIMG1,
    /// TRACE0 Peripheral
    pub TRACE0: TRACE0,
    /// TRACE1 Peripheral
    pub TRACE1: TRACE1,
    /// Low-power Temperature Sensor
    pub LP_TSENS: LP_TSENS,
    /// Two-Wire Automotive Interface
    pub TWAI0: TWAI0,
    /// Two-Wire Automotive Interface
    pub TWAI1: TWAI1,
    /// Two-Wire Automotive Interface
    pub TWAI2: TWAI2,
    /// UART (Universal Asynchronous Receiver-Transmitter) Controller 0
    pub UART0: UART0,
    /// UART (Universal Asynchronous Receiver-Transmitter) Controller 1
    pub UART1: UART1,
    /// UART (Universal Asynchronous Receiver-Transmitter) Controller 2
    pub UART2: UART2,
    /// UART (Universal Asynchronous Receiver-Transmitter) Controller 3
    pub UART3: UART3,
    /// UART (Universal Asynchronous Receiver-Transmitter) Controller 4
    pub UART4: UART4,
    /// Universal Host Controller Interface 0
    pub UHCI0: UHCI0,
    /// Full-speed USB Serial/JTAG Controller
    pub USB_DEVICE: USB_DEVICE,
    /// USB_WRAP Peripheral
    pub USB_WRAP: USB_WRAP,
    /// Ethernet MAC (DesignWare 10/100 MAC + DMA)
    pub EMAC: EMAC,
}

impl Peripherals {
    /// Returns all the peripherals *once*.
    #[inline]
    pub fn take() -> Option<Self> {
        if TAKEN.swap(true, Ordering::AcqRel) {
            None
        } else {
            Some(unsafe { Self::steal() })
        }
    }

    /// Unchecked version of `Peripherals::take`.
    ///
    /// # Safety
    ///
    /// Each of the returned peripherals must be used at most once.
    #[inline]
    pub unsafe fn steal() -> Self {
        unsafe {
            Peripherals {
                ADC: ADC::steal(),
                AES: AES::steal(),
                AHB_DMA: AHB_DMA::steal(),
                LP_I2C_ANA_MST: LP_I2C_ANA_MST::steal(),
                ASSIST_DEBUG: ASSIST_DEBUG::steal(),
                AXI_DMA: AXI_DMA::steal(),
                BITSCRAMBLER: BITSCRAMBLER::steal(),
                CACHE: CACHE::steal(),
                INTERRUPT_CORE0: INTERRUPT_CORE0::steal(),
                INTERRUPT_CORE1: INTERRUPT_CORE1::steal(),
                MIPI_CSI_BRIDGE: MIPI_CSI_BRIDGE::steal(),
                MIPI_CSI_HOST: MIPI_CSI_HOST::steal(),
                DMA: DMA::steal(),
                DS: DS::steal(),
                MIPI_DSI_BRIDGE: MIPI_DSI_BRIDGE::steal(),
                MIPI_DSI_HOST: MIPI_DSI_HOST::steal(),
                ECC: ECC::steal(),
                ECDSA: ECDSA::steal(),
                EFUSE: EFUSE::steal(),
                GPIO: GPIO::steal(),
                GPIO_SD: GPIO_SD::steal(),
                H264: H264::steal(),
                H264_DMA: H264_DMA::steal(),
                HMAC: HMAC::steal(),
                HP_SYS: HP_SYS::steal(),
                HP_SYS_CLKRST: HP_SYS_CLKRST::steal(),
                LP_HUK: LP_HUK::steal(),
                I2C0: I2C0::steal(),
                I2C1: I2C1::steal(),
                I2S0: I2S0::steal(),
                I2S1: I2S1::steal(),
                I2S2: I2S2::steal(),
                I3C_MST: I3C_MST::steal(),
                I3C_MST_MEM: I3C_MST_MEM::steal(),
                I3C_SLV: I3C_SLV::steal(),
                AXI_ICM: AXI_ICM::steal(),
                IO_MUX: IO_MUX::steal(),
                ISP: ISP::steal(),
                JPEG: JPEG::steal(),
                LCD_CAM: LCD_CAM::steal(),
                LEDC: LEDC::steal(),
                LP_INTR: LP_INTR::steal(),
                LP_PERI: LP_PERI::steal(),
                LP_SYS: LP_SYS::steal(),
                LP_ANA_PERI: LP_ANA_PERI::steal(),
                LP_AON_CLKRST: LP_AON_CLKRST::steal(),
                LP_GPIO: LP_GPIO::steal(),
                LP_I2C0: LP_I2C0::steal(),
                LP_I2S0: LP_I2S0::steal(),
                LP_IO_MUX: LP_IO_MUX::steal(),
                LP_UART: LP_UART::steal(),
                MCPWM0: MCPWM0::steal(),
                MCPWM1: MCPWM1::steal(),
                PARL_IO: PARL_IO::steal(),
                PAU: PAU::steal(),
                PCNT: PCNT::steal(),
                PMU: PMU::steal(),
                PPA: PPA::steal(),
                PVT: PVT::steal(),
                RMT: RMT::steal(),
                RSA: RSA::steal(),
                LP_ADC: LP_ADC::steal(),
                LP_TIMER: LP_TIMER::steal(),
                LP_TOUCH: LP_TOUCH::steal(),
                LP_WDT: LP_WDT::steal(),
                SDHOST: SDHOST::steal(),
                SHA: SHA::steal(),
                SOC_ETM: SOC_ETM::steal(),
                SPI0: SPI0::steal(),
                SPI1: SPI1::steal(),
                SPI2: SPI2::steal(),
                SPI3: SPI3::steal(),
                SYSTIMER: SYSTIMER::steal(),
                TIMG0: TIMG0::steal(),
                TIMG1: TIMG1::steal(),
                TRACE0: TRACE0::steal(),
                TRACE1: TRACE1::steal(),
                LP_TSENS: LP_TSENS::steal(),
                TWAI0: TWAI0::steal(),
                TWAI1: TWAI1::steal(),
                TWAI2: TWAI2::steal(),
                UART0: UART0::steal(),
                UART1: UART1::steal(),
                UART2: UART2::steal(),
                UART3: UART3::steal(),
                UART4: UART4::steal(),
                UHCI0: UHCI0::steal(),
                USB_DEVICE: USB_DEVICE::steal(),
                USB_WRAP: USB_WRAP::steal(),
                EMAC: EMAC::steal(),
            }
        }
    }

    /// Release the peripherals back to the pool
    ///
    /// After calling this, `take()` will succeed again.
    #[inline]
    pub fn release(self) {
        TAKEN.store(false, Ordering::Release);
    }
}
