//! Brings up an SD card on the SDHOST controller and checks it end to end.
//!
//! The card slot on the P4 Function-EV board uses the controller's own SD pins
//! plus one switch for the card's supply:
//!
//! ```text
//!   GPIO43  CLK
//!   GPIO44  CMD
//!   GPIO39  DAT0
//!   GPIO40  DAT1
//!   GPIO41  DAT2
//!   GPIO42  DAT3
//!   GPIO45  card power switch, low = on
//! ```
//!
//! The example powers the card through `GPIO45`, identifies it, prints what came
//! out of CID/CSD, reads the first block (the partition table sector, which any
//! formatted card has), then writes two blocks at the end of the card and reads
//! them back to compare. A failure panics with the reason.
//!
//! Pulling `GPIO45` high cuts the card's supply, which is what a low-power build
//! wants while the slot is unused: identification has to start over after the
//! card is powered again, but everything else comes back unchanged.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::sdhost::{BLOCK_SIZE, SdCard, SdHost};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Card power switch; low powers the card, high cuts it.
const SD_POWER: u32 = 45;

/// Card slot pins.
const SD_CLK: u32 = 43;
const SD_CMD: u32 = 44;
const SD_DAT0: u32 = 39;
const SD_DAT1: u32 = 40;
const SD_DAT2: u32 = 41;
const SD_DAT3: u32 = 42;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let delay = Delay::init();
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    // Power the card first: the on-chip LDO in `SdHost::init` supplies the
    // interface, this switch supplies the card itself, and the card needs a
    // moment before the first command.
    let mut power = pins.pin::<SD_POWER>();
    power.set_function_gpio();
    power.set_output_enabled(true);
    power.set_low();
    delay.delay_ms(20);

    let host = SdHost::init(dp.SDHOST, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST, &dp.PMU);
    host.set_pins(
        pins.pin::<SD_CLK>(),
        pins.pin::<SD_CMD>(),
        (
            pins.pin::<SD_DAT0>(),
            pins.pin::<SD_DAT1>(),
            pins.pin::<SD_DAT2>(),
            pins.pin::<SD_DAT3>(),
        ),
    );

    let card = match SdCard::init(&host) {
        Ok(card) => card,
        Err(error) => panic!("SD card identification failed: {:?}", error),
    };

    let cid = card.cid();
    defmt::info!(
        "SD card: rca={:#06x} blocks={} high_capacity={} clock={} Hz",
        card.rca(),
        card.blocks(),
        card.high_capacity(),
        host.frequency()
    );
    defmt::info!(
        "CID: {:02x}{:02x}{:02x}{:02x} {:02x}{:02x} {:02x}{:02x}{:02x}{:02x}{:02x}",
        cid[0] >> 24,
        cid[0] >> 16,
        cid[0] >> 8,
        cid[0],
        cid[1] >> 24,
        cid[1] >> 16,
        cid[2] >> 24,
        cid[2] >> 16,
        cid[2] >> 8,
        cid[2],
        cid[3] >> 24
    );

    // The first block of any formatted card is a partition table (or a
    // superblock) ending in the 0x55AA signature.
    let mut block = [0u8; BLOCK_SIZE];
    if let Err(error) = card.read_blocks(&host, 0, &mut block) {
        panic!("block 0 read failed: {:?}", error);
    }
    defmt::info!(
        "block 0 signature: {:02x} {:02x}",
        block[BLOCK_SIZE - 2],
        block[BLOCK_SIZE - 1]
    );

    // Write/read-back a two-block window at the end of the card, which nothing
    // else should be using: that covers the multi-block commands (CMD25/CMD18)
    // and the stop command that ends them.
    let first = card.blocks() - 2;
    let mut written = [0u8; 2 * BLOCK_SIZE];
    for (index, byte) in written.iter_mut().enumerate() {
        *byte = (index as u32 * 7 + 0x5a) as u8;
    }
    if let Err(error) = card.write_blocks(&host, first, &written) {
        panic!("block {} write failed: {:?}", first, error);
    }
    let mut read_back = [0u8; 2 * BLOCK_SIZE];
    if let Err(error) = card.read_blocks(&host, first, &mut read_back) {
        panic!("block {} read failed: {:?}", first, error);
    }
    assert!(
        read_back == written,
        "blocks {}..{} did not read back what was written",
        first,
        first + 2
    );

    defmt::info!("SD card self-check passed");

    loop {
        delay.delay_ms(1_000);
    }
}
