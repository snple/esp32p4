//! UART self-test using the peripheral's internal loopback.
//!
//! `UART1` is switched to loopback (`CONF0.LOOPBACK`), which feeds the
//! transmitter straight back into the receiver inside the peripheral, so no
//! wiring is needed. The example pushes a few patterns through and checks that
//! they come back unchanged, then reports the configured frame format and baud
//! rate over USB Serial/JTAG.
//!
//! Set `UART_TX_PIN` / `UART_RX_PIN` and drop the `loopback(true)` call to talk
//! to a real device instead.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::gpio::Pins;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::uart::{DataBits, Parity, StopBits, Uart1, UartConfig};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// GPIO used for TX when loopback is turned off.
const UART_TX_PIN: u32 = 20;
/// GPIO used for RX when loopback is turned off.
const UART_RX_PIN: u32 = 21;

const BAUD: u32 = 115_200;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    let config = UartConfig::default()
        .baudrate(BAUD)
        .format(DataBits::Eight, StopBits::One, Parity::None)
        .loopback(true);
    let uart = Uart1::init(dp.UART1, &config, &dp.HP_SYS_CLKRST);

    // Routing the pins is harmless in loopback mode and keeps the example one
    // line away from real hardware.
    uart.set_pins(pins.pin::<UART_TX_PIN>(), pins.pin::<UART_RX_PIN>());

    defmt::info!(
        "UART{}: {} baud (requested {}), 8N1, loopback",
        Uart1::NUM,
        uart.baudrate(UartConfig::default().clock, &dp.HP_SYS_CLKRST),
        BAUD
    );

    let mut ramp = [0u8; 64];
    for (index, byte) in ramp.iter_mut().enumerate() {
        *byte = index as u8;
    }

    for pattern in [&b"hello, uart!"[..], &b"\x00\x55\xaa\xff"[..], &ramp[..]] {
        uart.write(pattern);
        uart.flush();

        let mut received = [0u8; 64];
        let received = &mut received[..pattern.len()];
        uart.read_timeout(received, 50_000)
            .expect("loopback receive timed out");
        assert_eq!(received, pattern, "loopback corrupted the data");
    }

    defmt::info!("UART loopback verified");

    loop {
        // Echo anything a host sends once a real device is wired up.
        if let Some(byte) = uart.read_byte() {
            uart.write_byte(byte);
        }
    }
}
