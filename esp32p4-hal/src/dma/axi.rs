//! GDMA-AXI channel handles, the `AXI_DMA` controller.
//!
//! The AXI engine moves data for the peripherals on the AXI bus — LCD/CAM,
//! GP-SPI2/3, PARLIO, AES and SHA (TRM 6.1). It has three transmit/receive
//! pairs, [`Descriptor`] lists in memory and the same interrupt layout as
//! GDMA-AHB, so a peripheral driver written against
//! [`TransmitChannel`] / [`ReceiveChannel`] works with either controller.
//!
//! [`AxiRxChannel`] and [`AxiTxChannel`] are zero-sized tokens parameterised by
//! the channel index, like their AHB counterparts: the register names are fixed
//! at compile time and only indices 0–2 have implementations.
//!
//! # Memory
//!
//! [`AxiDma::init`] programs the two address windows the engine checks
//! descriptors against, the same values the reference driver uses: L2MEM from
//! `0x4FC0_0000` to `0x4FFC_0000`, and the cached flash/PSRAM window from
//! `0x4000_0000` to `0x4C00_0000`. Internal SRAM therefore works from its
//! ordinary address; the direct aliases (`0x8FF0_0000`, `0x8800_0000`) are
//! outside both windows.
//!
//! Descriptors follow the layout in [`super::descriptor`], reached through the
//! direct alias exactly as on GDMA-AHB. Buffers are the caller's memory and are
//! stored in the descriptor as given, so a cached buffer needs cache
//! maintenance; a receiving buffer must additionally be word aligned, since the
//! engine writes it in bursts.

use core::marker::PhantomData;

use esp32p4_pac::{AXI_DMA, HP_SYS_CLKRST, axi_dma};
use pastey::paste;

use super::descriptor::Descriptor;
use super::{PeripheralId, ReceiveChannel, RxInterrupt, TransmitChannel, TxInterrupt};
use crate::Steal;

/// Start of the internal-memory window the engine accepts, L2MEM.
const INTR_MEM_START: u32 = 0x4fc0_0000;
/// End of the internal-memory window, exclusive.
const INTR_MEM_END: u32 = 0x4ffc_0000;
/// Start of the external-memory window, the cached flash/PSRAM aliases.
const EXTR_MEM_START: u32 = 0x4000_0000;
/// End of the external-memory window, exclusive.
const EXTR_MEM_END: u32 = 0x4c00_0000;

/// A peripheral whose handshake a GDMA-AXI channel can use.
///
/// Values are `PERI_IN_SEL_CHn` / `PERI_OUT_SEL_CHn` from TRM table 6.4-2. They
/// are *not* the GDMA-AHB numbers: `1` selects GP-SPI2 here and
/// [`Dummy::D1`](super::Dummy::D1) there, which is why the two controllers have
/// separate enums.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum AxiPeripheral {
    /// LCD or camera.
    LcdCam = 0,
    /// GP-SPI2, both directions.
    Spi2 = 1,
    /// GP-SPI3, both directions.
    Spi3 = 2,
    /// Parallel IO.
    Parlio = 3,
    /// AES accelerator.
    Aes = 4,
    /// SHA accelerator.
    Sha = 5,
}

impl PeripheralId for AxiPeripheral {
    #[inline]
    fn id(self) -> u32 {
        self as u32
    }
}

/// The GDMA-AXI controller.
///
/// Owns the PAC peripheral; the channels are zero-sized handles over its
/// register blocks, the same arrangement as [`AhbDma`](super::AhbDma).
pub struct AxiDma {
    axi: AXI_DMA,
}

