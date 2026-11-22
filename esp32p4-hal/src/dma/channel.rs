//! GDMA-AHB channel handles.
//!
//! The controller has three transmit/receive channel pairs. A pair shares its
//! index: `TX channel n` feeds the peripheral on `RX channel n` (or, in
//! memory-to-memory mode, the receive channel directly). Channels are assigned
//! to peripherals by software, so a channel is just a pair of register blocks
//! plus the peripheral-selection value the driver programs.
//!
//! [`RxChannel`] and [`TxChannel`] are zero-sized tokens parameterised by the
//! channel index, the same way [`Pin<N>`](crate::gpio::Pin) is parameterised by
//! the pad: the register names are fixed at compile time and no lookup happens
//! at run time. Only indices 0–2 have implementations, so a typo does not
//! compile.

use core::marker::PhantomData;

use esp32p4_pac::{AHB_DMA, ahb_dma};
use pastey::paste;

use super::crc::Crc;
use super::descriptor::Descriptor;
use super::{Peripheral, PeripheralId};
use crate::Steal;

/// A `Dummy-n` peripheral id.
///
/// Memory-to-memory mode loops a transmit channel into its receive partner
/// through a reserved trigger value instead of a real peripheral. Both channels
/// of the pair have to select the *same* dummy id (TRM 6.4.3); which one does
/// not matter as long as no other channel uses it. GDMA-AHB accepts any of
/// these, GDMA-AXI only the ids from [`D6`](Self::D6) up: its `1` is GP-SPI2.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Dummy {
    /// `Dummy-1`.
    D1 = 1,
    /// `Dummy-6`.
    D6 = 6,
    /// `Dummy-7`.
    D7 = 7,
    /// `Dummy-9`.
    D9 = 9,
    /// `Dummy-11`.
    D11 = 11,
    /// `Dummy-12`.
    D12 = 12,
    /// `Dummy-13`.
    D13 = 13,
    /// `Dummy-14`.
    D14 = 14,
    /// `Dummy-15`.
    D15 = 15,
}

/// Receive-channel interrupt sources.
///
/// Bit positions are those of `IN_INT_RAW_CHn` / `IN_INT_ENA_CHn` /
/// `IN_INT_CLR_CHn`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum RxInterrupt {
    /// A descriptor's buffer is full.
    Done = 0,
    /// A descriptor carrying the EOF flag completed.
    SucEof = 1,
    /// Received data was flagged as erroneous (UHCI and PARLIO only).
    ErrEof = 2,
    /// The engine rejected a descriptor: `owner` clear, or address outside the
    /// access window.
    DscrErr = 3,
    /// A chain-ended buffer filled up; the engine had nowhere to put more data.
    DscrEmpty = 4,
    /// The receive FIFO overflowed.
    FifoOverflow = 5,
    /// The receive FIFO was read while empty.
    FifoUnderflow = 6,
}

impl RxInterrupt {
    /// The mask of this source in the interrupt registers.
    #[inline]
    pub const fn mask(self) -> u32 {
        1 << (self as u32)
    }
}

/// Transmit-channel interrupt sources.
///
/// Bit positions are those of `OUT_INT_RAW_CHn` / `OUT_INT_ENA_CHn` /
/// `OUT_INT_CLR_CHn`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum TxInterrupt {
    /// A descriptor's buffer was sent.
    Done = 0,
    /// A descriptor carrying the EOF flag was sent.
    Eof = 1,
    /// The engine rejected a descriptor.
    DscrErr = 2,
    /// The last descriptor of the chain carried the EOF flag and was sent.
    TotalEof = 3,
    /// The transmit FIFO overflowed.
    FifoOverflow = 4,
    /// The transmit FIFO ran dry mid-transfer.
    FifoUnderflow = 5,
}

impl TxInterrupt {
    /// The mask of this source in the interrupt registers.
    #[inline]
    pub const fn mask(self) -> u32 {
        1 << (self as u32)
    }
}

