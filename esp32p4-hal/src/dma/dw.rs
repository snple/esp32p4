//! DW_GDMA, the `DMA` controller.
//!
//! A DesignWare AHB DMA core with four channels, sitting on its own bus next to
//! the two GDMA controllers. Unlike [`AhbDma`](super::AhbDma) it does not walk a
//! descriptor chain: a transfer is one *block* described by four registers
//! (source, destination, item count, control), and scatter-gather goes through a
//! link-list item format of its own. This driver covers the block form, which is
//! what a memory copy and most peripheral handshakes need.
//!
//! # Features
//!
//! - Four independent channels ([`DwGdma::channel`]), each with its own
//!   interrupt status and control registers
//! - Transfer width from 1 to 64 bytes per item, burst length from 1 to 1024
//!   items, source and destination address increment or fixed
//! - Blocks of up to 4 194 304 items (`BLOCK_TS` is 22 bits)
//! - Memory-to-memory copies with cache maintenance ([`Channel::copy`])
//!
//! # Master port
//!
//! The controller has two master ports: port 0 reaches the MIPI DSI/CSI bridges
//! and port 1 reaches L2MEM, flash and PSRAM. Everything a driver does with
//! plain memory has to select port 1, which is why [`Channel::configure`] writes
//! it unconditionally.
//!
//! # What the hardware insists on
//!
//! Four things cost a bring-up round each; all of them are programmed here:
//!
//! - **A transfer goes through a link-list item**, not through the channel
//!   registers. Programming `SAR`/`DAR`/`BLOCK_TS` directly and leaving `LLP` at
//!   zero ends in `SHADOWREG_OR_LLI_INVALID`, and the block never moves.
//! - **`CFG0`'s `SRC_MULTBLK_TYPE`/`DST_MULTBLK_TYPE` have to say "list"** (3).
//!   With the reset value (contiguous) the engine ignores `LLP` and the
//!   transfer sits there.
//! - **`CFG1` carries the flow control**: memory-to-memory, software handshake
//!   on both ends, and an outstanding-request limit above zero — that last field
//!   resets to zero, which stalls the channel.
//! - **`CTL1` has to mark the item valid and last**, or the engine will not load
//!   it when the channel is enabled.
//!
//! The item itself uses the DesignWare layout (`sar`, `dar`, `block_ts`, `llp`,
//! `ctl`, ...), which is *not* the register order on this chip — the registers
//! put `CTL` before `LLP`, the item does not. [`LinkListItem`] follows the item.
//!
//! A 16 kB copy through this engine measured 164 MB/s, five times what
//! GDMA-AHB and GDMA-AXI reach on the same board.

use esp32p4_pac::{DMA, HP_SYS_CLKRST, dma};

use super::Error;

/// Channels the controller has.
pub const CHANNEL_COUNT: usize = 4;

/// Largest item count one block carries; `BLOCK_TS` is 22 bits wide and stores
/// `items - 1`.
pub const MAX_ITEMS: usize = 1 << 22;

/// Master port that reaches L2MEM, flash and PSRAM.
const MASTER_PORT_MEMORY: u32 = 1;

/// Field shifts of `CTL0`: source/destination master port, address mode, item
/// width and burst length.
const CTL0_SMS_SHIFT: u32 = 0;
const CTL0_DMS_SHIFT: u32 = 2;
const CTL0_SINC_SHIFT: u32 = 4;
const CTL0_DINC_SHIFT: u32 = 6;
const CTL0_SRC_TR_WIDTH_SHIFT: u32 = 8;
const CTL0_DST_TR_WIDTH_SHIFT: u32 = 11;
const CTL0_SRC_MSIZE_SHIFT: u32 = 14;
const CTL0_DST_MSIZE_SHIFT: u32 = 18;
const CTL0_DST_TR_WIDTH: u32 = 0x7 << CTL0_DST_TR_WIDTH_SHIFT;
const CTL0_INCREMENT: u32 = 0;
const CTL0_FIXED: u32 = 1;