impl AxiDma {
    /// Takes ownership of the controller and brings it up.
    ///
    /// Enables the bus clock, pulses the module reset and programs the memory
    /// windows used for descriptor checks.
    pub fn init(axi: AXI_DMA, clkrst: &HP_SYS_CLKRST) -> Self {
        clkrst.soc_clk_ctrl1.set_reg_axi_pdma_sys_clk_en();
        clkrst.hp_rst_en1.set_reg_rst_en_axi_pdma();
        clkrst.hp_rst_en1.clr_reg_rst_en_axi_pdma();

        // The register block has its own clock gate; without it the channel
        // registers read back as zero even though the bus clock is on.
        axi.misc_conf.set_clk_en();
        // AXI read and write data paths are separate state machines, both have
        // to be restarted so a driver that ran before us leaves no traffic.
        axi.misc_conf.set_axim_rst_rd_inter();
        axi.misc_conf.clr_axim_rst_rd_inter();
        axi.misc_conf.set_axim_rst_wr_inter();
        axi.misc_conf.clr_axim_rst_wr_inter();

        axi.intr_mem_start_addr
            .write_access_intr_mem_start_addr(INTR_MEM_START);
        axi.intr_mem_end_addr
            .write_access_intr_mem_end_addr(INTR_MEM_END);
        axi.extr_mem_start_addr
            .write_access_extr_mem_start_addr(EXTR_MEM_START);
        axi.extr_mem_end_addr
            .write_access_extr_mem_end_addr(EXTR_MEM_END);

        Self { axi }
    }