/// Receive channel `N`, 0–2.
pub struct RxChannel<const N: usize> {
    _marker: PhantomData<()>,
}

impl<const N: usize> RxChannel<N> {
    /// Channel index, 0–2.
    pub const CHANNEL: usize = N;

    /// A handle over the registers; the controller is reached through
    /// `AHB_DMA::PTR`, so the handle carries no state.
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Obtains a handle without ownership checks.
    ///
    /// Useful in an interrupt handler, where the owning driver lives in
    /// `main()`. The handle is zero-sized; only the register block is touched.
    ///
    /// # Safety
    ///
    /// The controller must have been initialised with
    /// [`AhbDma::init`](super::AhbDma::init), and the caller must not use the
    /// same channel from two contexts at once.
    #[inline]
    pub const unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            _marker: PhantomData,
        })
    }
}

/// Transmit channel `N`, 0–2.
pub struct TxChannel<const N: usize> {
    _marker: PhantomData<()>,
}

impl<const N: usize> TxChannel<N> {
    /// Channel index, 0–2.
    pub const CHANNEL: usize = N;

    /// A handle over the registers; see [`RxChannel::new`].
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Obtains a handle without ownership checks.
    ///
    /// See [`RxChannel::steal`] for the requirements.
    ///
    /// # Safety
    ///
    /// The controller must have been initialised with
    /// [`AhbDma::init`](super::AhbDma::init), and the caller must not use the
    /// same channel from two contexts at once.
    #[inline]
    pub const unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            _marker: PhantomData,
        })
    }
}