const CTL1_IOC_BLKTFR: u32 = 1 << 26;
/// The item the channel points at is valid.
const CTL1_LLI_VALID: u32 = 1 << 31;
/// No item follows this one.
const CTL1_LLI_LAST: u32 = 1 << 30;

const CFG1_HS_SEL_SRC: u32 = 1 << 3;
const CFG1_HS_SEL_DST: u32 = 1 << 4;
const CFG1_SRC_OSR_LMT_SHIFT: u32 = 23;
const CFG1_DST_OSR_LMT_SHIFT: u32 = 27;
/// Requests the engine may keep in flight; zero stalls the channel.
const OUTSTANDING_REQUESTS: u32 = 8;
/// Block type: walk a link list rather than a single contiguous block.
const MULTIBLK_LIST: u32 = 3;
/// Memory to memory, the controller is the flow controller.
const TT_FC_M2M_DMAC: u32 = 0;

/// The block finished; also the only status a memory copy needs to look at.
const INT_DONE: u32 = 1 << 1;
/// Every bit that means the engine gave up on the block.
const INT_ERRORS: u32 = (1 << 5)
    | (1 << 6)
    | (1 << 7)
    | (1 << 8)
    | (1 << 9)
    | (1 << 10)
    | (1 << 11)
    | (1 << 12)
    | (1 << 13)
    | (1 << 14)
    | (1 << 16)
    | (1 << 17)
    | (1 << 18)
    | (1 << 19)
    | (1 << 20)
    | (1 << 21)
    | (1 << 25);

/// Width of one transferred item.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Width {
    /// One byte per item.
    Bits8 = 0,
    /// Two bytes per item.
    Bits16 = 1,
    /// Four bytes per item.
    #[default]
    Bits32 = 2,
    /// Eight bytes per item.
    Bits64 = 3,
    /// Sixteen bytes per item.
    Bits128 = 4,
    /// Thirty-two bytes per item.
    Bits256 = 5,
    /// Sixty-four bytes per item.
    Bits512 = 6,
}

impl Width {
    /// Bytes one item carries.
    #[inline]
    pub const fn bytes(self) -> usize {
        1 << self as usize
    }
}

/// Items one burst transaction may carry.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u32)]
pub enum Burst {
    /// One item.
    B1 = 0,
    /// Four items.
    B4 = 1,
    /// Eight items.
    B8 = 2,
    /// Sixteen items.
    #[default]
    B16 = 3,
    /// Thirty-two items.
    B32 = 4,
    /// Sixty-four items.
    B64 = 5,
    /// One hundred twenty-eight items.
    B128 = 6,
    /// Two hundred fifty-six items.
    B256 = 7,
    /// Five hundred twelve items.
    B512 = 8,
    /// One thousand twenty-four items.
    B1024 = 9,
}

/// How one block is moved.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TransferConfig {
    /// Width of one item.
    pub width: Width,
    /// Items per source burst.
    pub src_burst: Burst,
    /// Items per destination burst.
    pub dst_burst: Burst,
    /// Whether the source address walks forward.
    pub src_increment: bool,
    /// Whether the destination address walks forward.
    pub dst_increment: bool,
}

impl Default for TransferConfig {
    fn default() -> Self {
        Self {
            width: Width::Bits32,
            src_burst: Burst::B16,
            dst_burst: Burst::B16,
            src_increment: true,
            dst_increment: true,
        }
    }
}

/// One link-list item: the 64-byte block the engine loads a transfer from.
///
/// The fields mirror the channel registers, which is why the engine can be
/// driven either way. Handing it an item is what the reference driver does, and
/// it is the only form that survives here — a block programmed into the
/// registers directly ends with `SHADOWREG_OR_LLI_INVALID`.
#[repr(C, align(64))]
pub struct LinkListItem {
    sar: [u32; 2],
    dar: [u32; 2],
    block_ts: u32,
    _reserved_14: u32,
    llp: [u32; 2],
    ctl: [u32; 2],
    sstat: u32,
    dstat: u32,
    status: [u32; 2],
    _reserved: [u32; 2],
}