    /// Obtains a handle without ownership checks.
    ///
    /// # Safety
    ///
    /// [`init`](Self::init) must have run, and the returned handle must not be
    /// used to reconfigure the controller while the owning instance is in use.
    #[inline]
    pub unsafe fn steal() -> crate::Steal<Self> {
        crate::Steal(Self {
            axi: unsafe { AXI_DMA::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &axi_dma::RegisterBlock {
        &self.axi
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> AXI_DMA {
        self.axi
    }

    /// Receive channel `N`.
    ///
    /// The handle is zero-sized; nothing stops two callers from asking for the
    /// same channel, so a driver has to keep track of which channels it owns.
    #[inline]
    pub fn rx_channel<const N: usize>(&self) -> AxiRxChannel<N> {
        AxiRxChannel::new()
    }

    /// Transmit channel `N`.
    ///
    /// See [`rx_channel`](Self::rx_channel).
    #[inline]
    pub fn tx_channel<const N: usize>(&self) -> AxiTxChannel<N> {
        AxiTxChannel::new()
    }
}

/// Receive channel `N`, 0–2.
pub struct AxiRxChannel<const N: usize> {
    _marker: PhantomData<()>,
}

impl<const N: usize> AxiRxChannel<N> {
    /// Channel index, 0–2.
    pub const CHANNEL: usize = N;

    /// A handle over the registers; the controller is reached through
    /// `AXI_DMA::PTR`, so the handle carries no state.
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
    /// The controller must have been initialised with [`AxiDma::init`], and the
    /// caller must not use the same channel from two contexts at once.
    #[inline]
    pub const unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            _marker: PhantomData,
        })
    }
}

/// Transmit channel `N`, 0–2.
pub struct AxiTxChannel<const N: usize> {
    _marker: PhantomData<()>,
}

impl<const N: usize> AxiTxChannel<N> {
    /// Channel index, 0–2.
    pub const CHANNEL: usize = N;

    /// A handle over the registers; see [`AxiRxChannel::new`].
    #[inline]
    pub(crate) const fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    /// Obtains a handle without ownership checks.
    ///
    /// See [`AxiRxChannel::steal`] for the requirements.
    ///
    /// # Safety
    ///
    /// The controller must have been initialised with [`AxiDma::init`], and the
    /// caller must not use the same channel from two contexts at once.
    #[inline]
    pub const unsafe fn steal() -> Steal<Self> {
        Steal(Self {
            _marker: PhantomData,
        })
    }
}

/// Interrupt bits the receive direction reports for one [`RxInterrupt`].
///
/// The AXI FIFOs are three levels deep and each level has its own overflow and
/// underflow bit, so one `RxInterrupt` covers a group of bits here while it is
/// a single bit on GDMA-AHB. The low bits line up with the AHB ones, which is
/// what lets both controllers share the enum.
const fn rx_mask(interrupt: RxInterrupt) -> u32 {
    match interrupt {
        // `INFIFO_L1/L2/L3_OVF` are bits 5, 7 and 9.
        RxInterrupt::FifoOverflow => 0x2a0,
        // `INFIFO_L1/L2/L3_UDF` are bits 6, 8 and 10.
        RxInterrupt::FifoUnderflow => 0x540,
        other => other.mask(),
    }
}

/// Interrupt bits the transmit direction reports for one [`TxInterrupt`]; see
/// [`rx_mask`].
const fn tx_mask(interrupt: TxInterrupt) -> u32 {
    match interrupt {
        // `OUTFIFO_L1/L2/L3_OVF` are bits 4, 6 and 8.
        TxInterrupt::FifoOverflow => 0x150,
        // `OUTFIFO_L1/L2/L3_UDF` are bits 5, 7 and 9.
        TxInterrupt::FifoUnderflow => 0x2a0,
        other => other.mask(),
    }
}

/// Generates the register-level body of one receive channel.
macro_rules! impl_rx_channel {
    ($n:literal) => {
        paste! {
            impl AxiRxChannel<$n> {
                /// The controller's register block.
                ///
                /// # Safety
                ///
                /// [`AxiDma::init`] has to have run.
                #[inline(always)]
                fn regs() -> &'static axi_dma::RegisterBlock {
                    // SAFETY: the controller is owned by `AxiDma`; a channel
                    // handle is a zero-sized token over its registers.
                    unsafe { &*AXI_DMA::PTR }
                }

                /// Connects the channel to a peripheral's receive handshake.
                ///
                /// One peripheral can feed only one channel at a time; the
                /// driver of the peripheral owns that choice.
                #[inline]
                pub fn connect(&self, peripheral: AxiPeripheral) {
                    let r = Self::regs();
                    r.[<in_peri_sel_ch $n>].write_peri_in_sel_ch(peripheral.id());
                    r.[<in_conf0_ch $n>].clr_mem_trans_en_ch();
                }

                /// Disconnects the channel from every peripheral.
                #[inline]
                pub fn disconnect(&self) {
                    let r = Self::regs();
                    r.[<in_peri_sel_ch $n>].write_peri_in_sel_ch(0x3f);
                    r.[<in_conf0_ch $n>].clr_mem_trans_en_ch();
                }

                /// Enables the descriptor owner check.
                ///
                /// With the check on, the engine skips descriptors whose
                /// `owner` bit is not [`Owner::Dma`](super::Owner::Dma).
                #[inline]
                pub fn check_owner(&self, enable: bool) {
                    let reg = &Self::regs().[<in_conf1_ch $n>];
                    if enable {
                        reg.set_in_check_owner_ch();
                    } else {
                        reg.clr_in_check_owner_ch();
                    }
                }

                /// Enables burst reads of the descriptors themselves.
                ///
                /// Data bursts are always on in the AXI engine, so only the
                /// descriptor fetch is switchable.
                #[inline]
                pub fn descriptor_burst(&self, enable: bool) {
                    let reg = &Self::regs().[<in_conf0_ch $n>];
                    if enable {
                        reg.set_indscr_burst_en_ch();
                    } else {
                        reg.clr_indscr_burst_en_ch();
                    }
                }

                /// Arbitration priority, 0–5; higher wins.
                #[inline]
                pub fn set_priority(&self, priority: u8) {
                    Self::regs().[<in_pri_ch $n>].write_rx_pri_ch(u32::from(priority));
                }

                /// Resets the channel's FSM and FIFO pointers.
                ///
                /// Without it a channel that was stopped mid-chain keeps its
                /// write offset, and the next chain starts in the wrong slot.
                #[inline]
                pub fn reset(&self) {
                    let reg = &Self::regs().[<in_conf0_ch $n>];
                    reg.set_in_rst_ch();
                    reg.clr_in_rst_ch();
                }

                /// Starts walking the descriptor list at `first`.
                #[inline]
                pub fn start(&self, first: &Descriptor) {
                    let r = Self::regs();
                    r.[<in_link2_ch $n>].write_inlink_addr_ch(first as *const Descriptor as u32);
                    r.[<in_link1_ch $n>].trigger_inlink_start_ch();
                }

                /// Appends a list mounted behind the current one.
                ///
                /// Point the last descriptor of the running list at the new
                /// head first, then call this; the engine picks the new
                /// descriptors up when it reaches it.
                #[inline]
                pub fn restart(&self) {
                    Self::regs().[<in_link1_ch $n>].trigger_inlink_restart_ch();
                }

                /// Stops after the descriptor the engine is working on.
                #[inline]
                pub fn stop(&self) {
                    Self::regs().[<in_link1_ch $n>].trigger_inlink_stop_ch();
                }

                /// Whether the descriptor FSM is idle.
                #[inline]
                pub fn is_idle(&self) -> bool {
                    Self::regs().[<in_link1_ch $n>].inlink_park_ch()
                }

                /// Address of the descriptor whose EOF was received.
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
                    self.interrupt_status() & rx_mask(interrupt) != 0
                }

                /// Enables or disables one interrupt source.
                #[inline]
                pub fn enable_interrupt(&self, interrupt: RxInterrupt, enable: bool) {
                    let mask = rx_mask(interrupt);
                    let reg = &Self::regs().[<in_int_ena_ch $n>];
                    if enable {
                        reg.modify_bits(|bits| bits | mask);
                    } else {
                        reg.modify_bits(|bits| bits & !mask);
                    }
                }

                /// Acknowledges one interrupt source.
                #[inline]
                pub fn clear_interrupt(&self, interrupt: RxInterrupt) {
                    Self::regs().[<in_int_clr_ch $n>].write_bits(rx_mask(interrupt));
                }

                /// Acknowledges every interrupt source.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    Self::regs().[<in_int_clr_ch $n>].write_bits(u32::MAX);
                }
            }

            impl ReceiveChannel for AxiRxChannel<$n> {
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
                    AxiRxChannel::<$n>::check_owner(self, enable);
                }

                fn descriptor_burst(&self, enable: bool) {
                    AxiRxChannel::<$n>::descriptor_burst(self, enable);
                }

                fn idle(&self) -> bool {
                    self.is_idle()
                }

                fn interrupt_pending(&self, interrupt: RxInterrupt) -> bool {
                    AxiRxChannel::<$n>::interrupt_pending(self, interrupt)
                }

                fn clear_interrupt(&self, interrupt: RxInterrupt) {
                    AxiRxChannel::<$n>::clear_interrupt(self, interrupt);
                }

                fn clear_all_interrupts(&self) {
                    AxiRxChannel::<$n>::clear_all_interrupts(self);
                }

                fn stop_chain(&self) {
                    AxiRxChannel::<$n>::stop(self);
                }
            }
        }
    };
}

