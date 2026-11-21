//! Puts I2C0 on the bus as a master and I2C1 as a slave, then checks the pair.
//!
//! One controller driving the bus and the other answering it is the only way to
//! test a slave: a single controller in slave mode has nobody to talk to. Both
//! are open-drain pads pulled up by the internal resistors, which is enough for
//! a bench wire but not for a real bus — fit the usual 2.2-4.7 kΩ pull-ups for
//! that.
//!
//! Wiring: `GPIO16`–`GPIO20` carries SDA, `GPIO17`–`GPIO21` carries SCL.
//!
//! Clock stretching is configured off. It exists so that a slow slave can hold
//! SCL while it services its FIFO, which means the master blocks inside
//! [`I2c0::read`] — and on a single core the slave never gets to run and release
//! it. With stretching off, one transfer has to fit the 32-byte FIFOs, which is
//! all these checks use.
//!
//! Every check prints `ok` or `FAIL` on its own, and the whole set repeats once a
//! second.

#![no_std]
#![no_main]

#[cfg(feature = "defmt-rtt")]
use defmt_rtt as _;
#[cfg(feature = "defmt")]
defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

use core::sync::atomic::{AtomicU32, Ordering};

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::{PinOps, Pins};
use esp32p4_hal::i2c::{
    Error, I2c0, I2c1Slave, I2cConfig, Interrupt as I2cInterrupt, SlaveConfig, StretchCause,
};
use esp32p4_hal::interrupt::{self, CpuInterrupt, InterruptHandler, Level as InterruptLevel};
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

/// Bytes per FIFO, and the largest transfer the master can issue at once.
const FIFO_DEPTH: usize = 32;

/// CPU interrupt line for the slave, free on this board.
const SLAVE_CPU_LINE: u8 = 23;

/// Bytes the interrupt moved out of the receive FIFO.
static DRAINED: AtomicU32 = AtomicU32::new(0);

/// Empties the receive FIFO whenever the slave holds SCL for it.
///
/// This is what clock stretching is for: the master is spinning inside its own
/// transfer, so a polling loop in `main` would never get to run and the FIFO it
/// waits on would stay full.
fn slave_isr() {
    // SAFETY: `I2c1Slave::init` has run and the main loop never reconfigures
    // the peripheral.
    let slave = unsafe { I2c1Slave::steal() };
    if slave.stretch_cause() == Some(StretchCause::ReceiveFull) {
        let mut scratch = [0u8; FIFO_DEPTH];
        let drained = slave.read_available(&mut scratch);
        DRAINED.fetch_add(drained as u32, Ordering::Relaxed);
    }
    slave.release_stretch();
    slave.clear_interrupt(I2cInterrupt::SlaveStretch);
}

/// Master SDA, jumpered to [`SDA_SLAVE`].
const SDA_MASTER: u32 = 16;
/// Master SCL, jumpered to [`SCL_SLAVE`].
const SCL_MASTER: u32 = 17;
/// Slave SDA.
const SDA_SLAVE: u32 = 20;
/// Slave SCL.
const SCL_SLAVE: u32 = 21;

/// The address the slave answers to.
const ADDRESS: u8 = 0x55;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
    let delay = Delay::init();

    let mut sda_master = pins.pin::<SDA_MASTER>();
    let mut scl_master = pins.pin::<SCL_MASTER>();
    let mut sda_slave = pins.pin::<SDA_SLAVE>();
    let mut scl_slave = pins.pin::<SCL_SLAVE>();
    // The pads are distinct types, so each one is configured on its own.
    sda_master.set_pull_up_enabled(true);
    scl_master.set_pull_up_enabled(true);
    sda_slave.set_pull_up_enabled(true);
    scl_slave.set_pull_up_enabled(true);

    let master = I2c0::init(dp.I2C0, &I2cConfig::default(), &dp.HP_SYS_CLKRST);
    master.set_pins(scl_master, sda_master);

    let slave = I2c1Slave::init(
        dp.I2C1,
        &SlaveConfig::default().address(ADDRESS).stretch(false),
        &dp.HP_SYS_CLKRST,
    );
    slave.set_pins(scl_slave, sda_slave);

    defmt::info!(
        "i2c slave: master I2C{} at {} Hz, slave I2C{} at {:#04x}",
        I2c0::NUM,
        I2cConfig::default().frequency,
        I2c1Slave::NUM,
        slave.address()
    );

    let _slave_interrupt = install_slave_interrupt(&slave);

    loop {
        check_address(&master, &slave);
        check_receive(&master, &slave);
        check_transmit(&master, &slave);
        check_full_fifo(&master, &slave);
        check_register_access(&master, &slave);
        check_non_blocking(&master, &slave);
        check_flags(&master, &slave);
        check_stretch(&master, &slave);
        delay.delay_ms(1_000);
    }
}