/// Generates the register-level body of one receive channel.
macro_rules! impl_rx_channel {
    ($n:literal) => {
        paste! {
            impl RxChannel<$n> {
                /// The controller's register block.
                ///
                /// # Safety
                ///
                /// [`AhbDma::init`](super::AhbDma::init) has to have run.
                #[inline(always)]
                fn regs() -> &'static ahb_dma::RegisterBlock {
                    // SAFETY: the controller is owned by `AhbDma`; a channel
                    // handle is a zero-sized token over its registers.
                    unsafe { &*AHB_DMA::PTR }
                }

                /// Connects the channel to a peripheral's receive handshake.
                ///
                /// One peripheral can feed only one channel at a time; the
                /// driver of the peripheral owns that choice.
                #[inline]
                pub fn connect(&self, peripheral: Peripheral) {
                    let r = Self::regs();
                    r.[<in_peri_sel_ch $n>].write_peri_in_sel_ch(peripheral as u32);
                    r.[<in_conf0_ch $n>].clr_mem_trans_en_ch();
                }

                /// Loops the paired transmit channel into this one.
                ///
                /// Both channels of the pair must select the same `dummy` id.
                /// Only pairs 1 and 2 can do this on GDMA-AHB (TRM 6.4.3).
                #[inline]
                pub fn connect_memory(&self, dummy: Dummy) {
                    let r = Self::regs();
                    r.[<in_peri_sel_ch $n>].write_peri_in_sel_ch(dummy as u32);
                    r.[<in_conf0_ch $n>].set_mem_trans_en_ch();
                }

                /// Disconnects the channel from every peripheral, including
                /// memory-to-memory mode.
                #[inline]
                pub fn disconnect(&self) {
                    let r = Self::regs();
                    r.[<in_peri_sel_ch $n>].write_peri_in_sel_ch(0x3f);
                    r.[<in_conf0_ch $n>].clr_mem_trans_en_ch();
                }

                /// Enables the descriptor owner check.
                ///
                /// With the check on, the engine skips descriptors whose
                /// `owner` bit is not [`Owner::Dma`](super::descriptor::Owner),
                /// which keeps it off buffers the core is still writing.
                #[inline]
                pub fn check_owner(&self, enable: bool) {
                    let reg = &Self::regs().[<in_conf1_ch $n>];
                    if enable {
                        reg.set_in_check_owner_ch();
                    } else {
                        reg.clr_in_check_owner_ch();
                    }
                }

                /// Enables burst reads of the data buffers.
                ///
                /// Off by default. A burst needs the buffer address and length
                /// aligned to the burst size, and must be off for buffers in
                /// PSRAM when it is ECC-protected.
                #[inline]
                pub fn data_burst(&self, enable: bool) {
                    let reg = &Self::regs().[<in_conf0_ch $n>];
                    if enable {
                        reg.set_in_data_burst_en_ch();
                    } else {
                        reg.clr_in_data_burst_en_ch();
                    }
                }

                /// Enables burst reads of the descriptors themselves.
                ///
                /// Descriptors are word aligned and live in internal memory, so
                /// this is safe whenever they are contiguous.
                #[inline]
                pub fn descriptor_burst(&self, enable: bool) {
                    let reg = &Self::regs().[<in_conf0_ch $n>];
                    if enable {
                        reg.set_indscr_burst_en_ch();
                    } else {
                        reg.clr_indscr_burst_en_ch();
                    }
                }

                /// Programs the CRC unit that runs over this channel's data.
                ///
                /// The engine then runs the LFSR over every byte it writes to
                /// memory, one descriptor chain at a time; the result is read
                /// back with [`crc_result`](Self::crc_result).
                ///
                /// # Panics
                ///
                /// Panics if `crc` is wider than [`Crc::AHB_MAX_WIDTH`] bits.
                #[inline]
                pub fn set_crc(&self, crc: &Crc) {
                    assert!(
                        crc.width() <= Crc::AHB_MAX_WIDTH,
                        "GDMA-AHB computes at most 32 CRC bits"
                    );
                    let r = Self::regs();
                    let width = &r.[<rx_crc_width_ch $n>];
                    width.write_rx_crc_width_ch(u32::from((crc.width() - 1) / 8));
                    r.[<in_crc_init_data_ch $n>].write_in_crc_init_data_ch(crc.init());
                    crc.for_each_mask(|bit, lfsr_mask, data_mask| {
                        r.[<rx_crc_en_addr_ch $n>].write_rx_crc_en_addr_ch(bit);
                        r.[<rx_crc_en_wr_data_ch $n>].write_rx_crc_en_wr_data_ch(lfsr_mask);
                        r.[<rx_crc_data_en_addr_ch $n>].write_rx_crc_data_en_addr_ch(bit);
                        r.[<rx_crc_data_en_wr_data_ch $n>]
                            .write_rx_crc_data_en_wr_data_ch(data_mask);
                        // Each mask pair only takes effect once it is latched.
                        width.set_rx_crc_lautch_flga_ch();
                        width.clr_rx_crc_lautch_flga_ch();
                    });
                    let clear = &r.[<in_crc_clear_ch $n>];
                    clear.set_in_crc_clear_ch();
                    clear.clr_in_crc_clear_ch();
                }

                /// CRC the engine computed over the last completed chain.
                #[inline]
                pub fn crc_result(&self) -> u32 {
                    Self::regs().[<in_crc_final_result_ch $n>].in_crc_final_result_ch()
                }

                /// Enables ETM control: the channel then starts on a task from
                /// the event matrix instead of [`start`](Self::start).
                #[inline]
                pub fn enable_etm(&self, enable: bool) {
                    let reg = &Self::regs().[<in_conf0_ch $n>];
                    if enable {
                        reg.set_in_etm_en_ch();
                    } else {
                        reg.clr_in_etm_en_ch();
                    }
                }

                /// Arbitration priority, 0–5; higher wins.
                #[inline]
                pub fn set_priority(&self, priority: u8) {
                    Self::regs().[<in_pri_ch $n>].write_rx_pri_ch(u32::from(priority));
                }

                /// Resets the channel's FSM and FIFO pointers.
                ///
                /// Does not touch the descriptors; remount them before
                /// restarting.
                #[inline]
                pub fn reset(&self) {
                    let reg = &Self::regs().[<in_conf0_ch $n>];
                    reg.set_in_rst_ch();
                    reg.clr_in_rst_ch();
                }

                /// Starts walking the descriptor list at `first`.
                ///
                /// The start bit clears itself once the engine picked the list
                /// up; from then on [`is_idle`](Self::is_idle) reports whether
                /// the channel still has work.
                #[inline]
                pub fn start(&self, first: &Descriptor) {
                    let r = Self::regs();
                    r.[<in_link_addr_ch $n>].write_inlink_addr_ch(first as *const Descriptor as u32);
                    r.[<in_link_ch $n>].trigger_inlink_start_ch();
                }

                /// Appends a list mounted behind the current one.
                ///
                /// Call this after pointing the last descriptor of the running
                /// list at the new head; the engine then picks the new
                /// descriptors up when it reaches that descriptor, or restarts
                /// the channel if it had already finished.
                #[inline]
                pub fn restart(&self) {
                    Self::regs().[<in_link_ch $n>].trigger_inlink_restart_ch();
                }

                /// Stops after the descriptor the engine is working on.
                #[inline]
                pub fn stop(&self) {
                    Self::regs().[<in_link_ch $n>].trigger_inlink_stop_ch();
                }

                /// Whether the descriptor FSM is idle.
                #[inline]
                pub fn is_idle(&self) -> bool {
                    Self::regs().[<in_link_ch $n>].inlink_park_ch()
                }

                /// Address of the descriptor whose EOF was received.
                ///
                /// With several EOF descriptors in a list, this is the most
                /// recent one, which is how a driver finds out how many buffers
                /// were filled since the last interrupt.
                #[inline]
                pub fn last_eof_descriptor(&self) -> *const Descriptor {
                    Self::regs().[<in_suc_eof_des_addr_ch $n>].in_suc_eof_des_addr_ch()
                        as usize as *const Descriptor
                }

                /// Address of the descriptor the engine is prefetching.
                #[inline]
                pub fn current_descriptor(&self) -> *const Descriptor {
                    Self::regs().[<in_dscr_ch $n>].inlink_dscr_ch() as usize as *const Descriptor
                }

                /// Raw interrupt status, in the bit positions of
                /// [`RxInterrupt`].
                #[inline]
                pub fn interrupt_status(&self) -> u32 {
                    Self::regs().[<in_int_raw_ch $n>].read_bits()
                }

                /// Whether `interrupt` is pending.
                #[inline]
                pub fn interrupt_pending(&self, interrupt: RxInterrupt) -> bool {
                    self.interrupt_status() & interrupt.mask() != 0
                }

                /// Enables or disables one interrupt source.
                #[inline]
                pub fn enable_interrupt(&self, interrupt: RxInterrupt, enable: bool) {
                    let reg = &Self::regs().[<in_int_ena_ch $n>];
                    if enable {
                        reg.modify_bits(|bits| bits | interrupt.mask());
                    } else {
                        reg.modify_bits(|bits| bits & !interrupt.mask());
                    }
                }

                /// Acknowledges one interrupt source.
                #[inline]
                pub fn clear_interrupt(&self, interrupt: RxInterrupt) {
                    Self::regs().[<in_int_clr_ch $n>].write_bits(interrupt.mask());
                }

                /// Acknowledges every interrupt source.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    Self::regs().[<in_int_clr_ch $n>].write_bits(u32::MAX);
                }
            }

            impl ReceiveChannel for RxChannel<$n> {
                fn connect_to(&self, peripheral: impl PeripheralId) {
                    let r = Self::regs();
                    r.[<in_peri_sel_ch $n>].write_peri_in_sel_ch(peripheral.id());
                    r.[<in_conf0_ch $n>].clr_mem_trans_en_ch();
                }

                fn reset_channel(&self) {
                    self.reset();
                }

                fn start_chain(&self, first: &Descriptor) {
                    self.start(first);
                }

                fn detach(&self) {
                    self.disconnect();
                }

                fn check_owner(&self, enable: bool) {
                    RxChannel::<$n>::check_owner(self, enable);
                }

                fn descriptor_burst(&self, enable: bool) {
                    RxChannel::<$n>::descriptor_burst(self, enable);
                }

                fn idle(&self) -> bool {
                    self.is_idle()
                }

                fn interrupt_pending(&self, interrupt: RxInterrupt) -> bool {
                    RxChannel::<$n>::interrupt_pending(self, interrupt)
                }

                fn clear_interrupt(&self, interrupt: RxInterrupt) {
                    RxChannel::<$n>::clear_interrupt(self, interrupt);
                }

                fn clear_all_interrupts(&self) {
                    RxChannel::<$n>::clear_all_interrupts(self);
                }

                fn stop_chain(&self) {
                    RxChannel::<$n>::stop(self);
                }
            }
        }
    };
}

