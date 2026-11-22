//! Measures GP-SPI2 as a DMA master, with GDMA-AXI moving the data.
//!
//! Two setups are used, both already wired on the board:
//!
//! ```text
//!   loopback                      SPI2 (master)          SPI3 (slave)
//!   GPIO20 SCK  ── (nothing)
//!   GPIO21 MOSI ──┐
//!   GPIO17 ◄──────┘  jumpered, sampled as MISO
//!
//!   master/slave                  SPI2 (master)          SPI3 (slave)
//!   GPIO20 SCK  ────────────────► GPIO16 SCK
//!   GPIO21 MOSI ────────────────► GPIO17 MOSI
//!   GPIO22 MISO ◄──────────────── GPIO18 MISO
//!   GPIO23 CS   ────────────────► GPIO19 CS
//! ```
//!
//! 1. The master's MISO is taken from pad 17, which is jumpered to its own MOSI
//!    pad: everything sent comes back, so a byte-exact echo verifies the whole
//!    DMA path without a second controller. Lengths above one descriptor and
//!    above the 64-byte CPU buffer are covered, and the bytes right behind the
//!    receive buffer are checked for engine damage.
//! 2. The same master against the SPI3 slave for the two half-duplex paths
//!    ([`Spi2::write_dma`] and [`Spi2::read_dma`]), which a loopback cannot
//!    exercise: with only one direction enabled, the clock has to be generated
//!    by the phase that runs.

#![no_std]
#![no_main]

// The default `defmt-usb` transport installs the USB Serial/JTAG logger; `main`
// opens its port first so log statements are not dropped.
// `--no-default-features --features defmt-rtt` selects RTT instead.
#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use esp32p4_hal::clock::ClockTree;
use esp32p4_hal::delay;
use esp32p4_hal::dma::{AxiDma, Descriptor, Error as DmaError};
use esp32p4_hal::efuse::Efuse;
use esp32p4_hal::machine_timer::MachineTimer;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::spi::{BitOrder, Config, Mode, SlaveConfig, Spi2, SpiSlave3};
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// SCK frequency; the slave follows the master, so it needs no clock setting.
const FREQUENCY: u32 = 1_000_000;

/// Master pins.
const MASTER_SCK: u32 = 20;
const MASTER_MOSI: u32 = 21;
const MASTER_MISO: u32 = 22;
const MASTER_CS: u32 = 23;
/// Pad jumpered to [`MASTER_MOSI`], used as the loopback source.
const LOOPBACK_MISO: u32 = 17;

/// Slave pins.
const SLAVE_SCK: u32 = 16;
const SLAVE_MOSI: u32 = 17;
const SLAVE_MISO: u32 = 18;
const SLAVE_CS: u32 = 19;

/// Largest payload tested; needs three descriptors per direction.
const BULK: usize = 8_192;
/// Descriptors one direction needs for [`BULK`].
const BULK_DESCRIPTORS: usize = BULK.div_ceil(4095);
/// Bytes of slack behind the receive buffer, to catch burst overruns.
const GUARD: usize = 8;

/// Transfer timeout: a 8192-byte transfer takes 65 ms at `FREQUENCY`, so only
/// a stuck channel reaches this.
const TIMEOUT_US: u64 = 400_000;

/// Lengths the loopback runs, in bytes: the CPU buffer size, an odd length, one
/// descriptor exactly and the multi-descriptor payload.
const LENGTHS: [usize; 4] = [64, 1_001, 4_095, BULK];

static mut DESCRIPTORS: [Descriptor; 2 * BULK_DESCRIPTORS] =
    [const { Descriptor::new() }; 2 * BULK_DESCRIPTORS];
static mut SENT: [u8; BULK] = [0; BULK];
static mut RECEIVED: [u8; BULK + GUARD] = [0; BULK + GUARD];

fn fill(buffer: &mut [u8], seed: u32) {
    for (index, byte) in buffer.iter_mut().enumerate() {
        *byte = (index as u32 * 11 + seed) as u8;
    }
}

/// Index of the first byte that differs, if any.
fn first_mismatch(a: &[u8], b: &[u8]) -> Option<usize> {
    a.iter().zip(b).position(|(x, y)| x != y)
}

