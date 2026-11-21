//! Runs GP-SPI2 as a master against GP-SPI3 as a slave.
//!
//! Four wires keep everything on one board:
//!
//! ```text
//!   SPI2 (master)            SPI3 (slave)
//!   GPIO20 SCK    ─────────► GPIO16 SCK
//!   GPIO21 MOSI   ─────────► GPIO17 MOSI
//!   GPIO22 MISO   ◄───────── GPIO18 MISO
//!   GPIO23 CS     ─────────► GPIO19 CS
//! ```
//!
//! Every mode (0-3) runs with 1, 4, 32 and 64 byte transfers. Each side sends
//! its own pattern and checks what came back, which exercises the clock phase,
//! the bit order and the 64-byte buffer edge on both controllers. A mismatch
//! panics with the mode and length that failed.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::spi::{Config, Mode, SlaveConfig, Spi2, SpiSlave3};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// SCK frequency; the slave follows the master, so it needs no clock setting.
const FREQUENCY: u32 = 1_000_000;

/// Master pins.
const MASTER_SCK: u32 = 20;
const MASTER_MOSI: u32 = 21;
const MASTER_MISO: u32 = 22;
const MASTER_CS: u32 = 23;

/// Slave pins.
const SLAVE_SCK: u32 = 16;
const SLAVE_MOSI: u32 = 17;
const SLAVE_MISO: u32 = 18;
const SLAVE_CS: u32 = 19;

/// Transfer lengths, ending at the 64-byte CPU buffer.
const LENGTHS: [usize; 4] = [1, 4, 32, 64];

const MODES: [Mode; 4] = [Mode::_0, Mode::_1, Mode::_2, Mode::_3];

/// Master sends this, the slave checks it.
fn master_pattern(len: usize) -> [u8; 64] {
    let mut buffer = [0u8; 64];
    for (index, byte) in buffer.iter_mut().enumerate().take(len) {
        *byte = (index as u32 * 3 + 0x80) as u8;
    }
    buffer
}

/// Slave sends this, the master checks it.
fn slave_pattern(len: usize) -> [u8; 64] {
    let mut buffer = [0u8; 64];
    for (index, byte) in buffer.iter_mut().enumerate().take(len) {
        *byte = (index as u32 * 5 + 0x21) as u8;
    }
    buffer
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = esp32p4_hal::gpio::Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    let master = Spi2::init(
        dp.SPI2,
        &Config::default().frequency(FREQUENCY).cs_timing(1, 1),
        &dp.HP_SYS_CLKRST,
    );
    master.set_pins(
        pins.pin::<MASTER_SCK>(),
        pins.pin::<MASTER_MOSI>(),
        pins.pin::<MASTER_MISO>(),
    );
    master.set_cs(pins.pin::<MASTER_CS>());

    let slave = SpiSlave3::init(dp.SPI3, &SlaveConfig::default(), &dp.HP_SYS_CLKRST);
    slave.set_pins(
        pins.pin::<SLAVE_SCK>(),
        pins.pin::<SLAVE_MOSI>(),
        pins.pin::<SLAVE_MISO>(),
        pins.pin::<SLAVE_CS>(),
    );

    defmt::info!(
        "SPI{} master at {} Hz, SPI{} slave",
        Spi2::NUM,
        master.frequency(),
        SpiSlave3::NUM
    );

    let mut master_rx = [0u8; 64];
    let mut slave_rx = [0u8; 64];

    for mode in MODES {
        master.set_mode(mode);
        slave.set_mode(mode);

        for len in LENGTHS {
            let mosi = master_pattern(len);
            let miso = slave_pattern(len);

            // The slave arms itself first: the master's CS assertion starts the
            // transaction.
            slave.start(&miso[..len]);
            master.transfer(&mosi[..len], &mut master_rx[..len]);
            let received = slave.finish(&mut slave_rx[..len]);

            assert_eq!(
                received, len,
                "slave clocked {} bytes, expected {}",
                received, len
            );
            assert_eq!(
                master_rx[..len],
                miso[..len],
                "master received the wrong bytes"
            );
            assert_eq!(
                slave_rx[..len],
                mosi[..len],
                "slave received the wrong bytes"
            );
        }
        defmt::info!("mode {:?}: all lengths verified", mode);
    }

    defmt::info!("SPI master/slave self-check passed");

    loop {
        delay::delay_ms(1_000);
    }
}