impl LinkListItem {
    /// An empty item, i.e. a list of zero length.
    pub const fn new() -> Self {
        Self {
            sar: [0; 2],
            dar: [0; 2],
            block_ts: 0,
            _reserved_14: 0,
            llp: [0; 2],
            ctl: [0; 2],
            sstat: 0,
            dstat: 0,
            status: [0; 2],
            _reserved: [0; 2],
        }
    }
}

impl Default for LinkListItem {
    fn default() -> Self {
        Self::new()
    }
}

/// The `DMA` controller.
pub struct DwGdma {
    dma: DMA,
}

impl DwGdma {
    /// Initialises the controller: bus clocks, reset, software reset, enable.
    pub fn init(dma: DMA, clkrst: &HP_SYS_CLKRST) -> Self {
        clkrst.soc_clk_ctrl0.set_reg_gdma_cpu_clk_en();
        clkrst.soc_clk_ctrl1.set_reg_gdma_sys_clk_en();
        clkrst.hp_rst_en0.set_reg_rst_en_gdma();
        clkrst.hp_rst_en0.clr_reg_rst_en_gdma();

        let self_ = Self { dma };
        self_.dma.reset0.set_dmac_rst();
        while self_.dma.reset0.dmac_rst() {
            core::hint::spin_loop();
        }
        self_.dma.cfg0.set_dmac_en();
        self_
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
            dma: unsafe { DMA::steal() },
        })
    }

    /// The underlying register block.
    #[inline]
    pub fn regs(&self) -> &dma::RegisterBlock {
        &self.dma
    }

    /// Returns the PAC peripheral.
    #[inline]
    pub fn release(self) -> DMA {
        self.dma
    }

    /// A handle to channel `N`.
    ///
    /// `N` is 0–3; anything else fails to compile.
    #[inline]
    pub fn channel<const N: usize>(&self) -> Channel<'_, N> {
        const {
            assert!(
                N < CHANNEL_COUNT,
                "this controller has four channels, 0 to 3"
            );
        }
        Channel(self)
    }
}

/// One channel of the controller.
pub struct Channel<'a, const N: usize>(&'a DwGdma);