/// Reports one check.
fn report(label: &str, ok: bool) {
    defmt::info!("{}: {}", label, if ok { "ok" } else { "FAIL" });
}

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = esp32p4_hal::gpio::Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);

    // `delay::micros` starts at a conservative boot rate; publishing the real
    // one keeps the timings below honest. The tree is handed back afterwards
    // because the drivers take the same register handle.
    let efuse = Efuse::init(dp.EFUSE);
    let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
    let _timer = MachineTimer::init(clocks.cpu_hz());
    let (clocks, _lp_aon) = clocks.release();

    let dma = AxiDma::init(dp.AXI_DMA, &clocks);
    let tx_dma = dma.tx_channel::<0>();
    let rx_dma = dma.rx_channel::<0>();

    let master = Spi2::init(
        dp.SPI2,
        &Config::default().frequency(FREQUENCY).cs_timing(1, 1),
        &clocks,
    );

    // SAFETY: the statics are used by nothing else, and only one transfer runs
    // at a time.
    let descriptors = unsafe { &mut *core::ptr::addr_of_mut!(DESCRIPTORS) };
    let sent = unsafe { &mut *core::ptr::addr_of_mut!(SENT) };
    let received = unsafe { &mut *core::ptr::addr_of_mut!(RECEIVED) };

    defmt::info!(
        "SPI{} master at {} Hz, GDMA-AXI",
        Spi2::NUM,
        master.frequency()
    );

    // ---- 1. Loopback: MOSI jumpered to the MISO input ---------------------
    master.set_pins(
        pins.pin::<MASTER_SCK>(),
        pins.pin::<MASTER_MOSI>(),
        pins.pin::<LOOPBACK_MISO>(),
    );
    master.set_cs(pins.pin::<MASTER_CS>());

    for mode in [Mode::_0, Mode::_3] {
        for order in [BitOrder::MsbFirst, BitOrder::LsbFirst] {
            master.set_mode(mode);
            master.set_bit_order(order);

            for &len in &LENGTHS {
                fill(&mut sent[..len], 7);
                // A pattern the engine would not produce by chance.
                received[..len + GUARD].fill(0xa5);

                let start = delay::micros();
                let result = master.transfer_dma(
                    &tx_dma,
                    &rx_dma,
                    &sent[..len],
                    &mut received[..len],
                    descriptors,
                    TIMEOUT_US,
                );
                let elapsed = delay::micros() - start;

                let echoed =
                    result == Ok(len) && first_mismatch(&received[..len], &sent[..len]).is_none();
                let guard_intact = received[len..len + GUARD].iter().all(|&b| b == 0xa5);
                defmt::info!(
                    "{:?} {:?} {:?}-byte loopback: {} ({} us)",
                    mode,
                    order,
                    len,
                    if echoed && guard_intact { "ok" } else { "FAIL" },
                    elapsed
                );
                if !guard_intact {
                    defmt::warn!("  the engine wrote past the receive buffer");
                }
            }
        }
    }

    // ---- 2. Half duplex against the SPI3 slave ---------------------------
    master.set_pins(
        pins.pin::<MASTER_SCK>(),
        pins.pin::<MASTER_MOSI>(),
        pins.pin::<MASTER_MISO>(),
    );
    master.set_mode(Mode::_0);
    master.set_bit_order(BitOrder::MsbFirst);
    let slave = SpiSlave3::init(dp.SPI3, &SlaveConfig::default(), &clocks);
    slave.set_pins(
        pins.pin::<SLAVE_SCK>(),
        pins.pin::<SLAVE_MOSI>(),
        pins.pin::<SLAVE_MISO>(),
        pins.pin::<SLAVE_CS>(),
    );

    let mut slave_rx = [0u8; 64];

    // Write only: the slave sends 0x00, the master must not clock it in.
    fill(&mut sent[..64], 21);
    slave.start(&[0u8; 64]);
    let written = master.write_dma(&tx_dma, &sent[..64], descriptors, TIMEOUT_US);
    let clocked = slave.finish(&mut slave_rx);
    report(
        "half-duplex write to the slave",
        written.is_ok() && clocked == 64 && first_mismatch(&slave_rx[..64], &sent[..64]).is_none(),
    );
    defmt::info!("  slave clocked {} bytes", clocked);

    // Read only: the slave has data ready, nothing is sent from this side.
    fill(&mut sent[..64], 33);
    received.fill(0);
    slave.start(&sent[..64]);
    let read = master.read_dma(&rx_dma, &mut received[..64], descriptors, TIMEOUT_US);
    report(
        "half-duplex read from the slave",
        read.is_ok() && first_mismatch(&received[..64], &sent[..64]).is_none(),
    );

    // ---- 3. The CPU path still works after a DMA transfer ---------------
    // It fills and drains the same FIFOs the engine was attached to, so this
    // catches a DMA transfer that left its paths enabled.
    fill(&mut sent[..32], 45);
    received[..32].fill(0);
    slave.start(&sent[..32]);
    master.transfer(&sent[..32], &mut received[..32]);
    let clocked = slave.finish(&mut slave_rx);
    report(
        "CPU transfer after DMA",
        clocked == 32 && first_mismatch(&received[..32], &sent[..32]).is_none(),
    );

    // ---- 4. A chain that is too short is refused, not attempted ---------
    let short = master.transfer_dma(
        &tx_dma,
        &rx_dma,
        &sent[..BULK],
        &mut received[..BULK],
        &descriptors[..2],
        TIMEOUT_US,
    );
    report(
        "descriptor shortage reported",
        short
            == Err(DmaError::NotEnoughDescriptors {
                needed: 2 * BULK_DESCRIPTORS,
                available: 2,
            }),
    );

    defmt::info!("SPI DMA self-check done");

    loop {
        delay::delay_ms(1_000);
    }
}