/// Operations a peripheral driver needs on a transmit channel.
///
/// The mirror image of [`ReceiveChannel`]: [`TxChannel`] is generated per
/// channel number, so a driver that accepts any channel goes through this trait
/// instead of naming one.
pub trait TransmitChannel {
    /// Connects the channel to a peripheral's transmit handshake.
    ///
    /// See [`ReceiveChannel::connect_to`].
    fn connect_to(&self, peripheral: impl PeripheralId);

    /// Resets the channel's engine state.
    fn reset_channel(&self);

    /// Starts the descriptor chain at `first`.
    fn start_chain(&self, first: &Descriptor);

    /// Disconnects the channel from every peripheral.
    fn detach(&self);

    /// Enables the descriptor owner check; see [`TxChannel::check_owner`].
    fn check_owner(&self, enable: bool);

    /// Enables burst reads of the descriptors; see
    /// [`TxChannel::descriptor_burst`].
    fn descriptor_burst(&self, enable: bool);

    /// Hands a finished descriptor back to software; see
    /// [`TxChannel::auto_writeback`].
    fn auto_writeback(&self, enable: bool);

    /// Reports EOF only once the peripheral consumed the data; see
    /// [`TxChannel::eof_till_data_popped`].
    fn eof_till_data_popped(&self, enable: bool);