/// Generates the register-level body of one transmit channel.
///
/// Unlike the AHB block, every `OUT_*` field of the AXI block carries the
/// channel index, so the accessors are pasted from the channel number.
macro_rules! impl_tx_channel {
    ($n:literal) => {
        paste! {
            impl AxiTxChannel<$n> {
                /// The controller's register block.
                ///
                /// # Safety
                ///
                /// [`AxiDma::init`] has to have run.
                #[inline(always)]
                fn regs() -> &'static axi_dma::RegisterBlock {
                    // SAFETY: see `AxiRxChannel::regs`.
                    unsafe { &*AXI_DMA::PTR }
                }

                /// Connects the channel to a peripheral's transmit handshake.
                #[inline]
                pub fn connect(&self, peripheral: AxiPeripheral) {
                    Self::regs().[<out_peri_sel_ch $n>].write_peri_out_sel_ch(peripheral.id());
                }

                /// Disconnects the channel from every peripheral.
                #[inline]
                pub fn disconnect(&self) {
                    Self::regs().[<out_peri_sel_ch $n>].write_peri_out_sel_ch(0x3f);
                }

                /// Enables the descriptor owner check; see
                /// [`AxiRxChannel::check_owner`].
                #[inline]
                pub fn check_owner(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf1_ch $n>];
                    if enable {
                        reg.set_out_check_owner_ch();
                    } else {
                        reg.clr_out_check_owner_ch();
                    }
                }

                /// Enables burst reads of the descriptors themselves.
                #[inline]
                pub fn descriptor_burst(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_outdscr_burst_en_ch $n>]();
                    } else {
                        reg.[<clr_outdscr_burst_en_ch $n>]();
                    }
                }

                /// Lets the engine clear the descriptor's owner bit once the
                /// buffer has been sent.
                #[inline]
                pub fn auto_writeback(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_out_auto_wrback_ch $n>]();
                    } else {
                        reg.[<clr_out_auto_wrback_ch $n>]();
                    }
                }

                /// Reports EOF only after the peripheral consumed the data
                /// (`out_eof_mode`).
                ///
                /// On by default in the reference driver: with the mode off the
                /// EOF is raised when the buffer left the FIFO, which is too
                /// early for a peripheral that is still shifting it out.
                #[inline]
                pub fn eof_till_data_popped(&self, enable: bool) {
                    let reg = &Self::regs().[<out_conf0_ch $n>];
                    if enable {
                        reg.[<set_out_eof_mode_ch $n>]();
                    } else {
                        reg.[<clr_out_eof_mode_ch $n>]();
                    }
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
                    reg.[<set_out_rst_ch $n>]();
                    reg.[<clr_out_rst_ch $n>]();
                }

                /// Starts walking the descriptor list at `first`.
                #[inline]
                pub fn start(&self, first: &Descriptor) {
                    let r = Self::regs();
                    r.[<out_link2_ch $n>]
                        .write_outlink_addr_ch(first as *const Descriptor as u32);
                    r.[<out_link1_ch $n>].trigger_outlink_start_ch();
                }

                /// Appends a list mounted behind the current one; see
                /// [`AxiRxChannel::restart`].
                #[inline]
                pub fn restart(&self) {
                    Self::regs().[<out_link1_ch $n>].trigger_outlink_restart_ch();
                }

                /// Stops after the descriptor the engine is working on.
                #[inline]
                pub fn stop(&self) {
                    Self::regs().[<out_link1_ch $n>].trigger_outlink_stop_ch();
                }

                /// Whether the descriptor FSM is idle.
                #[inline]
                pub fn is_idle(&self) -> bool {
                    Self::regs().[<out_link1_ch $n>].outlink_park_ch()
                }

                /// Address of the descriptor whose buffer finished sending.
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
                    self.interrupt_status() & tx_mask(interrupt) != 0
                }

                /// Enables or disables one interrupt source.
                #[inline]
                pub fn enable_interrupt(&self, interrupt: TxInterrupt, enable: bool) {
                    let mask = tx_mask(interrupt);
                    let reg = &Self::regs().[<out_int_ena_ch $n>];
                    if enable {
                        reg.modify_bits(|bits| bits | mask);
                    } else {
                        reg.modify_bits(|bits| bits & !mask);
                    }
                }

                /// Acknowledges one interrupt source.
                #[inline]
                pub fn clear_interrupt(&self, interrupt: TxInterrupt) {
                    Self::regs().[<out_int_clr_ch $n>].write_bits(tx_mask(interrupt));
                }

                /// Acknowledges every interrupt source.
                #[inline]
                pub fn clear_all_interrupts(&self) {
                    Self::regs().[<out_int_clr_ch $n>].write_bits(u32::MAX);
                }
            }

            impl TransmitChannel for AxiTxChannel<$n> {
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
                    AxiTxChannel::<$n>::check_owner(self, enable);
                }

                fn descriptor_burst(&self, enable: bool) {
                    AxiTxChannel::<$n>::descriptor_burst(self, enable);
                }

                fn auto_writeback(&self, enable: bool) {
                    AxiTxChannel::<$n>::auto_writeback(self, enable);
                }

                fn eof_till_data_popped(&self, enable: bool) {
                    AxiTxChannel::<$n>::eof_till_data_popped(self, enable);
                }

                fn idle(&self) -> bool {
                    self.is_idle()
                }

                fn interrupt_pending(&self, interrupt: TxInterrupt) -> bool {
                    AxiTxChannel::<$n>::interrupt_pending(self, interrupt)
                }

                fn clear_interrupt(&self, interrupt: TxInterrupt) {
                    AxiTxChannel::<$n>::clear_interrupt(self, interrupt);
                }

                fn stop_chain(&self) {
                    AxiTxChannel::<$n>::stop(self);
                }

                fn clear_all_interrupts(&self) {
                    AxiTxChannel::<$n>::clear_all_interrupts(self);
                }
            }
        }
    };
}

impl_rx_channel!(0);
impl_rx_channel!(1);
impl_rx_channel!(2);
impl_tx_channel!(0);
impl_tx_channel!(1);
impl_tx_channel!(2);