/// Installs the slave's stretch interrupt on a CPU line of its own.
///
/// The returned handler masks the line when it is dropped, so `main` has to keep
/// it for as long as the interrupt is wanted.
fn install_slave_interrupt(slave: &I2c1Slave) -> InterruptHandler {
    slave.clear_interrupt(I2cInterrupt::SlaveStretch);
    // SAFETY: the line is not used by any other driver, and the source flag was
    // cleared above so the first interrupt can only come from a new event.
    let handler = unsafe {
        interrupt::claim(
            I2c1Slave::SOURCE,
            CpuInterrupt::new(SLAVE_CPU_LINE),
            InterruptLevel::LOW,
            slave_isr,
        )
    };
    handler.enable();
    // SAFETY: the I2C1 source is claimed and its handler is installed.
    unsafe { interrupt::enable() };
    handler
}

/// Reports one check.
fn report(label: &str, ok: bool) {
    defmt::info!("{=str}: {} ", label, if ok { "ok" } else { "FAIL" });
}

/// The slave answers its own address and nothing else.
fn check_address(master: &I2c0, slave: &I2c1Slave) {
    let configured = slave.address() == ADDRESS;
    let answered = master.probe(ADDRESS).is_ok();
    let silent = master.probe(ADDRESS + 1) == Err(Error::NoAcknowledge);
    slave.clear_stop();
    defmt::info!(
        "address: configured {=bool}, {:#04x} acked {=bool}, {:#04x} nacked {=bool}",
        configured,
        ADDRESS,
        answered,
        ADDRESS + 1,
        silent
    );
    report("address", configured && answered && silent);
}

/// What the master writes arrives in the slave's receive FIFO.
fn check_receive(master: &I2c0, slave: &I2c1Slave) {
    let sent = [0x01, 0x23, 0x45, 0x67];
    slave.reset_fifos();
    let queued = master.write(ADDRESS, &sent).is_ok();

    let mut received = [0u8; 8];
    let len = slave.read(&mut received).unwrap_or(0);
    let matched = len == sent.len() && received[..len] == sent[..];
    defmt::info!(
        "receive: sent {=bool}, got {=usize} bytes {=[u8]}, matched {=bool}",
        queued,
        len,
        received[..len],
        matched
    );
    report("receive", queued && matched);
}

/// What the slave queues is what the master reads.
fn check_transmit(master: &I2c0, slave: &I2c1Slave) {
    let response = [0xde, 0xad, 0xbe, 0xef];
    slave.reset_fifos();
    let queued = slave.write(&response).unwrap_or(0) == response.len();
    let fifo_queued = slave.tx_fifo_count() as usize;

    let mut received = [0u8; 4];
    let read = master.read(ADDRESS, &mut received).is_ok();
    let fifo_drained = slave.tx_fifo_count() == 0;
    // The read transfer ends with a stop condition of its own.
    slave.clear_stop();

    let matched = received == response;
    defmt::info!(
        "transmit: queued {=bool} {=usize} in fifo, read {=bool}, got {=[u8]}, drained {=bool}",
        queued,
        fifo_queued,
        read,
        received,
        fifo_drained
    );
    report(
        "transmit",
        queued && fifo_queued == response.len() && read && matched && fifo_drained,
    );
}

/// A transfer that fills the FIFOs still arrives whole.
fn check_full_fifo(master: &I2c0, slave: &I2c1Slave) {
    const LEN: usize = 31;
    let mut sent = [0u8; LEN];
    for (index, byte) in sent.iter_mut().enumerate() {
        *byte = index as u8;
    }
    slave.reset_fifos();
    let queued = master.write(ADDRESS, &sent).is_ok();
    let depth = slave.rx_fifo_count() as usize;

    let mut received = [0u8; LEN];
    let len = slave.read(&mut received).unwrap_or(0);
    let matched = len == LEN && received == sent;
    defmt::info!(
        "full fifo: queued {=bool}, {=usize} bytes in fifo, read {=usize}, matched {=bool}",
        queued,
        depth,
        len,
        matched
    );
    report("full fifo", queued && depth == LEN && matched);
}