    /// Whether the engine has no descriptor left to process.
    fn idle(&self) -> bool;

    /// Whether `interrupt` is latched.
    fn interrupt_pending(&self, interrupt: TxInterrupt) -> bool;

    /// Acknowledges `interrupt`.
    fn clear_interrupt(&self, interrupt: TxInterrupt);

    /// Stops after the descriptor the engine is working on.
    fn stop_chain(&self);

    /// Acknowledges every interrupt source.
    fn clear_all_interrupts(&self);
}

/// Operations a peripheral driver needs on a receive channel.
///
/// [`RxChannel`] is generated per channel number, so a driver that accepts any
/// channel (the ADC's pattern scan, for instance) goes through this trait
/// instead of naming one.
pub trait ReceiveChannel {
    /// Connects the channel to a peripheral's receive handshake.
    ///
    /// Takes [`Peripheral`] for this controller and
    /// [`AxiPeripheral`](super::AxiPeripheral) for GDMA-AXI; the two are not
    /// interchangeable.
    fn connect_to(&self, peripheral: impl PeripheralId);

    /// Resets the channel's engine state.
    fn reset_channel(&self);

    /// Starts the descriptor chain at `first`.
    fn start_chain(&self, first: &Descriptor);

    /// Disconnects the channel from every peripheral.
    fn detach(&self);

    /// Enables the descriptor owner check; see [`RxChannel::check_owner`].
    fn check_owner(&self, enable: bool);

    /// Enables burst reads of the descriptors; see
    /// [`RxChannel::descriptor_burst`].
    fn descriptor_burst(&self, enable: bool);

    /// Whether the engine has no descriptor left to process.
    fn idle(&self) -> bool;