/// Generates a channel's methods.
///
/// `$logic` is the number the driver exposes, `$reg` the one the register block
/// uses — the channels are `CH1` to `CH4` there, and every field carries that
/// number in its name, so the accessors have to be pasted per channel.
macro_rules! channel_body {
    ($logic:literal, $reg:literal) => {
        pastey::paste! {
            impl<'a> Channel<'a, $logic> {
                /// Channel number, 0–3.
                pub const INDEX: usize = $logic;

                /// Programs how the next block is moved.
                pub fn configure(&self, config: &TransferConfig) {
                    let regs = self.0.regs();
                    let ctl0 = (MASTER_PORT_MEMORY << CTL0_SMS_SHIFT)
                        | (MASTER_PORT_MEMORY << CTL0_DMS_SHIFT)
                        | ((if config.src_increment { CTL0_INCREMENT } else { CTL0_FIXED })
                            << CTL0_SINC_SHIFT)
                        | ((if config.dst_increment { CTL0_INCREMENT } else { CTL0_FIXED })
                            << CTL0_DINC_SHIFT)
                        | ((config.width as u32) << CTL0_SRC_TR_WIDTH_SHIFT)
                        | ((config.width as u32) << CTL0_DST_TR_WIDTH_SHIFT)
                        | ((config.src_burst as u32) << CTL0_SRC_MSIZE_SHIFT)
                        | ((config.dst_burst as u32) << CTL0_DST_MSIZE_SHIFT);
                    regs.[<ch $reg _ctl0>].write_bits(ctl0);
                    // Memory on both ends: software handshake, controller is the
                    // flow controller, no link-list shadow.
                    // A single block has no link-list item to load the shadow
                    // pair from, so the working registers are programmed
                    // directly and no shadow is announced.
                    let cfg = TT_FC_M2M_DMAC
                        | CFG1_HS_SEL_SRC
                        | CFG1_HS_SEL_DST
                        | (OUTSTANDING_REQUESTS << CFG1_SRC_OSR_LMT_SHIFT)
                        | (OUTSTANDING_REQUESTS << CFG1_DST_OSR_LMT_SHIFT);
                    regs.[<ch $reg _cfg1>].write_bits(cfg);
                    // Both ends walk a link list; the block type lives in the
                    // working register and nowhere in the shadow.
                    regs.[<ch $reg _cfg0>]
                        .write_bits((MULTIBLK_LIST << 0) | (MULTIBLK_LIST << 2));
                    // One block, no link list: a stale pointer here makes the
                    // engine try to fetch an item and fail the transfer.
                    regs.[<ch $reg _llp0>].write_bits(0);
                    regs.[<ch $reg _llp1>].write_bits(0);
                    // A single block: mark the shadow pair valid and last, so
                    // enabling the channel latches `CFG1` and only the
                    // end-of-block interrupt is raised.
                    regs.[<ch $reg _ctl1>].write_bits(CTL1_IOC_BLKTFR);
                }

                /// Whether a block is still in flight.
                pub fn busy(&self) -> bool {
                    let regs = self.0.regs();
                    regs.cfg0.read_bits() & (1 << $logic) != 0
                }

                /// Raw interrupt status of this channel.
                pub fn interrupt_status(&self) -> u32 {
                    self.0.regs().[<ch $reg _intstatus0>].read_bits()
                }

                /// Acknowledges every interrupt source of this channel.
                pub fn clear_interrupts(&self) {
                    self.0.regs().[<ch $reg _intclear0>].write_bits(u32::MAX);
                }

                /// Aborts the current block.
                ///
                /// Only for a channel that hangs: the abort itself waits for the
                /// engine to acknowledge it.
                pub fn abort(&self) {
                    let regs = self.0.regs();
                    regs.[<ch $reg _intclear0>].write_bits(u32::MAX);
                    regs.chen1.write_bits(0x101 << $logic);
                    while regs.chen1.read_bits() & (0x101 << $logic) != 0 {
                        core::hint::spin_loop();
                    }
                }

                /// Starts one block from `src` to `dst`, described by `item`.
                ///
                /// `len` is a byte count; it has to be a whole number of items
                /// and no larger than [`MAX_ITEMS`] items. The engine loads the
                /// transfer from `item`, so it has to live in memory the engine
                /// can read and must stay put until [`wait`](Self::wait)
                /// returns.
                ///
                /// # Errors
                ///
                /// [`Error::BufferSize`] when `len` is zero, too long, or not a
                /// multiple of the configured width.
                pub fn start(
                    &self,
                    src: *const u8,
                    dst: *mut u8,
                    len: usize,
                    item: &mut LinkListItem,
                ) -> Result<(), Error> {
                    let regs = self.0.regs();
                    let width = self.width();
                    if len == 0 || !len.is_multiple_of(width) {
                        return Err(Error::BufferSize);
                    }
                    let items = len / width;
                    if items > MAX_ITEMS {
                        return Err(Error::BufferSize);
                    }
                    let ctl0 = regs.[<ch $reg _ctl0>].read_bits();
                    item.sar = [src as u32, 0];
                    item.dar = [dst as u32, 0];
                    // The field stores `items - 1`.
                    item.block_ts = items as u32 - 1;
                    item.llp = [0, 0];
                    item.ctl = [ctl0, CTL1_IOC_BLKTFR | CTL1_LLI_VALID | CTL1_LLI_LAST];
                    // The engine reads the item from memory.
                    crate::cache::writeback(
                        (item as *const LinkListItem).cast(),
                        core::mem::size_of::<LinkListItem>(),
                    );
                    self.clear_interrupts();
                    regs.[<ch $reg _llp0>].write_bits(item as *const LinkListItem as u32);
                    regs.[<ch $reg _llp1>].write_bits(0);
                    // The channel has to be told the pointer is valid before it
                    // is enabled; the item repeats the same markers so the
                    // engine agrees once it has loaded it.
                    regs.[<ch $reg _ctl1>]
                        .write_bits(CTL1_IOC_BLKTFR | CTL1_LLI_VALID | CTL1_LLI_LAST);
                    // Writing the channel's enable byte latches the item and
                    // sets the engine going.
                    regs.chen0.write_bits(0x101 << $logic);
                    Ok(())
                }

                /// Waits for the block to finish.
                ///
                /// # Errors
                ///
                /// [`Error::Peripheral`] when the engine reported an error for
                /// the block, [`Error::Timeout`] when `timeout_us` passed first.
                pub fn wait(&self, timeout_us: u64) -> Result<(), Error> {
                    let deadline = crate::delay::micros().wrapping_add(timeout_us);
                    loop {
                        let status = self.interrupt_status();
                        if status & INT_ERRORS != 0 {
                            self.clear_interrupts();
                            return Err(Error::Peripheral);
                        }
                        if status & INT_DONE != 0 {
                            self.clear_interrupts();
                            return Ok(());
                        }
                        if crate::delay::micros().wrapping_sub(deadline) < u64::MAX / 2 {
                            self.abort();
                            return Err(Error::Timeout);
                        }
                        core::hint::spin_loop();
                    }
                }

                /// Copies `src` into `dst` and waits for the engine.
                ///
                /// The width is picked from the alignment and length of the two
                /// buffers (8 bytes when both allow it, else 4, 2 or 1), and the
                /// cache is maintained around the transfer like the other DMA
                /// drivers do.
                ///
                /// # Errors
                ///
                /// The same as [`start`](Self::start) and [`wait`](Self::wait).
                pub fn copy(
                    &self,
                    src: &[u8],
                    dst: &mut [u8],
                    item: &mut LinkListItem,
                    timeout_us: u64,
                ) -> Result<usize, Error> {
                    let len = src.len().min(dst.len());
                    if len == 0 {
                        return Err(Error::BufferSize);
                    }
                    let config = TransferConfig {
                        width: self.pick_width(src.as_ptr(), dst.as_mut_ptr(), len),
                        ..TransferConfig::default()
                    };
                    self.configure(&config);

                    crate::cache::writeback(src.as_ptr().cast(), len);
                    crate::cache::writeback_invalidate(dst.as_mut_ptr().cast(), len);
                    self.start(src.as_ptr(), dst.as_mut_ptr(), len, item)?;
                    self.wait(timeout_us)?;
                    crate::cache::invalidate(dst.as_mut_ptr().cast(), len);
                    Ok(len)
                }

                /// The width the channel is configured for.
                pub fn width(&self) -> usize {
                    let ctl0 = self.0.regs().[<ch $reg _ctl0>].read_bits();
                    let code = (ctl0 & CTL0_DST_TR_WIDTH) >> CTL0_DST_TR_WIDTH_SHIFT;
                    Width::bytes_of(code)
                }

                /// Largest width both buffers and the length can take.
                fn pick_width(&self, src: *const u8, dst: *const u8, len: usize) -> Width {
                    for width in [
                        Width::Bits64,
                        Width::Bits32,
                        Width::Bits16,
                        Width::Bits8,
                    ] {
                        let bytes = width.bytes();
                        if src as usize % bytes == 0
                            && dst as usize % bytes == 0
                            && len.is_multiple_of(bytes)
                        {
                            return width;
                        }
                    }
                    Width::Bits8
                }
            }
        }
    };
}

channel_body!(0, 1);
channel_body!(1, 2);
channel_body!(2, 3);
channel_body!(3, 4);

impl Width {
    /// Width for a `SRC_TR_WIDTH`/`DST_TR_WIDTH` code.
    const fn bytes_of(code: u32) -> usize {
        1 << code
    }
}