/// A register-style access, split into a write and a read transaction.
fn check_register_access(master: &I2c0, slave: &I2c1Slave) {
    const REGISTER: u8 = 0x10;
    const VALUE: [u8; 2] = [0xab, 0xcd];

    slave.reset_fifos();
    let requested = master.write(ADDRESS, &[REGISTER]).is_ok();
    let mut register = [0u8; 1];
    let got_register = slave.read(&mut register).unwrap_or(0) == 1 && register[0] == REGISTER;

    slave.write(&VALUE).unwrap_or(0);
    let mut value = [0u8; 2];
    let read = master.read(ADDRESS, &mut value).is_ok();
    slave.clear_stop();

    defmt::info!(
        "register: write {=bool}, slave saw {=[u8]}, read {=bool}, value {=[u8]}",
        requested,
        register,
        read,
        value
    );
    report(
        "register",
        requested && got_register && read && value == VALUE,
    );
}

/// The FIFO can be serviced without blocking.
fn check_non_blocking(master: &I2c0, slave: &I2c1Slave) {
    let sent = [0x11, 0x22, 0x33];
    slave.reset_fifos();
    let written = master.write(ADDRESS, &sent).is_ok();

    let mut received = [0u8; 8];
    let len = slave.read_available(&mut received);
    slave.clear_stop();

    let matched = len == sent.len() && received[..len] == sent[..];

    // And the other way round: queue by hand, then let the master take it.
    let response = [0x77, 0x88];
    let queued = slave.write_available(&response) == response.len();
    let mut read_back = [0u8; 2];
    let read = master.read(ADDRESS, &mut read_back).is_ok();
    slave.clear_stop();

    defmt::info!(
        "non blocking: wrote {=bool}, available {=usize} {=[u8]}, queued {=bool}, read {=bool} {=[u8]}",
        written,
        len,
        received[..len],
        queued,
        read,
        read_back
    );
    report(
        "non blocking",
        written && matched && queued && read && read_back == response,
    );
}

/// START and STOP are latched, and clearing them sticks.
fn check_flags(master: &I2c0, slave: &I2c1Slave) {
    slave.reset_fifos();
    slave.clear_start();
    let written = master.write(ADDRESS, &[0x5a]).is_ok();
    let mut sink = [0u8; 1];
    slave.read_available(&mut sink);

    let start = slave.start_detected();
    let stop = slave.stop_detected();
    slave.clear_start();
    slave.clear_stop();
    let cleared = !slave.start_detected() && !slave.stop_detected();
    let idle = !slave.is_bus_busy() && !slave.is_addressed();

    defmt::info!(
        "flags: wrote {=bool}, start {=bool}, stop {=bool}, cleared {=bool}, idle {=bool}",
        written,
        start,
        stop,
        cleared,
        idle
    );
    report("flags", written && start && stop && cleared && idle);
}

/// A full receive FIFO holds SCL until the interrupt empties it.
///
/// Two transfers of 31 bytes run through a 32-byte FIFO, so the second one can
/// only finish if the slave stretches the clock at byte 32 and the interrupt
/// drains the FIFO while the master waits.
fn check_stretch(master: &I2c0, slave: &I2c1Slave) {
    slave.reset_fifos();
    slave.enable_interrupt(I2cInterrupt::SlaveStretch);

    DRAINED.store(0, Ordering::Relaxed);
    slave.set_stretch(true);
    slave.clear_stop();

    let first = master.write(ADDRESS, &[0x5a; 31]).is_ok();
    let second = master.write(ADDRESS, &[0xa5; 31]).is_ok();
    let drained = DRAINED.load(Ordering::Relaxed);

    slave.disable_interrupt(I2cInterrupt::SlaveStretch);
    slave.set_stretch(false);
    slave.reset_fifos();
    slave.clear_stop();

    // The bus has to be usable again, and the controller still has to answer.
    let recovered = master.probe(ADDRESS).is_ok();
    slave.clear_stop();

    defmt::info!(
        "stretch: first {=bool}, second {=bool}, drained {=u32} bytes in the interrupt, bus recovered {=bool}",
        first,
        second,
        drained,
        recovered
    );
    report(
        "stretch",
        first && second && drained >= FIFO_DEPTH as u32 && recovered,
    );
}