    /// Whether `interrupt` is latched.
    fn interrupt_pending(&self, interrupt: RxInterrupt) -> bool;

    /// Acknowledges `interrupt`.
    fn clear_interrupt(&self, interrupt: RxInterrupt);

    /// Acknowledges every interrupt source.
    fn clear_all_interrupts(&self);

    /// Stops after the descriptor the engine is working on.
    fn stop_chain(&self);
}

/// Generates the register-level body of one transmit channel.
///
/// The Tx channel-0 `OUT_CONF0` register is a stand-alone register in the SVD
/// while channels 1 and 2 come from an array, so their field names differ by a
/// trailing index; the accessor names are passed in instead of pasted.
macro_rules! impl_tx_channel {
    (
        $n:literal,
        data_burst: $data_burst:ident,
        dscr_burst: $dscr_burst:ident,
        eof_mode: $eof_mode:ident,
        auto_wrback: $auto_wrback:ident,
        etm: $etm:ident,
        rst: $rst:ident
    ) => {
        paste! {
            impl TxChannel<$n> {
                /// The controller's register block.
                ///
                /// # Safety
                ///
                /// [`AhbDma::init`](super::AhbDma::init) has to have run.
                #[inline(always)]
                fn regs() -> &'static ahb_dma::RegisterBlock {
                    // SAFETY: see `RxChannel::regs`.
                    unsafe { &*AHB_DMA::PTR }
                }

                /// Connects the channel to a peripheral's transmit handshake.
                #[inline]
                pub fn connect(&self, peripheral: Peripheral) {
                    Self::regs().[<out_peri_sel_ch $n>].write_peri_out_sel_ch(peripheral as u32);
                }

                /// Loops this channel into its receive partner.
                ///
                /// Both channels of the pair must select the same `dummy` id.
                #[inline]
                pub fn connect_memory(&self, dummy: Dummy) {
                    Self::regs().[<out_peri_sel_ch $n>].write_peri_out_sel_ch(dummy as u32);
                }

                /// Disconnects the channel from every peripheral.
                #[inline]
                pub fn disconnect(&self) {
                    Self::regs().[<out_peri_sel_ch $n>].write_peri_out_sel_ch(0x3f);
                }

                /// Enables the descriptor owner check; see
                /// [`RxChannel::check_owner`].
                #[inline]
                pub fn check_owner(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf1_ch $n>];
                    if enable {
                        reg.set_out_check_owner_ch();
                    } else {
                        reg.clr_out_check_owner_ch();
                    }
                }

                /// Enables burst reads of the data buffers.
                #[inline]
                pub fn data_burst(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_ $data_burst>]();
                    } else {
                        reg.[<clr_ $data_burst>]();
                    }
                }

                /// Enables burst reads of the descriptors themselves.
                #[inline]
                pub fn descriptor_burst(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_ $dscr_burst>]();
                    } else {
                        reg.[<clr_ $dscr_burst>]();
                    }
                }

                /// Lets the engine clear the descriptor's owner bit once the
                /// buffer has been sent.
                ///
                /// Without it software has to clear the bit itself before a
                /// descriptor can be reused.
                #[inline]
                pub fn auto_writeback(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_ $auto_wrback>]();
                    } else {
                        reg.[<clr_ $auto_wrback>]();
                    }
                }

                /// Reports EOF only after the peripheral consumed the data
                /// (`out_eof_mode`).
                ///
                /// On by default in the reference driver: with the mode off the
                /// EOF interrupt can fire while bytes are still queued in the
                /// channel FIFO.
                #[inline]
                pub fn eof_till_data_popped(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_ $eof_mode>]();
                    } else {
                        reg.[<clr_ $eof_mode>]();
                    }
                }

                /// Enables ETM control: the channel then starts on a task from
                /// the event matrix instead of [`start`](Self::start).
                #[inline]
                pub fn enable_etm(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_ $etm>]();
                    } else {
                        reg.[<clr_ $etm>]();
                    }
                }

                /// Programs the CRC unit that runs over this channel's data.
                ///
                /// The engine runs the LFSR over every byte it reads from
                /// memory, one descriptor chain at a time; the result is read
                /// back with [`crc_result`](Self::crc_result).
                ///
                /// # Panics
                ///
                /// Panics if `crc` is wider than [`Crc::AHB_MAX_WIDTH`] bits.
                #[inline]
                pub fn set_crc(&self, crc: &Crc) {
                    assert!(
                        crc.width() <= Crc::AHB_MAX_WIDTH,
                        "GDMA-AHB computes at most 32 CRC bits"
                    );
                    let r = Self::regs();
                    let width = &r.[<tx_crc_width_ch $n>];
                    width.write_tx_crc_width_ch(u32::from((crc.width() - 1) / 8));
                    r.[<out_crc_init_data_ch $n>].write_out_crc_init_data_ch(crc.init());
                    crc.for_each_mask(|bit, lfsr_mask, data_mask| {
                        r.[<tx_crc_en_addr_ch $n>].write_tx_crc_en_addr_ch(bit);
                        r.[<tx_crc_en_wr_data_ch $n>].write_tx_crc_en_wr_data_ch(lfsr_mask);
                        r.[<tx_crc_data_en_addr_ch $n>].write_tx_crc_data_en_addr_ch(bit);
                        r.[<tx_crc_data_en_wr_data_ch $n>]
                            .write_tx_crc_data_en_wr_data_ch(data_mask);
                        width.set_tx_crc_lautch_flga_ch();
                        width.clr_tx_crc_lautch_flga_ch();
                    });
                    let clear = &r.[<out_crc_clear_ch $n>];
                    clear.set_out_crc_clear_ch();
                    clear.clr_out_crc_clear_ch();
                }

                /// CRC the engine computed over the last completed chain.
                #[inline]
                pub fn crc_result(&self) -> u32 {
                    Self::regs().[<out_crc_final_result_ch $n>].out_crc_final_result_ch()
                }

                /// Arbitration priority, 0–5; higher wins.
                #[inline]
                pub fn set_priority(&self, priority: u8) {
                    Self::regs().[<out_pri_ch $n>].write_tx_pri_ch(u32::from(priority));
                }

                /// Resets the channel's FSM and FIFO pointers.
                #[inline]
                pub fn reset(&self) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    reg.[<set_ $rst>]();
                    reg.[<clr_ $rst>]();
                }

                /// Starts walking the descriptor list at `first`.
                #[inline]
                pub fn start(&self, first: &Descriptor) {
                    let r = Self::regs();
                    r.[<out_link_addr_ch $n>].write_outlink_addr_ch(first as *const Descriptor as u32);
                    r.[<out_link_ch $n>].trigger_outlink_start_ch();
                }

                /// Appends a list mounted behind the current one; see
                /// [`RxChannel::restart`].
                #[inline]
                pub fn restart(&self) {
                    Self::regs().[<out_link_ch $n>].trigger_outlink_restart_ch();
                }

                /// Stops after the descriptor the engine is working on.
                #[inline]
                pub fn stop(&self) {
                    Self::regs().[<out_link_ch $n>].trigger_outlink_stop_ch();
                }

                /// Whether the descriptor FSM is idle.
                #[inline]
                pub fn is_idle(&self) -> bool {
                    Self::regs().[<out_link_ch $n>].outlink_park_ch()
                }

                /// Address of the descriptor whose EOF was sent.
                #[inline]
                pub fn last_eof_descriptor(&self) -> *const Descriptor {
                    Self::regs().[<out_eof_des_addr_ch $n>].out_eof_des_addr_ch() as usize
                        as *const Descriptor
                }

                /// Address of the descriptor the engine is prefetching.
                #[inline]
                pub fn current_descriptor(&self) -> *const Descriptor {
                    Self::regs().[<out_dscr_ch $n>].outlink_dscr_ch() as usize as *const Descriptor
                }

                /// Raw interrupt status, in the bit positions of
                /// [`TxInterrupt`].
                #[inline]
                pub fn interrupt_status(&self) -> u32 {
                    Self::regs().[<out_int_raw_ch $n>].read_bits()
                }

                /// Whether `interrupt` is pending.
                #[inline]
                pub fn interrupt_pending(&self, interrupt: TxInterrupt) -> bool {
                    self.interrupt_status() & interrupt.mask() != 0
                }

                /// Enables or disables one interrupt source.
                #[inline]
                pub fn enable_interrupt(&self, interrupt: TxInterrupt, enable: bool) {
                    let reg = &Self::regs().[<out_int_ena_ch $n>];
                    if enable {
                        reg.modify_bits(|bits| bits | interrupt.mask());
                    } else {
                        reg.modify_bits(|bits| bits & !interrupt.mask());
                    }
                }

                /// Acknowledges one interrupt source.
                #[inline]
                pub fn clear_interrupt(&self, interrupt: TxInterrupt) {
                    Self::regs().[<out_int_clr_ch $n>].write_bits(interrupt.mask());
                }

                /// Acknowledges every interrupt source.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    Self::regs().[<out_int_clr_ch $n>].write_bits(u32::MAX);
                }
            }

            impl TransmitChannel for TxChannel<$n> {
                fn connect_to(&self, peripheral: impl PeripheralId) {
                    Self::regs().[<out_peri_sel_ch $n>].write_peri_out_sel_ch(peripheral.id());
                }

                fn reset_channel(&self) {
                    self.reset();
                }

                fn start_chain(&self, first: &Descriptor) {
                    self.start(first);
                }

                fn detach(&self) {
                    self.disconnect();
                }

                fn check_owner(&self, enable: bool) {
                    TxChannel::<$n>::check_owner(self, enable);
                }

                fn descriptor_burst(&self, enable: bool) {
                    TxChannel::<$n>::descriptor_burst(self, enable);
                }

                fn auto_writeback(&self, enable: bool) {
                    TxChannel::<$n>::auto_writeback(self, enable);
                }

                fn eof_till_data_popped(&self, enable: bool) {
                    TxChannel::<$n>::eof_till_data_popped(self, enable);
                }

                fn idle(&self) -> bool {
                    self.is_idle()
                }

                fn interrupt_pending(&self, interrupt: TxInterrupt) -> bool {
                    TxChannel::<$n>::interrupt_pending(self, interrupt)
                }

                fn clear_interrupt(&self, interrupt: TxInterrupt) {
                    TxChannel::<$n>::clear_interrupt(self, interrupt);
                }

                fn stop_chain(&self) {
                    self.stop();
                }

                fn clear_all_interrupts(&self) {
                    TxChannel::<$n>::clear_all_interrupts(self);
                }
            }
        }
    };
}

impl_rx_channel!(0);
impl_rx_channel!(1);
impl_rx_channel!(2);

impl_tx_channel!(
    0,
    data_burst: out_data_burst_en_ch0,
    dscr_burst: outdscr_burst_en_ch0,
    eof_mode: out_eof_mode_ch0,
    auto_wrback: out_auto_wrback_ch0,
    etm: out_etm_en_ch0,
    rst: out_rst_ch0
);
impl_tx_channel!(
    1,
    data_burst: out_data_burst_en_ch,
    dscr_burst: outdscr_burst_en_ch,
    eof_mode: out_eof_mode_ch,
    auto_wrback: out_auto_wrback_ch,
    etm: out_etm_en_ch,
    rst: out_rst_ch
);
impl_tx_channel!(
    2,
    data_burst: out_data_burst_en_ch,
    dscr_burst: outdscr_burst_en_ch,
    eof_mode: out_eof_mode_ch,
    auto_wrback: out_auto_wrback_ch,
    etm: out_etm_en_ch,
    rst: out_rst_ch
);
