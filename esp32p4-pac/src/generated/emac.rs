//! EMAC
//! Ethernet MAC (DesignWare 10/100 MAC + DMA)

#![allow(dead_code)]

pub const BASE: u32 = 0x50098000;

/// EMACCONFIG Register
///
/// EMACCONFIG register
///
/// Address offset: 0x0000
/// Reset value: 0x00008000
///
/// ## Fields
///
/// - **SAIRC** `[30:28]` (RW)
///
///   SAIRC
///
/// - **ASS2KP** `[27]` (RW)
///
///   ASS2KP
///
/// - **EMACCRC_STRIP** `[25]` (RW)
///
///   EMACCRC_STRIP
///
/// - **EMACWATCHDOG** `[23]` (RW)
///
///   EMACWATCHDOG
///
/// - **EMACJABBER** `[22]` (RW)
///
///   EMACJABBER
///
/// - **EMACJUMBOFRAME** `[20]` (RW)
///
///   EMACJUMBOFRAME
///
/// - **EMACINTERFRAMEGAP** `[19:17]` (RW)
///
///   EMACINTERFRAMEGAP
///
/// - **EMACDISABLECRS** `[16]` (RW)
///
///   EMACDISABLECRS
///
/// - **EMACMII** `[15]` (RW)
///
///   EMACMII
///
/// - **EMACFESPEED** `[14]` (RW)
///
///   EMACFESPEED
///
/// - **EMACRXOWN** `[13]` (RW)
///
///   EMACRXOWN
///
/// - **EMACLOOPBACK** `[12]` (RW)
///
///   EMACLOOPBACK
///
/// - **EMACDUPLEX** `[11]` (RW)
///
///   EMACDUPLEX
///
/// - **EMACRXIPCOFFLOAD** `[10]` (RW)
///
///   EMACRXIPCOFFLOAD
///
/// - **EMACRETRY** `[9]` (RW)
///
///   EMACRETRY
///
/// - **EMACPADCRCSTRIP** `[7]` (RW)
///
///   EMACPADCRCSTRIP
///
/// - **EMACBACKOFFLIMIT** `[6:5]` (RW)
///
///   EMACBACKOFFLIMIT
///
/// - **EMACDEFERRALCHECK** `[4]` (RW)
///
///   EMACDEFERRALCHECK
///
/// - **EMACTX** `[3]` (RW)
///
///   EMACTX
///
/// - **EMACRX** `[2]` (RW)
///
///   EMACRX
///
/// - **PLTF** `[1:0]` (RW)
///
///   PLTF
#[repr(transparent)]
pub struct Emacconfig {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacconfig {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacconfig {
    // ======== Field Constants ========

    /// **SAIRC**
    ///
    /// SAIRC
    ///
    /// Bits `[30:28]`, mask `0x70000000`
    pub const SAIRC_OFFSET: u32 = 28;
    pub const SAIRC_MASK: u32 = 0x70000000;

    /// **ASS2KP**
    ///
    /// ASS2KP
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const ASS2KP_OFFSET: u32 = 27;
    pub const ASS2KP_MASK: u32 = 0x08000000;
    pub const ASS2KP: u32 = 1 << 27;

    /// **EMACCRC_STRIP**
    ///
    /// EMACCRC_STRIP
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const EMACCRC_STRIP_OFFSET: u32 = 25;
    pub const EMACCRC_STRIP_MASK: u32 = 0x02000000;
    pub const EMACCRC_STRIP: u32 = 1 << 25;

    /// **EMACWATCHDOG**
    ///
    /// EMACWATCHDOG
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const EMACWATCHDOG_OFFSET: u32 = 23;
    pub const EMACWATCHDOG_MASK: u32 = 0x00800000;
    pub const EMACWATCHDOG: u32 = 1 << 23;

    /// **EMACJABBER**
    ///
    /// EMACJABBER
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const EMACJABBER_OFFSET: u32 = 22;
    pub const EMACJABBER_MASK: u32 = 0x00400000;
    pub const EMACJABBER: u32 = 1 << 22;

    /// **EMACJUMBOFRAME**
    ///
    /// EMACJUMBOFRAME
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const EMACJUMBOFRAME_OFFSET: u32 = 20;
    pub const EMACJUMBOFRAME_MASK: u32 = 0x00100000;
    pub const EMACJUMBOFRAME: u32 = 1 << 20;

    /// **EMACINTERFRAMEGAP**
    ///
    /// EMACINTERFRAMEGAP
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const EMACINTERFRAMEGAP_OFFSET: u32 = 17;
    pub const EMACINTERFRAMEGAP_MASK: u32 = 0x000e0000;

    /// **EMACDISABLECRS**
    ///
    /// EMACDISABLECRS
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const EMACDISABLECRS_OFFSET: u32 = 16;
    pub const EMACDISABLECRS_MASK: u32 = 0x00010000;
    pub const EMACDISABLECRS: u32 = 1 << 16;

    /// **EMACMII**
    ///
    /// EMACMII
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const EMACMII_OFFSET: u32 = 15;
    pub const EMACMII_MASK: u32 = 0x00008000;
    pub const EMACMII: u32 = 1 << 15;

    /// **EMACFESPEED**
    ///
    /// EMACFESPEED
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EMACFESPEED_OFFSET: u32 = 14;
    pub const EMACFESPEED_MASK: u32 = 0x00004000;
    pub const EMACFESPEED: u32 = 1 << 14;

    /// **EMACRXOWN**
    ///
    /// EMACRXOWN
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const EMACRXOWN_OFFSET: u32 = 13;
    pub const EMACRXOWN_MASK: u32 = 0x00002000;
    pub const EMACRXOWN: u32 = 1 << 13;

    /// **EMACLOOPBACK**
    ///
    /// EMACLOOPBACK
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const EMACLOOPBACK_OFFSET: u32 = 12;
    pub const EMACLOOPBACK_MASK: u32 = 0x00001000;
    pub const EMACLOOPBACK: u32 = 1 << 12;

    /// **EMACDUPLEX**
    ///
    /// EMACDUPLEX
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const EMACDUPLEX_OFFSET: u32 = 11;
    pub const EMACDUPLEX_MASK: u32 = 0x00000800;
    pub const EMACDUPLEX: u32 = 1 << 11;

    /// **EMACRXIPCOFFLOAD**
    ///
    /// EMACRXIPCOFFLOAD
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EMACRXIPCOFFLOAD_OFFSET: u32 = 10;
    pub const EMACRXIPCOFFLOAD_MASK: u32 = 0x00000400;
    pub const EMACRXIPCOFFLOAD: u32 = 1 << 10;

    /// **EMACRETRY**
    ///
    /// EMACRETRY
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const EMACRETRY_OFFSET: u32 = 9;
    pub const EMACRETRY_MASK: u32 = 0x00000200;
    pub const EMACRETRY: u32 = 1 << 9;

    /// **EMACPADCRCSTRIP**
    ///
    /// EMACPADCRCSTRIP
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const EMACPADCRCSTRIP_OFFSET: u32 = 7;
    pub const EMACPADCRCSTRIP_MASK: u32 = 0x00000080;
    pub const EMACPADCRCSTRIP: u32 = 1 << 7;

    /// **EMACBACKOFFLIMIT**
    ///
    /// EMACBACKOFFLIMIT
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const EMACBACKOFFLIMIT_OFFSET: u32 = 5;
    pub const EMACBACKOFFLIMIT_MASK: u32 = 0x00000060;

    /// **EMACDEFERRALCHECK**
    ///
    /// EMACDEFERRALCHECK
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const EMACDEFERRALCHECK_OFFSET: u32 = 4;
    pub const EMACDEFERRALCHECK_MASK: u32 = 0x00000010;
    pub const EMACDEFERRALCHECK: u32 = 1 << 4;

    /// **EMACTX**
    ///
    /// EMACTX
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const EMACTX_OFFSET: u32 = 3;
    pub const EMACTX_MASK: u32 = 0x00000008;
    pub const EMACTX: u32 = 1 << 3;

    /// **EMACRX**
    ///
    /// EMACRX
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const EMACRX_OFFSET: u32 = 2;
    pub const EMACRX_MASK: u32 = 0x00000004;
    pub const EMACRX: u32 = 1 << 2;

    /// **PLTF**
    ///
    /// PLTF
    ///
    /// Bits `[1:0]`, mask `0x00000003`
    pub const PLTF_OFFSET: u32 = 0;
    pub const PLTF_MASK: u32 = 0x00000003;

    // ======== Field Accessors ========

    /// **SAIRC** `[30:28]` (RW)
    ///
    /// SAIRC
    #[inline(always)]
    pub fn sairc(&self) -> u32 {
        (self.value.read_bits() & Self::SAIRC_MASK) >> Self::SAIRC_OFFSET
    }

    /// Write **SAIRC** `[30:28]` (RW)
    ///
    /// SAIRC
    #[inline(always)]
    pub fn write_sairc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SAIRC_MASK) | ((value << Self::SAIRC_OFFSET) & Self::SAIRC_MASK)
        });
    }

    /// **ASS2KP** `[27]` (RW)
    ///
    /// ASS2KP
    #[inline(always)]
    pub fn ass2kp(&self) -> bool {
        (self.value.read_bits() & Self::ASS2KP_MASK) != 0
    }

    /// Set ASS2KP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ass2kp(&self) {
        self.value.modify_bits(|r| r | Self::ASS2KP_MASK);
    }

    /// Clear ASS2KP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ass2kp(&self) {
        self.value.modify_bits(|r| r & !Self::ASS2KP_MASK);
    }

    /// **EMACCRC_STRIP** `[25]` (RW)
    ///
    /// EMACCRC_STRIP
    #[inline(always)]
    pub fn emaccrc_strip(&self) -> bool {
        (self.value.read_bits() & Self::EMACCRC_STRIP_MASK) != 0
    }

    /// Set EMACCRC_STRIP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emaccrc_strip(&self) {
        self.value.modify_bits(|r| r | Self::EMACCRC_STRIP_MASK);
    }

    /// Clear EMACCRC_STRIP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emaccrc_strip(&self) {
        self.value.modify_bits(|r| r & !Self::EMACCRC_STRIP_MASK);
    }

    /// **EMACWATCHDOG** `[23]` (RW)
    ///
    /// EMACWATCHDOG
    #[inline(always)]
    pub fn emacwatchdog(&self) -> bool {
        (self.value.read_bits() & Self::EMACWATCHDOG_MASK) != 0
    }

    /// Set EMACWATCHDOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacwatchdog(&self) {
        self.value.modify_bits(|r| r | Self::EMACWATCHDOG_MASK);
    }

    /// Clear EMACWATCHDOG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacwatchdog(&self) {
        self.value.modify_bits(|r| r & !Self::EMACWATCHDOG_MASK);
    }

    /// **EMACJABBER** `[22]` (RW)
    ///
    /// EMACJABBER
    #[inline(always)]
    pub fn emacjabber(&self) -> bool {
        (self.value.read_bits() & Self::EMACJABBER_MASK) != 0
    }

    /// Set EMACJABBER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacjabber(&self) {
        self.value.modify_bits(|r| r | Self::EMACJABBER_MASK);
    }

    /// Clear EMACJABBER bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacjabber(&self) {
        self.value.modify_bits(|r| r & !Self::EMACJABBER_MASK);
    }

    /// **EMACJUMBOFRAME** `[20]` (RW)
    ///
    /// EMACJUMBOFRAME
    #[inline(always)]
    pub fn emacjumboframe(&self) -> bool {
        (self.value.read_bits() & Self::EMACJUMBOFRAME_MASK) != 0
    }

    /// Set EMACJUMBOFRAME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacjumboframe(&self) {
        self.value.modify_bits(|r| r | Self::EMACJUMBOFRAME_MASK);
    }

    /// Clear EMACJUMBOFRAME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacjumboframe(&self) {
        self.value.modify_bits(|r| r & !Self::EMACJUMBOFRAME_MASK);
    }

    /// **EMACINTERFRAMEGAP** `[19:17]` (RW)
    ///
    /// EMACINTERFRAMEGAP
    #[inline(always)]
    pub fn emacinterframegap(&self) -> u32 {
        (self.value.read_bits() & Self::EMACINTERFRAMEGAP_MASK) >> Self::EMACINTERFRAMEGAP_OFFSET
    }

    /// Write **EMACINTERFRAMEGAP** `[19:17]` (RW)
    ///
    /// EMACINTERFRAMEGAP
    #[inline(always)]
    pub fn write_emacinterframegap(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EMACINTERFRAMEGAP_MASK)
                | ((value << Self::EMACINTERFRAMEGAP_OFFSET) & Self::EMACINTERFRAMEGAP_MASK)
        });
    }

    /// **EMACDISABLECRS** `[16]` (RW)
    ///
    /// EMACDISABLECRS
    #[inline(always)]
    pub fn emacdisablecrs(&self) -> bool {
        (self.value.read_bits() & Self::EMACDISABLECRS_MASK) != 0
    }

    /// Set EMACDISABLECRS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacdisablecrs(&self) {
        self.value.modify_bits(|r| r | Self::EMACDISABLECRS_MASK);
    }

    /// Clear EMACDISABLECRS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacdisablecrs(&self) {
        self.value.modify_bits(|r| r & !Self::EMACDISABLECRS_MASK);
    }

    /// **EMACMII** `[15]` (RW)
    ///
    /// EMACMII
    #[inline(always)]
    pub fn emacmii(&self) -> bool {
        (self.value.read_bits() & Self::EMACMII_MASK) != 0
    }

    /// Set EMACMII bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacmii(&self) {
        self.value.modify_bits(|r| r | Self::EMACMII_MASK);
    }

    /// Clear EMACMII bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacmii(&self) {
        self.value.modify_bits(|r| r & !Self::EMACMII_MASK);
    }

    /// **EMACFESPEED** `[14]` (RW)
    ///
    /// EMACFESPEED
    #[inline(always)]
    pub fn emacfespeed(&self) -> bool {
        (self.value.read_bits() & Self::EMACFESPEED_MASK) != 0
    }

    /// Set EMACFESPEED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacfespeed(&self) {
        self.value.modify_bits(|r| r | Self::EMACFESPEED_MASK);
    }

    /// Clear EMACFESPEED bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacfespeed(&self) {
        self.value.modify_bits(|r| r & !Self::EMACFESPEED_MASK);
    }

    /// **EMACRXOWN** `[13]` (RW)
    ///
    /// EMACRXOWN
    #[inline(always)]
    pub fn emacrxown(&self) -> bool {
        (self.value.read_bits() & Self::EMACRXOWN_MASK) != 0
    }

    /// Set EMACRXOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacrxown(&self) {
        self.value.modify_bits(|r| r | Self::EMACRXOWN_MASK);
    }

    /// Clear EMACRXOWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacrxown(&self) {
        self.value.modify_bits(|r| r & !Self::EMACRXOWN_MASK);
    }

    /// **EMACLOOPBACK** `[12]` (RW)
    ///
    /// EMACLOOPBACK
    #[inline(always)]
    pub fn emacloopback(&self) -> bool {
        (self.value.read_bits() & Self::EMACLOOPBACK_MASK) != 0
    }

    /// Set EMACLOOPBACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacloopback(&self) {
        self.value.modify_bits(|r| r | Self::EMACLOOPBACK_MASK);
    }

    /// Clear EMACLOOPBACK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacloopback(&self) {
        self.value.modify_bits(|r| r & !Self::EMACLOOPBACK_MASK);
    }

    /// **EMACDUPLEX** `[11]` (RW)
    ///
    /// EMACDUPLEX
    #[inline(always)]
    pub fn emacduplex(&self) -> bool {
        (self.value.read_bits() & Self::EMACDUPLEX_MASK) != 0
    }

    /// Set EMACDUPLEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacduplex(&self) {
        self.value.modify_bits(|r| r | Self::EMACDUPLEX_MASK);
    }

    /// Clear EMACDUPLEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacduplex(&self) {
        self.value.modify_bits(|r| r & !Self::EMACDUPLEX_MASK);
    }

    /// **EMACRXIPCOFFLOAD** `[10]` (RW)
    ///
    /// EMACRXIPCOFFLOAD
    #[inline(always)]
    pub fn emacrxipcoffload(&self) -> bool {
        (self.value.read_bits() & Self::EMACRXIPCOFFLOAD_MASK) != 0
    }

    /// Set EMACRXIPCOFFLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacrxipcoffload(&self) {
        self.value.modify_bits(|r| r | Self::EMACRXIPCOFFLOAD_MASK);
    }

    /// Clear EMACRXIPCOFFLOAD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacrxipcoffload(&self) {
        self.value.modify_bits(|r| r & !Self::EMACRXIPCOFFLOAD_MASK);
    }

    /// **EMACRETRY** `[9]` (RW)
    ///
    /// EMACRETRY
    #[inline(always)]
    pub fn emacretry(&self) -> bool {
        (self.value.read_bits() & Self::EMACRETRY_MASK) != 0
    }

    /// Set EMACRETRY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacretry(&self) {
        self.value.modify_bits(|r| r | Self::EMACRETRY_MASK);
    }

    /// Clear EMACRETRY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacretry(&self) {
        self.value.modify_bits(|r| r & !Self::EMACRETRY_MASK);
    }

    /// **EMACPADCRCSTRIP** `[7]` (RW)
    ///
    /// EMACPADCRCSTRIP
    #[inline(always)]
    pub fn emacpadcrcstrip(&self) -> bool {
        (self.value.read_bits() & Self::EMACPADCRCSTRIP_MASK) != 0
    }

    /// Set EMACPADCRCSTRIP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacpadcrcstrip(&self) {
        self.value.modify_bits(|r| r | Self::EMACPADCRCSTRIP_MASK);
    }

    /// Clear EMACPADCRCSTRIP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacpadcrcstrip(&self) {
        self.value.modify_bits(|r| r & !Self::EMACPADCRCSTRIP_MASK);
    }

    /// **EMACBACKOFFLIMIT** `[6:5]` (RW)
    ///
    /// EMACBACKOFFLIMIT
    #[inline(always)]
    pub fn emacbackofflimit(&self) -> u32 {
        (self.value.read_bits() & Self::EMACBACKOFFLIMIT_MASK) >> Self::EMACBACKOFFLIMIT_OFFSET
    }

    /// Write **EMACBACKOFFLIMIT** `[6:5]` (RW)
    ///
    /// EMACBACKOFFLIMIT
    #[inline(always)]
    pub fn write_emacbackofflimit(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::EMACBACKOFFLIMIT_MASK)
                | ((value << Self::EMACBACKOFFLIMIT_OFFSET) & Self::EMACBACKOFFLIMIT_MASK)
        });
    }

    /// **EMACDEFERRALCHECK** `[4]` (RW)
    ///
    /// EMACDEFERRALCHECK
    #[inline(always)]
    pub fn emacdeferralcheck(&self) -> bool {
        (self.value.read_bits() & Self::EMACDEFERRALCHECK_MASK) != 0
    }

    /// Set EMACDEFERRALCHECK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacdeferralcheck(&self) {
        self.value.modify_bits(|r| r | Self::EMACDEFERRALCHECK_MASK);
    }

    /// Clear EMACDEFERRALCHECK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacdeferralcheck(&self) {
        self.value
            .modify_bits(|r| r & !Self::EMACDEFERRALCHECK_MASK);
    }

    /// **EMACTX** `[3]` (RW)
    ///
    /// EMACTX
    #[inline(always)]
    pub fn emactx(&self) -> bool {
        (self.value.read_bits() & Self::EMACTX_MASK) != 0
    }

    /// Set EMACTX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emactx(&self) {
        self.value.modify_bits(|r| r | Self::EMACTX_MASK);
    }

    /// Clear EMACTX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emactx(&self) {
        self.value.modify_bits(|r| r & !Self::EMACTX_MASK);
    }

    /// **EMACRX** `[2]` (RW)
    ///
    /// EMACRX
    #[inline(always)]
    pub fn emacrx(&self) -> bool {
        (self.value.read_bits() & Self::EMACRX_MASK) != 0
    }

    /// Set EMACRX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emacrx(&self) {
        self.value.modify_bits(|r| r | Self::EMACRX_MASK);
    }

    /// Clear EMACRX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emacrx(&self) {
        self.value.modify_bits(|r| r & !Self::EMACRX_MASK);
    }

    /// **PLTF** `[1:0]` (RW)
    ///
    /// PLTF
    #[inline(always)]
    pub fn pltf(&self) -> u32 {
        (self.value.read_bits() & Self::PLTF_MASK) >> Self::PLTF_OFFSET
    }

    /// Write **PLTF** `[1:0]` (RW)
    ///
    /// PLTF
    #[inline(always)]
    pub fn write_pltf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PLTF_MASK) | ((value << Self::PLTF_OFFSET) & Self::PLTF_MASK)
        });
    }
}

/// EMACCONFIG value type for batch read/modify/write operations
///
/// Use `emacconfig.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacconfig.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacconfigVal {
    bits: u32,
}

impl EmacconfigVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00008000 }
    }

    /// Read **SAIRC** `[30:28]`
    ///
    /// SAIRC
    #[inline(always)]
    pub const fn sairc(&self) -> u32 {
        (self.bits & Emacconfig::SAIRC_MASK) >> Emacconfig::SAIRC_OFFSET
    }

    /// Read **ASS2KP** `[27:27]`
    ///
    /// ASS2KP
    #[inline(always)]
    pub const fn ass2kp(&self) -> bool {
        (self.bits & Emacconfig::ASS2KP_MASK) != 0
    }

    /// Read **EMACCRC_STRIP** `[25:25]`
    ///
    /// EMACCRC_STRIP
    #[inline(always)]
    pub const fn emaccrc_strip(&self) -> bool {
        (self.bits & Emacconfig::EMACCRC_STRIP_MASK) != 0
    }

    /// Read **EMACWATCHDOG** `[23:23]`
    ///
    /// EMACWATCHDOG
    #[inline(always)]
    pub const fn emacwatchdog(&self) -> bool {
        (self.bits & Emacconfig::EMACWATCHDOG_MASK) != 0
    }

    /// Read **EMACJABBER** `[22:22]`
    ///
    /// EMACJABBER
    #[inline(always)]
    pub const fn emacjabber(&self) -> bool {
        (self.bits & Emacconfig::EMACJABBER_MASK) != 0
    }

    /// Read **EMACJUMBOFRAME** `[20:20]`
    ///
    /// EMACJUMBOFRAME
    #[inline(always)]
    pub const fn emacjumboframe(&self) -> bool {
        (self.bits & Emacconfig::EMACJUMBOFRAME_MASK) != 0
    }

    /// Read **EMACINTERFRAMEGAP** `[19:17]`
    ///
    /// EMACINTERFRAMEGAP
    #[inline(always)]
    pub const fn emacinterframegap(&self) -> u32 {
        (self.bits & Emacconfig::EMACINTERFRAMEGAP_MASK) >> Emacconfig::EMACINTERFRAMEGAP_OFFSET
    }

    /// Read **EMACDISABLECRS** `[16:16]`
    ///
    /// EMACDISABLECRS
    #[inline(always)]
    pub const fn emacdisablecrs(&self) -> bool {
        (self.bits & Emacconfig::EMACDISABLECRS_MASK) != 0
    }

    /// Read **EMACMII** `[15:15]`
    ///
    /// EMACMII
    #[inline(always)]
    pub const fn emacmii(&self) -> bool {
        (self.bits & Emacconfig::EMACMII_MASK) != 0
    }

    /// Read **EMACFESPEED** `[14:14]`
    ///
    /// EMACFESPEED
    #[inline(always)]
    pub const fn emacfespeed(&self) -> bool {
        (self.bits & Emacconfig::EMACFESPEED_MASK) != 0
    }

    /// Read **EMACRXOWN** `[13:13]`
    ///
    /// EMACRXOWN
    #[inline(always)]
    pub const fn emacrxown(&self) -> bool {
        (self.bits & Emacconfig::EMACRXOWN_MASK) != 0
    }

    /// Read **EMACLOOPBACK** `[12:12]`
    ///
    /// EMACLOOPBACK
    #[inline(always)]
    pub const fn emacloopback(&self) -> bool {
        (self.bits & Emacconfig::EMACLOOPBACK_MASK) != 0
    }

    /// Read **EMACDUPLEX** `[11:11]`
    ///
    /// EMACDUPLEX
    #[inline(always)]
    pub const fn emacduplex(&self) -> bool {
        (self.bits & Emacconfig::EMACDUPLEX_MASK) != 0
    }

    /// Read **EMACRXIPCOFFLOAD** `[10:10]`
    ///
    /// EMACRXIPCOFFLOAD
    #[inline(always)]
    pub const fn emacrxipcoffload(&self) -> bool {
        (self.bits & Emacconfig::EMACRXIPCOFFLOAD_MASK) != 0
    }

    /// Read **EMACRETRY** `[9:9]`
    ///
    /// EMACRETRY
    #[inline(always)]
    pub const fn emacretry(&self) -> bool {
        (self.bits & Emacconfig::EMACRETRY_MASK) != 0
    }

    /// Read **EMACPADCRCSTRIP** `[7:7]`
    ///
    /// EMACPADCRCSTRIP
    #[inline(always)]
    pub const fn emacpadcrcstrip(&self) -> bool {
        (self.bits & Emacconfig::EMACPADCRCSTRIP_MASK) != 0
    }

    /// Read **EMACBACKOFFLIMIT** `[6:5]`
    ///
    /// EMACBACKOFFLIMIT
    #[inline(always)]
    pub const fn emacbackofflimit(&self) -> u32 {
        (self.bits & Emacconfig::EMACBACKOFFLIMIT_MASK) >> Emacconfig::EMACBACKOFFLIMIT_OFFSET
    }

    /// Read **EMACDEFERRALCHECK** `[4:4]`
    ///
    /// EMACDEFERRALCHECK
    #[inline(always)]
    pub const fn emacdeferralcheck(&self) -> bool {
        (self.bits & Emacconfig::EMACDEFERRALCHECK_MASK) != 0
    }

    /// Read **EMACTX** `[3:3]`
    ///
    /// EMACTX
    #[inline(always)]
    pub const fn emactx(&self) -> bool {
        (self.bits & Emacconfig::EMACTX_MASK) != 0
    }

    /// Read **EMACRX** `[2:2]`
    ///
    /// EMACRX
    #[inline(always)]
    pub const fn emacrx(&self) -> bool {
        (self.bits & Emacconfig::EMACRX_MASK) != 0
    }

    /// Read **PLTF** `[1:0]`
    ///
    /// PLTF
    #[inline(always)]
    pub const fn pltf(&self) -> u32 {
        (self.bits & Emacconfig::PLTF_MASK) >> Emacconfig::PLTF_OFFSET
    }

    /// Set **SAIRC** `[30:28]`
    ///
    /// SAIRC
    #[inline(always)]
    pub const fn with_sairc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacconfig::SAIRC_MASK)
                | ((value << Emacconfig::SAIRC_OFFSET) & Emacconfig::SAIRC_MASK),
        }
    }

    /// Set **ASS2KP** `[27:27]`
    ///
    /// ASS2KP
    #[inline(always)]
    pub const fn with_ass2kp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::ASS2KP_MASK
            } else {
                self.bits & !Emacconfig::ASS2KP_MASK
            },
        }
    }

    /// Set **EMACCRC_STRIP** `[25:25]`
    ///
    /// EMACCRC_STRIP
    #[inline(always)]
    pub const fn with_emaccrc_strip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACCRC_STRIP_MASK
            } else {
                self.bits & !Emacconfig::EMACCRC_STRIP_MASK
            },
        }
    }

    /// Set **EMACWATCHDOG** `[23:23]`
    ///
    /// EMACWATCHDOG
    #[inline(always)]
    pub const fn with_emacwatchdog(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACWATCHDOG_MASK
            } else {
                self.bits & !Emacconfig::EMACWATCHDOG_MASK
            },
        }
    }

    /// Set **EMACJABBER** `[22:22]`
    ///
    /// EMACJABBER
    #[inline(always)]
    pub const fn with_emacjabber(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACJABBER_MASK
            } else {
                self.bits & !Emacconfig::EMACJABBER_MASK
            },
        }
    }

    /// Set **EMACJUMBOFRAME** `[20:20]`
    ///
    /// EMACJUMBOFRAME
    #[inline(always)]
    pub const fn with_emacjumboframe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACJUMBOFRAME_MASK
            } else {
                self.bits & !Emacconfig::EMACJUMBOFRAME_MASK
            },
        }
    }

    /// Set **EMACINTERFRAMEGAP** `[19:17]`
    ///
    /// EMACINTERFRAMEGAP
    #[inline(always)]
    pub const fn with_emacinterframegap(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacconfig::EMACINTERFRAMEGAP_MASK)
                | ((value << Emacconfig::EMACINTERFRAMEGAP_OFFSET)
                    & Emacconfig::EMACINTERFRAMEGAP_MASK),
        }
    }

    /// Set **EMACDISABLECRS** `[16:16]`
    ///
    /// EMACDISABLECRS
    #[inline(always)]
    pub const fn with_emacdisablecrs(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACDISABLECRS_MASK
            } else {
                self.bits & !Emacconfig::EMACDISABLECRS_MASK
            },
        }
    }

    /// Set **EMACMII** `[15:15]`
    ///
    /// EMACMII
    #[inline(always)]
    pub const fn with_emacmii(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACMII_MASK
            } else {
                self.bits & !Emacconfig::EMACMII_MASK
            },
        }
    }

    /// Set **EMACFESPEED** `[14:14]`
    ///
    /// EMACFESPEED
    #[inline(always)]
    pub const fn with_emacfespeed(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACFESPEED_MASK
            } else {
                self.bits & !Emacconfig::EMACFESPEED_MASK
            },
        }
    }

    /// Set **EMACRXOWN** `[13:13]`
    ///
    /// EMACRXOWN
    #[inline(always)]
    pub const fn with_emacrxown(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACRXOWN_MASK
            } else {
                self.bits & !Emacconfig::EMACRXOWN_MASK
            },
        }
    }

    /// Set **EMACLOOPBACK** `[12:12]`
    ///
    /// EMACLOOPBACK
    #[inline(always)]
    pub const fn with_emacloopback(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACLOOPBACK_MASK
            } else {
                self.bits & !Emacconfig::EMACLOOPBACK_MASK
            },
        }
    }

    /// Set **EMACDUPLEX** `[11:11]`
    ///
    /// EMACDUPLEX
    #[inline(always)]
    pub const fn with_emacduplex(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACDUPLEX_MASK
            } else {
                self.bits & !Emacconfig::EMACDUPLEX_MASK
            },
        }
    }

    /// Set **EMACRXIPCOFFLOAD** `[10:10]`
    ///
    /// EMACRXIPCOFFLOAD
    #[inline(always)]
    pub const fn with_emacrxipcoffload(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACRXIPCOFFLOAD_MASK
            } else {
                self.bits & !Emacconfig::EMACRXIPCOFFLOAD_MASK
            },
        }
    }

    /// Set **EMACRETRY** `[9:9]`
    ///
    /// EMACRETRY
    #[inline(always)]
    pub const fn with_emacretry(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACRETRY_MASK
            } else {
                self.bits & !Emacconfig::EMACRETRY_MASK
            },
        }
    }

    /// Set **EMACPADCRCSTRIP** `[7:7]`
    ///
    /// EMACPADCRCSTRIP
    #[inline(always)]
    pub const fn with_emacpadcrcstrip(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACPADCRCSTRIP_MASK
            } else {
                self.bits & !Emacconfig::EMACPADCRCSTRIP_MASK
            },
        }
    }

    /// Set **EMACBACKOFFLIMIT** `[6:5]`
    ///
    /// EMACBACKOFFLIMIT
    #[inline(always)]
    pub const fn with_emacbackofflimit(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacconfig::EMACBACKOFFLIMIT_MASK)
                | ((value << Emacconfig::EMACBACKOFFLIMIT_OFFSET)
                    & Emacconfig::EMACBACKOFFLIMIT_MASK),
        }
    }

    /// Set **EMACDEFERRALCHECK** `[4:4]`
    ///
    /// EMACDEFERRALCHECK
    #[inline(always)]
    pub const fn with_emacdeferralcheck(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACDEFERRALCHECK_MASK
            } else {
                self.bits & !Emacconfig::EMACDEFERRALCHECK_MASK
            },
        }
    }

    /// Set **EMACTX** `[3:3]`
    ///
    /// EMACTX
    #[inline(always)]
    pub const fn with_emactx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACTX_MASK
            } else {
                self.bits & !Emacconfig::EMACTX_MASK
            },
        }
    }

    /// Set **EMACRX** `[2:2]`
    ///
    /// EMACRX
    #[inline(always)]
    pub const fn with_emacrx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacconfig::EMACRX_MASK
            } else {
                self.bits & !Emacconfig::EMACRX_MASK
            },
        }
    }

    /// Set **PLTF** `[1:0]`
    ///
    /// PLTF
    #[inline(always)]
    pub const fn with_pltf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacconfig::PLTF_MASK)
                | ((value << Emacconfig::PLTF_OFFSET) & Emacconfig::PLTF_MASK),
        }
    }
}

impl Default for EmacconfigVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacconfig {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacconfigVal {
        EmacconfigVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacconfigVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacconfigVal) -> EmacconfigVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACFF Register
///
/// EMACFF register
///
/// Address offset: 0x0004
/// Reset value: 0x00010000
///
/// ## Fields
///
/// - **RECEIVE_ALL** `[31]` (RW)
///
///   RECEIVE_ALL
///
/// - **VTFE** `[16]` (RW)
///
///   VTFE
///
/// - **SAFE** `[9]` (RW)
///
///   SAFE
///
/// - **SAIF** `[8]` (RW)
///
///   SAIF
///
/// - **PCF** `[7:6]` (RW)
///
///   PCF
///
/// - **DBF** `[5]` (RW)
///
///   DBF
///
/// - **PAM** `[4]` (RW)
///
///   PAM
///
/// - **DAIF** `[3]` (RW)
///
///   DAIF
///
/// - **PMODE** `[0]` (RW)
///
///   PMODE
#[repr(transparent)]
pub struct Emacff {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacff {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacff {
    // ======== Field Constants ========

    /// **RECEIVE_ALL**
    ///
    /// RECEIVE_ALL
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RECEIVE_ALL_OFFSET: u32 = 31;
    pub const RECEIVE_ALL_MASK: u32 = 0x80000000;
    pub const RECEIVE_ALL: u32 = 1 << 31;

    /// **VTFE**
    ///
    /// VTFE
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const VTFE_OFFSET: u32 = 16;
    pub const VTFE_MASK: u32 = 0x00010000;
    pub const VTFE: u32 = 1 << 16;

    /// **SAFE**
    ///
    /// SAFE
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const SAFE_OFFSET: u32 = 9;
    pub const SAFE_MASK: u32 = 0x00000200;
    pub const SAFE: u32 = 1 << 9;

    /// **SAIF**
    ///
    /// SAIF
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const SAIF_OFFSET: u32 = 8;
    pub const SAIF_MASK: u32 = 0x00000100;
    pub const SAIF: u32 = 1 << 8;

    /// **PCF**
    ///
    /// PCF
    ///
    /// Bits `[7:6]`, mask `0x000000c0`
    pub const PCF_OFFSET: u32 = 6;
    pub const PCF_MASK: u32 = 0x000000c0;

    /// **DBF**
    ///
    /// DBF
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DBF_OFFSET: u32 = 5;
    pub const DBF_MASK: u32 = 0x00000020;
    pub const DBF: u32 = 1 << 5;

    /// **PAM**
    ///
    /// PAM
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const PAM_OFFSET: u32 = 4;
    pub const PAM_MASK: u32 = 0x00000010;
    pub const PAM: u32 = 1 << 4;

    /// **DAIF**
    ///
    /// DAIF
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DAIF_OFFSET: u32 = 3;
    pub const DAIF_MASK: u32 = 0x00000008;
    pub const DAIF: u32 = 1 << 3;

    /// **PMODE**
    ///
    /// PMODE
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PMODE_OFFSET: u32 = 0;
    pub const PMODE_MASK: u32 = 0x00000001;
    pub const PMODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RECEIVE_ALL** `[31]` (RW)
    ///
    /// RECEIVE_ALL
    #[inline(always)]
    pub fn receive_all(&self) -> bool {
        (self.value.read_bits() & Self::RECEIVE_ALL_MASK) != 0
    }

    /// Set RECEIVE_ALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_receive_all(&self) {
        self.value.modify_bits(|r| r | Self::RECEIVE_ALL_MASK);
    }

    /// Clear RECEIVE_ALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_receive_all(&self) {
        self.value.modify_bits(|r| r & !Self::RECEIVE_ALL_MASK);
    }

    /// **VTFE** `[16]` (RW)
    ///
    /// VTFE
    #[inline(always)]
    pub fn vtfe(&self) -> bool {
        (self.value.read_bits() & Self::VTFE_MASK) != 0
    }

    /// Set VTFE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vtfe(&self) {
        self.value.modify_bits(|r| r | Self::VTFE_MASK);
    }

    /// Clear VTFE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vtfe(&self) {
        self.value.modify_bits(|r| r & !Self::VTFE_MASK);
    }

    /// **SAFE** `[9]` (RW)
    ///
    /// SAFE
    #[inline(always)]
    pub fn safe(&self) -> bool {
        (self.value.read_bits() & Self::SAFE_MASK) != 0
    }

    /// Set SAFE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_safe(&self) {
        self.value.modify_bits(|r| r | Self::SAFE_MASK);
    }

    /// Clear SAFE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_safe(&self) {
        self.value.modify_bits(|r| r & !Self::SAFE_MASK);
    }

    /// **SAIF** `[8]` (RW)
    ///
    /// SAIF
    #[inline(always)]
    pub fn saif(&self) -> bool {
        (self.value.read_bits() & Self::SAIF_MASK) != 0
    }

    /// Set SAIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_saif(&self) {
        self.value.modify_bits(|r| r | Self::SAIF_MASK);
    }

    /// Clear SAIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_saif(&self) {
        self.value.modify_bits(|r| r & !Self::SAIF_MASK);
    }

    /// **PCF** `[7:6]` (RW)
    ///
    /// PCF
    #[inline(always)]
    pub fn pcf(&self) -> u32 {
        (self.value.read_bits() & Self::PCF_MASK) >> Self::PCF_OFFSET
    }

    /// Write **PCF** `[7:6]` (RW)
    ///
    /// PCF
    #[inline(always)]
    pub fn write_pcf(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PCF_MASK) | ((value << Self::PCF_OFFSET) & Self::PCF_MASK)
        });
    }

    /// **DBF** `[5]` (RW)
    ///
    /// DBF
    #[inline(always)]
    pub fn dbf(&self) -> bool {
        (self.value.read_bits() & Self::DBF_MASK) != 0
    }

    /// Set DBF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dbf(&self) {
        self.value.modify_bits(|r| r | Self::DBF_MASK);
    }

    /// Clear DBF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dbf(&self) {
        self.value.modify_bits(|r| r & !Self::DBF_MASK);
    }

    /// **PAM** `[4]` (RW)
    ///
    /// PAM
    #[inline(always)]
    pub fn pam(&self) -> bool {
        (self.value.read_bits() & Self::PAM_MASK) != 0
    }

    /// Set PAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pam(&self) {
        self.value.modify_bits(|r| r | Self::PAM_MASK);
    }

    /// Clear PAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pam(&self) {
        self.value.modify_bits(|r| r & !Self::PAM_MASK);
    }

    /// **DAIF** `[3]` (RW)
    ///
    /// DAIF
    #[inline(always)]
    pub fn daif(&self) -> bool {
        (self.value.read_bits() & Self::DAIF_MASK) != 0
    }

    /// Set DAIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_daif(&self) {
        self.value.modify_bits(|r| r | Self::DAIF_MASK);
    }

    /// Clear DAIF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_daif(&self) {
        self.value.modify_bits(|r| r & !Self::DAIF_MASK);
    }

    /// **PMODE** `[0]` (RW)
    ///
    /// PMODE
    #[inline(always)]
    pub fn pmode(&self) -> bool {
        (self.value.read_bits() & Self::PMODE_MASK) != 0
    }

    /// Set PMODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pmode(&self) {
        self.value.modify_bits(|r| r | Self::PMODE_MASK);
    }

    /// Clear PMODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pmode(&self) {
        self.value.modify_bits(|r| r & !Self::PMODE_MASK);
    }
}

/// EMACFF value type for batch read/modify/write operations
///
/// Use `emacff.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacff.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacffVal {
    bits: u32,
}

impl EmacffVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00010000 }
    }

    /// Read **RECEIVE_ALL** `[31:31]`
    ///
    /// RECEIVE_ALL
    #[inline(always)]
    pub const fn receive_all(&self) -> bool {
        (self.bits & Emacff::RECEIVE_ALL_MASK) != 0
    }

    /// Read **VTFE** `[16:16]`
    ///
    /// VTFE
    #[inline(always)]
    pub const fn vtfe(&self) -> bool {
        (self.bits & Emacff::VTFE_MASK) != 0
    }

    /// Read **SAFE** `[9:9]`
    ///
    /// SAFE
    #[inline(always)]
    pub const fn safe(&self) -> bool {
        (self.bits & Emacff::SAFE_MASK) != 0
    }

    /// Read **SAIF** `[8:8]`
    ///
    /// SAIF
    #[inline(always)]
    pub const fn saif(&self) -> bool {
        (self.bits & Emacff::SAIF_MASK) != 0
    }

    /// Read **PCF** `[7:6]`
    ///
    /// PCF
    #[inline(always)]
    pub const fn pcf(&self) -> u32 {
        (self.bits & Emacff::PCF_MASK) >> Emacff::PCF_OFFSET
    }

    /// Read **DBF** `[5:5]`
    ///
    /// DBF
    #[inline(always)]
    pub const fn dbf(&self) -> bool {
        (self.bits & Emacff::DBF_MASK) != 0
    }

    /// Read **PAM** `[4:4]`
    ///
    /// PAM
    #[inline(always)]
    pub const fn pam(&self) -> bool {
        (self.bits & Emacff::PAM_MASK) != 0
    }

    /// Read **DAIF** `[3:3]`
    ///
    /// DAIF
    #[inline(always)]
    pub const fn daif(&self) -> bool {
        (self.bits & Emacff::DAIF_MASK) != 0
    }

    /// Read **PMODE** `[0:0]`
    ///
    /// PMODE
    #[inline(always)]
    pub const fn pmode(&self) -> bool {
        (self.bits & Emacff::PMODE_MASK) != 0
    }

    /// Set **RECEIVE_ALL** `[31:31]`
    ///
    /// RECEIVE_ALL
    #[inline(always)]
    pub const fn with_receive_all(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::RECEIVE_ALL_MASK
            } else {
                self.bits & !Emacff::RECEIVE_ALL_MASK
            },
        }
    }

    /// Set **VTFE** `[16:16]`
    ///
    /// VTFE
    #[inline(always)]
    pub const fn with_vtfe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::VTFE_MASK
            } else {
                self.bits & !Emacff::VTFE_MASK
            },
        }
    }

    /// Set **SAFE** `[9:9]`
    ///
    /// SAFE
    #[inline(always)]
    pub const fn with_safe(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::SAFE_MASK
            } else {
                self.bits & !Emacff::SAFE_MASK
            },
        }
    }

    /// Set **SAIF** `[8:8]`
    ///
    /// SAIF
    #[inline(always)]
    pub const fn with_saif(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::SAIF_MASK
            } else {
                self.bits & !Emacff::SAIF_MASK
            },
        }
    }

    /// Set **PCF** `[7:6]`
    ///
    /// PCF
    #[inline(always)]
    pub const fn with_pcf(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacff::PCF_MASK)
                | ((value << Emacff::PCF_OFFSET) & Emacff::PCF_MASK),
        }
    }

    /// Set **DBF** `[5:5]`
    ///
    /// DBF
    #[inline(always)]
    pub const fn with_dbf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::DBF_MASK
            } else {
                self.bits & !Emacff::DBF_MASK
            },
        }
    }

    /// Set **PAM** `[4:4]`
    ///
    /// PAM
    #[inline(always)]
    pub const fn with_pam(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::PAM_MASK
            } else {
                self.bits & !Emacff::PAM_MASK
            },
        }
    }

    /// Set **DAIF** `[3:3]`
    ///
    /// DAIF
    #[inline(always)]
    pub const fn with_daif(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::DAIF_MASK
            } else {
                self.bits & !Emacff::DAIF_MASK
            },
        }
    }

    /// Set **PMODE** `[0:0]`
    ///
    /// PMODE
    #[inline(always)]
    pub const fn with_pmode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacff::PMODE_MASK
            } else {
                self.bits & !Emacff::PMODE_MASK
            },
        }
    }
}

impl Default for EmacffVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacff {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacffVal {
        EmacffVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacffVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacffVal) -> EmacffVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACMIIADDR Register
///
/// EMACMIIADDR register
///
/// Address offset: 0x0010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MIIDEV** `[15:11]` (RW)
///
///   MIIDEV
///
/// - **MIIREG** `[10:6]` (RW)
///
///   MIIREG
///
/// - **MIICSRCLK** `[5:2]` (RW)
///
///   MIICSRCLK
///
/// - **MIIWRITE** `[1]` (RW)
///
///   MIIWRITE
///
/// - **MIIBUSY** `[0]` (RW)
///
///   MIIBUSY
#[repr(transparent)]
pub struct Emacmiiaddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacmiiaddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacmiiaddr {
    // ======== Field Constants ========

    /// **MIIDEV**
    ///
    /// MIIDEV
    ///
    /// Bits `[15:11]`, mask `0x0000f800`
    pub const MIIDEV_OFFSET: u32 = 11;
    pub const MIIDEV_MASK: u32 = 0x0000f800;

    /// **MIIREG**
    ///
    /// MIIREG
    ///
    /// Bits `[10:6]`, mask `0x000007c0`
    pub const MIIREG_OFFSET: u32 = 6;
    pub const MIIREG_MASK: u32 = 0x000007c0;

    /// **MIICSRCLK**
    ///
    /// MIICSRCLK
    ///
    /// Bits `[5:2]`, mask `0x0000003c`
    pub const MIICSRCLK_OFFSET: u32 = 2;
    pub const MIICSRCLK_MASK: u32 = 0x0000003c;

    /// **MIIWRITE**
    ///
    /// MIIWRITE
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MIIWRITE_OFFSET: u32 = 1;
    pub const MIIWRITE_MASK: u32 = 0x00000002;
    pub const MIIWRITE: u32 = 1 << 1;

    /// **MIIBUSY**
    ///
    /// MIIBUSY
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MIIBUSY_OFFSET: u32 = 0;
    pub const MIIBUSY_MASK: u32 = 0x00000001;
    pub const MIIBUSY: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MIIDEV** `[15:11]` (RW)
    ///
    /// MIIDEV
    #[inline(always)]
    pub fn miidev(&self) -> u32 {
        (self.value.read_bits() & Self::MIIDEV_MASK) >> Self::MIIDEV_OFFSET
    }

    /// Write **MIIDEV** `[15:11]` (RW)
    ///
    /// MIIDEV
    #[inline(always)]
    pub fn write_miidev(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MIIDEV_MASK) | ((value << Self::MIIDEV_OFFSET) & Self::MIIDEV_MASK)
        });
    }

    /// **MIIREG** `[10:6]` (RW)
    ///
    /// MIIREG
    #[inline(always)]
    pub fn miireg(&self) -> u32 {
        (self.value.read_bits() & Self::MIIREG_MASK) >> Self::MIIREG_OFFSET
    }

    /// Write **MIIREG** `[10:6]` (RW)
    ///
    /// MIIREG
    #[inline(always)]
    pub fn write_miireg(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MIIREG_MASK) | ((value << Self::MIIREG_OFFSET) & Self::MIIREG_MASK)
        });
    }

    /// **MIICSRCLK** `[5:2]` (RW)
    ///
    /// MIICSRCLK
    #[inline(always)]
    pub fn miicsrclk(&self) -> u32 {
        (self.value.read_bits() & Self::MIICSRCLK_MASK) >> Self::MIICSRCLK_OFFSET
    }

    /// Write **MIICSRCLK** `[5:2]` (RW)
    ///
    /// MIICSRCLK
    #[inline(always)]
    pub fn write_miicsrclk(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MIICSRCLK_MASK) | ((value << Self::MIICSRCLK_OFFSET) & Self::MIICSRCLK_MASK)
        });
    }

    /// **MIIWRITE** `[1]` (RW)
    ///
    /// MIIWRITE
    #[inline(always)]
    pub fn miiwrite(&self) -> bool {
        (self.value.read_bits() & Self::MIIWRITE_MASK) != 0
    }

    /// Set MIIWRITE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_miiwrite(&self) {
        self.value.modify_bits(|r| r | Self::MIIWRITE_MASK);
    }

    /// Clear MIIWRITE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_miiwrite(&self) {
        self.value.modify_bits(|r| r & !Self::MIIWRITE_MASK);
    }

    /// **MIIBUSY** `[0]` (RW)
    ///
    /// MIIBUSY
    #[inline(always)]
    pub fn miibusy(&self) -> bool {
        (self.value.read_bits() & Self::MIIBUSY_MASK) != 0
    }

    /// Set MIIBUSY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_miibusy(&self) {
        self.value.modify_bits(|r| r | Self::MIIBUSY_MASK);
    }

    /// Clear MIIBUSY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_miibusy(&self) {
        self.value.modify_bits(|r| r & !Self::MIIBUSY_MASK);
    }
}

/// EMACMIIADDR value type for batch read/modify/write operations
///
/// Use `emacmiiaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacmiiaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacmiiaddrVal {
    bits: u32,
}

impl EmacmiiaddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **MIIDEV** `[15:11]`
    ///
    /// MIIDEV
    #[inline(always)]
    pub const fn miidev(&self) -> u32 {
        (self.bits & Emacmiiaddr::MIIDEV_MASK) >> Emacmiiaddr::MIIDEV_OFFSET
    }

    /// Read **MIIREG** `[10:6]`
    ///
    /// MIIREG
    #[inline(always)]
    pub const fn miireg(&self) -> u32 {
        (self.bits & Emacmiiaddr::MIIREG_MASK) >> Emacmiiaddr::MIIREG_OFFSET
    }

    /// Read **MIICSRCLK** `[5:2]`
    ///
    /// MIICSRCLK
    #[inline(always)]
    pub const fn miicsrclk(&self) -> u32 {
        (self.bits & Emacmiiaddr::MIICSRCLK_MASK) >> Emacmiiaddr::MIICSRCLK_OFFSET
    }

    /// Read **MIIWRITE** `[1:1]`
    ///
    /// MIIWRITE
    #[inline(always)]
    pub const fn miiwrite(&self) -> bool {
        (self.bits & Emacmiiaddr::MIIWRITE_MASK) != 0
    }

    /// Read **MIIBUSY** `[0:0]`
    ///
    /// MIIBUSY
    #[inline(always)]
    pub const fn miibusy(&self) -> bool {
        (self.bits & Emacmiiaddr::MIIBUSY_MASK) != 0
    }

    /// Set **MIIDEV** `[15:11]`
    ///
    /// MIIDEV
    #[inline(always)]
    pub const fn with_miidev(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacmiiaddr::MIIDEV_MASK)
                | ((value << Emacmiiaddr::MIIDEV_OFFSET) & Emacmiiaddr::MIIDEV_MASK),
        }
    }

    /// Set **MIIREG** `[10:6]`
    ///
    /// MIIREG
    #[inline(always)]
    pub const fn with_miireg(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacmiiaddr::MIIREG_MASK)
                | ((value << Emacmiiaddr::MIIREG_OFFSET) & Emacmiiaddr::MIIREG_MASK),
        }
    }

    /// Set **MIICSRCLK** `[5:2]`
    ///
    /// MIICSRCLK
    #[inline(always)]
    pub const fn with_miicsrclk(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacmiiaddr::MIICSRCLK_MASK)
                | ((value << Emacmiiaddr::MIICSRCLK_OFFSET) & Emacmiiaddr::MIICSRCLK_MASK),
        }
    }

    /// Set **MIIWRITE** `[1:1]`
    ///
    /// MIIWRITE
    #[inline(always)]
    pub const fn with_miiwrite(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacmiiaddr::MIIWRITE_MASK
            } else {
                self.bits & !Emacmiiaddr::MIIWRITE_MASK
            },
        }
    }

    /// Set **MIIBUSY** `[0:0]`
    ///
    /// MIIBUSY
    #[inline(always)]
    pub const fn with_miibusy(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacmiiaddr::MIIBUSY_MASK
            } else {
                self.bits & !Emacmiiaddr::MIIBUSY_MASK
            },
        }
    }
}

impl Default for EmacmiiaddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacmiiaddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacmiiaddrVal {
        EmacmiiaddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacmiiaddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacmiiaddrVal) -> EmacmiiaddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACMIIDATA Register
///
/// EMACMIIDATA register
///
/// Address offset: 0x0014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MII_DATA** `[15:0]` (RW)
///
///   MII_DATA
#[repr(transparent)]
pub struct Emacmiidata {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacmiidata {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacmiidata {
    // ======== Field Constants ========

    /// **MII_DATA**
    ///
    /// MII_DATA
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MII_DATA_OFFSET: u32 = 0;
    pub const MII_DATA_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **MII_DATA** `[15:0]` (RW)
    ///
    /// MII_DATA
    #[inline(always)]
    pub fn mii_data(&self) -> u32 {
        (self.value.read_bits() & Self::MII_DATA_MASK) >> Self::MII_DATA_OFFSET
    }

    /// Write **MII_DATA** `[15:0]` (RW)
    ///
    /// MII_DATA
    #[inline(always)]
    pub fn write_mii_data(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MII_DATA_MASK) | ((value << Self::MII_DATA_OFFSET) & Self::MII_DATA_MASK)
        });
    }
}

/// EMACMIIDATA value type for batch read/modify/write operations
///
/// Use `emacmiidata.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacmiidata.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacmiidataVal {
    bits: u32,
}

impl EmacmiidataVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **MII_DATA** `[15:0]`
    ///
    /// MII_DATA
    #[inline(always)]
    pub const fn mii_data(&self) -> u32 {
        (self.bits & Emacmiidata::MII_DATA_MASK) >> Emacmiidata::MII_DATA_OFFSET
    }

    /// Set **MII_DATA** `[15:0]`
    ///
    /// MII_DATA
    #[inline(always)]
    pub const fn with_mii_data(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacmiidata::MII_DATA_MASK)
                | ((value << Emacmiidata::MII_DATA_OFFSET) & Emacmiidata::MII_DATA_MASK),
        }
    }
}

impl Default for EmacmiidataVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacmiidata {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacmiidataVal {
        EmacmiidataVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacmiidataVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacmiidataVal) -> EmacmiidataVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACFC Register
///
/// EMACFC register
///
/// Address offset: 0x0018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PAUSE_TIME** `[31:16]` (RW)
///
///   PAUSE_TIME
///
/// - **DZPQ** `[7]` (RW)
///
///   DZPQ
///
/// - **PLT** `[5:4]` (RW)
///
///   PLT
///
/// - **UPFD** `[3]` (RW)
///
///   UPFD
///
/// - **RFCE** `[2]` (RW)
///
///   RFCE
///
/// - **TFCE** `[1]` (RW)
///
///   TFCE
///
/// - **FCBBA** `[0]` (RW)
///
///   FCBBA
#[repr(transparent)]
pub struct Emacfc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacfc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacfc {
    // ======== Field Constants ========

    /// **PAUSE_TIME**
    ///
    /// PAUSE_TIME
    ///
    /// Bits `[31:16]`, mask `0xffff0000`
    pub const PAUSE_TIME_OFFSET: u32 = 16;
    pub const PAUSE_TIME_MASK: u32 = 0xffff0000;

    /// **DZPQ**
    ///
    /// DZPQ
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DZPQ_OFFSET: u32 = 7;
    pub const DZPQ_MASK: u32 = 0x00000080;
    pub const DZPQ: u32 = 1 << 7;

    /// **PLT**
    ///
    /// PLT
    ///
    /// Bits `[5:4]`, mask `0x00000030`
    pub const PLT_OFFSET: u32 = 4;
    pub const PLT_MASK: u32 = 0x00000030;

    /// **UPFD**
    ///
    /// UPFD
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const UPFD_OFFSET: u32 = 3;
    pub const UPFD_MASK: u32 = 0x00000008;
    pub const UPFD: u32 = 1 << 3;

    /// **RFCE**
    ///
    /// RFCE
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RFCE_OFFSET: u32 = 2;
    pub const RFCE_MASK: u32 = 0x00000004;
    pub const RFCE: u32 = 1 << 2;

    /// **TFCE**
    ///
    /// TFCE
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TFCE_OFFSET: u32 = 1;
    pub const TFCE_MASK: u32 = 0x00000002;
    pub const TFCE: u32 = 1 << 1;

    /// **FCBBA**
    ///
    /// FCBBA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const FCBBA_OFFSET: u32 = 0;
    pub const FCBBA_MASK: u32 = 0x00000001;
    pub const FCBBA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **PAUSE_TIME** `[31:16]` (RW)
    ///
    /// PAUSE_TIME
    #[inline(always)]
    pub fn pause_time(&self) -> u32 {
        (self.value.read_bits() & Self::PAUSE_TIME_MASK) >> Self::PAUSE_TIME_OFFSET
    }

    /// Write **PAUSE_TIME** `[31:16]` (RW)
    ///
    /// PAUSE_TIME
    #[inline(always)]
    pub fn write_pause_time(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PAUSE_TIME_MASK)
                | ((value << Self::PAUSE_TIME_OFFSET) & Self::PAUSE_TIME_MASK)
        });
    }

    /// **DZPQ** `[7]` (RW)
    ///
    /// DZPQ
    #[inline(always)]
    pub fn dzpq(&self) -> bool {
        (self.value.read_bits() & Self::DZPQ_MASK) != 0
    }

    /// Set DZPQ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dzpq(&self) {
        self.value.modify_bits(|r| r | Self::DZPQ_MASK);
    }

    /// Clear DZPQ bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dzpq(&self) {
        self.value.modify_bits(|r| r & !Self::DZPQ_MASK);
    }

    /// **PLT** `[5:4]` (RW)
    ///
    /// PLT
    #[inline(always)]
    pub fn plt(&self) -> u32 {
        (self.value.read_bits() & Self::PLT_MASK) >> Self::PLT_OFFSET
    }

    /// Write **PLT** `[5:4]` (RW)
    ///
    /// PLT
    #[inline(always)]
    pub fn write_plt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PLT_MASK) | ((value << Self::PLT_OFFSET) & Self::PLT_MASK)
        });
    }

    /// **UPFD** `[3]` (RW)
    ///
    /// UPFD
    #[inline(always)]
    pub fn upfd(&self) -> bool {
        (self.value.read_bits() & Self::UPFD_MASK) != 0
    }

    /// Set UPFD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_upfd(&self) {
        self.value.modify_bits(|r| r | Self::UPFD_MASK);
    }

    /// Clear UPFD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_upfd(&self) {
        self.value.modify_bits(|r| r & !Self::UPFD_MASK);
    }

    /// **RFCE** `[2]` (RW)
    ///
    /// RFCE
    #[inline(always)]
    pub fn rfce(&self) -> bool {
        (self.value.read_bits() & Self::RFCE_MASK) != 0
    }

    /// Set RFCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rfce(&self) {
        self.value.modify_bits(|r| r | Self::RFCE_MASK);
    }

    /// Clear RFCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rfce(&self) {
        self.value.modify_bits(|r| r & !Self::RFCE_MASK);
    }

    /// **TFCE** `[1]` (RW)
    ///
    /// TFCE
    #[inline(always)]
    pub fn tfce(&self) -> bool {
        (self.value.read_bits() & Self::TFCE_MASK) != 0
    }

    /// Set TFCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tfce(&self) {
        self.value.modify_bits(|r| r | Self::TFCE_MASK);
    }

    /// Clear TFCE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tfce(&self) {
        self.value.modify_bits(|r| r & !Self::TFCE_MASK);
    }

    /// **FCBBA** `[0]` (RW)
    ///
    /// FCBBA
    #[inline(always)]
    pub fn fcbba(&self) -> bool {
        (self.value.read_bits() & Self::FCBBA_MASK) != 0
    }

    /// Set FCBBA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fcbba(&self) {
        self.value.modify_bits(|r| r | Self::FCBBA_MASK);
    }

    /// Clear FCBBA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fcbba(&self) {
        self.value.modify_bits(|r| r & !Self::FCBBA_MASK);
    }
}

/// EMACFC value type for batch read/modify/write operations
///
/// Use `emacfc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacfc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacfcVal {
    bits: u32,
}

impl EmacfcVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **PAUSE_TIME** `[31:16]`
    ///
    /// PAUSE_TIME
    #[inline(always)]
    pub const fn pause_time(&self) -> u32 {
        (self.bits & Emacfc::PAUSE_TIME_MASK) >> Emacfc::PAUSE_TIME_OFFSET
    }

    /// Read **DZPQ** `[7:7]`
    ///
    /// DZPQ
    #[inline(always)]
    pub const fn dzpq(&self) -> bool {
        (self.bits & Emacfc::DZPQ_MASK) != 0
    }

    /// Read **PLT** `[5:4]`
    ///
    /// PLT
    #[inline(always)]
    pub const fn plt(&self) -> u32 {
        (self.bits & Emacfc::PLT_MASK) >> Emacfc::PLT_OFFSET
    }

    /// Read **UPFD** `[3:3]`
    ///
    /// UPFD
    #[inline(always)]
    pub const fn upfd(&self) -> bool {
        (self.bits & Emacfc::UPFD_MASK) != 0
    }

    /// Read **RFCE** `[2:2]`
    ///
    /// RFCE
    #[inline(always)]
    pub const fn rfce(&self) -> bool {
        (self.bits & Emacfc::RFCE_MASK) != 0
    }

    /// Read **TFCE** `[1:1]`
    ///
    /// TFCE
    #[inline(always)]
    pub const fn tfce(&self) -> bool {
        (self.bits & Emacfc::TFCE_MASK) != 0
    }

    /// Read **FCBBA** `[0:0]`
    ///
    /// FCBBA
    #[inline(always)]
    pub const fn fcbba(&self) -> bool {
        (self.bits & Emacfc::FCBBA_MASK) != 0
    }

    /// Set **PAUSE_TIME** `[31:16]`
    ///
    /// PAUSE_TIME
    #[inline(always)]
    pub const fn with_pause_time(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacfc::PAUSE_TIME_MASK)
                | ((value << Emacfc::PAUSE_TIME_OFFSET) & Emacfc::PAUSE_TIME_MASK),
        }
    }

    /// Set **DZPQ** `[7:7]`
    ///
    /// DZPQ
    #[inline(always)]
    pub const fn with_dzpq(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacfc::DZPQ_MASK
            } else {
                self.bits & !Emacfc::DZPQ_MASK
            },
        }
    }

    /// Set **PLT** `[5:4]`
    ///
    /// PLT
    #[inline(always)]
    pub const fn with_plt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacfc::PLT_MASK)
                | ((value << Emacfc::PLT_OFFSET) & Emacfc::PLT_MASK),
        }
    }

    /// Set **UPFD** `[3:3]`
    ///
    /// UPFD
    #[inline(always)]
    pub const fn with_upfd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacfc::UPFD_MASK
            } else {
                self.bits & !Emacfc::UPFD_MASK
            },
        }
    }

    /// Set **RFCE** `[2:2]`
    ///
    /// RFCE
    #[inline(always)]
    pub const fn with_rfce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacfc::RFCE_MASK
            } else {
                self.bits & !Emacfc::RFCE_MASK
            },
        }
    }

    /// Set **TFCE** `[1:1]`
    ///
    /// TFCE
    #[inline(always)]
    pub const fn with_tfce(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacfc::TFCE_MASK
            } else {
                self.bits & !Emacfc::TFCE_MASK
            },
        }
    }

    /// Set **FCBBA** `[0:0]`
    ///
    /// FCBBA
    #[inline(always)]
    pub const fn with_fcbba(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacfc::FCBBA_MASK
            } else {
                self.bits & !Emacfc::FCBBA_MASK
            },
        }
    }
}

impl Default for EmacfcVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacfc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacfcVal {
        EmacfcVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacfcVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacfcVal) -> EmacfcVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACVLANTAG Register
///
/// EMACVLANTAG register
///
/// Address offset: 0x001c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ESVL** `[18]` (RW)
///
///   ESVL
///
/// - **VTIM** `[17]` (RW)
///
///   VTIM
///
/// - **ETV** `[16]` (RW)
///
///   ETV
///
/// - **VL** `[15:0]` (RW)
///
///   VL
#[repr(transparent)]
pub struct Emacvlantag {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacvlantag {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacvlantag {
    // ======== Field Constants ========

    /// **ESVL**
    ///
    /// ESVL
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const ESVL_OFFSET: u32 = 18;
    pub const ESVL_MASK: u32 = 0x00040000;
    pub const ESVL: u32 = 1 << 18;

    /// **VTIM**
    ///
    /// VTIM
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const VTIM_OFFSET: u32 = 17;
    pub const VTIM_MASK: u32 = 0x00020000;
    pub const VTIM: u32 = 1 << 17;

    /// **ETV**
    ///
    /// ETV
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const ETV_OFFSET: u32 = 16;
    pub const ETV_MASK: u32 = 0x00010000;
    pub const ETV: u32 = 1 << 16;

    /// **VL**
    ///
    /// VL
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const VL_OFFSET: u32 = 0;
    pub const VL_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ESVL** `[18]` (RW)
    ///
    /// ESVL
    #[inline(always)]
    pub fn esvl(&self) -> bool {
        (self.value.read_bits() & Self::ESVL_MASK) != 0
    }

    /// Set ESVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_esvl(&self) {
        self.value.modify_bits(|r| r | Self::ESVL_MASK);
    }

    /// Clear ESVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_esvl(&self) {
        self.value.modify_bits(|r| r & !Self::ESVL_MASK);
    }

    /// **VTIM** `[17]` (RW)
    ///
    /// VTIM
    #[inline(always)]
    pub fn vtim(&self) -> bool {
        (self.value.read_bits() & Self::VTIM_MASK) != 0
    }

    /// Set VTIM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vtim(&self) {
        self.value.modify_bits(|r| r | Self::VTIM_MASK);
    }

    /// Clear VTIM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vtim(&self) {
        self.value.modify_bits(|r| r & !Self::VTIM_MASK);
    }

    /// **ETV** `[16]` (RW)
    ///
    /// ETV
    #[inline(always)]
    pub fn etv(&self) -> bool {
        (self.value.read_bits() & Self::ETV_MASK) != 0
    }

    /// Set ETV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_etv(&self) {
        self.value.modify_bits(|r| r | Self::ETV_MASK);
    }

    /// Clear ETV bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_etv(&self) {
        self.value.modify_bits(|r| r & !Self::ETV_MASK);
    }

    /// **VL** `[15:0]` (RW)
    ///
    /// VL
    #[inline(always)]
    pub fn vl(&self) -> u32 {
        (self.value.read_bits() & Self::VL_MASK) >> Self::VL_OFFSET
    }

    /// Write **VL** `[15:0]` (RW)
    ///
    /// VL
    #[inline(always)]
    pub fn write_vl(&self, value: u32) {
        self.value
            .modify_bits(|r| (r & !Self::VL_MASK) | ((value << Self::VL_OFFSET) & Self::VL_MASK));
    }
}

/// EMACVLANTAG value type for batch read/modify/write operations
///
/// Use `emacvlantag.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacvlantag.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacvlantagVal {
    bits: u32,
}

impl EmacvlantagVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ESVL** `[18:18]`
    ///
    /// ESVL
    #[inline(always)]
    pub const fn esvl(&self) -> bool {
        (self.bits & Emacvlantag::ESVL_MASK) != 0
    }

    /// Read **VTIM** `[17:17]`
    ///
    /// VTIM
    #[inline(always)]
    pub const fn vtim(&self) -> bool {
        (self.bits & Emacvlantag::VTIM_MASK) != 0
    }

    /// Read **ETV** `[16:16]`
    ///
    /// ETV
    #[inline(always)]
    pub const fn etv(&self) -> bool {
        (self.bits & Emacvlantag::ETV_MASK) != 0
    }

    /// Read **VL** `[15:0]`
    ///
    /// VL
    #[inline(always)]
    pub const fn vl(&self) -> u32 {
        (self.bits & Emacvlantag::VL_MASK) >> Emacvlantag::VL_OFFSET
    }

    /// Set **ESVL** `[18:18]`
    ///
    /// ESVL
    #[inline(always)]
    pub const fn with_esvl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacvlantag::ESVL_MASK
            } else {
                self.bits & !Emacvlantag::ESVL_MASK
            },
        }
    }

    /// Set **VTIM** `[17:17]`
    ///
    /// VTIM
    #[inline(always)]
    pub const fn with_vtim(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacvlantag::VTIM_MASK
            } else {
                self.bits & !Emacvlantag::VTIM_MASK
            },
        }
    }

    /// Set **ETV** `[16:16]`
    ///
    /// ETV
    #[inline(always)]
    pub const fn with_etv(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacvlantag::ETV_MASK
            } else {
                self.bits & !Emacvlantag::ETV_MASK
            },
        }
    }

    /// Set **VL** `[15:0]`
    ///
    /// VL
    #[inline(always)]
    pub const fn with_vl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacvlantag::VL_MASK)
                | ((value << Emacvlantag::VL_OFFSET) & Emacvlantag::VL_MASK),
        }
    }
}

impl Default for EmacvlantagVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacvlantag {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacvlantagVal {
        EmacvlantagVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacvlantagVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacvlantagVal) -> EmacvlantagVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACDEBUG Register
///
/// EMACDEBUG register
///
/// Address offset: 0x0024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **MTLTSFFS** `[25]` (RW)
///
///   MTLTSFFS
///
/// - **MTLTFNES** `[24]` (RW)
///
///   MTLTFNES
///
/// - **MTLTFWCS** `[22]` (RW)
///
///   MTLTFWCS
///
/// - **MTLTFRCS** `[21:20]` (RW)
///
///   MTLTFRCS
///
/// - **MACTP** `[19]` (RW)
///
///   MACTP
///
/// - **MACTFCS** `[18:17]` (RW)
///
///   MACTFCS
///
/// - **MACTPES** `[16]` (RW)
///
///   MACTPES
///
/// - **MTLRFFLS** `[9:8]` (RW)
///
///   MTLRFFLS
///
/// - **MTLRFRCS** `[6:5]` (RW)
///
///   MTLRFRCS
///
/// - **MTLRFWCAS** `[4]` (RW)
///
///   MTLRFWCAS
///
/// - **MACRFFCS** `[2:1]` (RW)
///
///   MACRFFCS
///
/// - **MACRPES** `[0]` (RW)
///
///   MACRPES
#[repr(transparent)]
pub struct Emacdebug {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacdebug {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacdebug {
    // ======== Field Constants ========

    /// **MTLTSFFS**
    ///
    /// MTLTSFFS
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const MTLTSFFS_OFFSET: u32 = 25;
    pub const MTLTSFFS_MASK: u32 = 0x02000000;
    pub const MTLTSFFS: u32 = 1 << 25;

    /// **MTLTFNES**
    ///
    /// MTLTFNES
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const MTLTFNES_OFFSET: u32 = 24;
    pub const MTLTFNES_MASK: u32 = 0x01000000;
    pub const MTLTFNES: u32 = 1 << 24;

    /// **MTLTFWCS**
    ///
    /// MTLTFWCS
    ///
    /// Bits `[22:22]`, mask `0x00400000`
    pub const MTLTFWCS_OFFSET: u32 = 22;
    pub const MTLTFWCS_MASK: u32 = 0x00400000;
    pub const MTLTFWCS: u32 = 1 << 22;

    /// **MTLTFRCS**
    ///
    /// MTLTFRCS
    ///
    /// Bits `[21:20]`, mask `0x00300000`
    pub const MTLTFRCS_OFFSET: u32 = 20;
    pub const MTLTFRCS_MASK: u32 = 0x00300000;

    /// **MACTP**
    ///
    /// MACTP
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const MACTP_OFFSET: u32 = 19;
    pub const MACTP_MASK: u32 = 0x00080000;
    pub const MACTP: u32 = 1 << 19;

    /// **MACTFCS**
    ///
    /// MACTFCS
    ///
    /// Bits `[18:17]`, mask `0x00060000`
    pub const MACTFCS_OFFSET: u32 = 17;
    pub const MACTFCS_MASK: u32 = 0x00060000;

    /// **MACTPES**
    ///
    /// MACTPES
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const MACTPES_OFFSET: u32 = 16;
    pub const MACTPES_MASK: u32 = 0x00010000;
    pub const MACTPES: u32 = 1 << 16;

    /// **MTLRFFLS**
    ///
    /// MTLRFFLS
    ///
    /// Bits `[9:8]`, mask `0x00000300`
    pub const MTLRFFLS_OFFSET: u32 = 8;
    pub const MTLRFFLS_MASK: u32 = 0x00000300;

    /// **MTLRFRCS**
    ///
    /// MTLRFRCS
    ///
    /// Bits `[6:5]`, mask `0x00000060`
    pub const MTLRFRCS_OFFSET: u32 = 5;
    pub const MTLRFRCS_MASK: u32 = 0x00000060;

    /// **MTLRFWCAS**
    ///
    /// MTLRFWCAS
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const MTLRFWCAS_OFFSET: u32 = 4;
    pub const MTLRFWCAS_MASK: u32 = 0x00000010;
    pub const MTLRFWCAS: u32 = 1 << 4;

    /// **MACRFFCS**
    ///
    /// MACRFFCS
    ///
    /// Bits `[2:1]`, mask `0x00000006`
    pub const MACRFFCS_OFFSET: u32 = 1;
    pub const MACRFFCS_MASK: u32 = 0x00000006;

    /// **MACRPES**
    ///
    /// MACRPES
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const MACRPES_OFFSET: u32 = 0;
    pub const MACRPES_MASK: u32 = 0x00000001;
    pub const MACRPES: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **MTLTSFFS** `[25]` (RW)
    ///
    /// MTLTSFFS
    #[inline(always)]
    pub fn mtltsffs(&self) -> bool {
        (self.value.read_bits() & Self::MTLTSFFS_MASK) != 0
    }

    /// Set MTLTSFFS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mtltsffs(&self) {
        self.value.modify_bits(|r| r | Self::MTLTSFFS_MASK);
    }

    /// Clear MTLTSFFS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mtltsffs(&self) {
        self.value.modify_bits(|r| r & !Self::MTLTSFFS_MASK);
    }

    /// **MTLTFNES** `[24]` (RW)
    ///
    /// MTLTFNES
    #[inline(always)]
    pub fn mtltfnes(&self) -> bool {
        (self.value.read_bits() & Self::MTLTFNES_MASK) != 0
    }

    /// Set MTLTFNES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mtltfnes(&self) {
        self.value.modify_bits(|r| r | Self::MTLTFNES_MASK);
    }

    /// Clear MTLTFNES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mtltfnes(&self) {
        self.value.modify_bits(|r| r & !Self::MTLTFNES_MASK);
    }

    /// **MTLTFWCS** `[22]` (RW)
    ///
    /// MTLTFWCS
    #[inline(always)]
    pub fn mtltfwcs(&self) -> bool {
        (self.value.read_bits() & Self::MTLTFWCS_MASK) != 0
    }

    /// Set MTLTFWCS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mtltfwcs(&self) {
        self.value.modify_bits(|r| r | Self::MTLTFWCS_MASK);
    }

    /// Clear MTLTFWCS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mtltfwcs(&self) {
        self.value.modify_bits(|r| r & !Self::MTLTFWCS_MASK);
    }

    /// **MTLTFRCS** `[21:20]` (RW)
    ///
    /// MTLTFRCS
    #[inline(always)]
    pub fn mtltfrcs(&self) -> u32 {
        (self.value.read_bits() & Self::MTLTFRCS_MASK) >> Self::MTLTFRCS_OFFSET
    }

    /// Write **MTLTFRCS** `[21:20]` (RW)
    ///
    /// MTLTFRCS
    #[inline(always)]
    pub fn write_mtltfrcs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MTLTFRCS_MASK) | ((value << Self::MTLTFRCS_OFFSET) & Self::MTLTFRCS_MASK)
        });
    }

    /// **MACTP** `[19]` (RW)
    ///
    /// MACTP
    #[inline(always)]
    pub fn mactp(&self) -> bool {
        (self.value.read_bits() & Self::MACTP_MASK) != 0
    }

    /// Set MACTP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mactp(&self) {
        self.value.modify_bits(|r| r | Self::MACTP_MASK);
    }

    /// Clear MACTP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mactp(&self) {
        self.value.modify_bits(|r| r & !Self::MACTP_MASK);
    }

    /// **MACTFCS** `[18:17]` (RW)
    ///
    /// MACTFCS
    #[inline(always)]
    pub fn mactfcs(&self) -> u32 {
        (self.value.read_bits() & Self::MACTFCS_MASK) >> Self::MACTFCS_OFFSET
    }

    /// Write **MACTFCS** `[18:17]` (RW)
    ///
    /// MACTFCS
    #[inline(always)]
    pub fn write_mactfcs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MACTFCS_MASK) | ((value << Self::MACTFCS_OFFSET) & Self::MACTFCS_MASK)
        });
    }

    /// **MACTPES** `[16]` (RW)
    ///
    /// MACTPES
    #[inline(always)]
    pub fn mactpes(&self) -> bool {
        (self.value.read_bits() & Self::MACTPES_MASK) != 0
    }

    /// Set MACTPES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mactpes(&self) {
        self.value.modify_bits(|r| r | Self::MACTPES_MASK);
    }

    /// Clear MACTPES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mactpes(&self) {
        self.value.modify_bits(|r| r & !Self::MACTPES_MASK);
    }

    /// **MTLRFFLS** `[9:8]` (RW)
    ///
    /// MTLRFFLS
    #[inline(always)]
    pub fn mtlrffls(&self) -> u32 {
        (self.value.read_bits() & Self::MTLRFFLS_MASK) >> Self::MTLRFFLS_OFFSET
    }

    /// Write **MTLRFFLS** `[9:8]` (RW)
    ///
    /// MTLRFFLS
    #[inline(always)]
    pub fn write_mtlrffls(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MTLRFFLS_MASK) | ((value << Self::MTLRFFLS_OFFSET) & Self::MTLRFFLS_MASK)
        });
    }

    /// **MTLRFRCS** `[6:5]` (RW)
    ///
    /// MTLRFRCS
    #[inline(always)]
    pub fn mtlrfrcs(&self) -> u32 {
        (self.value.read_bits() & Self::MTLRFRCS_MASK) >> Self::MTLRFRCS_OFFSET
    }

    /// Write **MTLRFRCS** `[6:5]` (RW)
    ///
    /// MTLRFRCS
    #[inline(always)]
    pub fn write_mtlrfrcs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MTLRFRCS_MASK) | ((value << Self::MTLRFRCS_OFFSET) & Self::MTLRFRCS_MASK)
        });
    }

    /// **MTLRFWCAS** `[4]` (RW)
    ///
    /// MTLRFWCAS
    #[inline(always)]
    pub fn mtlrfwcas(&self) -> bool {
        (self.value.read_bits() & Self::MTLRFWCAS_MASK) != 0
    }

    /// Set MTLRFWCAS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mtlrfwcas(&self) {
        self.value.modify_bits(|r| r | Self::MTLRFWCAS_MASK);
    }

    /// Clear MTLRFWCAS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mtlrfwcas(&self) {
        self.value.modify_bits(|r| r & !Self::MTLRFWCAS_MASK);
    }

    /// **MACRFFCS** `[2:1]` (RW)
    ///
    /// MACRFFCS
    #[inline(always)]
    pub fn macrffcs(&self) -> u32 {
        (self.value.read_bits() & Self::MACRFFCS_MASK) >> Self::MACRFFCS_OFFSET
    }

    /// Write **MACRFFCS** `[2:1]` (RW)
    ///
    /// MACRFFCS
    #[inline(always)]
    pub fn write_macrffcs(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MACRFFCS_MASK) | ((value << Self::MACRFFCS_OFFSET) & Self::MACRFFCS_MASK)
        });
    }

    /// **MACRPES** `[0]` (RW)
    ///
    /// MACRPES
    #[inline(always)]
    pub fn macrpes(&self) -> bool {
        (self.value.read_bits() & Self::MACRPES_MASK) != 0
    }

    /// Set MACRPES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_macrpes(&self) {
        self.value.modify_bits(|r| r | Self::MACRPES_MASK);
    }

    /// Clear MACRPES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_macrpes(&self) {
        self.value.modify_bits(|r| r & !Self::MACRPES_MASK);
    }
}

/// EMACDEBUG value type for batch read/modify/write operations
///
/// Use `emacdebug.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacdebug.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacdebugVal {
    bits: u32,
}

impl EmacdebugVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **MTLTSFFS** `[25:25]`
    ///
    /// MTLTSFFS
    #[inline(always)]
    pub const fn mtltsffs(&self) -> bool {
        (self.bits & Emacdebug::MTLTSFFS_MASK) != 0
    }

    /// Read **MTLTFNES** `[24:24]`
    ///
    /// MTLTFNES
    #[inline(always)]
    pub const fn mtltfnes(&self) -> bool {
        (self.bits & Emacdebug::MTLTFNES_MASK) != 0
    }

    /// Read **MTLTFWCS** `[22:22]`
    ///
    /// MTLTFWCS
    #[inline(always)]
    pub const fn mtltfwcs(&self) -> bool {
        (self.bits & Emacdebug::MTLTFWCS_MASK) != 0
    }

    /// Read **MTLTFRCS** `[21:20]`
    ///
    /// MTLTFRCS
    #[inline(always)]
    pub const fn mtltfrcs(&self) -> u32 {
        (self.bits & Emacdebug::MTLTFRCS_MASK) >> Emacdebug::MTLTFRCS_OFFSET
    }

    /// Read **MACTP** `[19:19]`
    ///
    /// MACTP
    #[inline(always)]
    pub const fn mactp(&self) -> bool {
        (self.bits & Emacdebug::MACTP_MASK) != 0
    }

    /// Read **MACTFCS** `[18:17]`
    ///
    /// MACTFCS
    #[inline(always)]
    pub const fn mactfcs(&self) -> u32 {
        (self.bits & Emacdebug::MACTFCS_MASK) >> Emacdebug::MACTFCS_OFFSET
    }

    /// Read **MACTPES** `[16:16]`
    ///
    /// MACTPES
    #[inline(always)]
    pub const fn mactpes(&self) -> bool {
        (self.bits & Emacdebug::MACTPES_MASK) != 0
    }

    /// Read **MTLRFFLS** `[9:8]`
    ///
    /// MTLRFFLS
    #[inline(always)]
    pub const fn mtlrffls(&self) -> u32 {
        (self.bits & Emacdebug::MTLRFFLS_MASK) >> Emacdebug::MTLRFFLS_OFFSET
    }

    /// Read **MTLRFRCS** `[6:5]`
    ///
    /// MTLRFRCS
    #[inline(always)]
    pub const fn mtlrfrcs(&self) -> u32 {
        (self.bits & Emacdebug::MTLRFRCS_MASK) >> Emacdebug::MTLRFRCS_OFFSET
    }

    /// Read **MTLRFWCAS** `[4:4]`
    ///
    /// MTLRFWCAS
    #[inline(always)]
    pub const fn mtlrfwcas(&self) -> bool {
        (self.bits & Emacdebug::MTLRFWCAS_MASK) != 0
    }

    /// Read **MACRFFCS** `[2:1]`
    ///
    /// MACRFFCS
    #[inline(always)]
    pub const fn macrffcs(&self) -> u32 {
        (self.bits & Emacdebug::MACRFFCS_MASK) >> Emacdebug::MACRFFCS_OFFSET
    }

    /// Read **MACRPES** `[0:0]`
    ///
    /// MACRPES
    #[inline(always)]
    pub const fn macrpes(&self) -> bool {
        (self.bits & Emacdebug::MACRPES_MASK) != 0
    }

    /// Set **MTLTSFFS** `[25:25]`
    ///
    /// MTLTSFFS
    #[inline(always)]
    pub const fn with_mtltsffs(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacdebug::MTLTSFFS_MASK
            } else {
                self.bits & !Emacdebug::MTLTSFFS_MASK
            },
        }
    }

    /// Set **MTLTFNES** `[24:24]`
    ///
    /// MTLTFNES
    #[inline(always)]
    pub const fn with_mtltfnes(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacdebug::MTLTFNES_MASK
            } else {
                self.bits & !Emacdebug::MTLTFNES_MASK
            },
        }
    }

    /// Set **MTLTFWCS** `[22:22]`
    ///
    /// MTLTFWCS
    #[inline(always)]
    pub const fn with_mtltfwcs(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacdebug::MTLTFWCS_MASK
            } else {
                self.bits & !Emacdebug::MTLTFWCS_MASK
            },
        }
    }

    /// Set **MTLTFRCS** `[21:20]`
    ///
    /// MTLTFRCS
    #[inline(always)]
    pub const fn with_mtltfrcs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacdebug::MTLTFRCS_MASK)
                | ((value << Emacdebug::MTLTFRCS_OFFSET) & Emacdebug::MTLTFRCS_MASK),
        }
    }

    /// Set **MACTP** `[19:19]`
    ///
    /// MACTP
    #[inline(always)]
    pub const fn with_mactp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacdebug::MACTP_MASK
            } else {
                self.bits & !Emacdebug::MACTP_MASK
            },
        }
    }

    /// Set **MACTFCS** `[18:17]`
    ///
    /// MACTFCS
    #[inline(always)]
    pub const fn with_mactfcs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacdebug::MACTFCS_MASK)
                | ((value << Emacdebug::MACTFCS_OFFSET) & Emacdebug::MACTFCS_MASK),
        }
    }

    /// Set **MACTPES** `[16:16]`
    ///
    /// MACTPES
    #[inline(always)]
    pub const fn with_mactpes(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacdebug::MACTPES_MASK
            } else {
                self.bits & !Emacdebug::MACTPES_MASK
            },
        }
    }

    /// Set **MTLRFFLS** `[9:8]`
    ///
    /// MTLRFFLS
    #[inline(always)]
    pub const fn with_mtlrffls(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacdebug::MTLRFFLS_MASK)
                | ((value << Emacdebug::MTLRFFLS_OFFSET) & Emacdebug::MTLRFFLS_MASK),
        }
    }

    /// Set **MTLRFRCS** `[6:5]`
    ///
    /// MTLRFRCS
    #[inline(always)]
    pub const fn with_mtlrfrcs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacdebug::MTLRFRCS_MASK)
                | ((value << Emacdebug::MTLRFRCS_OFFSET) & Emacdebug::MTLRFRCS_MASK),
        }
    }

    /// Set **MTLRFWCAS** `[4:4]`
    ///
    /// MTLRFWCAS
    #[inline(always)]
    pub const fn with_mtlrfwcas(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacdebug::MTLRFWCAS_MASK
            } else {
                self.bits & !Emacdebug::MTLRFWCAS_MASK
            },
        }
    }

    /// Set **MACRFFCS** `[2:1]`
    ///
    /// MACRFFCS
    #[inline(always)]
    pub const fn with_macrffcs(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacdebug::MACRFFCS_MASK)
                | ((value << Emacdebug::MACRFFCS_OFFSET) & Emacdebug::MACRFFCS_MASK),
        }
    }

    /// Set **MACRPES** `[0:0]`
    ///
    /// MACRPES
    #[inline(always)]
    pub const fn with_macrpes(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacdebug::MACRPES_MASK
            } else {
                self.bits & !Emacdebug::MACRPES_MASK
            },
        }
    }
}

impl Default for EmacdebugVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacdebug {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacdebugVal {
        EmacdebugVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacdebugVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacdebugVal) -> EmacdebugVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMT_RWUFFR Register
///
/// PMT_RWUFFR register
///
/// Address offset: 0x0028
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PMT_RWUFFR** `[31:0]` (RW)
///
///   PMT_RWUFFR
#[repr(transparent)]
pub struct PmtRwuffr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmtRwuffr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmtRwuffr {
    // ======== Field Constants ========

    /// **PMT_RWUFFR**
    ///
    /// PMT_RWUFFR
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const PMT_RWUFFR_OFFSET: u32 = 0;
    pub const PMT_RWUFFR_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **PMT_RWUFFR** `[31:0]` (RW)
    ///
    /// PMT_RWUFFR
    #[inline(always)]
    pub fn pmt_rwuffr(&self) -> u32 {
        (self.value.read_bits() & Self::PMT_RWUFFR_MASK) >> Self::PMT_RWUFFR_OFFSET
    }

    /// Write **PMT_RWUFFR** `[31:0]` (RW)
    ///
    /// PMT_RWUFFR
    #[inline(always)]
    pub fn write_pmt_rwuffr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PMT_RWUFFR_MASK)
                | ((value << Self::PMT_RWUFFR_OFFSET) & Self::PMT_RWUFFR_MASK)
        });
    }
}

/// PMT_RWUFFR value type for batch read/modify/write operations
///
/// Use `pmtrwuffr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmtrwuffr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmtRwuffrVal {
    bits: u32,
}

impl PmtRwuffrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **PMT_RWUFFR** `[31:0]`
    ///
    /// PMT_RWUFFR
    #[inline(always)]
    pub const fn pmt_rwuffr(&self) -> u32 {
        (self.bits & PmtRwuffr::PMT_RWUFFR_MASK) >> PmtRwuffr::PMT_RWUFFR_OFFSET
    }

    /// Set **PMT_RWUFFR** `[31:0]`
    ///
    /// PMT_RWUFFR
    #[inline(always)]
    pub const fn with_pmt_rwuffr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmtRwuffr::PMT_RWUFFR_MASK)
                | ((value << PmtRwuffr::PMT_RWUFFR_OFFSET) & PmtRwuffr::PMT_RWUFFR_MASK),
        }
    }
}

impl Default for PmtRwuffrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PmtRwuffr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmtRwuffrVal {
        PmtRwuffrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmtRwuffrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmtRwuffrVal) -> PmtRwuffrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// PMT_CSR Register
///
/// PMT_CSR register
///
/// Address offset: 0x002c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RWKFILTRST** `[31]` (RW)
///
///   RWKFILTRST
///
/// - **RWKPTR** `[28:24]` (RW)
///
///   RWKPTR
///
/// - **GLBLUCAST** `[9]` (RW)
///
///   GLBLUCAST
///
/// - **RWKPRCVD** `[6]` (RW)
///
///   RWKPRCVD
///
/// - **MGKPRCVD** `[5]` (RW)
///
///   MGKPRCVD
///
/// - **RWKPKTEN** `[2]` (RW)
///
///   RWKPKTEN
///
/// - **MGKPKTEN** `[1]` (RW)
///
///   MGKPKTEN
///
/// - **PWRDWN** `[0]` (RW)
///
///   PWRDWN
#[repr(transparent)]
pub struct PmtCsr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for PmtCsr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl PmtCsr {
    // ======== Field Constants ========

    /// **RWKFILTRST**
    ///
    /// RWKFILTRST
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const RWKFILTRST_OFFSET: u32 = 31;
    pub const RWKFILTRST_MASK: u32 = 0x80000000;
    pub const RWKFILTRST: u32 = 1 << 31;

    /// **RWKPTR**
    ///
    /// RWKPTR
    ///
    /// Bits `[28:24]`, mask `0x1f000000`
    pub const RWKPTR_OFFSET: u32 = 24;
    pub const RWKPTR_MASK: u32 = 0x1f000000;

    /// **GLBLUCAST**
    ///
    /// GLBLUCAST
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const GLBLUCAST_OFFSET: u32 = 9;
    pub const GLBLUCAST_MASK: u32 = 0x00000200;
    pub const GLBLUCAST: u32 = 1 << 9;

    /// **RWKPRCVD**
    ///
    /// RWKPRCVD
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RWKPRCVD_OFFSET: u32 = 6;
    pub const RWKPRCVD_MASK: u32 = 0x00000040;
    pub const RWKPRCVD: u32 = 1 << 6;

    /// **MGKPRCVD**
    ///
    /// MGKPRCVD
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const MGKPRCVD_OFFSET: u32 = 5;
    pub const MGKPRCVD_MASK: u32 = 0x00000020;
    pub const MGKPRCVD: u32 = 1 << 5;

    /// **RWKPKTEN**
    ///
    /// RWKPKTEN
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RWKPKTEN_OFFSET: u32 = 2;
    pub const RWKPKTEN_MASK: u32 = 0x00000004;
    pub const RWKPKTEN: u32 = 1 << 2;

    /// **MGKPKTEN**
    ///
    /// MGKPKTEN
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const MGKPKTEN_OFFSET: u32 = 1;
    pub const MGKPKTEN_MASK: u32 = 0x00000002;
    pub const MGKPKTEN: u32 = 1 << 1;

    /// **PWRDWN**
    ///
    /// PWRDWN
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const PWRDWN_OFFSET: u32 = 0;
    pub const PWRDWN_MASK: u32 = 0x00000001;
    pub const PWRDWN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **RWKFILTRST** `[31]` (RW)
    ///
    /// RWKFILTRST
    #[inline(always)]
    pub fn rwkfiltrst(&self) -> bool {
        (self.value.read_bits() & Self::RWKFILTRST_MASK) != 0
    }

    /// Set RWKFILTRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rwkfiltrst(&self) {
        self.value.modify_bits(|r| r | Self::RWKFILTRST_MASK);
    }

    /// Clear RWKFILTRST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rwkfiltrst(&self) {
        self.value.modify_bits(|r| r & !Self::RWKFILTRST_MASK);
    }

    /// **RWKPTR** `[28:24]` (RW)
    ///
    /// RWKPTR
    #[inline(always)]
    pub fn rwkptr(&self) -> u32 {
        (self.value.read_bits() & Self::RWKPTR_MASK) >> Self::RWKPTR_OFFSET
    }

    /// Write **RWKPTR** `[28:24]` (RW)
    ///
    /// RWKPTR
    #[inline(always)]
    pub fn write_rwkptr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RWKPTR_MASK) | ((value << Self::RWKPTR_OFFSET) & Self::RWKPTR_MASK)
        });
    }

    /// **GLBLUCAST** `[9]` (RW)
    ///
    /// GLBLUCAST
    #[inline(always)]
    pub fn glblucast(&self) -> bool {
        (self.value.read_bits() & Self::GLBLUCAST_MASK) != 0
    }

    /// Set GLBLUCAST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_glblucast(&self) {
        self.value.modify_bits(|r| r | Self::GLBLUCAST_MASK);
    }

    /// Clear GLBLUCAST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_glblucast(&self) {
        self.value.modify_bits(|r| r & !Self::GLBLUCAST_MASK);
    }

    /// **RWKPRCVD** `[6]` (RW)
    ///
    /// RWKPRCVD
    #[inline(always)]
    pub fn rwkprcvd(&self) -> bool {
        (self.value.read_bits() & Self::RWKPRCVD_MASK) != 0
    }

    /// Set RWKPRCVD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rwkprcvd(&self) {
        self.value.modify_bits(|r| r | Self::RWKPRCVD_MASK);
    }

    /// Clear RWKPRCVD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rwkprcvd(&self) {
        self.value.modify_bits(|r| r & !Self::RWKPRCVD_MASK);
    }

    /// **MGKPRCVD** `[5]` (RW)
    ///
    /// MGKPRCVD
    #[inline(always)]
    pub fn mgkprcvd(&self) -> bool {
        (self.value.read_bits() & Self::MGKPRCVD_MASK) != 0
    }

    /// Set MGKPRCVD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mgkprcvd(&self) {
        self.value.modify_bits(|r| r | Self::MGKPRCVD_MASK);
    }

    /// Clear MGKPRCVD bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mgkprcvd(&self) {
        self.value.modify_bits(|r| r & !Self::MGKPRCVD_MASK);
    }

    /// **RWKPKTEN** `[2]` (RW)
    ///
    /// RWKPKTEN
    #[inline(always)]
    pub fn rwkpkten(&self) -> bool {
        (self.value.read_bits() & Self::RWKPKTEN_MASK) != 0
    }

    /// Set RWKPKTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rwkpkten(&self) {
        self.value.modify_bits(|r| r | Self::RWKPKTEN_MASK);
    }

    /// Clear RWKPKTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rwkpkten(&self) {
        self.value.modify_bits(|r| r & !Self::RWKPKTEN_MASK);
    }

    /// **MGKPKTEN** `[1]` (RW)
    ///
    /// MGKPKTEN
    #[inline(always)]
    pub fn mgkpkten(&self) -> bool {
        (self.value.read_bits() & Self::MGKPKTEN_MASK) != 0
    }

    /// Set MGKPKTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_mgkpkten(&self) {
        self.value.modify_bits(|r| r | Self::MGKPKTEN_MASK);
    }

    /// Clear MGKPKTEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_mgkpkten(&self) {
        self.value.modify_bits(|r| r & !Self::MGKPKTEN_MASK);
    }

    /// **PWRDWN** `[0]` (RW)
    ///
    /// PWRDWN
    #[inline(always)]
    pub fn pwrdwn(&self) -> bool {
        (self.value.read_bits() & Self::PWRDWN_MASK) != 0
    }

    /// Set PWRDWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwrdwn(&self) {
        self.value.modify_bits(|r| r | Self::PWRDWN_MASK);
    }

    /// Clear PWRDWN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwrdwn(&self) {
        self.value.modify_bits(|r| r & !Self::PWRDWN_MASK);
    }
}

/// PMT_CSR value type for batch read/modify/write operations
///
/// Use `pmtcsr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `pmtcsr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct PmtCsrVal {
    bits: u32,
}

impl PmtCsrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **RWKFILTRST** `[31:31]`
    ///
    /// RWKFILTRST
    #[inline(always)]
    pub const fn rwkfiltrst(&self) -> bool {
        (self.bits & PmtCsr::RWKFILTRST_MASK) != 0
    }

    /// Read **RWKPTR** `[28:24]`
    ///
    /// RWKPTR
    #[inline(always)]
    pub const fn rwkptr(&self) -> u32 {
        (self.bits & PmtCsr::RWKPTR_MASK) >> PmtCsr::RWKPTR_OFFSET
    }

    /// Read **GLBLUCAST** `[9:9]`
    ///
    /// GLBLUCAST
    #[inline(always)]
    pub const fn glblucast(&self) -> bool {
        (self.bits & PmtCsr::GLBLUCAST_MASK) != 0
    }

    /// Read **RWKPRCVD** `[6:6]`
    ///
    /// RWKPRCVD
    #[inline(always)]
    pub const fn rwkprcvd(&self) -> bool {
        (self.bits & PmtCsr::RWKPRCVD_MASK) != 0
    }

    /// Read **MGKPRCVD** `[5:5]`
    ///
    /// MGKPRCVD
    #[inline(always)]
    pub const fn mgkprcvd(&self) -> bool {
        (self.bits & PmtCsr::MGKPRCVD_MASK) != 0
    }

    /// Read **RWKPKTEN** `[2:2]`
    ///
    /// RWKPKTEN
    #[inline(always)]
    pub const fn rwkpkten(&self) -> bool {
        (self.bits & PmtCsr::RWKPKTEN_MASK) != 0
    }

    /// Read **MGKPKTEN** `[1:1]`
    ///
    /// MGKPKTEN
    #[inline(always)]
    pub const fn mgkpkten(&self) -> bool {
        (self.bits & PmtCsr::MGKPKTEN_MASK) != 0
    }

    /// Read **PWRDWN** `[0:0]`
    ///
    /// PWRDWN
    #[inline(always)]
    pub const fn pwrdwn(&self) -> bool {
        (self.bits & PmtCsr::PWRDWN_MASK) != 0
    }

    /// Set **RWKFILTRST** `[31:31]`
    ///
    /// RWKFILTRST
    #[inline(always)]
    pub const fn with_rwkfiltrst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmtCsr::RWKFILTRST_MASK
            } else {
                self.bits & !PmtCsr::RWKFILTRST_MASK
            },
        }
    }

    /// Set **RWKPTR** `[28:24]`
    ///
    /// RWKPTR
    #[inline(always)]
    pub const fn with_rwkptr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !PmtCsr::RWKPTR_MASK)
                | ((value << PmtCsr::RWKPTR_OFFSET) & PmtCsr::RWKPTR_MASK),
        }
    }

    /// Set **GLBLUCAST** `[9:9]`
    ///
    /// GLBLUCAST
    #[inline(always)]
    pub const fn with_glblucast(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmtCsr::GLBLUCAST_MASK
            } else {
                self.bits & !PmtCsr::GLBLUCAST_MASK
            },
        }
    }

    /// Set **RWKPRCVD** `[6:6]`
    ///
    /// RWKPRCVD
    #[inline(always)]
    pub const fn with_rwkprcvd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmtCsr::RWKPRCVD_MASK
            } else {
                self.bits & !PmtCsr::RWKPRCVD_MASK
            },
        }
    }

    /// Set **MGKPRCVD** `[5:5]`
    ///
    /// MGKPRCVD
    #[inline(always)]
    pub const fn with_mgkprcvd(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmtCsr::MGKPRCVD_MASK
            } else {
                self.bits & !PmtCsr::MGKPRCVD_MASK
            },
        }
    }

    /// Set **RWKPKTEN** `[2:2]`
    ///
    /// RWKPKTEN
    #[inline(always)]
    pub const fn with_rwkpkten(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmtCsr::RWKPKTEN_MASK
            } else {
                self.bits & !PmtCsr::RWKPKTEN_MASK
            },
        }
    }

    /// Set **MGKPKTEN** `[1:1]`
    ///
    /// MGKPKTEN
    #[inline(always)]
    pub const fn with_mgkpkten(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmtCsr::MGKPKTEN_MASK
            } else {
                self.bits & !PmtCsr::MGKPKTEN_MASK
            },
        }
    }

    /// Set **PWRDWN** `[0:0]`
    ///
    /// PWRDWN
    #[inline(always)]
    pub const fn with_pwrdwn(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | PmtCsr::PWRDWN_MASK
            } else {
                self.bits & !PmtCsr::PWRDWN_MASK
            },
        }
    }
}

impl Default for PmtCsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl PmtCsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> PmtCsrVal {
        PmtCsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: PmtCsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(PmtCsrVal) -> PmtCsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACLPI_CSR Register
///
/// EMACLPI_CSR register
///
/// Address offset: 0x0030
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LPITXA** `[19]` (RW)
///
///   LPITXA
///
/// - **PLS** `[17]` (RW)
///
///   PLS
///
/// - **LPIEN** `[16]` (RW)
///
///   LPIEN
///
/// - **RLPIST** `[9]` (RW)
///
///   RLPIST
///
/// - **TLPIST** `[8]` (RW)
///
///   TLPIST
///
/// - **RLPIEX** `[3]` (RW)
///
///   RLPIEX
///
/// - **RLPIEN** `[2]` (RW)
///
///   RLPIEN
///
/// - **TLPIEX** `[1]` (RW)
///
///   TLPIEX
///
/// - **TLPIEN** `[0]` (RW)
///
///   TLPIEN
#[repr(transparent)]
pub struct EmaclpiCsr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for EmaclpiCsr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl EmaclpiCsr {
    // ======== Field Constants ========

    /// **LPITXA**
    ///
    /// LPITXA
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const LPITXA_OFFSET: u32 = 19;
    pub const LPITXA_MASK: u32 = 0x00080000;
    pub const LPITXA: u32 = 1 << 19;

    /// **PLS**
    ///
    /// PLS
    ///
    /// Bits `[17:17]`, mask `0x00020000`
    pub const PLS_OFFSET: u32 = 17;
    pub const PLS_MASK: u32 = 0x00020000;
    pub const PLS: u32 = 1 << 17;

    /// **LPIEN**
    ///
    /// LPIEN
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const LPIEN_OFFSET: u32 = 16;
    pub const LPIEN_MASK: u32 = 0x00010000;
    pub const LPIEN: u32 = 1 << 16;

    /// **RLPIST**
    ///
    /// RLPIST
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RLPIST_OFFSET: u32 = 9;
    pub const RLPIST_MASK: u32 = 0x00000200;
    pub const RLPIST: u32 = 1 << 9;

    /// **TLPIST**
    ///
    /// TLPIST
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TLPIST_OFFSET: u32 = 8;
    pub const TLPIST_MASK: u32 = 0x00000100;
    pub const TLPIST: u32 = 1 << 8;

    /// **RLPIEX**
    ///
    /// RLPIEX
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const RLPIEX_OFFSET: u32 = 3;
    pub const RLPIEX_MASK: u32 = 0x00000008;
    pub const RLPIEX: u32 = 1 << 3;

    /// **RLPIEN**
    ///
    /// RLPIEN
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const RLPIEN_OFFSET: u32 = 2;
    pub const RLPIEN_MASK: u32 = 0x00000004;
    pub const RLPIEN: u32 = 1 << 2;

    /// **TLPIEX**
    ///
    /// TLPIEX
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TLPIEX_OFFSET: u32 = 1;
    pub const TLPIEX_MASK: u32 = 0x00000002;
    pub const TLPIEX: u32 = 1 << 1;

    /// **TLPIEN**
    ///
    /// TLPIEN
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TLPIEN_OFFSET: u32 = 0;
    pub const TLPIEN_MASK: u32 = 0x00000001;
    pub const TLPIEN: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **LPITXA** `[19]` (RW)
    ///
    /// LPITXA
    #[inline(always)]
    pub fn lpitxa(&self) -> bool {
        (self.value.read_bits() & Self::LPITXA_MASK) != 0
    }

    /// Set LPITXA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lpitxa(&self) {
        self.value.modify_bits(|r| r | Self::LPITXA_MASK);
    }

    /// Clear LPITXA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lpitxa(&self) {
        self.value.modify_bits(|r| r & !Self::LPITXA_MASK);
    }

    /// **PLS** `[17]` (RW)
    ///
    /// PLS
    #[inline(always)]
    pub fn pls(&self) -> bool {
        (self.value.read_bits() & Self::PLS_MASK) != 0
    }

    /// Set PLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pls(&self) {
        self.value.modify_bits(|r| r | Self::PLS_MASK);
    }

    /// Clear PLS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pls(&self) {
        self.value.modify_bits(|r| r & !Self::PLS_MASK);
    }

    /// **LPIEN** `[16]` (RW)
    ///
    /// LPIEN
    #[inline(always)]
    pub fn lpien(&self) -> bool {
        (self.value.read_bits() & Self::LPIEN_MASK) != 0
    }

    /// Set LPIEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lpien(&self) {
        self.value.modify_bits(|r| r | Self::LPIEN_MASK);
    }

    /// Clear LPIEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lpien(&self) {
        self.value.modify_bits(|r| r & !Self::LPIEN_MASK);
    }

    /// **RLPIST** `[9]` (RW)
    ///
    /// RLPIST
    #[inline(always)]
    pub fn rlpist(&self) -> bool {
        (self.value.read_bits() & Self::RLPIST_MASK) != 0
    }

    /// Set RLPIST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rlpist(&self) {
        self.value.modify_bits(|r| r | Self::RLPIST_MASK);
    }

    /// Clear RLPIST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rlpist(&self) {
        self.value.modify_bits(|r| r & !Self::RLPIST_MASK);
    }

    /// **TLPIST** `[8]` (RW)
    ///
    /// TLPIST
    #[inline(always)]
    pub fn tlpist(&self) -> bool {
        (self.value.read_bits() & Self::TLPIST_MASK) != 0
    }

    /// Set TLPIST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tlpist(&self) {
        self.value.modify_bits(|r| r | Self::TLPIST_MASK);
    }

    /// Clear TLPIST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tlpist(&self) {
        self.value.modify_bits(|r| r & !Self::TLPIST_MASK);
    }

    /// **RLPIEX** `[3]` (RW)
    ///
    /// RLPIEX
    #[inline(always)]
    pub fn rlpiex(&self) -> bool {
        (self.value.read_bits() & Self::RLPIEX_MASK) != 0
    }

    /// Set RLPIEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rlpiex(&self) {
        self.value.modify_bits(|r| r | Self::RLPIEX_MASK);
    }

    /// Clear RLPIEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rlpiex(&self) {
        self.value.modify_bits(|r| r & !Self::RLPIEX_MASK);
    }

    /// **RLPIEN** `[2]` (RW)
    ///
    /// RLPIEN
    #[inline(always)]
    pub fn rlpien(&self) -> bool {
        (self.value.read_bits() & Self::RLPIEN_MASK) != 0
    }

    /// Set RLPIEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_rlpien(&self) {
        self.value.modify_bits(|r| r | Self::RLPIEN_MASK);
    }

    /// Clear RLPIEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_rlpien(&self) {
        self.value.modify_bits(|r| r & !Self::RLPIEN_MASK);
    }

    /// **TLPIEX** `[1]` (RW)
    ///
    /// TLPIEX
    #[inline(always)]
    pub fn tlpiex(&self) -> bool {
        (self.value.read_bits() & Self::TLPIEX_MASK) != 0
    }

    /// Set TLPIEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tlpiex(&self) {
        self.value.modify_bits(|r| r | Self::TLPIEX_MASK);
    }

    /// Clear TLPIEX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tlpiex(&self) {
        self.value.modify_bits(|r| r & !Self::TLPIEX_MASK);
    }

    /// **TLPIEN** `[0]` (RW)
    ///
    /// TLPIEN
    #[inline(always)]
    pub fn tlpien(&self) -> bool {
        (self.value.read_bits() & Self::TLPIEN_MASK) != 0
    }

    /// Set TLPIEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tlpien(&self) {
        self.value.modify_bits(|r| r | Self::TLPIEN_MASK);
    }

    /// Clear TLPIEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tlpien(&self) {
        self.value.modify_bits(|r| r & !Self::TLPIEN_MASK);
    }
}

/// EMACLPI_CSR value type for batch read/modify/write operations
///
/// Use `emaclpicsr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emaclpicsr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmaclpiCsrVal {
    bits: u32,
}

impl EmaclpiCsrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **LPITXA** `[19:19]`
    ///
    /// LPITXA
    #[inline(always)]
    pub const fn lpitxa(&self) -> bool {
        (self.bits & EmaclpiCsr::LPITXA_MASK) != 0
    }

    /// Read **PLS** `[17:17]`
    ///
    /// PLS
    #[inline(always)]
    pub const fn pls(&self) -> bool {
        (self.bits & EmaclpiCsr::PLS_MASK) != 0
    }

    /// Read **LPIEN** `[16:16]`
    ///
    /// LPIEN
    #[inline(always)]
    pub const fn lpien(&self) -> bool {
        (self.bits & EmaclpiCsr::LPIEN_MASK) != 0
    }

    /// Read **RLPIST** `[9:9]`
    ///
    /// RLPIST
    #[inline(always)]
    pub const fn rlpist(&self) -> bool {
        (self.bits & EmaclpiCsr::RLPIST_MASK) != 0
    }

    /// Read **TLPIST** `[8:8]`
    ///
    /// TLPIST
    #[inline(always)]
    pub const fn tlpist(&self) -> bool {
        (self.bits & EmaclpiCsr::TLPIST_MASK) != 0
    }

    /// Read **RLPIEX** `[3:3]`
    ///
    /// RLPIEX
    #[inline(always)]
    pub const fn rlpiex(&self) -> bool {
        (self.bits & EmaclpiCsr::RLPIEX_MASK) != 0
    }

    /// Read **RLPIEN** `[2:2]`
    ///
    /// RLPIEN
    #[inline(always)]
    pub const fn rlpien(&self) -> bool {
        (self.bits & EmaclpiCsr::RLPIEN_MASK) != 0
    }

    /// Read **TLPIEX** `[1:1]`
    ///
    /// TLPIEX
    #[inline(always)]
    pub const fn tlpiex(&self) -> bool {
        (self.bits & EmaclpiCsr::TLPIEX_MASK) != 0
    }

    /// Read **TLPIEN** `[0:0]`
    ///
    /// TLPIEN
    #[inline(always)]
    pub const fn tlpien(&self) -> bool {
        (self.bits & EmaclpiCsr::TLPIEN_MASK) != 0
    }

    /// Set **LPITXA** `[19:19]`
    ///
    /// LPITXA
    #[inline(always)]
    pub const fn with_lpitxa(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::LPITXA_MASK
            } else {
                self.bits & !EmaclpiCsr::LPITXA_MASK
            },
        }
    }

    /// Set **PLS** `[17:17]`
    ///
    /// PLS
    #[inline(always)]
    pub const fn with_pls(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::PLS_MASK
            } else {
                self.bits & !EmaclpiCsr::PLS_MASK
            },
        }
    }

    /// Set **LPIEN** `[16:16]`
    ///
    /// LPIEN
    #[inline(always)]
    pub const fn with_lpien(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::LPIEN_MASK
            } else {
                self.bits & !EmaclpiCsr::LPIEN_MASK
            },
        }
    }

    /// Set **RLPIST** `[9:9]`
    ///
    /// RLPIST
    #[inline(always)]
    pub const fn with_rlpist(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::RLPIST_MASK
            } else {
                self.bits & !EmaclpiCsr::RLPIST_MASK
            },
        }
    }

    /// Set **TLPIST** `[8:8]`
    ///
    /// TLPIST
    #[inline(always)]
    pub const fn with_tlpist(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::TLPIST_MASK
            } else {
                self.bits & !EmaclpiCsr::TLPIST_MASK
            },
        }
    }

    /// Set **RLPIEX** `[3:3]`
    ///
    /// RLPIEX
    #[inline(always)]
    pub const fn with_rlpiex(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::RLPIEX_MASK
            } else {
                self.bits & !EmaclpiCsr::RLPIEX_MASK
            },
        }
    }

    /// Set **RLPIEN** `[2:2]`
    ///
    /// RLPIEN
    #[inline(always)]
    pub const fn with_rlpien(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::RLPIEN_MASK
            } else {
                self.bits & !EmaclpiCsr::RLPIEN_MASK
            },
        }
    }

    /// Set **TLPIEX** `[1:1]`
    ///
    /// TLPIEX
    #[inline(always)]
    pub const fn with_tlpiex(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::TLPIEX_MASK
            } else {
                self.bits & !EmaclpiCsr::TLPIEX_MASK
            },
        }
    }

    /// Set **TLPIEN** `[0:0]`
    ///
    /// TLPIEN
    #[inline(always)]
    pub const fn with_tlpien(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | EmaclpiCsr::TLPIEN_MASK
            } else {
                self.bits & !EmaclpiCsr::TLPIEN_MASK
            },
        }
    }
}

impl Default for EmaclpiCsrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl EmaclpiCsr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmaclpiCsrVal {
        EmaclpiCsrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmaclpiCsrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmaclpiCsrVal) -> EmaclpiCsrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACLPITIMERSCONTROL Register
///
/// EMACLPITIMERSCONTROL register
///
/// Address offset: 0x0034
/// Reset value: 0x03e80000
///
/// ## Fields
///
/// - **LPI_LS_TIMER** `[25:16]` (RW)
///
///   LPI_LS_TIMER
///
/// - **LPI_TW_TIMER** `[15:0]` (RW)
///
///   LPI_TW_TIMER
#[repr(transparent)]
pub struct Emaclpitimerscontrol {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emaclpitimerscontrol {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emaclpitimerscontrol {
    // ======== Field Constants ========

    /// **LPI_LS_TIMER**
    ///
    /// LPI_LS_TIMER
    ///
    /// Bits `[25:16]`, mask `0x03ff0000`
    pub const LPI_LS_TIMER_OFFSET: u32 = 16;
    pub const LPI_LS_TIMER_MASK: u32 = 0x03ff0000;

    /// **LPI_TW_TIMER**
    ///
    /// LPI_TW_TIMER
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const LPI_TW_TIMER_OFFSET: u32 = 0;
    pub const LPI_TW_TIMER_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **LPI_LS_TIMER** `[25:16]` (RW)
    ///
    /// LPI_LS_TIMER
    #[inline(always)]
    pub fn lpi_ls_timer(&self) -> u32 {
        (self.value.read_bits() & Self::LPI_LS_TIMER_MASK) >> Self::LPI_LS_TIMER_OFFSET
    }

    /// Write **LPI_LS_TIMER** `[25:16]` (RW)
    ///
    /// LPI_LS_TIMER
    #[inline(always)]
    pub fn write_lpi_ls_timer(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LPI_LS_TIMER_MASK)
                | ((value << Self::LPI_LS_TIMER_OFFSET) & Self::LPI_LS_TIMER_MASK)
        });
    }

    /// **LPI_TW_TIMER** `[15:0]` (RW)
    ///
    /// LPI_TW_TIMER
    #[inline(always)]
    pub fn lpi_tw_timer(&self) -> u32 {
        (self.value.read_bits() & Self::LPI_TW_TIMER_MASK) >> Self::LPI_TW_TIMER_OFFSET
    }

    /// Write **LPI_TW_TIMER** `[15:0]` (RW)
    ///
    /// LPI_TW_TIMER
    #[inline(always)]
    pub fn write_lpi_tw_timer(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::LPI_TW_TIMER_MASK)
                | ((value << Self::LPI_TW_TIMER_OFFSET) & Self::LPI_TW_TIMER_MASK)
        });
    }
}

/// EMACLPITIMERSCONTROL value type for batch read/modify/write operations
///
/// Use `emaclpitimerscontrol.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emaclpitimerscontrol.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmaclpitimerscontrolVal {
    bits: u32,
}

impl EmaclpitimerscontrolVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x03e80000 }
    }

    /// Read **LPI_LS_TIMER** `[25:16]`
    ///
    /// LPI_LS_TIMER
    #[inline(always)]
    pub const fn lpi_ls_timer(&self) -> u32 {
        (self.bits & Emaclpitimerscontrol::LPI_LS_TIMER_MASK)
            >> Emaclpitimerscontrol::LPI_LS_TIMER_OFFSET
    }

    /// Read **LPI_TW_TIMER** `[15:0]`
    ///
    /// LPI_TW_TIMER
    #[inline(always)]
    pub const fn lpi_tw_timer(&self) -> u32 {
        (self.bits & Emaclpitimerscontrol::LPI_TW_TIMER_MASK)
            >> Emaclpitimerscontrol::LPI_TW_TIMER_OFFSET
    }

    /// Set **LPI_LS_TIMER** `[25:16]`
    ///
    /// LPI_LS_TIMER
    #[inline(always)]
    pub const fn with_lpi_ls_timer(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emaclpitimerscontrol::LPI_LS_TIMER_MASK)
                | ((value << Emaclpitimerscontrol::LPI_LS_TIMER_OFFSET)
                    & Emaclpitimerscontrol::LPI_LS_TIMER_MASK),
        }
    }

    /// Set **LPI_TW_TIMER** `[15:0]`
    ///
    /// LPI_TW_TIMER
    #[inline(always)]
    pub const fn with_lpi_tw_timer(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emaclpitimerscontrol::LPI_TW_TIMER_MASK)
                | ((value << Emaclpitimerscontrol::LPI_TW_TIMER_OFFSET)
                    & Emaclpitimerscontrol::LPI_TW_TIMER_MASK),
        }
    }
}

impl Default for EmaclpitimerscontrolVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emaclpitimerscontrol {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmaclpitimerscontrolVal {
        EmaclpitimerscontrolVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmaclpitimerscontrolVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmaclpitimerscontrolVal) -> EmaclpitimerscontrolVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACINTS Register
///
/// EMACINTS register
///
/// Address offset: 0x0038
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LPIINTS** `[10]` (RW)
///
///   LPIINTS
///
/// - **TINTS** `[9]` (RW)
///
///   TINTS
///
/// - **PMTINTS** `[3]` (RW)
///
///   PMTINTS
#[repr(transparent)]
pub struct Emacints {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacints {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacints {
    // ======== Field Constants ========

    /// **LPIINTS**
    ///
    /// LPIINTS
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LPIINTS_OFFSET: u32 = 10;
    pub const LPIINTS_MASK: u32 = 0x00000400;
    pub const LPIINTS: u32 = 1 << 10;

    /// **TINTS**
    ///
    /// TINTS
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TINTS_OFFSET: u32 = 9;
    pub const TINTS_MASK: u32 = 0x00000200;
    pub const TINTS: u32 = 1 << 9;

    /// **PMTINTS**
    ///
    /// PMTINTS
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PMTINTS_OFFSET: u32 = 3;
    pub const PMTINTS_MASK: u32 = 0x00000008;
    pub const PMTINTS: u32 = 1 << 3;

    // ======== Field Accessors ========

    /// **LPIINTS** `[10]` (RW)
    ///
    /// LPIINTS
    #[inline(always)]
    pub fn lpiints(&self) -> bool {
        (self.value.read_bits() & Self::LPIINTS_MASK) != 0
    }

    /// Set LPIINTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lpiints(&self) {
        self.value.modify_bits(|r| r | Self::LPIINTS_MASK);
    }

    /// Clear LPIINTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lpiints(&self) {
        self.value.modify_bits(|r| r & !Self::LPIINTS_MASK);
    }

    /// **TINTS** `[9]` (RW)
    ///
    /// TINTS
    #[inline(always)]
    pub fn tints(&self) -> bool {
        (self.value.read_bits() & Self::TINTS_MASK) != 0
    }

    /// Set TINTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tints(&self) {
        self.value.modify_bits(|r| r | Self::TINTS_MASK);
    }

    /// Clear TINTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tints(&self) {
        self.value.modify_bits(|r| r & !Self::TINTS_MASK);
    }

    /// **PMTINTS** `[3]` (RW)
    ///
    /// PMTINTS
    #[inline(always)]
    pub fn pmtints(&self) -> bool {
        (self.value.read_bits() & Self::PMTINTS_MASK) != 0
    }

    /// Set PMTINTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pmtints(&self) {
        self.value.modify_bits(|r| r | Self::PMTINTS_MASK);
    }

    /// Clear PMTINTS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pmtints(&self) {
        self.value.modify_bits(|r| r & !Self::PMTINTS_MASK);
    }
}

/// EMACINTS value type for batch read/modify/write operations
///
/// Use `emacints.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacints.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacintsVal {
    bits: u32,
}

impl EmacintsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **LPIINTS** `[10:10]`
    ///
    /// LPIINTS
    #[inline(always)]
    pub const fn lpiints(&self) -> bool {
        (self.bits & Emacints::LPIINTS_MASK) != 0
    }

    /// Read **TINTS** `[9:9]`
    ///
    /// TINTS
    #[inline(always)]
    pub const fn tints(&self) -> bool {
        (self.bits & Emacints::TINTS_MASK) != 0
    }

    /// Read **PMTINTS** `[3:3]`
    ///
    /// PMTINTS
    #[inline(always)]
    pub const fn pmtints(&self) -> bool {
        (self.bits & Emacints::PMTINTS_MASK) != 0
    }

    /// Set **LPIINTS** `[10:10]`
    ///
    /// LPIINTS
    #[inline(always)]
    pub const fn with_lpiints(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacints::LPIINTS_MASK
            } else {
                self.bits & !Emacints::LPIINTS_MASK
            },
        }
    }

    /// Set **TINTS** `[9:9]`
    ///
    /// TINTS
    #[inline(always)]
    pub const fn with_tints(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacints::TINTS_MASK
            } else {
                self.bits & !Emacints::TINTS_MASK
            },
        }
    }

    /// Set **PMTINTS** `[3:3]`
    ///
    /// PMTINTS
    #[inline(always)]
    pub const fn with_pmtints(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacints::PMTINTS_MASK
            } else {
                self.bits & !Emacints::PMTINTS_MASK
            },
        }
    }
}

impl Default for EmacintsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacints {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacintsVal {
        EmacintsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacintsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacintsVal) -> EmacintsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACINTMASK Register
///
/// EMACINTMASK register
///
/// Address offset: 0x003c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **LPIINTMASK** `[10]` (RW)
///
///   LPIINTMASK
///
/// - **TSINTMASK** `[9]` (RW)
///
///   TSINTMASK
///
/// - **PMTINTMASK** `[3]` (RW)
///
///   PMTINTMASK
#[repr(transparent)]
pub struct Emacintmask {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacintmask {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacintmask {
    // ======== Field Constants ========

    /// **LPIINTMASK**
    ///
    /// LPIINTMASK
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const LPIINTMASK_OFFSET: u32 = 10;
    pub const LPIINTMASK_MASK: u32 = 0x00000400;
    pub const LPIINTMASK: u32 = 1 << 10;

    /// **TSINTMASK**
    ///
    /// TSINTMASK
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TSINTMASK_OFFSET: u32 = 9;
    pub const TSINTMASK_MASK: u32 = 0x00000200;
    pub const TSINTMASK: u32 = 1 << 9;

    /// **PMTINTMASK**
    ///
    /// PMTINTMASK
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const PMTINTMASK_OFFSET: u32 = 3;
    pub const PMTINTMASK_MASK: u32 = 0x00000008;
    pub const PMTINTMASK: u32 = 1 << 3;

    // ======== Field Accessors ========

    /// **LPIINTMASK** `[10]` (RW)
    ///
    /// LPIINTMASK
    #[inline(always)]
    pub fn lpiintmask(&self) -> bool {
        (self.value.read_bits() & Self::LPIINTMASK_MASK) != 0
    }

    /// Set LPIINTMASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_lpiintmask(&self) {
        self.value.modify_bits(|r| r | Self::LPIINTMASK_MASK);
    }

    /// Clear LPIINTMASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_lpiintmask(&self) {
        self.value.modify_bits(|r| r & !Self::LPIINTMASK_MASK);
    }

    /// **TSINTMASK** `[9]` (RW)
    ///
    /// TSINTMASK
    #[inline(always)]
    pub fn tsintmask(&self) -> bool {
        (self.value.read_bits() & Self::TSINTMASK_MASK) != 0
    }

    /// Set TSINTMASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsintmask(&self) {
        self.value.modify_bits(|r| r | Self::TSINTMASK_MASK);
    }

    /// Clear TSINTMASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsintmask(&self) {
        self.value.modify_bits(|r| r & !Self::TSINTMASK_MASK);
    }

    /// **PMTINTMASK** `[3]` (RW)
    ///
    /// PMTINTMASK
    #[inline(always)]
    pub fn pmtintmask(&self) -> bool {
        (self.value.read_bits() & Self::PMTINTMASK_MASK) != 0
    }

    /// Set PMTINTMASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pmtintmask(&self) {
        self.value.modify_bits(|r| r | Self::PMTINTMASK_MASK);
    }

    /// Clear PMTINTMASK bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pmtintmask(&self) {
        self.value.modify_bits(|r| r & !Self::PMTINTMASK_MASK);
    }
}

/// EMACINTMASK value type for batch read/modify/write operations
///
/// Use `emacintmask.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacintmask.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacintmaskVal {
    bits: u32,
}

impl EmacintmaskVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **LPIINTMASK** `[10:10]`
    ///
    /// LPIINTMASK
    #[inline(always)]
    pub const fn lpiintmask(&self) -> bool {
        (self.bits & Emacintmask::LPIINTMASK_MASK) != 0
    }

    /// Read **TSINTMASK** `[9:9]`
    ///
    /// TSINTMASK
    #[inline(always)]
    pub const fn tsintmask(&self) -> bool {
        (self.bits & Emacintmask::TSINTMASK_MASK) != 0
    }

    /// Read **PMTINTMASK** `[3:3]`
    ///
    /// PMTINTMASK
    #[inline(always)]
    pub const fn pmtintmask(&self) -> bool {
        (self.bits & Emacintmask::PMTINTMASK_MASK) != 0
    }

    /// Set **LPIINTMASK** `[10:10]`
    ///
    /// LPIINTMASK
    #[inline(always)]
    pub const fn with_lpiintmask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacintmask::LPIINTMASK_MASK
            } else {
                self.bits & !Emacintmask::LPIINTMASK_MASK
            },
        }
    }

    /// Set **TSINTMASK** `[9:9]`
    ///
    /// TSINTMASK
    #[inline(always)]
    pub const fn with_tsintmask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacintmask::TSINTMASK_MASK
            } else {
                self.bits & !Emacintmask::TSINTMASK_MASK
            },
        }
    }

    /// Set **PMTINTMASK** `[3:3]`
    ///
    /// PMTINTMASK
    #[inline(always)]
    pub const fn with_pmtintmask(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacintmask::PMTINTMASK_MASK
            } else {
                self.bits & !Emacintmask::PMTINTMASK_MASK
            },
        }
    }
}

impl Default for EmacintmaskVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacintmask {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacintmaskVal {
        EmacintmaskVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacintmaskVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacintmaskVal) -> EmacintmaskVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR0HIGH Register
///
/// EMACADDR0HIGH register
///
/// Address offset: 0x0040
/// Reset value: 0x8000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE0** `[31]` (RW)
///
///   ADDRESS_ENABLE0
///
/// - **MAC_ADDRESS0_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS0_HI
#[repr(transparent)]
pub struct Emacaddr0high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr0high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr0high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE0**
    ///
    /// ADDRESS_ENABLE0
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE0_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE0_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE0: u32 = 1 << 31;

    /// **MAC_ADDRESS0_HI**
    ///
    /// MAC_ADDRESS0_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS0_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS0_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE0** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE0
    #[inline(always)]
    pub fn address_enable0(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE0_MASK) != 0
    }

    /// Set ADDRESS_ENABLE0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable0(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE0_MASK);
    }

    /// Clear ADDRESS_ENABLE0 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable0(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE0_MASK);
    }

    /// **MAC_ADDRESS0_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS0_HI
    #[inline(always)]
    pub fn mac_address0_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS0_HI_MASK) >> Self::MAC_ADDRESS0_HI_OFFSET
    }

    /// Write **MAC_ADDRESS0_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS0_HI
    #[inline(always)]
    pub fn write_mac_address0_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS0_HI_MASK)
                | ((value << Self::MAC_ADDRESS0_HI_OFFSET) & Self::MAC_ADDRESS0_HI_MASK)
        });
    }
}

/// EMACADDR0HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr0high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr0high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr0highVal {
    bits: u32,
}

impl Emacaddr0highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x8000ffff }
    }

    /// Read **ADDRESS_ENABLE0** `[31:31]`
    ///
    /// ADDRESS_ENABLE0
    #[inline(always)]
    pub const fn address_enable0(&self) -> bool {
        (self.bits & Emacaddr0high::ADDRESS_ENABLE0_MASK) != 0
    }

    /// Read **MAC_ADDRESS0_HI** `[15:0]`
    ///
    /// MAC_ADDRESS0_HI
    #[inline(always)]
    pub const fn mac_address0_hi(&self) -> u32 {
        (self.bits & Emacaddr0high::MAC_ADDRESS0_HI_MASK) >> Emacaddr0high::MAC_ADDRESS0_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE0** `[31:31]`
    ///
    /// ADDRESS_ENABLE0
    #[inline(always)]
    pub const fn with_address_enable0(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr0high::ADDRESS_ENABLE0_MASK
            } else {
                self.bits & !Emacaddr0high::ADDRESS_ENABLE0_MASK
            },
        }
    }

    /// Set **MAC_ADDRESS0_HI** `[15:0]`
    ///
    /// MAC_ADDRESS0_HI
    #[inline(always)]
    pub const fn with_mac_address0_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr0high::MAC_ADDRESS0_HI_MASK)
                | ((value << Emacaddr0high::MAC_ADDRESS0_HI_OFFSET)
                    & Emacaddr0high::MAC_ADDRESS0_HI_MASK),
        }
    }
}

impl Default for Emacaddr0highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr0high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr0highVal {
        Emacaddr0highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr0highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr0highVal) -> Emacaddr0highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR0LOW Register
///
/// EMACADDR0LOW register
///
/// Address offset: 0x0044
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr0low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr0low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr0low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR0LOW value type for batch read/modify/write operations
///
/// Use `emacaddr0low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr0low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr0lowVal {
    bits: u32,
}

impl Emacaddr0lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr0low::ADDRESS_LOW_MASK) >> Emacaddr0low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr0low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr0low::ADDRESS_LOW_OFFSET) & Emacaddr0low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr0lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr0low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr0lowVal {
        Emacaddr0lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr0lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr0lowVal) -> Emacaddr0lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR1HIGH Register
///
/// EMACADDR1HIGH register
///
/// Address offset: 0x0048
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE1** `[31]` (RW)
///
///   ADDRESS_ENABLE1
///
/// - **SOURCE_ADDRESS** `[30]` (RW)
///
///   SOURCE_ADDRESS
///
/// - **MASK_BYTE_CONTROL** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL
///
/// - **MAC_ADDRESS1_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS1_HI
#[repr(transparent)]
pub struct Emacaddr1high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr1high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr1high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE1**
    ///
    /// ADDRESS_ENABLE1
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE1_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE1_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE1: u32 = 1 << 31;

    /// **SOURCE_ADDRESS**
    ///
    /// SOURCE_ADDRESS
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL**
    ///
    /// MASK_BYTE_CONTROL
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS1_HI**
    ///
    /// MAC_ADDRESS1_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS1_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS1_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE1** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE1
    #[inline(always)]
    pub fn address_enable1(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE1_MASK) != 0
    }

    /// Set ADDRESS_ENABLE1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable1(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE1_MASK);
    }

    /// Clear ADDRESS_ENABLE1 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable1(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE1_MASK);
    }

    /// **SOURCE_ADDRESS** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS
    #[inline(always)]
    pub fn source_address(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS_MASK) != 0
    }

    /// Set SOURCE_ADDRESS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS_MASK);
    }

    /// Clear SOURCE_ADDRESS bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS_MASK);
    }

    /// **MASK_BYTE_CONTROL** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL
    #[inline(always)]
    pub fn mask_byte_control(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL_MASK) >> Self::MASK_BYTE_CONTROL_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL
    #[inline(always)]
    pub fn write_mask_byte_control(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL_MASK)
                | ((value << Self::MASK_BYTE_CONTROL_OFFSET) & Self::MASK_BYTE_CONTROL_MASK)
        });
    }

    /// **MAC_ADDRESS1_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS1_HI
    #[inline(always)]
    pub fn mac_address1_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS1_HI_MASK) >> Self::MAC_ADDRESS1_HI_OFFSET
    }

    /// Write **MAC_ADDRESS1_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS1_HI
    #[inline(always)]
    pub fn write_mac_address1_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS1_HI_MASK)
                | ((value << Self::MAC_ADDRESS1_HI_OFFSET) & Self::MAC_ADDRESS1_HI_MASK)
        });
    }
}

/// EMACADDR1HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr1high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr1high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr1highVal {
    bits: u32,
}

impl Emacaddr1highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE1** `[31:31]`
    ///
    /// ADDRESS_ENABLE1
    #[inline(always)]
    pub const fn address_enable1(&self) -> bool {
        (self.bits & Emacaddr1high::ADDRESS_ENABLE1_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS** `[30:30]`
    ///
    /// SOURCE_ADDRESS
    #[inline(always)]
    pub const fn source_address(&self) -> bool {
        (self.bits & Emacaddr1high::SOURCE_ADDRESS_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL
    #[inline(always)]
    pub const fn mask_byte_control(&self) -> u32 {
        (self.bits & Emacaddr1high::MASK_BYTE_CONTROL_MASK)
            >> Emacaddr1high::MASK_BYTE_CONTROL_OFFSET
    }

    /// Read **MAC_ADDRESS1_HI** `[15:0]`
    ///
    /// MAC_ADDRESS1_HI
    #[inline(always)]
    pub const fn mac_address1_hi(&self) -> u32 {
        (self.bits & Emacaddr1high::MAC_ADDRESS1_HI_MASK) >> Emacaddr1high::MAC_ADDRESS1_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE1** `[31:31]`
    ///
    /// ADDRESS_ENABLE1
    #[inline(always)]
    pub const fn with_address_enable1(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr1high::ADDRESS_ENABLE1_MASK
            } else {
                self.bits & !Emacaddr1high::ADDRESS_ENABLE1_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS** `[30:30]`
    ///
    /// SOURCE_ADDRESS
    #[inline(always)]
    pub const fn with_source_address(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr1high::SOURCE_ADDRESS_MASK
            } else {
                self.bits & !Emacaddr1high::SOURCE_ADDRESS_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL
    #[inline(always)]
    pub const fn with_mask_byte_control(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr1high::MASK_BYTE_CONTROL_MASK)
                | ((value << Emacaddr1high::MASK_BYTE_CONTROL_OFFSET)
                    & Emacaddr1high::MASK_BYTE_CONTROL_MASK),
        }
    }

    /// Set **MAC_ADDRESS1_HI** `[15:0]`
    ///
    /// MAC_ADDRESS1_HI
    #[inline(always)]
    pub const fn with_mac_address1_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr1high::MAC_ADDRESS1_HI_MASK)
                | ((value << Emacaddr1high::MAC_ADDRESS1_HI_OFFSET)
                    & Emacaddr1high::MAC_ADDRESS1_HI_MASK),
        }
    }
}

impl Default for Emacaddr1highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr1high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr1highVal {
        Emacaddr1highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr1highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr1highVal) -> Emacaddr1highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR1LOW Register
///
/// EMACADDR1LOW register
///
/// Address offset: 0x004c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr1low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr1low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr1low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR1LOW value type for batch read/modify/write operations
///
/// Use `emacaddr1low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr1low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr1lowVal {
    bits: u32,
}

impl Emacaddr1lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr1low::ADDRESS_LOW_MASK) >> Emacaddr1low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr1low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr1low::ADDRESS_LOW_OFFSET) & Emacaddr1low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr1lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr1low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr1lowVal {
        Emacaddr1lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr1lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr1lowVal) -> Emacaddr1lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR2HIGH Register
///
/// EMACADDR2HIGH register
///
/// Address offset: 0x0050
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE2** `[31]` (RW)
///
///   ADDRESS_ENABLE2
///
/// - **SOURCE_ADDRESS2** `[30]` (RW)
///
///   SOURCE_ADDRESS2
///
/// - **MASK_BYTE_CONTROL2** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL2
///
/// - **MAC_ADDRESS2_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS2_HI
#[repr(transparent)]
pub struct Emacaddr2high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr2high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr2high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE2**
    ///
    /// ADDRESS_ENABLE2
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE2_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE2_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE2: u32 = 1 << 31;

    /// **SOURCE_ADDRESS2**
    ///
    /// SOURCE_ADDRESS2
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS2_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS2_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS2: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL2**
    ///
    /// MASK_BYTE_CONTROL2
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL2_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL2_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS2_HI**
    ///
    /// MAC_ADDRESS2_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS2_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS2_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE2** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE2
    #[inline(always)]
    pub fn address_enable2(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE2_MASK) != 0
    }

    /// Set ADDRESS_ENABLE2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable2(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE2_MASK);
    }

    /// Clear ADDRESS_ENABLE2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable2(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE2_MASK);
    }

    /// **SOURCE_ADDRESS2** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS2
    #[inline(always)]
    pub fn source_address2(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS2_MASK) != 0
    }

    /// Set SOURCE_ADDRESS2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address2(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS2_MASK);
    }

    /// Clear SOURCE_ADDRESS2 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address2(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS2_MASK);
    }

    /// **MASK_BYTE_CONTROL2** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL2
    #[inline(always)]
    pub fn mask_byte_control2(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL2_MASK) >> Self::MASK_BYTE_CONTROL2_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL2** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL2
    #[inline(always)]
    pub fn write_mask_byte_control2(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL2_MASK)
                | ((value << Self::MASK_BYTE_CONTROL2_OFFSET) & Self::MASK_BYTE_CONTROL2_MASK)
        });
    }

    /// **MAC_ADDRESS2_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS2_HI
    #[inline(always)]
    pub fn mac_address2_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS2_HI_MASK) >> Self::MAC_ADDRESS2_HI_OFFSET
    }

    /// Write **MAC_ADDRESS2_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS2_HI
    #[inline(always)]
    pub fn write_mac_address2_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS2_HI_MASK)
                | ((value << Self::MAC_ADDRESS2_HI_OFFSET) & Self::MAC_ADDRESS2_HI_MASK)
        });
    }
}

/// EMACADDR2HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr2high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr2high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr2highVal {
    bits: u32,
}

impl Emacaddr2highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE2** `[31:31]`
    ///
    /// ADDRESS_ENABLE2
    #[inline(always)]
    pub const fn address_enable2(&self) -> bool {
        (self.bits & Emacaddr2high::ADDRESS_ENABLE2_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS2** `[30:30]`
    ///
    /// SOURCE_ADDRESS2
    #[inline(always)]
    pub const fn source_address2(&self) -> bool {
        (self.bits & Emacaddr2high::SOURCE_ADDRESS2_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL2** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL2
    #[inline(always)]
    pub const fn mask_byte_control2(&self) -> u32 {
        (self.bits & Emacaddr2high::MASK_BYTE_CONTROL2_MASK)
            >> Emacaddr2high::MASK_BYTE_CONTROL2_OFFSET
    }

    /// Read **MAC_ADDRESS2_HI** `[15:0]`
    ///
    /// MAC_ADDRESS2_HI
    #[inline(always)]
    pub const fn mac_address2_hi(&self) -> u32 {
        (self.bits & Emacaddr2high::MAC_ADDRESS2_HI_MASK) >> Emacaddr2high::MAC_ADDRESS2_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE2** `[31:31]`
    ///
    /// ADDRESS_ENABLE2
    #[inline(always)]
    pub const fn with_address_enable2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr2high::ADDRESS_ENABLE2_MASK
            } else {
                self.bits & !Emacaddr2high::ADDRESS_ENABLE2_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS2** `[30:30]`
    ///
    /// SOURCE_ADDRESS2
    #[inline(always)]
    pub const fn with_source_address2(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr2high::SOURCE_ADDRESS2_MASK
            } else {
                self.bits & !Emacaddr2high::SOURCE_ADDRESS2_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL2** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL2
    #[inline(always)]
    pub const fn with_mask_byte_control2(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr2high::MASK_BYTE_CONTROL2_MASK)
                | ((value << Emacaddr2high::MASK_BYTE_CONTROL2_OFFSET)
                    & Emacaddr2high::MASK_BYTE_CONTROL2_MASK),
        }
    }

    /// Set **MAC_ADDRESS2_HI** `[15:0]`
    ///
    /// MAC_ADDRESS2_HI
    #[inline(always)]
    pub const fn with_mac_address2_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr2high::MAC_ADDRESS2_HI_MASK)
                | ((value << Emacaddr2high::MAC_ADDRESS2_HI_OFFSET)
                    & Emacaddr2high::MAC_ADDRESS2_HI_MASK),
        }
    }
}

impl Default for Emacaddr2highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr2high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr2highVal {
        Emacaddr2highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr2highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr2highVal) -> Emacaddr2highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR2LOW Register
///
/// EMACADDR2LOW register
///
/// Address offset: 0x0054
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr2low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr2low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr2low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR2LOW value type for batch read/modify/write operations
///
/// Use `emacaddr2low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr2low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr2lowVal {
    bits: u32,
}

impl Emacaddr2lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr2low::ADDRESS_LOW_MASK) >> Emacaddr2low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr2low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr2low::ADDRESS_LOW_OFFSET) & Emacaddr2low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr2lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr2low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr2lowVal {
        Emacaddr2lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr2lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr2lowVal) -> Emacaddr2lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR3HIGH Register
///
/// EMACADDR3HIGH register
///
/// Address offset: 0x0058
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE3** `[31]` (RW)
///
///   ADDRESS_ENABLE3
///
/// - **SOURCE_ADDRESS3** `[30]` (RW)
///
///   SOURCE_ADDRESS3
///
/// - **MASK_BYTE_CONTROL3** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL3
///
/// - **MAC_ADDRESS3_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS3_HI
#[repr(transparent)]
pub struct Emacaddr3high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr3high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr3high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE3**
    ///
    /// ADDRESS_ENABLE3
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE3_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE3_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE3: u32 = 1 << 31;

    /// **SOURCE_ADDRESS3**
    ///
    /// SOURCE_ADDRESS3
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS3_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS3_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS3: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL3**
    ///
    /// MASK_BYTE_CONTROL3
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL3_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL3_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS3_HI**
    ///
    /// MAC_ADDRESS3_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS3_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS3_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE3** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE3
    #[inline(always)]
    pub fn address_enable3(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE3_MASK) != 0
    }

    /// Set ADDRESS_ENABLE3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable3(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE3_MASK);
    }

    /// Clear ADDRESS_ENABLE3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable3(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE3_MASK);
    }

    /// **SOURCE_ADDRESS3** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS3
    #[inline(always)]
    pub fn source_address3(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS3_MASK) != 0
    }

    /// Set SOURCE_ADDRESS3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address3(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS3_MASK);
    }

    /// Clear SOURCE_ADDRESS3 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address3(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS3_MASK);
    }

    /// **MASK_BYTE_CONTROL3** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL3
    #[inline(always)]
    pub fn mask_byte_control3(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL3_MASK) >> Self::MASK_BYTE_CONTROL3_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL3** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL3
    #[inline(always)]
    pub fn write_mask_byte_control3(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL3_MASK)
                | ((value << Self::MASK_BYTE_CONTROL3_OFFSET) & Self::MASK_BYTE_CONTROL3_MASK)
        });
    }

    /// **MAC_ADDRESS3_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS3_HI
    #[inline(always)]
    pub fn mac_address3_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS3_HI_MASK) >> Self::MAC_ADDRESS3_HI_OFFSET
    }

    /// Write **MAC_ADDRESS3_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS3_HI
    #[inline(always)]
    pub fn write_mac_address3_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS3_HI_MASK)
                | ((value << Self::MAC_ADDRESS3_HI_OFFSET) & Self::MAC_ADDRESS3_HI_MASK)
        });
    }
}

/// EMACADDR3HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr3high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr3high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr3highVal {
    bits: u32,
}

impl Emacaddr3highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE3** `[31:31]`
    ///
    /// ADDRESS_ENABLE3
    #[inline(always)]
    pub const fn address_enable3(&self) -> bool {
        (self.bits & Emacaddr3high::ADDRESS_ENABLE3_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS3** `[30:30]`
    ///
    /// SOURCE_ADDRESS3
    #[inline(always)]
    pub const fn source_address3(&self) -> bool {
        (self.bits & Emacaddr3high::SOURCE_ADDRESS3_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL3** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL3
    #[inline(always)]
    pub const fn mask_byte_control3(&self) -> u32 {
        (self.bits & Emacaddr3high::MASK_BYTE_CONTROL3_MASK)
            >> Emacaddr3high::MASK_BYTE_CONTROL3_OFFSET
    }

    /// Read **MAC_ADDRESS3_HI** `[15:0]`
    ///
    /// MAC_ADDRESS3_HI
    #[inline(always)]
    pub const fn mac_address3_hi(&self) -> u32 {
        (self.bits & Emacaddr3high::MAC_ADDRESS3_HI_MASK) >> Emacaddr3high::MAC_ADDRESS3_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE3** `[31:31]`
    ///
    /// ADDRESS_ENABLE3
    #[inline(always)]
    pub const fn with_address_enable3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr3high::ADDRESS_ENABLE3_MASK
            } else {
                self.bits & !Emacaddr3high::ADDRESS_ENABLE3_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS3** `[30:30]`
    ///
    /// SOURCE_ADDRESS3
    #[inline(always)]
    pub const fn with_source_address3(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr3high::SOURCE_ADDRESS3_MASK
            } else {
                self.bits & !Emacaddr3high::SOURCE_ADDRESS3_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL3** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL3
    #[inline(always)]
    pub const fn with_mask_byte_control3(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr3high::MASK_BYTE_CONTROL3_MASK)
                | ((value << Emacaddr3high::MASK_BYTE_CONTROL3_OFFSET)
                    & Emacaddr3high::MASK_BYTE_CONTROL3_MASK),
        }
    }

    /// Set **MAC_ADDRESS3_HI** `[15:0]`
    ///
    /// MAC_ADDRESS3_HI
    #[inline(always)]
    pub const fn with_mac_address3_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr3high::MAC_ADDRESS3_HI_MASK)
                | ((value << Emacaddr3high::MAC_ADDRESS3_HI_OFFSET)
                    & Emacaddr3high::MAC_ADDRESS3_HI_MASK),
        }
    }
}

impl Default for Emacaddr3highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr3high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr3highVal {
        Emacaddr3highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr3highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr3highVal) -> Emacaddr3highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR3LOW Register
///
/// EMACADDR3LOW register
///
/// Address offset: 0x005c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr3low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr3low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr3low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR3LOW value type for batch read/modify/write operations
///
/// Use `emacaddr3low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr3low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr3lowVal {
    bits: u32,
}

impl Emacaddr3lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr3low::ADDRESS_LOW_MASK) >> Emacaddr3low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr3low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr3low::ADDRESS_LOW_OFFSET) & Emacaddr3low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr3lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr3low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr3lowVal {
        Emacaddr3lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr3lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr3lowVal) -> Emacaddr3lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR4HIGH Register
///
/// EMACADDR4HIGH register
///
/// Address offset: 0x0060
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE4** `[31]` (RW)
///
///   ADDRESS_ENABLE4
///
/// - **SOURCE_ADDRESS4** `[30]` (RW)
///
///   SOURCE_ADDRESS4
///
/// - **MASK_BYTE_CONTROL4** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL4
///
/// - **MAC_ADDRESS4_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS4_HI
#[repr(transparent)]
pub struct Emacaddr4high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr4high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr4high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE4**
    ///
    /// ADDRESS_ENABLE4
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE4_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE4_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE4: u32 = 1 << 31;

    /// **SOURCE_ADDRESS4**
    ///
    /// SOURCE_ADDRESS4
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS4_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS4_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS4: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL4**
    ///
    /// MASK_BYTE_CONTROL4
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL4_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL4_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS4_HI**
    ///
    /// MAC_ADDRESS4_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS4_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS4_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE4** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE4
    #[inline(always)]
    pub fn address_enable4(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE4_MASK) != 0
    }

    /// Set ADDRESS_ENABLE4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable4(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE4_MASK);
    }

    /// Clear ADDRESS_ENABLE4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable4(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE4_MASK);
    }

    /// **SOURCE_ADDRESS4** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS4
    #[inline(always)]
    pub fn source_address4(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS4_MASK) != 0
    }

    /// Set SOURCE_ADDRESS4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address4(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS4_MASK);
    }

    /// Clear SOURCE_ADDRESS4 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address4(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS4_MASK);
    }

    /// **MASK_BYTE_CONTROL4** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL4
    #[inline(always)]
    pub fn mask_byte_control4(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL4_MASK) >> Self::MASK_BYTE_CONTROL4_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL4** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL4
    #[inline(always)]
    pub fn write_mask_byte_control4(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL4_MASK)
                | ((value << Self::MASK_BYTE_CONTROL4_OFFSET) & Self::MASK_BYTE_CONTROL4_MASK)
        });
    }

    /// **MAC_ADDRESS4_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS4_HI
    #[inline(always)]
    pub fn mac_address4_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS4_HI_MASK) >> Self::MAC_ADDRESS4_HI_OFFSET
    }

    /// Write **MAC_ADDRESS4_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS4_HI
    #[inline(always)]
    pub fn write_mac_address4_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS4_HI_MASK)
                | ((value << Self::MAC_ADDRESS4_HI_OFFSET) & Self::MAC_ADDRESS4_HI_MASK)
        });
    }
}

/// EMACADDR4HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr4high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr4high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr4highVal {
    bits: u32,
}

impl Emacaddr4highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE4** `[31:31]`
    ///
    /// ADDRESS_ENABLE4
    #[inline(always)]
    pub const fn address_enable4(&self) -> bool {
        (self.bits & Emacaddr4high::ADDRESS_ENABLE4_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS4** `[30:30]`
    ///
    /// SOURCE_ADDRESS4
    #[inline(always)]
    pub const fn source_address4(&self) -> bool {
        (self.bits & Emacaddr4high::SOURCE_ADDRESS4_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL4** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL4
    #[inline(always)]
    pub const fn mask_byte_control4(&self) -> u32 {
        (self.bits & Emacaddr4high::MASK_BYTE_CONTROL4_MASK)
            >> Emacaddr4high::MASK_BYTE_CONTROL4_OFFSET
    }

    /// Read **MAC_ADDRESS4_HI** `[15:0]`
    ///
    /// MAC_ADDRESS4_HI
    #[inline(always)]
    pub const fn mac_address4_hi(&self) -> u32 {
        (self.bits & Emacaddr4high::MAC_ADDRESS4_HI_MASK) >> Emacaddr4high::MAC_ADDRESS4_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE4** `[31:31]`
    ///
    /// ADDRESS_ENABLE4
    #[inline(always)]
    pub const fn with_address_enable4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr4high::ADDRESS_ENABLE4_MASK
            } else {
                self.bits & !Emacaddr4high::ADDRESS_ENABLE4_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS4** `[30:30]`
    ///
    /// SOURCE_ADDRESS4
    #[inline(always)]
    pub const fn with_source_address4(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr4high::SOURCE_ADDRESS4_MASK
            } else {
                self.bits & !Emacaddr4high::SOURCE_ADDRESS4_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL4** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL4
    #[inline(always)]
    pub const fn with_mask_byte_control4(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr4high::MASK_BYTE_CONTROL4_MASK)
                | ((value << Emacaddr4high::MASK_BYTE_CONTROL4_OFFSET)
                    & Emacaddr4high::MASK_BYTE_CONTROL4_MASK),
        }
    }

    /// Set **MAC_ADDRESS4_HI** `[15:0]`
    ///
    /// MAC_ADDRESS4_HI
    #[inline(always)]
    pub const fn with_mac_address4_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr4high::MAC_ADDRESS4_HI_MASK)
                | ((value << Emacaddr4high::MAC_ADDRESS4_HI_OFFSET)
                    & Emacaddr4high::MAC_ADDRESS4_HI_MASK),
        }
    }
}

impl Default for Emacaddr4highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr4high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr4highVal {
        Emacaddr4highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr4highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr4highVal) -> Emacaddr4highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR4LOW Register
///
/// EMACADDR4LOW register
///
/// Address offset: 0x0064
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr4low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr4low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr4low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR4LOW value type for batch read/modify/write operations
///
/// Use `emacaddr4low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr4low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr4lowVal {
    bits: u32,
}

impl Emacaddr4lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr4low::ADDRESS_LOW_MASK) >> Emacaddr4low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr4low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr4low::ADDRESS_LOW_OFFSET) & Emacaddr4low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr4lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr4low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr4lowVal {
        Emacaddr4lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr4lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr4lowVal) -> Emacaddr4lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR5HIGH Register
///
/// EMACADDR5HIGH register
///
/// Address offset: 0x0068
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE5** `[31]` (RW)
///
///   ADDRESS_ENABLE5
///
/// - **SOURCE_ADDRESS5** `[30]` (RW)
///
///   SOURCE_ADDRESS5
///
/// - **MASK_BYTE_CONTROL5** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL5
///
/// - **MAC_ADDRESS5_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS5_HI
#[repr(transparent)]
pub struct Emacaddr5high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr5high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr5high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE5**
    ///
    /// ADDRESS_ENABLE5
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE5_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE5_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE5: u32 = 1 << 31;

    /// **SOURCE_ADDRESS5**
    ///
    /// SOURCE_ADDRESS5
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS5_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS5_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS5: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL5**
    ///
    /// MASK_BYTE_CONTROL5
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL5_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL5_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS5_HI**
    ///
    /// MAC_ADDRESS5_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS5_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS5_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE5** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE5
    #[inline(always)]
    pub fn address_enable5(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE5_MASK) != 0
    }

    /// Set ADDRESS_ENABLE5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable5(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE5_MASK);
    }

    /// Clear ADDRESS_ENABLE5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable5(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE5_MASK);
    }

    /// **SOURCE_ADDRESS5** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS5
    #[inline(always)]
    pub fn source_address5(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS5_MASK) != 0
    }

    /// Set SOURCE_ADDRESS5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address5(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS5_MASK);
    }

    /// Clear SOURCE_ADDRESS5 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address5(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS5_MASK);
    }

    /// **MASK_BYTE_CONTROL5** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL5
    #[inline(always)]
    pub fn mask_byte_control5(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL5_MASK) >> Self::MASK_BYTE_CONTROL5_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL5** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL5
    #[inline(always)]
    pub fn write_mask_byte_control5(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL5_MASK)
                | ((value << Self::MASK_BYTE_CONTROL5_OFFSET) & Self::MASK_BYTE_CONTROL5_MASK)
        });
    }

    /// **MAC_ADDRESS5_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS5_HI
    #[inline(always)]
    pub fn mac_address5_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS5_HI_MASK) >> Self::MAC_ADDRESS5_HI_OFFSET
    }

    /// Write **MAC_ADDRESS5_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS5_HI
    #[inline(always)]
    pub fn write_mac_address5_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS5_HI_MASK)
                | ((value << Self::MAC_ADDRESS5_HI_OFFSET) & Self::MAC_ADDRESS5_HI_MASK)
        });
    }
}

/// EMACADDR5HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr5high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr5high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr5highVal {
    bits: u32,
}

impl Emacaddr5highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE5** `[31:31]`
    ///
    /// ADDRESS_ENABLE5
    #[inline(always)]
    pub const fn address_enable5(&self) -> bool {
        (self.bits & Emacaddr5high::ADDRESS_ENABLE5_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS5** `[30:30]`
    ///
    /// SOURCE_ADDRESS5
    #[inline(always)]
    pub const fn source_address5(&self) -> bool {
        (self.bits & Emacaddr5high::SOURCE_ADDRESS5_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL5** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL5
    #[inline(always)]
    pub const fn mask_byte_control5(&self) -> u32 {
        (self.bits & Emacaddr5high::MASK_BYTE_CONTROL5_MASK)
            >> Emacaddr5high::MASK_BYTE_CONTROL5_OFFSET
    }

    /// Read **MAC_ADDRESS5_HI** `[15:0]`
    ///
    /// MAC_ADDRESS5_HI
    #[inline(always)]
    pub const fn mac_address5_hi(&self) -> u32 {
        (self.bits & Emacaddr5high::MAC_ADDRESS5_HI_MASK) >> Emacaddr5high::MAC_ADDRESS5_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE5** `[31:31]`
    ///
    /// ADDRESS_ENABLE5
    #[inline(always)]
    pub const fn with_address_enable5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr5high::ADDRESS_ENABLE5_MASK
            } else {
                self.bits & !Emacaddr5high::ADDRESS_ENABLE5_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS5** `[30:30]`
    ///
    /// SOURCE_ADDRESS5
    #[inline(always)]
    pub const fn with_source_address5(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr5high::SOURCE_ADDRESS5_MASK
            } else {
                self.bits & !Emacaddr5high::SOURCE_ADDRESS5_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL5** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL5
    #[inline(always)]
    pub const fn with_mask_byte_control5(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr5high::MASK_BYTE_CONTROL5_MASK)
                | ((value << Emacaddr5high::MASK_BYTE_CONTROL5_OFFSET)
                    & Emacaddr5high::MASK_BYTE_CONTROL5_MASK),
        }
    }

    /// Set **MAC_ADDRESS5_HI** `[15:0]`
    ///
    /// MAC_ADDRESS5_HI
    #[inline(always)]
    pub const fn with_mac_address5_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr5high::MAC_ADDRESS5_HI_MASK)
                | ((value << Emacaddr5high::MAC_ADDRESS5_HI_OFFSET)
                    & Emacaddr5high::MAC_ADDRESS5_HI_MASK),
        }
    }
}

impl Default for Emacaddr5highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr5high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr5highVal {
        Emacaddr5highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr5highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr5highVal) -> Emacaddr5highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR5LOW Register
///
/// EMACADDR5LOW register
///
/// Address offset: 0x006c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr5low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr5low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr5low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR5LOW value type for batch read/modify/write operations
///
/// Use `emacaddr5low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr5low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr5lowVal {
    bits: u32,
}

impl Emacaddr5lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr5low::ADDRESS_LOW_MASK) >> Emacaddr5low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr5low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr5low::ADDRESS_LOW_OFFSET) & Emacaddr5low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr5lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr5low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr5lowVal {
        Emacaddr5lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr5lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr5lowVal) -> Emacaddr5lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR6HIGH Register
///
/// EMACADDR6HIGH register
///
/// Address offset: 0x0070
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE6** `[31]` (RW)
///
///   ADDRESS_ENABLE6
///
/// - **SOURCE_ADDRESS6** `[30]` (RW)
///
///   SOURCE_ADDRESS6
///
/// - **MASK_BYTE_CONTROL6** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL6
///
/// - **MAC_ADDRESS6_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS6_HI
#[repr(transparent)]
pub struct Emacaddr6high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr6high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr6high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE6**
    ///
    /// ADDRESS_ENABLE6
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE6_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE6_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE6: u32 = 1 << 31;

    /// **SOURCE_ADDRESS6**
    ///
    /// SOURCE_ADDRESS6
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS6_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS6_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS6: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL6**
    ///
    /// MASK_BYTE_CONTROL6
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL6_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL6_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS6_HI**
    ///
    /// MAC_ADDRESS6_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS6_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS6_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE6** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE6
    #[inline(always)]
    pub fn address_enable6(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE6_MASK) != 0
    }

    /// Set ADDRESS_ENABLE6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable6(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE6_MASK);
    }

    /// Clear ADDRESS_ENABLE6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable6(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE6_MASK);
    }

    /// **SOURCE_ADDRESS6** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS6
    #[inline(always)]
    pub fn source_address6(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS6_MASK) != 0
    }

    /// Set SOURCE_ADDRESS6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address6(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS6_MASK);
    }

    /// Clear SOURCE_ADDRESS6 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address6(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS6_MASK);
    }

    /// **MASK_BYTE_CONTROL6** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL6
    #[inline(always)]
    pub fn mask_byte_control6(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL6_MASK) >> Self::MASK_BYTE_CONTROL6_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL6** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL6
    #[inline(always)]
    pub fn write_mask_byte_control6(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL6_MASK)
                | ((value << Self::MASK_BYTE_CONTROL6_OFFSET) & Self::MASK_BYTE_CONTROL6_MASK)
        });
    }

    /// **MAC_ADDRESS6_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS6_HI
    #[inline(always)]
    pub fn mac_address6_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS6_HI_MASK) >> Self::MAC_ADDRESS6_HI_OFFSET
    }

    /// Write **MAC_ADDRESS6_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS6_HI
    #[inline(always)]
    pub fn write_mac_address6_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS6_HI_MASK)
                | ((value << Self::MAC_ADDRESS6_HI_OFFSET) & Self::MAC_ADDRESS6_HI_MASK)
        });
    }
}

/// EMACADDR6HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr6high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr6high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr6highVal {
    bits: u32,
}

impl Emacaddr6highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE6** `[31:31]`
    ///
    /// ADDRESS_ENABLE6
    #[inline(always)]
    pub const fn address_enable6(&self) -> bool {
        (self.bits & Emacaddr6high::ADDRESS_ENABLE6_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS6** `[30:30]`
    ///
    /// SOURCE_ADDRESS6
    #[inline(always)]
    pub const fn source_address6(&self) -> bool {
        (self.bits & Emacaddr6high::SOURCE_ADDRESS6_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL6** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL6
    #[inline(always)]
    pub const fn mask_byte_control6(&self) -> u32 {
        (self.bits & Emacaddr6high::MASK_BYTE_CONTROL6_MASK)
            >> Emacaddr6high::MASK_BYTE_CONTROL6_OFFSET
    }

    /// Read **MAC_ADDRESS6_HI** `[15:0]`
    ///
    /// MAC_ADDRESS6_HI
    #[inline(always)]
    pub const fn mac_address6_hi(&self) -> u32 {
        (self.bits & Emacaddr6high::MAC_ADDRESS6_HI_MASK) >> Emacaddr6high::MAC_ADDRESS6_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE6** `[31:31]`
    ///
    /// ADDRESS_ENABLE6
    #[inline(always)]
    pub const fn with_address_enable6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr6high::ADDRESS_ENABLE6_MASK
            } else {
                self.bits & !Emacaddr6high::ADDRESS_ENABLE6_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS6** `[30:30]`
    ///
    /// SOURCE_ADDRESS6
    #[inline(always)]
    pub const fn with_source_address6(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr6high::SOURCE_ADDRESS6_MASK
            } else {
                self.bits & !Emacaddr6high::SOURCE_ADDRESS6_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL6** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL6
    #[inline(always)]
    pub const fn with_mask_byte_control6(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr6high::MASK_BYTE_CONTROL6_MASK)
                | ((value << Emacaddr6high::MASK_BYTE_CONTROL6_OFFSET)
                    & Emacaddr6high::MASK_BYTE_CONTROL6_MASK),
        }
    }

    /// Set **MAC_ADDRESS6_HI** `[15:0]`
    ///
    /// MAC_ADDRESS6_HI
    #[inline(always)]
    pub const fn with_mac_address6_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr6high::MAC_ADDRESS6_HI_MASK)
                | ((value << Emacaddr6high::MAC_ADDRESS6_HI_OFFSET)
                    & Emacaddr6high::MAC_ADDRESS6_HI_MASK),
        }
    }
}

impl Default for Emacaddr6highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr6high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr6highVal {
        Emacaddr6highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr6highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr6highVal) -> Emacaddr6highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR6LOW Register
///
/// EMACADDR6LOW register
///
/// Address offset: 0x0074
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr6low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr6low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr6low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR6LOW value type for batch read/modify/write operations
///
/// Use `emacaddr6low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr6low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr6lowVal {
    bits: u32,
}

impl Emacaddr6lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr6low::ADDRESS_LOW_MASK) >> Emacaddr6low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr6low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr6low::ADDRESS_LOW_OFFSET) & Emacaddr6low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr6lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr6low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr6lowVal {
        Emacaddr6lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr6lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr6lowVal) -> Emacaddr6lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR7HIGH Register
///
/// EMACADDR7HIGH register
///
/// Address offset: 0x0078
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE7** `[31]` (RW)
///
///   ADDRESS_ENABLE7
///
/// - **SOURCE_ADDRESS7** `[30]` (RW)
///
///   SOURCE_ADDRESS7
///
/// - **MASK_BYTE_CONTROL7** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL7
///
/// - **MAC_ADDRESS7_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS7_HI
#[repr(transparent)]
pub struct Emacaddr7high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr7high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr7high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE7**
    ///
    /// ADDRESS_ENABLE7
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE7_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE7_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE7: u32 = 1 << 31;

    /// **SOURCE_ADDRESS7**
    ///
    /// SOURCE_ADDRESS7
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS7_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS7_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS7: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL7**
    ///
    /// MASK_BYTE_CONTROL7
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL7_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL7_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS7_HI**
    ///
    /// MAC_ADDRESS7_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS7_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS7_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE7** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE7
    #[inline(always)]
    pub fn address_enable7(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE7_MASK) != 0
    }

    /// Set ADDRESS_ENABLE7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable7(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE7_MASK);
    }

    /// Clear ADDRESS_ENABLE7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable7(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE7_MASK);
    }

    /// **SOURCE_ADDRESS7** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS7
    #[inline(always)]
    pub fn source_address7(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS7_MASK) != 0
    }

    /// Set SOURCE_ADDRESS7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address7(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS7_MASK);
    }

    /// Clear SOURCE_ADDRESS7 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address7(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS7_MASK);
    }

    /// **MASK_BYTE_CONTROL7** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL7
    #[inline(always)]
    pub fn mask_byte_control7(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL7_MASK) >> Self::MASK_BYTE_CONTROL7_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL7** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL7
    #[inline(always)]
    pub fn write_mask_byte_control7(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL7_MASK)
                | ((value << Self::MASK_BYTE_CONTROL7_OFFSET) & Self::MASK_BYTE_CONTROL7_MASK)
        });
    }

    /// **MAC_ADDRESS7_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS7_HI
    #[inline(always)]
    pub fn mac_address7_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS7_HI_MASK) >> Self::MAC_ADDRESS7_HI_OFFSET
    }

    /// Write **MAC_ADDRESS7_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS7_HI
    #[inline(always)]
    pub fn write_mac_address7_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS7_HI_MASK)
                | ((value << Self::MAC_ADDRESS7_HI_OFFSET) & Self::MAC_ADDRESS7_HI_MASK)
        });
    }
}

/// EMACADDR7HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr7high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr7high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr7highVal {
    bits: u32,
}

impl Emacaddr7highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE7** `[31:31]`
    ///
    /// ADDRESS_ENABLE7
    #[inline(always)]
    pub const fn address_enable7(&self) -> bool {
        (self.bits & Emacaddr7high::ADDRESS_ENABLE7_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS7** `[30:30]`
    ///
    /// SOURCE_ADDRESS7
    #[inline(always)]
    pub const fn source_address7(&self) -> bool {
        (self.bits & Emacaddr7high::SOURCE_ADDRESS7_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL7** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL7
    #[inline(always)]
    pub const fn mask_byte_control7(&self) -> u32 {
        (self.bits & Emacaddr7high::MASK_BYTE_CONTROL7_MASK)
            >> Emacaddr7high::MASK_BYTE_CONTROL7_OFFSET
    }

    /// Read **MAC_ADDRESS7_HI** `[15:0]`
    ///
    /// MAC_ADDRESS7_HI
    #[inline(always)]
    pub const fn mac_address7_hi(&self) -> u32 {
        (self.bits & Emacaddr7high::MAC_ADDRESS7_HI_MASK) >> Emacaddr7high::MAC_ADDRESS7_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE7** `[31:31]`
    ///
    /// ADDRESS_ENABLE7
    #[inline(always)]
    pub const fn with_address_enable7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr7high::ADDRESS_ENABLE7_MASK
            } else {
                self.bits & !Emacaddr7high::ADDRESS_ENABLE7_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS7** `[30:30]`
    ///
    /// SOURCE_ADDRESS7
    #[inline(always)]
    pub const fn with_source_address7(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr7high::SOURCE_ADDRESS7_MASK
            } else {
                self.bits & !Emacaddr7high::SOURCE_ADDRESS7_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL7** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL7
    #[inline(always)]
    pub const fn with_mask_byte_control7(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr7high::MASK_BYTE_CONTROL7_MASK)
                | ((value << Emacaddr7high::MASK_BYTE_CONTROL7_OFFSET)
                    & Emacaddr7high::MASK_BYTE_CONTROL7_MASK),
        }
    }

    /// Set **MAC_ADDRESS7_HI** `[15:0]`
    ///
    /// MAC_ADDRESS7_HI
    #[inline(always)]
    pub const fn with_mac_address7_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr7high::MAC_ADDRESS7_HI_MASK)
                | ((value << Emacaddr7high::MAC_ADDRESS7_HI_OFFSET)
                    & Emacaddr7high::MAC_ADDRESS7_HI_MASK),
        }
    }
}

impl Default for Emacaddr7highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr7high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr7highVal {
        Emacaddr7highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr7highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr7highVal) -> Emacaddr7highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR7LOW Register
///
/// EMACADDR7LOW register
///
/// Address offset: 0x007c
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr7low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr7low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr7low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR7LOW value type for batch read/modify/write operations
///
/// Use `emacaddr7low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr7low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr7lowVal {
    bits: u32,
}

impl Emacaddr7lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr7low::ADDRESS_LOW_MASK) >> Emacaddr7low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr7low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr7low::ADDRESS_LOW_OFFSET) & Emacaddr7low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr7lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr7low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr7lowVal {
        Emacaddr7lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr7lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr7lowVal) -> Emacaddr7lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR8HIGH Register
///
/// EMACADDR8HIGH register
///
/// Address offset: 0x0080
/// Reset value: 0x0000ffff
///
/// ## Fields
///
/// - **ADDRESS_ENABLE8** `[31]` (RW)
///
///   ADDRESS_ENABLE8
///
/// - **SOURCE_ADDRESS8** `[30]` (RW)
///
///   SOURCE_ADDRESS8
///
/// - **MASK_BYTE_CONTROL8** `[29:24]` (RW)
///
///   MASK_BYTE_CONTROL8
///
/// - **MAC_ADDRESS8_HI** `[15:0]` (RW)
///
///   MAC_ADDRESS8_HI
#[repr(transparent)]
pub struct Emacaddr8high {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr8high {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr8high {
    // ======== Field Constants ========

    /// **ADDRESS_ENABLE8**
    ///
    /// ADDRESS_ENABLE8
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDRESS_ENABLE8_OFFSET: u32 = 31;
    pub const ADDRESS_ENABLE8_MASK: u32 = 0x80000000;
    pub const ADDRESS_ENABLE8: u32 = 1 << 31;

    /// **SOURCE_ADDRESS8**
    ///
    /// SOURCE_ADDRESS8
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const SOURCE_ADDRESS8_OFFSET: u32 = 30;
    pub const SOURCE_ADDRESS8_MASK: u32 = 0x40000000;
    pub const SOURCE_ADDRESS8: u32 = 1 << 30;

    /// **MASK_BYTE_CONTROL8**
    ///
    /// MASK_BYTE_CONTROL8
    ///
    /// Bits `[29:24]`, mask `0x3f000000`
    pub const MASK_BYTE_CONTROL8_OFFSET: u32 = 24;
    pub const MASK_BYTE_CONTROL8_MASK: u32 = 0x3f000000;

    /// **MAC_ADDRESS8_HI**
    ///
    /// MAC_ADDRESS8_HI
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MAC_ADDRESS8_HI_OFFSET: u32 = 0;
    pub const MAC_ADDRESS8_HI_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **ADDRESS_ENABLE8** `[31]` (RW)
    ///
    /// ADDRESS_ENABLE8
    #[inline(always)]
    pub fn address_enable8(&self) -> bool {
        (self.value.read_bits() & Self::ADDRESS_ENABLE8_MASK) != 0
    }

    /// Set ADDRESS_ENABLE8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_address_enable8(&self) {
        self.value.modify_bits(|r| r | Self::ADDRESS_ENABLE8_MASK);
    }

    /// Clear ADDRESS_ENABLE8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_address_enable8(&self) {
        self.value.modify_bits(|r| r & !Self::ADDRESS_ENABLE8_MASK);
    }

    /// **SOURCE_ADDRESS8** `[30]` (RW)
    ///
    /// SOURCE_ADDRESS8
    #[inline(always)]
    pub fn source_address8(&self) -> bool {
        (self.value.read_bits() & Self::SOURCE_ADDRESS8_MASK) != 0
    }

    /// Set SOURCE_ADDRESS8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_source_address8(&self) {
        self.value.modify_bits(|r| r | Self::SOURCE_ADDRESS8_MASK);
    }

    /// Clear SOURCE_ADDRESS8 bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_source_address8(&self) {
        self.value.modify_bits(|r| r & !Self::SOURCE_ADDRESS8_MASK);
    }

    /// **MASK_BYTE_CONTROL8** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL8
    #[inline(always)]
    pub fn mask_byte_control8(&self) -> u32 {
        (self.value.read_bits() & Self::MASK_BYTE_CONTROL8_MASK) >> Self::MASK_BYTE_CONTROL8_OFFSET
    }

    /// Write **MASK_BYTE_CONTROL8** `[29:24]` (RW)
    ///
    /// MASK_BYTE_CONTROL8
    #[inline(always)]
    pub fn write_mask_byte_control8(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MASK_BYTE_CONTROL8_MASK)
                | ((value << Self::MASK_BYTE_CONTROL8_OFFSET) & Self::MASK_BYTE_CONTROL8_MASK)
        });
    }

    /// **MAC_ADDRESS8_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS8_HI
    #[inline(always)]
    pub fn mac_address8_hi(&self) -> u32 {
        (self.value.read_bits() & Self::MAC_ADDRESS8_HI_MASK) >> Self::MAC_ADDRESS8_HI_OFFSET
    }

    /// Write **MAC_ADDRESS8_HI** `[15:0]` (RW)
    ///
    /// MAC_ADDRESS8_HI
    #[inline(always)]
    pub fn write_mac_address8_hi(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MAC_ADDRESS8_HI_MASK)
                | ((value << Self::MAC_ADDRESS8_HI_OFFSET) & Self::MAC_ADDRESS8_HI_MASK)
        });
    }
}

/// EMACADDR8HIGH value type for batch read/modify/write operations
///
/// Use `emacaddr8high.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr8high.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr8highVal {
    bits: u32,
}

impl Emacaddr8highVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x0000ffff }
    }

    /// Read **ADDRESS_ENABLE8** `[31:31]`
    ///
    /// ADDRESS_ENABLE8
    #[inline(always)]
    pub const fn address_enable8(&self) -> bool {
        (self.bits & Emacaddr8high::ADDRESS_ENABLE8_MASK) != 0
    }

    /// Read **SOURCE_ADDRESS8** `[30:30]`
    ///
    /// SOURCE_ADDRESS8
    #[inline(always)]
    pub const fn source_address8(&self) -> bool {
        (self.bits & Emacaddr8high::SOURCE_ADDRESS8_MASK) != 0
    }

    /// Read **MASK_BYTE_CONTROL8** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL8
    #[inline(always)]
    pub const fn mask_byte_control8(&self) -> u32 {
        (self.bits & Emacaddr8high::MASK_BYTE_CONTROL8_MASK)
            >> Emacaddr8high::MASK_BYTE_CONTROL8_OFFSET
    }

    /// Read **MAC_ADDRESS8_HI** `[15:0]`
    ///
    /// MAC_ADDRESS8_HI
    #[inline(always)]
    pub const fn mac_address8_hi(&self) -> u32 {
        (self.bits & Emacaddr8high::MAC_ADDRESS8_HI_MASK) >> Emacaddr8high::MAC_ADDRESS8_HI_OFFSET
    }

    /// Set **ADDRESS_ENABLE8** `[31:31]`
    ///
    /// ADDRESS_ENABLE8
    #[inline(always)]
    pub const fn with_address_enable8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr8high::ADDRESS_ENABLE8_MASK
            } else {
                self.bits & !Emacaddr8high::ADDRESS_ENABLE8_MASK
            },
        }
    }

    /// Set **SOURCE_ADDRESS8** `[30:30]`
    ///
    /// SOURCE_ADDRESS8
    #[inline(always)]
    pub const fn with_source_address8(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacaddr8high::SOURCE_ADDRESS8_MASK
            } else {
                self.bits & !Emacaddr8high::SOURCE_ADDRESS8_MASK
            },
        }
    }

    /// Set **MASK_BYTE_CONTROL8** `[29:24]`
    ///
    /// MASK_BYTE_CONTROL8
    #[inline(always)]
    pub const fn with_mask_byte_control8(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr8high::MASK_BYTE_CONTROL8_MASK)
                | ((value << Emacaddr8high::MASK_BYTE_CONTROL8_OFFSET)
                    & Emacaddr8high::MASK_BYTE_CONTROL8_MASK),
        }
    }

    /// Set **MAC_ADDRESS8_HI** `[15:0]`
    ///
    /// MAC_ADDRESS8_HI
    #[inline(always)]
    pub const fn with_mac_address8_hi(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr8high::MAC_ADDRESS8_HI_MASK)
                | ((value << Emacaddr8high::MAC_ADDRESS8_HI_OFFSET)
                    & Emacaddr8high::MAC_ADDRESS8_HI_MASK),
        }
    }
}

impl Default for Emacaddr8highVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr8high {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr8highVal {
        Emacaddr8highVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr8highVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr8highVal) -> Emacaddr8highVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACADDR8LOW Register
///
/// EMACADDR8LOW register
///
/// Address offset: 0x0084
/// Reset value: 0xffffffff
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacaddr8low {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacaddr8low {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacaddr8low {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACADDR8LOW value type for batch read/modify/write operations
///
/// Use `emacaddr8low.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacaddr8low.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacaddr8lowVal {
    bits: u32,
}

impl Emacaddr8lowVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0xffffffff }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacaddr8low::ADDRESS_LOW_MASK) >> Emacaddr8low::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacaddr8low::ADDRESS_LOW_MASK)
                | ((value << Emacaddr8low::ADDRESS_LOW_OFFSET) & Emacaddr8low::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacaddr8lowVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacaddr8low {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacaddr8lowVal {
        Emacaddr8lowVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacaddr8lowVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacaddr8lowVal) -> Emacaddr8lowVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACCSTATUS Register
///
/// EMACCSTATUS register
///
/// Address offset: 0x00d8
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **JABBER_TIMEOUT** `[4]` (RW)
///
///   JABBER_TIMEOUT
///
/// - **LINK_MODE** `[0]` (RW)
///
///   LINK_MODE
#[repr(transparent)]
pub struct Emaccstatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emaccstatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emaccstatus {
    // ======== Field Constants ========

    /// **JABBER_TIMEOUT**
    ///
    /// JABBER_TIMEOUT
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const JABBER_TIMEOUT_OFFSET: u32 = 4;
    pub const JABBER_TIMEOUT_MASK: u32 = 0x00000010;
    pub const JABBER_TIMEOUT: u32 = 1 << 4;

    /// **LINK_MODE**
    ///
    /// LINK_MODE
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const LINK_MODE_OFFSET: u32 = 0;
    pub const LINK_MODE_MASK: u32 = 0x00000001;
    pub const LINK_MODE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **JABBER_TIMEOUT** `[4]` (RW)
    ///
    /// JABBER_TIMEOUT
    #[inline(always)]
    pub fn jabber_timeout(&self) -> bool {
        (self.value.read_bits() & Self::JABBER_TIMEOUT_MASK) != 0
    }

    /// Set JABBER_TIMEOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_jabber_timeout(&self) {
        self.value.modify_bits(|r| r | Self::JABBER_TIMEOUT_MASK);
    }

    /// Clear JABBER_TIMEOUT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_jabber_timeout(&self) {
        self.value.modify_bits(|r| r & !Self::JABBER_TIMEOUT_MASK);
    }

    /// **LINK_MODE** `[0]` (RW)
    ///
    /// LINK_MODE
    #[inline(always)]
    pub fn link_mode(&self) -> bool {
        (self.value.read_bits() & Self::LINK_MODE_MASK) != 0
    }

    /// Set LINK_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_link_mode(&self) {
        self.value.modify_bits(|r| r | Self::LINK_MODE_MASK);
    }

    /// Clear LINK_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_link_mode(&self) {
        self.value.modify_bits(|r| r & !Self::LINK_MODE_MASK);
    }
}

/// EMACCSTATUS value type for batch read/modify/write operations
///
/// Use `emaccstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emaccstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmaccstatusVal {
    bits: u32,
}

impl EmaccstatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **JABBER_TIMEOUT** `[4:4]`
    ///
    /// JABBER_TIMEOUT
    #[inline(always)]
    pub const fn jabber_timeout(&self) -> bool {
        (self.bits & Emaccstatus::JABBER_TIMEOUT_MASK) != 0
    }

    /// Read **LINK_MODE** `[0:0]`
    ///
    /// LINK_MODE
    #[inline(always)]
    pub const fn link_mode(&self) -> bool {
        (self.bits & Emaccstatus::LINK_MODE_MASK) != 0
    }

    /// Set **JABBER_TIMEOUT** `[4:4]`
    ///
    /// JABBER_TIMEOUT
    #[inline(always)]
    pub const fn with_jabber_timeout(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emaccstatus::JABBER_TIMEOUT_MASK
            } else {
                self.bits & !Emaccstatus::JABBER_TIMEOUT_MASK
            },
        }
    }

    /// Set **LINK_MODE** `[0:0]`
    ///
    /// LINK_MODE
    #[inline(always)]
    pub const fn with_link_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emaccstatus::LINK_MODE_MASK
            } else {
                self.bits & !Emaccstatus::LINK_MODE_MASK
            },
        }
    }
}

impl Default for EmaccstatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emaccstatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmaccstatusVal {
        EmaccstatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmaccstatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmaccstatusVal) -> EmaccstatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACWDOGTO Register
///
/// EMACWDOGTO register
///
/// Address offset: 0x00dc
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **PWDOGEN** `[16]` (RW)
///
///   PWDOGEN
///
/// - **WDOGTO** `[13:0]` (RW)
///
///   WDOGTO
#[repr(transparent)]
pub struct Emacwdogto {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacwdogto {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacwdogto {
    // ======== Field Constants ========

    /// **PWDOGEN**
    ///
    /// PWDOGEN
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const PWDOGEN_OFFSET: u32 = 16;
    pub const PWDOGEN_MASK: u32 = 0x00010000;
    pub const PWDOGEN: u32 = 1 << 16;

    /// **WDOGTO**
    ///
    /// WDOGTO
    ///
    /// Bits `[13:0]`, mask `0x00003fff`
    pub const WDOGTO_OFFSET: u32 = 0;
    pub const WDOGTO_MASK: u32 = 0x00003fff;

    // ======== Field Accessors ========

    /// **PWDOGEN** `[16]` (RW)
    ///
    /// PWDOGEN
    #[inline(always)]
    pub fn pwdogen(&self) -> bool {
        (self.value.read_bits() & Self::PWDOGEN_MASK) != 0
    }

    /// Set PWDOGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pwdogen(&self) {
        self.value.modify_bits(|r| r | Self::PWDOGEN_MASK);
    }

    /// Clear PWDOGEN bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pwdogen(&self) {
        self.value.modify_bits(|r| r & !Self::PWDOGEN_MASK);
    }

    /// **WDOGTO** `[13:0]` (RW)
    ///
    /// WDOGTO
    #[inline(always)]
    pub fn wdogto(&self) -> u32 {
        (self.value.read_bits() & Self::WDOGTO_MASK) >> Self::WDOGTO_OFFSET
    }

    /// Write **WDOGTO** `[13:0]` (RW)
    ///
    /// WDOGTO
    #[inline(always)]
    pub fn write_wdogto(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::WDOGTO_MASK) | ((value << Self::WDOGTO_OFFSET) & Self::WDOGTO_MASK)
        });
    }
}

/// EMACWDOGTO value type for batch read/modify/write operations
///
/// Use `emacwdogto.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacwdogto.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacwdogtoVal {
    bits: u32,
}

impl EmacwdogtoVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **PWDOGEN** `[16:16]`
    ///
    /// PWDOGEN
    #[inline(always)]
    pub const fn pwdogen(&self) -> bool {
        (self.bits & Emacwdogto::PWDOGEN_MASK) != 0
    }

    /// Read **WDOGTO** `[13:0]`
    ///
    /// WDOGTO
    #[inline(always)]
    pub const fn wdogto(&self) -> u32 {
        (self.bits & Emacwdogto::WDOGTO_MASK) >> Emacwdogto::WDOGTO_OFFSET
    }

    /// Set **PWDOGEN** `[16:16]`
    ///
    /// PWDOGEN
    #[inline(always)]
    pub const fn with_pwdogen(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacwdogto::PWDOGEN_MASK
            } else {
                self.bits & !Emacwdogto::PWDOGEN_MASK
            },
        }
    }

    /// Set **WDOGTO** `[13:0]`
    ///
    /// WDOGTO
    #[inline(always)]
    pub const fn with_wdogto(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacwdogto::WDOGTO_MASK)
                | ((value << Emacwdogto::WDOGTO_OFFSET) & Emacwdogto::WDOGTO_MASK),
        }
    }
}

impl Default for EmacwdogtoVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacwdogto {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacwdogtoVal {
        EmacwdogtoVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacwdogtoVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacwdogtoVal) -> EmacwdogtoVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACTXVLTCTRL Register
///
/// EMACTXVLTCTRL register
///
/// Address offset: 0x0584
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **CSVL** `[19]` (RW)
///
///   CSVL
///
/// - **VLP** `[18]` (RW)
///
///   VLP
///
/// - **VLC** `[17:16]` (RW)
///
///   VLC
///
/// - **VLT** `[15:0]` (RW)
///
///   VLT
#[repr(transparent)]
pub struct Emactxvltctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emactxvltctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emactxvltctrl {
    // ======== Field Constants ========

    /// **CSVL**
    ///
    /// CSVL
    ///
    /// Bits `[19:19]`, mask `0x00080000`
    pub const CSVL_OFFSET: u32 = 19;
    pub const CSVL_MASK: u32 = 0x00080000;
    pub const CSVL: u32 = 1 << 19;

    /// **VLP**
    ///
    /// VLP
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const VLP_OFFSET: u32 = 18;
    pub const VLP_MASK: u32 = 0x00040000;
    pub const VLP: u32 = 1 << 18;

    /// **VLC**
    ///
    /// VLC
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const VLC_OFFSET: u32 = 16;
    pub const VLC_MASK: u32 = 0x00030000;

    /// **VLT**
    ///
    /// VLT
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const VLT_OFFSET: u32 = 0;
    pub const VLT_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **CSVL** `[19]` (RW)
    ///
    /// CSVL
    #[inline(always)]
    pub fn csvl(&self) -> bool {
        (self.value.read_bits() & Self::CSVL_MASK) != 0
    }

    /// Set CSVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_csvl(&self) {
        self.value.modify_bits(|r| r | Self::CSVL_MASK);
    }

    /// Clear CSVL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_csvl(&self) {
        self.value.modify_bits(|r| r & !Self::CSVL_MASK);
    }

    /// **VLP** `[18]` (RW)
    ///
    /// VLP
    #[inline(always)]
    pub fn vlp(&self) -> bool {
        (self.value.read_bits() & Self::VLP_MASK) != 0
    }

    /// Set VLP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_vlp(&self) {
        self.value.modify_bits(|r| r | Self::VLP_MASK);
    }

    /// Clear VLP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_vlp(&self) {
        self.value.modify_bits(|r| r & !Self::VLP_MASK);
    }

    /// **VLC** `[17:16]` (RW)
    ///
    /// VLC
    #[inline(always)]
    pub fn vlc(&self) -> u32 {
        (self.value.read_bits() & Self::VLC_MASK) >> Self::VLC_OFFSET
    }

    /// Write **VLC** `[17:16]` (RW)
    ///
    /// VLC
    #[inline(always)]
    pub fn write_vlc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VLC_MASK) | ((value << Self::VLC_OFFSET) & Self::VLC_MASK)
        });
    }

    /// **VLT** `[15:0]` (RW)
    ///
    /// VLT
    #[inline(always)]
    pub fn vlt(&self) -> u32 {
        (self.value.read_bits() & Self::VLT_MASK) >> Self::VLT_OFFSET
    }

    /// Write **VLT** `[15:0]` (RW)
    ///
    /// VLT
    #[inline(always)]
    pub fn write_vlt(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::VLT_MASK) | ((value << Self::VLT_OFFSET) & Self::VLT_MASK)
        });
    }
}

/// EMACTXVLTCTRL value type for batch read/modify/write operations
///
/// Use `emactxvltctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emactxvltctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmactxvltctrlVal {
    bits: u32,
}

impl EmactxvltctrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **CSVL** `[19:19]`
    ///
    /// CSVL
    #[inline(always)]
    pub const fn csvl(&self) -> bool {
        (self.bits & Emactxvltctrl::CSVL_MASK) != 0
    }

    /// Read **VLP** `[18:18]`
    ///
    /// VLP
    #[inline(always)]
    pub const fn vlp(&self) -> bool {
        (self.bits & Emactxvltctrl::VLP_MASK) != 0
    }

    /// Read **VLC** `[17:16]`
    ///
    /// VLC
    #[inline(always)]
    pub const fn vlc(&self) -> u32 {
        (self.bits & Emactxvltctrl::VLC_MASK) >> Emactxvltctrl::VLC_OFFSET
    }

    /// Read **VLT** `[15:0]`
    ///
    /// VLT
    #[inline(always)]
    pub const fn vlt(&self) -> u32 {
        (self.bits & Emactxvltctrl::VLT_MASK) >> Emactxvltctrl::VLT_OFFSET
    }

    /// Set **CSVL** `[19:19]`
    ///
    /// CSVL
    #[inline(always)]
    pub const fn with_csvl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactxvltctrl::CSVL_MASK
            } else {
                self.bits & !Emactxvltctrl::CSVL_MASK
            },
        }
    }

    /// Set **VLP** `[18:18]`
    ///
    /// VLP
    #[inline(always)]
    pub const fn with_vlp(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactxvltctrl::VLP_MASK
            } else {
                self.bits & !Emactxvltctrl::VLP_MASK
            },
        }
    }

    /// Set **VLC** `[17:16]`
    ///
    /// VLC
    #[inline(always)]
    pub const fn with_vlc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emactxvltctrl::VLC_MASK)
                | ((value << Emactxvltctrl::VLC_OFFSET) & Emactxvltctrl::VLC_MASK),
        }
    }

    /// Set **VLT** `[15:0]`
    ///
    /// VLT
    #[inline(always)]
    pub const fn with_vlt(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emactxvltctrl::VLT_MASK)
                | ((value << Emactxvltctrl::VLT_OFFSET) & Emactxvltctrl::VLT_MASK),
        }
    }
}

impl Default for EmactxvltctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emactxvltctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmactxvltctrlVal {
        EmactxvltctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmactxvltctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmactxvltctrlVal) -> EmactxvltctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACTSTPCTRL Register
///
/// EMACTSTPCTRL register
///
/// Address offset: 0x0700
/// Reset value: 0x00002000
///
/// ## Fields
///
/// - **TSENMACADDR** `[18]` (RW)
///
///   TSENMACADDR
///
/// - **SNAPTYPSEL** `[17:16]` (RW)
///
///   SNAPTYPSEL
///
/// - **TSMSTRENA** `[15]` (RW)
///
///   TSMSTRENA
///
/// - **TSEVNTENA** `[14]` (RW)
///
///   TSEVNTENA
///
/// - **TSIPV4ENA** `[13]` (RW)
///
///   TSIPV4ENA
///
/// - **TSIPV6ENA** `[12]` (RW)
///
///   TSIPV6ENA
///
/// - **TSIPENA** `[11]` (RW)
///
///   TSIPENA
///
/// - **TSVER2ENA** `[10]` (RW)
///
///   TSVER2ENA
///
/// - **TSCTRLSSR** `[9]` (RW)
///
///   TSCTRLSSR
///
/// - **TSENALL** `[8]` (RW)
///
///   TSENALL
///
/// - **TSADDREG** `[5]` (RW)
///
///   TSADDREG
///
/// - **TSTRIG** `[4]` (RW)
///
///   TSTRIG
///
/// - **TSUPDT** `[3]` (RW)
///
///   TSUPDT
///
/// - **TSINIT** `[2]` (RW)
///
///   TSINIT
///
/// - **TSCFUPDT** `[1]` (RW)
///
///   TSCFUPDT
///
/// - **TSENA** `[0]` (RW)
///
///   TSENA
#[repr(transparent)]
pub struct Emactstpctrl {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emactstpctrl {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emactstpctrl {
    // ======== Field Constants ========

    /// **TSENMACADDR**
    ///
    /// TSENMACADDR
    ///
    /// Bits `[18:18]`, mask `0x00040000`
    pub const TSENMACADDR_OFFSET: u32 = 18;
    pub const TSENMACADDR_MASK: u32 = 0x00040000;
    pub const TSENMACADDR: u32 = 1 << 18;

    /// **SNAPTYPSEL**
    ///
    /// SNAPTYPSEL
    ///
    /// Bits `[17:16]`, mask `0x00030000`
    pub const SNAPTYPSEL_OFFSET: u32 = 16;
    pub const SNAPTYPSEL_MASK: u32 = 0x00030000;

    /// **TSMSTRENA**
    ///
    /// TSMSTRENA
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const TSMSTRENA_OFFSET: u32 = 15;
    pub const TSMSTRENA_MASK: u32 = 0x00008000;
    pub const TSMSTRENA: u32 = 1 << 15;

    /// **TSEVNTENA**
    ///
    /// TSEVNTENA
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const TSEVNTENA_OFFSET: u32 = 14;
    pub const TSEVNTENA_MASK: u32 = 0x00004000;
    pub const TSEVNTENA: u32 = 1 << 14;

    /// **TSIPV4ENA**
    ///
    /// TSIPV4ENA
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const TSIPV4ENA_OFFSET: u32 = 13;
    pub const TSIPV4ENA_MASK: u32 = 0x00002000;
    pub const TSIPV4ENA: u32 = 1 << 13;

    /// **TSIPV6ENA**
    ///
    /// TSIPV6ENA
    ///
    /// Bits `[12:12]`, mask `0x00001000`
    pub const TSIPV6ENA_OFFSET: u32 = 12;
    pub const TSIPV6ENA_MASK: u32 = 0x00001000;
    pub const TSIPV6ENA: u32 = 1 << 12;

    /// **TSIPENA**
    ///
    /// TSIPENA
    ///
    /// Bits `[11:11]`, mask `0x00000800`
    pub const TSIPENA_OFFSET: u32 = 11;
    pub const TSIPENA_MASK: u32 = 0x00000800;
    pub const TSIPENA: u32 = 1 << 11;

    /// **TSVER2ENA**
    ///
    /// TSVER2ENA
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const TSVER2ENA_OFFSET: u32 = 10;
    pub const TSVER2ENA_MASK: u32 = 0x00000400;
    pub const TSVER2ENA: u32 = 1 << 10;

    /// **TSCTRLSSR**
    ///
    /// TSCTRLSSR
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const TSCTRLSSR_OFFSET: u32 = 9;
    pub const TSCTRLSSR_MASK: u32 = 0x00000200;
    pub const TSCTRLSSR: u32 = 1 << 9;

    /// **TSENALL**
    ///
    /// TSENALL
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const TSENALL_OFFSET: u32 = 8;
    pub const TSENALL_MASK: u32 = 0x00000100;
    pub const TSENALL: u32 = 1 << 8;

    /// **TSADDREG**
    ///
    /// TSADDREG
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TSADDREG_OFFSET: u32 = 5;
    pub const TSADDREG_MASK: u32 = 0x00000020;
    pub const TSADDREG: u32 = 1 << 5;

    /// **TSTRIG**
    ///
    /// TSTRIG
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const TSTRIG_OFFSET: u32 = 4;
    pub const TSTRIG_MASK: u32 = 0x00000010;
    pub const TSTRIG: u32 = 1 << 4;

    /// **TSUPDT**
    ///
    /// TSUPDT
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TSUPDT_OFFSET: u32 = 3;
    pub const TSUPDT_MASK: u32 = 0x00000008;
    pub const TSUPDT: u32 = 1 << 3;

    /// **TSINIT**
    ///
    /// TSINIT
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TSINIT_OFFSET: u32 = 2;
    pub const TSINIT_MASK: u32 = 0x00000004;
    pub const TSINIT: u32 = 1 << 2;

    /// **TSCFUPDT**
    ///
    /// TSCFUPDT
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TSCFUPDT_OFFSET: u32 = 1;
    pub const TSCFUPDT_MASK: u32 = 0x00000002;
    pub const TSCFUPDT: u32 = 1 << 1;

    /// **TSENA**
    ///
    /// TSENA
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TSENA_OFFSET: u32 = 0;
    pub const TSENA_MASK: u32 = 0x00000001;
    pub const TSENA: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TSENMACADDR** `[18]` (RW)
    ///
    /// TSENMACADDR
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> bool {
        (self.value.read_bits() & Self::TSENMACADDR_MASK) != 0
    }

    /// Set TSENMACADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsenmacaddr(&self) {
        self.value.modify_bits(|r| r | Self::TSENMACADDR_MASK);
    }

    /// Clear TSENMACADDR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsenmacaddr(&self) {
        self.value.modify_bits(|r| r & !Self::TSENMACADDR_MASK);
    }

    /// **SNAPTYPSEL** `[17:16]` (RW)
    ///
    /// SNAPTYPSEL
    #[inline(always)]
    pub fn snaptypsel(&self) -> u32 {
        (self.value.read_bits() & Self::SNAPTYPSEL_MASK) >> Self::SNAPTYPSEL_OFFSET
    }

    /// Write **SNAPTYPSEL** `[17:16]` (RW)
    ///
    /// SNAPTYPSEL
    #[inline(always)]
    pub fn write_snaptypsel(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SNAPTYPSEL_MASK)
                | ((value << Self::SNAPTYPSEL_OFFSET) & Self::SNAPTYPSEL_MASK)
        });
    }

    /// **TSMSTRENA** `[15]` (RW)
    ///
    /// TSMSTRENA
    #[inline(always)]
    pub fn tsmstrena(&self) -> bool {
        (self.value.read_bits() & Self::TSMSTRENA_MASK) != 0
    }

    /// Set TSMSTRENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsmstrena(&self) {
        self.value.modify_bits(|r| r | Self::TSMSTRENA_MASK);
    }

    /// Clear TSMSTRENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsmstrena(&self) {
        self.value.modify_bits(|r| r & !Self::TSMSTRENA_MASK);
    }

    /// **TSEVNTENA** `[14]` (RW)
    ///
    /// TSEVNTENA
    #[inline(always)]
    pub fn tsevntena(&self) -> bool {
        (self.value.read_bits() & Self::TSEVNTENA_MASK) != 0
    }

    /// Set TSEVNTENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsevntena(&self) {
        self.value.modify_bits(|r| r | Self::TSEVNTENA_MASK);
    }

    /// Clear TSEVNTENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsevntena(&self) {
        self.value.modify_bits(|r| r & !Self::TSEVNTENA_MASK);
    }

    /// **TSIPV4ENA** `[13]` (RW)
    ///
    /// TSIPV4ENA
    #[inline(always)]
    pub fn tsipv4ena(&self) -> bool {
        (self.value.read_bits() & Self::TSIPV4ENA_MASK) != 0
    }

    /// Set TSIPV4ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsipv4ena(&self) {
        self.value.modify_bits(|r| r | Self::TSIPV4ENA_MASK);
    }

    /// Clear TSIPV4ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsipv4ena(&self) {
        self.value.modify_bits(|r| r & !Self::TSIPV4ENA_MASK);
    }

    /// **TSIPV6ENA** `[12]` (RW)
    ///
    /// TSIPV6ENA
    #[inline(always)]
    pub fn tsipv6ena(&self) -> bool {
        (self.value.read_bits() & Self::TSIPV6ENA_MASK) != 0
    }

    /// Set TSIPV6ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsipv6ena(&self) {
        self.value.modify_bits(|r| r | Self::TSIPV6ENA_MASK);
    }

    /// Clear TSIPV6ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsipv6ena(&self) {
        self.value.modify_bits(|r| r & !Self::TSIPV6ENA_MASK);
    }

    /// **TSIPENA** `[11]` (RW)
    ///
    /// TSIPENA
    #[inline(always)]
    pub fn tsipena(&self) -> bool {
        (self.value.read_bits() & Self::TSIPENA_MASK) != 0
    }

    /// Set TSIPENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsipena(&self) {
        self.value.modify_bits(|r| r | Self::TSIPENA_MASK);
    }

    /// Clear TSIPENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsipena(&self) {
        self.value.modify_bits(|r| r & !Self::TSIPENA_MASK);
    }

    /// **TSVER2ENA** `[10]` (RW)
    ///
    /// TSVER2ENA
    #[inline(always)]
    pub fn tsver2ena(&self) -> bool {
        (self.value.read_bits() & Self::TSVER2ENA_MASK) != 0
    }

    /// Set TSVER2ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsver2ena(&self) {
        self.value.modify_bits(|r| r | Self::TSVER2ENA_MASK);
    }

    /// Clear TSVER2ENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsver2ena(&self) {
        self.value.modify_bits(|r| r & !Self::TSVER2ENA_MASK);
    }

    /// **TSCTRLSSR** `[9]` (RW)
    ///
    /// TSCTRLSSR
    #[inline(always)]
    pub fn tsctrlssr(&self) -> bool {
        (self.value.read_bits() & Self::TSCTRLSSR_MASK) != 0
    }

    /// Set TSCTRLSSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsctrlssr(&self) {
        self.value.modify_bits(|r| r | Self::TSCTRLSSR_MASK);
    }

    /// Clear TSCTRLSSR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsctrlssr(&self) {
        self.value.modify_bits(|r| r & !Self::TSCTRLSSR_MASK);
    }

    /// **TSENALL** `[8]` (RW)
    ///
    /// TSENALL
    #[inline(always)]
    pub fn tsenall(&self) -> bool {
        (self.value.read_bits() & Self::TSENALL_MASK) != 0
    }

    /// Set TSENALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsenall(&self) {
        self.value.modify_bits(|r| r | Self::TSENALL_MASK);
    }

    /// Clear TSENALL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsenall(&self) {
        self.value.modify_bits(|r| r & !Self::TSENALL_MASK);
    }

    /// **TSADDREG** `[5]` (RW)
    ///
    /// TSADDREG
    #[inline(always)]
    pub fn tsaddreg(&self) -> bool {
        (self.value.read_bits() & Self::TSADDREG_MASK) != 0
    }

    /// Set TSADDREG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsaddreg(&self) {
        self.value.modify_bits(|r| r | Self::TSADDREG_MASK);
    }

    /// Clear TSADDREG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsaddreg(&self) {
        self.value.modify_bits(|r| r & !Self::TSADDREG_MASK);
    }

    /// **TSTRIG** `[4]` (RW)
    ///
    /// TSTRIG
    #[inline(always)]
    pub fn tstrig(&self) -> bool {
        (self.value.read_bits() & Self::TSTRIG_MASK) != 0
    }

    /// Set TSTRIG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tstrig(&self) {
        self.value.modify_bits(|r| r | Self::TSTRIG_MASK);
    }

    /// Clear TSTRIG bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tstrig(&self) {
        self.value.modify_bits(|r| r & !Self::TSTRIG_MASK);
    }

    /// **TSUPDT** `[3]` (RW)
    ///
    /// TSUPDT
    #[inline(always)]
    pub fn tsupdt(&self) -> bool {
        (self.value.read_bits() & Self::TSUPDT_MASK) != 0
    }

    /// Set TSUPDT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsupdt(&self) {
        self.value.modify_bits(|r| r | Self::TSUPDT_MASK);
    }

    /// Clear TSUPDT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsupdt(&self) {
        self.value.modify_bits(|r| r & !Self::TSUPDT_MASK);
    }

    /// **TSINIT** `[2]` (RW)
    ///
    /// TSINIT
    #[inline(always)]
    pub fn tsinit(&self) -> bool {
        (self.value.read_bits() & Self::TSINIT_MASK) != 0
    }

    /// Set TSINIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsinit(&self) {
        self.value.modify_bits(|r| r | Self::TSINIT_MASK);
    }

    /// Clear TSINIT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsinit(&self) {
        self.value.modify_bits(|r| r & !Self::TSINIT_MASK);
    }

    /// **TSCFUPDT** `[1]` (RW)
    ///
    /// TSCFUPDT
    #[inline(always)]
    pub fn tscfupdt(&self) -> bool {
        (self.value.read_bits() & Self::TSCFUPDT_MASK) != 0
    }

    /// Set TSCFUPDT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tscfupdt(&self) {
        self.value.modify_bits(|r| r | Self::TSCFUPDT_MASK);
    }

    /// Clear TSCFUPDT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tscfupdt(&self) {
        self.value.modify_bits(|r| r & !Self::TSCFUPDT_MASK);
    }

    /// **TSENA** `[0]` (RW)
    ///
    /// TSENA
    #[inline(always)]
    pub fn tsena(&self) -> bool {
        (self.value.read_bits() & Self::TSENA_MASK) != 0
    }

    /// Set TSENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tsena(&self) {
        self.value.modify_bits(|r| r | Self::TSENA_MASK);
    }

    /// Clear TSENA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tsena(&self) {
        self.value.modify_bits(|r| r & !Self::TSENA_MASK);
    }
}

/// EMACTSTPCTRL value type for batch read/modify/write operations
///
/// Use `emactstpctrl.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emactstpctrl.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmactstpctrlVal {
    bits: u32,
}

impl EmactstpctrlVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00002000 }
    }

    /// Read **TSENMACADDR** `[18:18]`
    ///
    /// TSENMACADDR
    #[inline(always)]
    pub const fn tsenmacaddr(&self) -> bool {
        (self.bits & Emactstpctrl::TSENMACADDR_MASK) != 0
    }

    /// Read **SNAPTYPSEL** `[17:16]`
    ///
    /// SNAPTYPSEL
    #[inline(always)]
    pub const fn snaptypsel(&self) -> u32 {
        (self.bits & Emactstpctrl::SNAPTYPSEL_MASK) >> Emactstpctrl::SNAPTYPSEL_OFFSET
    }

    /// Read **TSMSTRENA** `[15:15]`
    ///
    /// TSMSTRENA
    #[inline(always)]
    pub const fn tsmstrena(&self) -> bool {
        (self.bits & Emactstpctrl::TSMSTRENA_MASK) != 0
    }

    /// Read **TSEVNTENA** `[14:14]`
    ///
    /// TSEVNTENA
    #[inline(always)]
    pub const fn tsevntena(&self) -> bool {
        (self.bits & Emactstpctrl::TSEVNTENA_MASK) != 0
    }

    /// Read **TSIPV4ENA** `[13:13]`
    ///
    /// TSIPV4ENA
    #[inline(always)]
    pub const fn tsipv4ena(&self) -> bool {
        (self.bits & Emactstpctrl::TSIPV4ENA_MASK) != 0
    }

    /// Read **TSIPV6ENA** `[12:12]`
    ///
    /// TSIPV6ENA
    #[inline(always)]
    pub const fn tsipv6ena(&self) -> bool {
        (self.bits & Emactstpctrl::TSIPV6ENA_MASK) != 0
    }

    /// Read **TSIPENA** `[11:11]`
    ///
    /// TSIPENA
    #[inline(always)]
    pub const fn tsipena(&self) -> bool {
        (self.bits & Emactstpctrl::TSIPENA_MASK) != 0
    }

    /// Read **TSVER2ENA** `[10:10]`
    ///
    /// TSVER2ENA
    #[inline(always)]
    pub const fn tsver2ena(&self) -> bool {
        (self.bits & Emactstpctrl::TSVER2ENA_MASK) != 0
    }

    /// Read **TSCTRLSSR** `[9:9]`
    ///
    /// TSCTRLSSR
    #[inline(always)]
    pub const fn tsctrlssr(&self) -> bool {
        (self.bits & Emactstpctrl::TSCTRLSSR_MASK) != 0
    }

    /// Read **TSENALL** `[8:8]`
    ///
    /// TSENALL
    #[inline(always)]
    pub const fn tsenall(&self) -> bool {
        (self.bits & Emactstpctrl::TSENALL_MASK) != 0
    }

    /// Read **TSADDREG** `[5:5]`
    ///
    /// TSADDREG
    #[inline(always)]
    pub const fn tsaddreg(&self) -> bool {
        (self.bits & Emactstpctrl::TSADDREG_MASK) != 0
    }

    /// Read **TSTRIG** `[4:4]`
    ///
    /// TSTRIG
    #[inline(always)]
    pub const fn tstrig(&self) -> bool {
        (self.bits & Emactstpctrl::TSTRIG_MASK) != 0
    }

    /// Read **TSUPDT** `[3:3]`
    ///
    /// TSUPDT
    #[inline(always)]
    pub const fn tsupdt(&self) -> bool {
        (self.bits & Emactstpctrl::TSUPDT_MASK) != 0
    }

    /// Read **TSINIT** `[2:2]`
    ///
    /// TSINIT
    #[inline(always)]
    pub const fn tsinit(&self) -> bool {
        (self.bits & Emactstpctrl::TSINIT_MASK) != 0
    }

    /// Read **TSCFUPDT** `[1:1]`
    ///
    /// TSCFUPDT
    #[inline(always)]
    pub const fn tscfupdt(&self) -> bool {
        (self.bits & Emactstpctrl::TSCFUPDT_MASK) != 0
    }

    /// Read **TSENA** `[0:0]`
    ///
    /// TSENA
    #[inline(always)]
    pub const fn tsena(&self) -> bool {
        (self.bits & Emactstpctrl::TSENA_MASK) != 0
    }

    /// Set **TSENMACADDR** `[18:18]`
    ///
    /// TSENMACADDR
    #[inline(always)]
    pub const fn with_tsenmacaddr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSENMACADDR_MASK
            } else {
                self.bits & !Emactstpctrl::TSENMACADDR_MASK
            },
        }
    }

    /// Set **SNAPTYPSEL** `[17:16]`
    ///
    /// SNAPTYPSEL
    #[inline(always)]
    pub const fn with_snaptypsel(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emactstpctrl::SNAPTYPSEL_MASK)
                | ((value << Emactstpctrl::SNAPTYPSEL_OFFSET) & Emactstpctrl::SNAPTYPSEL_MASK),
        }
    }

    /// Set **TSMSTRENA** `[15:15]`
    ///
    /// TSMSTRENA
    #[inline(always)]
    pub const fn with_tsmstrena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSMSTRENA_MASK
            } else {
                self.bits & !Emactstpctrl::TSMSTRENA_MASK
            },
        }
    }

    /// Set **TSEVNTENA** `[14:14]`
    ///
    /// TSEVNTENA
    #[inline(always)]
    pub const fn with_tsevntena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSEVNTENA_MASK
            } else {
                self.bits & !Emactstpctrl::TSEVNTENA_MASK
            },
        }
    }

    /// Set **TSIPV4ENA** `[13:13]`
    ///
    /// TSIPV4ENA
    #[inline(always)]
    pub const fn with_tsipv4ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSIPV4ENA_MASK
            } else {
                self.bits & !Emactstpctrl::TSIPV4ENA_MASK
            },
        }
    }

    /// Set **TSIPV6ENA** `[12:12]`
    ///
    /// TSIPV6ENA
    #[inline(always)]
    pub const fn with_tsipv6ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSIPV6ENA_MASK
            } else {
                self.bits & !Emactstpctrl::TSIPV6ENA_MASK
            },
        }
    }

    /// Set **TSIPENA** `[11:11]`
    ///
    /// TSIPENA
    #[inline(always)]
    pub const fn with_tsipena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSIPENA_MASK
            } else {
                self.bits & !Emactstpctrl::TSIPENA_MASK
            },
        }
    }

    /// Set **TSVER2ENA** `[10:10]`
    ///
    /// TSVER2ENA
    #[inline(always)]
    pub const fn with_tsver2ena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSVER2ENA_MASK
            } else {
                self.bits & !Emactstpctrl::TSVER2ENA_MASK
            },
        }
    }

    /// Set **TSCTRLSSR** `[9:9]`
    ///
    /// TSCTRLSSR
    #[inline(always)]
    pub const fn with_tsctrlssr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSCTRLSSR_MASK
            } else {
                self.bits & !Emactstpctrl::TSCTRLSSR_MASK
            },
        }
    }

    /// Set **TSENALL** `[8:8]`
    ///
    /// TSENALL
    #[inline(always)]
    pub const fn with_tsenall(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSENALL_MASK
            } else {
                self.bits & !Emactstpctrl::TSENALL_MASK
            },
        }
    }

    /// Set **TSADDREG** `[5:5]`
    ///
    /// TSADDREG
    #[inline(always)]
    pub const fn with_tsaddreg(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSADDREG_MASK
            } else {
                self.bits & !Emactstpctrl::TSADDREG_MASK
            },
        }
    }

    /// Set **TSTRIG** `[4:4]`
    ///
    /// TSTRIG
    #[inline(always)]
    pub const fn with_tstrig(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSTRIG_MASK
            } else {
                self.bits & !Emactstpctrl::TSTRIG_MASK
            },
        }
    }

    /// Set **TSUPDT** `[3:3]`
    ///
    /// TSUPDT
    #[inline(always)]
    pub const fn with_tsupdt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSUPDT_MASK
            } else {
                self.bits & !Emactstpctrl::TSUPDT_MASK
            },
        }
    }

    /// Set **TSINIT** `[2:2]`
    ///
    /// TSINIT
    #[inline(always)]
    pub const fn with_tsinit(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSINIT_MASK
            } else {
                self.bits & !Emactstpctrl::TSINIT_MASK
            },
        }
    }

    /// Set **TSCFUPDT** `[1:1]`
    ///
    /// TSCFUPDT
    #[inline(always)]
    pub const fn with_tscfupdt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSCFUPDT_MASK
            } else {
                self.bits & !Emactstpctrl::TSCFUPDT_MASK
            },
        }
    }

    /// Set **TSENA** `[0:0]`
    ///
    /// TSENA
    #[inline(always)]
    pub const fn with_tsena(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpctrl::TSENA_MASK
            } else {
                self.bits & !Emactstpctrl::TSENA_MASK
            },
        }
    }
}

impl Default for EmactstpctrlVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emactstpctrl {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmactstpctrlVal {
        EmactstpctrlVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmactstpctrlVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmactstpctrlVal) -> EmactstpctrlVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACSUB2NDINCR Register
///
/// EMACSUB2NDINCR register
///
/// Address offset: 0x0704
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **SSINC** `[7:0]` (RW)
///
///   SSINC
#[repr(transparent)]
pub struct Emacsub2ndincr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacsub2ndincr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacsub2ndincr {
    // ======== Field Constants ========

    /// **SSINC**
    ///
    /// SSINC
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const SSINC_OFFSET: u32 = 0;
    pub const SSINC_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **SSINC** `[7:0]` (RW)
    ///
    /// SSINC
    #[inline(always)]
    pub fn ssinc(&self) -> u32 {
        (self.value.read_bits() & Self::SSINC_MASK) >> Self::SSINC_OFFSET
    }

    /// Write **SSINC** `[7:0]` (RW)
    ///
    /// SSINC
    #[inline(always)]
    pub fn write_ssinc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::SSINC_MASK) | ((value << Self::SSINC_OFFSET) & Self::SSINC_MASK)
        });
    }
}

/// EMACSUB2NDINCR value type for batch read/modify/write operations
///
/// Use `emacsub2ndincr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacsub2ndincr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacsub2ndincrVal {
    bits: u32,
}

impl Emacsub2ndincrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **SSINC** `[7:0]`
    ///
    /// SSINC
    #[inline(always)]
    pub const fn ssinc(&self) -> u32 {
        (self.bits & Emacsub2ndincr::SSINC_MASK) >> Emacsub2ndincr::SSINC_OFFSET
    }

    /// Set **SSINC** `[7:0]`
    ///
    /// SSINC
    #[inline(always)]
    pub const fn with_ssinc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacsub2ndincr::SSINC_MASK)
                | ((value << Emacsub2ndincr::SSINC_OFFSET) & Emacsub2ndincr::SSINC_MASK),
        }
    }
}

impl Default for Emacsub2ndincrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacsub2ndincr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacsub2ndincrVal {
        Emacsub2ndincrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacsub2ndincrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacsub2ndincrVal) -> Emacsub2ndincrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACSYSTIM2ND Register
///
/// EMACSYSTIM2ND register
///
/// Address offset: 0x0708
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emacsystim2nd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacsystim2nd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacsystim2nd {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACSYSTIM2ND value type for batch read/modify/write operations
///
/// Use `emacsystim2nd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacsystim2nd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacsystim2ndVal {
    bits: u32,
}

impl Emacsystim2ndVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emacsystim2nd::ADDRESS_LOW_MASK) >> Emacsystim2nd::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacsystim2nd::ADDRESS_LOW_MASK)
                | ((value << Emacsystim2nd::ADDRESS_LOW_OFFSET) & Emacsystim2nd::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emacsystim2ndVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacsystim2nd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacsystim2ndVal {
        Emacsystim2ndVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacsystim2ndVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacsystim2ndVal) -> Emacsystim2ndVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACSYSTIMNS Register
///
/// EMACSYSTIMNS register
///
/// Address offset: 0x070c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TSSS** `[30:0]` (RW)
///
///   TSSS
#[repr(transparent)]
pub struct Emacsystimns {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacsystimns {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacsystimns {
    // ======== Field Constants ========

    /// **TSSS**
    ///
    /// TSSS
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const TSSS_OFFSET: u32 = 0;
    pub const TSSS_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// **TSSS** `[30:0]` (RW)
    ///
    /// TSSS
    #[inline(always)]
    pub fn tsss(&self) -> u32 {
        (self.value.read_bits() & Self::TSSS_MASK) >> Self::TSSS_OFFSET
    }

    /// Write **TSSS** `[30:0]` (RW)
    ///
    /// TSSS
    #[inline(always)]
    pub fn write_tsss(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TSSS_MASK) | ((value << Self::TSSS_OFFSET) & Self::TSSS_MASK)
        });
    }
}

/// EMACSYSTIMNS value type for batch read/modify/write operations
///
/// Use `emacsystimns.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacsystimns.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacsystimnsVal {
    bits: u32,
}

impl EmacsystimnsVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **TSSS** `[30:0]`
    ///
    /// TSSS
    #[inline(always)]
    pub const fn tsss(&self) -> u32 {
        (self.bits & Emacsystimns::TSSS_MASK) >> Emacsystimns::TSSS_OFFSET
    }

    /// Set **TSSS** `[30:0]`
    ///
    /// TSSS
    #[inline(always)]
    pub const fn with_tsss(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacsystimns::TSSS_MASK)
                | ((value << Emacsystimns::TSSS_OFFSET) & Emacsystimns::TSSS_MASK),
        }
    }
}

impl Default for EmacsystimnsVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacsystimns {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacsystimnsVal {
        EmacsystimnsVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacsystimnsVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacsystimnsVal) -> EmacsystimnsVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMAC2NDUPT Register
///
/// EMAC2NDUPT register
///
/// Address offset: 0x0710
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emac2ndupt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emac2ndupt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emac2ndupt {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMAC2NDUPT value type for batch read/modify/write operations
///
/// Use `emac2ndupt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emac2ndupt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emac2nduptVal {
    bits: u32,
}

impl Emac2nduptVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emac2ndupt::ADDRESS_LOW_MASK) >> Emac2ndupt::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emac2ndupt::ADDRESS_LOW_MASK)
                | ((value << Emac2ndupt::ADDRESS_LOW_OFFSET) & Emac2ndupt::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emac2nduptVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emac2ndupt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emac2nduptVal {
        Emac2nduptVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emac2nduptVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emac2nduptVal) -> Emac2nduptVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACNSUPT Register
///
/// EMACNSUPT register
///
/// Address offset: 0x0714
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDSUB** `[31]` (RW)
///
///   ADDSUB
///
/// - **TSSSUPD** `[30:0]` (RW)
///
///   TSSSUPD
#[repr(transparent)]
pub struct Emacnsupt {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacnsupt {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacnsupt {
    // ======== Field Constants ========

    /// **ADDSUB**
    ///
    /// ADDSUB
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const ADDSUB_OFFSET: u32 = 31;
    pub const ADDSUB_MASK: u32 = 0x80000000;
    pub const ADDSUB: u32 = 1 << 31;

    /// **TSSSUPD**
    ///
    /// TSSSUPD
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const TSSSUPD_OFFSET: u32 = 0;
    pub const TSSSUPD_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// **ADDSUB** `[31]` (RW)
    ///
    /// ADDSUB
    #[inline(always)]
    pub fn addsub(&self) -> bool {
        (self.value.read_bits() & Self::ADDSUB_MASK) != 0
    }

    /// Set ADDSUB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_addsub(&self) {
        self.value.modify_bits(|r| r | Self::ADDSUB_MASK);
    }

    /// Clear ADDSUB bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_addsub(&self) {
        self.value.modify_bits(|r| r & !Self::ADDSUB_MASK);
    }

    /// **TSSSUPD** `[30:0]` (RW)
    ///
    /// TSSSUPD
    #[inline(always)]
    pub fn tsssupd(&self) -> u32 {
        (self.value.read_bits() & Self::TSSSUPD_MASK) >> Self::TSSSUPD_OFFSET
    }

    /// Write **TSSSUPD** `[30:0]` (RW)
    ///
    /// TSSSUPD
    #[inline(always)]
    pub fn write_tsssupd(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TSSSUPD_MASK) | ((value << Self::TSSSUPD_OFFSET) & Self::TSSSUPD_MASK)
        });
    }
}

/// EMACNSUPT value type for batch read/modify/write operations
///
/// Use `emacnsupt.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacnsupt.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmacnsuptVal {
    bits: u32,
}

impl EmacnsuptVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDSUB** `[31:31]`
    ///
    /// ADDSUB
    #[inline(always)]
    pub const fn addsub(&self) -> bool {
        (self.bits & Emacnsupt::ADDSUB_MASK) != 0
    }

    /// Read **TSSSUPD** `[30:0]`
    ///
    /// TSSSUPD
    #[inline(always)]
    pub const fn tsssupd(&self) -> u32 {
        (self.bits & Emacnsupt::TSSSUPD_MASK) >> Emacnsupt::TSSSUPD_OFFSET
    }

    /// Set **ADDSUB** `[31:31]`
    ///
    /// ADDSUB
    #[inline(always)]
    pub const fn with_addsub(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emacnsupt::ADDSUB_MASK
            } else {
                self.bits & !Emacnsupt::ADDSUB_MASK
            },
        }
    }

    /// Set **TSSSUPD** `[30:0]`
    ///
    /// TSSSUPD
    #[inline(always)]
    pub const fn with_tsssupd(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacnsupt::TSSSUPD_MASK)
                | ((value << Emacnsupt::TSSSUPD_OFFSET) & Emacnsupt::TSSSUPD_MASK),
        }
    }
}

impl Default for EmacnsuptVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacnsupt {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmacnsuptVal {
        EmacnsuptVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmacnsuptVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmacnsuptVal) -> EmacnsuptVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACTSTPADDEND Register
///
/// EMACTSTPADDEND register
///
/// Address offset: 0x0718
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emactstpaddend {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emactstpaddend {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emactstpaddend {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACTSTPADDEND value type for batch read/modify/write operations
///
/// Use `emactstpaddend.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emactstpaddend.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmactstpaddendVal {
    bits: u32,
}

impl EmactstpaddendVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emactstpaddend::ADDRESS_LOW_MASK) >> Emactstpaddend::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emactstpaddend::ADDRESS_LOW_MASK)
                | ((value << Emactstpaddend::ADDRESS_LOW_OFFSET)
                    & Emactstpaddend::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for EmactstpaddendVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emactstpaddend {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmactstpaddendVal {
        EmactstpaddendVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmactstpaddendVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmactstpaddendVal) -> EmactstpaddendVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACTGTIME2ND Register
///
/// EMACTGTIME2ND register
///
/// Address offset: 0x071c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Emactgtime2nd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emactgtime2nd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emactgtime2nd {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// EMACTGTIME2ND value type for batch read/modify/write operations
///
/// Use `emactgtime2nd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emactgtime2nd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emactgtime2ndVal {
    bits: u32,
}

impl Emactgtime2ndVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Emactgtime2nd::ADDRESS_LOW_MASK) >> Emactgtime2nd::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emactgtime2nd::ADDRESS_LOW_MASK)
                | ((value << Emactgtime2nd::ADDRESS_LOW_OFFSET) & Emactgtime2nd::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for Emactgtime2ndVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emactgtime2nd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emactgtime2ndVal {
        Emactgtime2ndVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emactgtime2ndVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emactgtime2ndVal) -> Emactgtime2ndVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACTGTIMENS Register
///
/// EMACTGTIMENS register
///
/// Address offset: 0x0720
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TTSLO** `[30:0]` (RW)
///
///   TTSLO
#[repr(transparent)]
pub struct Emactgtimens {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emactgtimens {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emactgtimens {
    // ======== Field Constants ========

    /// **TTSLO**
    ///
    /// TTSLO
    ///
    /// Bits `[30:0]`, mask `0x7fffffff`
    pub const TTSLO_OFFSET: u32 = 0;
    pub const TTSLO_MASK: u32 = 0x7fffffff;

    // ======== Field Accessors ========

    /// **TTSLO** `[30:0]` (RW)
    ///
    /// TTSLO
    #[inline(always)]
    pub fn ttslo(&self) -> u32 {
        (self.value.read_bits() & Self::TTSLO_MASK) >> Self::TTSLO_OFFSET
    }

    /// Write **TTSLO** `[30:0]` (RW)
    ///
    /// TTSLO
    #[inline(always)]
    pub fn write_ttslo(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TTSLO_MASK) | ((value << Self::TTSLO_OFFSET) & Self::TTSLO_MASK)
        });
    }
}

/// EMACTGTIMENS value type for batch read/modify/write operations
///
/// Use `emactgtimens.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emactgtimens.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmactgtimensVal {
    bits: u32,
}

impl EmactgtimensVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **TTSLO** `[30:0]`
    ///
    /// TTSLO
    #[inline(always)]
    pub const fn ttslo(&self) -> u32 {
        (self.bits & Emactgtimens::TTSLO_MASK) >> Emactgtimens::TTSLO_OFFSET
    }

    /// Set **TTSLO** `[30:0]`
    ///
    /// TTSLO
    #[inline(always)]
    pub const fn with_ttslo(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emactgtimens::TTSLO_MASK)
                | ((value << Emactgtimens::TTSLO_OFFSET) & Emactgtimens::TTSLO_MASK),
        }
    }
}

impl Default for EmactgtimensVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emactgtimens {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmactgtimensVal {
        EmactgtimensVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmactgtimensVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmactgtimensVal) -> EmactgtimensVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACSYSTIMHIGHWORD2ND Register
///
/// EMACSYSTIMHIGHWORD2ND register
///
/// Address offset: 0x0724
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TSHWR** `[15:0]` (RW)
///
///   TSHWR
#[repr(transparent)]
pub struct Emacsystimhighword2nd {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emacsystimhighword2nd {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emacsystimhighword2nd {
    // ======== Field Constants ========

    /// **TSHWR**
    ///
    /// TSHWR
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const TSHWR_OFFSET: u32 = 0;
    pub const TSHWR_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **TSHWR** `[15:0]` (RW)
    ///
    /// TSHWR
    #[inline(always)]
    pub fn tshwr(&self) -> u32 {
        (self.value.read_bits() & Self::TSHWR_MASK) >> Self::TSHWR_OFFSET
    }

    /// Write **TSHWR** `[15:0]` (RW)
    ///
    /// TSHWR
    #[inline(always)]
    pub fn write_tshwr(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TSHWR_MASK) | ((value << Self::TSHWR_OFFSET) & Self::TSHWR_MASK)
        });
    }
}

/// EMACSYSTIMHIGHWORD2ND value type for batch read/modify/write operations
///
/// Use `emacsystimhighword2nd.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emacsystimhighword2nd.write_val(val)`.
#[derive(Clone, Copy)]
pub struct Emacsystimhighword2ndVal {
    bits: u32,
}

impl Emacsystimhighword2ndVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **TSHWR** `[15:0]`
    ///
    /// TSHWR
    #[inline(always)]
    pub const fn tshwr(&self) -> u32 {
        (self.bits & Emacsystimhighword2nd::TSHWR_MASK) >> Emacsystimhighword2nd::TSHWR_OFFSET
    }

    /// Set **TSHWR** `[15:0]`
    ///
    /// TSHWR
    #[inline(always)]
    pub const fn with_tshwr(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Emacsystimhighword2nd::TSHWR_MASK)
                | ((value << Emacsystimhighword2nd::TSHWR_OFFSET)
                    & Emacsystimhighword2nd::TSHWR_MASK),
        }
    }
}

impl Default for Emacsystimhighword2ndVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emacsystimhighword2nd {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> Emacsystimhighword2ndVal {
        Emacsystimhighword2ndVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: Emacsystimhighword2ndVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(Emacsystimhighword2ndVal) -> Emacsystimhighword2ndVal>(
        &self,
        f: F,
    ) {
        self.write_val(f(self.read_val()))
    }
}

/// EMACTSTPSTATUS Register
///
/// EMACTSTPSTATUS register
///
/// Address offset: 0x0728
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TSTRGTERR** `[3]` (RW)
///
///   TSTRGTERR
///
/// - **TSTARGT** `[1]` (RW)
///
///   TSTARGT
///
/// - **TSSOVF** `[0]` (RW)
///
///   TSSOVF
#[repr(transparent)]
pub struct Emactstpstatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Emactstpstatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Emactstpstatus {
    // ======== Field Constants ========

    /// **TSTRGTERR**
    ///
    /// TSTRGTERR
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TSTRGTERR_OFFSET: u32 = 3;
    pub const TSTRGTERR_MASK: u32 = 0x00000008;
    pub const TSTRGTERR: u32 = 1 << 3;

    /// **TSTARGT**
    ///
    /// TSTARGT
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TSTARGT_OFFSET: u32 = 1;
    pub const TSTARGT_MASK: u32 = 0x00000002;
    pub const TSTARGT: u32 = 1 << 1;

    /// **TSSOVF**
    ///
    /// TSSOVF
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TSSOVF_OFFSET: u32 = 0;
    pub const TSSOVF_MASK: u32 = 0x00000001;
    pub const TSSOVF: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **TSTRGTERR** `[3]` (RW)
    ///
    /// TSTRGTERR
    #[inline(always)]
    pub fn tstrgterr(&self) -> bool {
        (self.value.read_bits() & Self::TSTRGTERR_MASK) != 0
    }

    /// Set TSTRGTERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tstrgterr(&self) {
        self.value.modify_bits(|r| r | Self::TSTRGTERR_MASK);
    }

    /// Clear TSTRGTERR bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tstrgterr(&self) {
        self.value.modify_bits(|r| r & !Self::TSTRGTERR_MASK);
    }

    /// **TSTARGT** `[1]` (RW)
    ///
    /// TSTARGT
    #[inline(always)]
    pub fn tstargt(&self) -> bool {
        (self.value.read_bits() & Self::TSTARGT_MASK) != 0
    }

    /// Set TSTARGT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tstargt(&self) {
        self.value.modify_bits(|r| r | Self::TSTARGT_MASK);
    }

    /// Clear TSTARGT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tstargt(&self) {
        self.value.modify_bits(|r| r & !Self::TSTARGT_MASK);
    }

    /// **TSSOVF** `[0]` (RW)
    ///
    /// TSSOVF
    #[inline(always)]
    pub fn tssovf(&self) -> bool {
        (self.value.read_bits() & Self::TSSOVF_MASK) != 0
    }

    /// Set TSSOVF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_tssovf(&self) {
        self.value.modify_bits(|r| r | Self::TSSOVF_MASK);
    }

    /// Clear TSSOVF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_tssovf(&self) {
        self.value.modify_bits(|r| r & !Self::TSSOVF_MASK);
    }
}

/// EMACTSTPSTATUS value type for batch read/modify/write operations
///
/// Use `emactstpstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `emactstpstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct EmactstpstatusVal {
    bits: u32,
}

impl EmactstpstatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **TSTRGTERR** `[3:3]`
    ///
    /// TSTRGTERR
    #[inline(always)]
    pub const fn tstrgterr(&self) -> bool {
        (self.bits & Emactstpstatus::TSTRGTERR_MASK) != 0
    }

    /// Read **TSTARGT** `[1:1]`
    ///
    /// TSTARGT
    #[inline(always)]
    pub const fn tstargt(&self) -> bool {
        (self.bits & Emactstpstatus::TSTARGT_MASK) != 0
    }

    /// Read **TSSOVF** `[0:0]`
    ///
    /// TSSOVF
    #[inline(always)]
    pub const fn tssovf(&self) -> bool {
        (self.bits & Emactstpstatus::TSSOVF_MASK) != 0
    }

    /// Set **TSTRGTERR** `[3:3]`
    ///
    /// TSTRGTERR
    #[inline(always)]
    pub const fn with_tstrgterr(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpstatus::TSTRGTERR_MASK
            } else {
                self.bits & !Emactstpstatus::TSTRGTERR_MASK
            },
        }
    }

    /// Set **TSTARGT** `[1:1]`
    ///
    /// TSTARGT
    #[inline(always)]
    pub const fn with_tstargt(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpstatus::TSTARGT_MASK
            } else {
                self.bits & !Emactstpstatus::TSTARGT_MASK
            },
        }
    }

    /// Set **TSSOVF** `[0:0]`
    ///
    /// TSSOVF
    #[inline(always)]
    pub const fn with_tssovf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Emactstpstatus::TSSOVF_MASK
            } else {
                self.bits & !Emactstpstatus::TSSOVF_MASK
            },
        }
    }
}

impl Default for EmactstpstatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Emactstpstatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> EmactstpstatusVal {
        EmactstpstatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: EmactstpstatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(EmactstpstatusVal) -> EmactstpstatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMABUSMODE Register
///
/// DMABUSMODE register
///
/// Address offset: 0x1000
/// Reset value: 0x00020101
///
/// ## Fields
///
/// - **DMAREBUILDINCRBURST** `[31]` (RW)
///
///   DMAREBUILDINCRBURST
///
/// - **DMATRANSMITPRIORITY** `[27]` (RW)
///
///   DMATRANSMITPRIORITY
///
/// - **DMAMIXEDBURST** `[26]` (RW)
///
///   DMAMIXEDBURST
///
/// - **DMAADDRALIBEA** `[25]` (RW)
///
///   DMAADDRALIBEA
///
/// - **PBLX8_MODE** `[24]` (RW)
///
///   PBLX8_MODE
///
/// - **USE_SEP_PBL** `[23]` (RW)
///
///   USE_SEP_PBL
///
/// - **RX_DMA_PBL** `[22:17]` (RW)
///
///   RX_DMA_PBL
///
/// - **FIXED_BURST** `[16]` (RW)
///
///   FIXED_BURST
///
/// - **PRI_RATIO** `[15:14]` (RW)
///
///   PRI_RATIO
///
/// - **PROG_BURST_LEN** `[13:8]` (RW)
///
///   PROG_BURST_LEN
///
/// - **ALT_DESC_SIZE** `[7]` (RW)
///
///   ALT_DESC_SIZE
///
/// - **DESC_SKIP_LEN** `[6:2]` (RW)
///
///   DESC_SKIP_LEN
///
/// - **DMA_ARB_SCH** `[1]` (RW)
///
///   DMA_ARB_SCH
///
/// - **SW_RST** `[0]` (RW)
///
///   SW_RST
#[repr(transparent)]
pub struct Dmabusmode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmabusmode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmabusmode {
    // ======== Field Constants ========

    /// **DMAREBUILDINCRBURST**
    ///
    /// DMAREBUILDINCRBURST
    ///
    /// Bits `[31:31]`, mask `0x80000000`
    pub const DMAREBUILDINCRBURST_OFFSET: u32 = 31;
    pub const DMAREBUILDINCRBURST_MASK: u32 = 0x80000000;
    pub const DMAREBUILDINCRBURST: u32 = 1 << 31;

    /// **DMATRANSMITPRIORITY**
    ///
    /// DMATRANSMITPRIORITY
    ///
    /// Bits `[27:27]`, mask `0x08000000`
    pub const DMATRANSMITPRIORITY_OFFSET: u32 = 27;
    pub const DMATRANSMITPRIORITY_MASK: u32 = 0x08000000;
    pub const DMATRANSMITPRIORITY: u32 = 1 << 27;

    /// **DMAMIXEDBURST**
    ///
    /// DMAMIXEDBURST
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const DMAMIXEDBURST_OFFSET: u32 = 26;
    pub const DMAMIXEDBURST_MASK: u32 = 0x04000000;
    pub const DMAMIXEDBURST: u32 = 1 << 26;

    /// **DMAADDRALIBEA**
    ///
    /// DMAADDRALIBEA
    ///
    /// Bits `[25:25]`, mask `0x02000000`
    pub const DMAADDRALIBEA_OFFSET: u32 = 25;
    pub const DMAADDRALIBEA_MASK: u32 = 0x02000000;
    pub const DMAADDRALIBEA: u32 = 1 << 25;

    /// **PBLX8_MODE**
    ///
    /// PBLX8_MODE
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const PBLX8_MODE_OFFSET: u32 = 24;
    pub const PBLX8_MODE_MASK: u32 = 0x01000000;
    pub const PBLX8_MODE: u32 = 1 << 24;

    /// **USE_SEP_PBL**
    ///
    /// USE_SEP_PBL
    ///
    /// Bits `[23:23]`, mask `0x00800000`
    pub const USE_SEP_PBL_OFFSET: u32 = 23;
    pub const USE_SEP_PBL_MASK: u32 = 0x00800000;
    pub const USE_SEP_PBL: u32 = 1 << 23;

    /// **RX_DMA_PBL**
    ///
    /// RX_DMA_PBL
    ///
    /// Bits `[22:17]`, mask `0x007e0000`
    pub const RX_DMA_PBL_OFFSET: u32 = 17;
    pub const RX_DMA_PBL_MASK: u32 = 0x007e0000;

    /// **FIXED_BURST**
    ///
    /// FIXED_BURST
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const FIXED_BURST_OFFSET: u32 = 16;
    pub const FIXED_BURST_MASK: u32 = 0x00010000;
    pub const FIXED_BURST: u32 = 1 << 16;

    /// **PRI_RATIO**
    ///
    /// PRI_RATIO
    ///
    /// Bits `[15:14]`, mask `0x0000c000`
    pub const PRI_RATIO_OFFSET: u32 = 14;
    pub const PRI_RATIO_MASK: u32 = 0x0000c000;

    /// **PROG_BURST_LEN**
    ///
    /// PROG_BURST_LEN
    ///
    /// Bits `[13:8]`, mask `0x00003f00`
    pub const PROG_BURST_LEN_OFFSET: u32 = 8;
    pub const PROG_BURST_LEN_MASK: u32 = 0x00003f00;

    /// **ALT_DESC_SIZE**
    ///
    /// ALT_DESC_SIZE
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const ALT_DESC_SIZE_OFFSET: u32 = 7;
    pub const ALT_DESC_SIZE_MASK: u32 = 0x00000080;
    pub const ALT_DESC_SIZE: u32 = 1 << 7;

    /// **DESC_SKIP_LEN**
    ///
    /// DESC_SKIP_LEN
    ///
    /// Bits `[6:2]`, mask `0x0000007c`
    pub const DESC_SKIP_LEN_OFFSET: u32 = 2;
    pub const DESC_SKIP_LEN_MASK: u32 = 0x0000007c;

    /// **DMA_ARB_SCH**
    ///
    /// DMA_ARB_SCH
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DMA_ARB_SCH_OFFSET: u32 = 1;
    pub const DMA_ARB_SCH_MASK: u32 = 0x00000002;
    pub const DMA_ARB_SCH: u32 = 1 << 1;

    /// **SW_RST**
    ///
    /// SW_RST
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const SW_RST_OFFSET: u32 = 0;
    pub const SW_RST_MASK: u32 = 0x00000001;
    pub const SW_RST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMAREBUILDINCRBURST** `[31]` (RW)
    ///
    /// DMAREBUILDINCRBURST
    #[inline(always)]
    pub fn dmarebuildincrburst(&self) -> bool {
        (self.value.read_bits() & Self::DMAREBUILDINCRBURST_MASK) != 0
    }

    /// Set DMAREBUILDINCRBURST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmarebuildincrburst(&self) {
        self.value
            .modify_bits(|r| r | Self::DMAREBUILDINCRBURST_MASK);
    }

    /// Clear DMAREBUILDINCRBURST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmarebuildincrburst(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMAREBUILDINCRBURST_MASK);
    }

    /// **DMATRANSMITPRIORITY** `[27]` (RW)
    ///
    /// DMATRANSMITPRIORITY
    #[inline(always)]
    pub fn dmatransmitpriority(&self) -> bool {
        (self.value.read_bits() & Self::DMATRANSMITPRIORITY_MASK) != 0
    }

    /// Set DMATRANSMITPRIORITY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmatransmitpriority(&self) {
        self.value
            .modify_bits(|r| r | Self::DMATRANSMITPRIORITY_MASK);
    }

    /// Clear DMATRANSMITPRIORITY bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmatransmitpriority(&self) {
        self.value
            .modify_bits(|r| r & !Self::DMATRANSMITPRIORITY_MASK);
    }

    /// **DMAMIXEDBURST** `[26]` (RW)
    ///
    /// DMAMIXEDBURST
    #[inline(always)]
    pub fn dmamixedburst(&self) -> bool {
        (self.value.read_bits() & Self::DMAMIXEDBURST_MASK) != 0
    }

    /// Set DMAMIXEDBURST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmamixedburst(&self) {
        self.value.modify_bits(|r| r | Self::DMAMIXEDBURST_MASK);
    }

    /// Clear DMAMIXEDBURST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmamixedburst(&self) {
        self.value.modify_bits(|r| r & !Self::DMAMIXEDBURST_MASK);
    }

    /// **DMAADDRALIBEA** `[25]` (RW)
    ///
    /// DMAADDRALIBEA
    #[inline(always)]
    pub fn dmaaddralibea(&self) -> bool {
        (self.value.read_bits() & Self::DMAADDRALIBEA_MASK) != 0
    }

    /// Set DMAADDRALIBEA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmaaddralibea(&self) {
        self.value.modify_bits(|r| r | Self::DMAADDRALIBEA_MASK);
    }

    /// Clear DMAADDRALIBEA bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmaaddralibea(&self) {
        self.value.modify_bits(|r| r & !Self::DMAADDRALIBEA_MASK);
    }

    /// **PBLX8_MODE** `[24]` (RW)
    ///
    /// PBLX8_MODE
    #[inline(always)]
    pub fn pblx8_mode(&self) -> bool {
        (self.value.read_bits() & Self::PBLX8_MODE_MASK) != 0
    }

    /// Set PBLX8_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_pblx8_mode(&self) {
        self.value.modify_bits(|r| r | Self::PBLX8_MODE_MASK);
    }

    /// Clear PBLX8_MODE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_pblx8_mode(&self) {
        self.value.modify_bits(|r| r & !Self::PBLX8_MODE_MASK);
    }

    /// **USE_SEP_PBL** `[23]` (RW)
    ///
    /// USE_SEP_PBL
    #[inline(always)]
    pub fn use_sep_pbl(&self) -> bool {
        (self.value.read_bits() & Self::USE_SEP_PBL_MASK) != 0
    }

    /// Set USE_SEP_PBL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_use_sep_pbl(&self) {
        self.value.modify_bits(|r| r | Self::USE_SEP_PBL_MASK);
    }

    /// Clear USE_SEP_PBL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_use_sep_pbl(&self) {
        self.value.modify_bits(|r| r & !Self::USE_SEP_PBL_MASK);
    }

    /// **RX_DMA_PBL** `[22:17]` (RW)
    ///
    /// RX_DMA_PBL
    #[inline(always)]
    pub fn rx_dma_pbl(&self) -> u32 {
        (self.value.read_bits() & Self::RX_DMA_PBL_MASK) >> Self::RX_DMA_PBL_OFFSET
    }

    /// Write **RX_DMA_PBL** `[22:17]` (RW)
    ///
    /// RX_DMA_PBL
    #[inline(always)]
    pub fn write_rx_dma_pbl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_DMA_PBL_MASK)
                | ((value << Self::RX_DMA_PBL_OFFSET) & Self::RX_DMA_PBL_MASK)
        });
    }

    /// **FIXED_BURST** `[16]` (RW)
    ///
    /// FIXED_BURST
    #[inline(always)]
    pub fn fixed_burst(&self) -> bool {
        (self.value.read_bits() & Self::FIXED_BURST_MASK) != 0
    }

    /// Set FIXED_BURST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fixed_burst(&self) {
        self.value.modify_bits(|r| r | Self::FIXED_BURST_MASK);
    }

    /// Clear FIXED_BURST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fixed_burst(&self) {
        self.value.modify_bits(|r| r & !Self::FIXED_BURST_MASK);
    }

    /// **PRI_RATIO** `[15:14]` (RW)
    ///
    /// PRI_RATIO
    #[inline(always)]
    pub fn pri_ratio(&self) -> u32 {
        (self.value.read_bits() & Self::PRI_RATIO_MASK) >> Self::PRI_RATIO_OFFSET
    }

    /// Write **PRI_RATIO** `[15:14]` (RW)
    ///
    /// PRI_RATIO
    #[inline(always)]
    pub fn write_pri_ratio(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PRI_RATIO_MASK) | ((value << Self::PRI_RATIO_OFFSET) & Self::PRI_RATIO_MASK)
        });
    }

    /// **PROG_BURST_LEN** `[13:8]` (RW)
    ///
    /// PROG_BURST_LEN
    #[inline(always)]
    pub fn prog_burst_len(&self) -> u32 {
        (self.value.read_bits() & Self::PROG_BURST_LEN_MASK) >> Self::PROG_BURST_LEN_OFFSET
    }

    /// Write **PROG_BURST_LEN** `[13:8]` (RW)
    ///
    /// PROG_BURST_LEN
    #[inline(always)]
    pub fn write_prog_burst_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::PROG_BURST_LEN_MASK)
                | ((value << Self::PROG_BURST_LEN_OFFSET) & Self::PROG_BURST_LEN_MASK)
        });
    }

    /// **ALT_DESC_SIZE** `[7]` (RW)
    ///
    /// ALT_DESC_SIZE
    #[inline(always)]
    pub fn alt_desc_size(&self) -> bool {
        (self.value.read_bits() & Self::ALT_DESC_SIZE_MASK) != 0
    }

    /// Set ALT_DESC_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_alt_desc_size(&self) {
        self.value.modify_bits(|r| r | Self::ALT_DESC_SIZE_MASK);
    }

    /// Clear ALT_DESC_SIZE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_alt_desc_size(&self) {
        self.value.modify_bits(|r| r & !Self::ALT_DESC_SIZE_MASK);
    }

    /// **DESC_SKIP_LEN** `[6:2]` (RW)
    ///
    /// DESC_SKIP_LEN
    #[inline(always)]
    pub fn desc_skip_len(&self) -> u32 {
        (self.value.read_bits() & Self::DESC_SKIP_LEN_MASK) >> Self::DESC_SKIP_LEN_OFFSET
    }

    /// Write **DESC_SKIP_LEN** `[6:2]` (RW)
    ///
    /// DESC_SKIP_LEN
    #[inline(always)]
    pub fn write_desc_skip_len(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::DESC_SKIP_LEN_MASK)
                | ((value << Self::DESC_SKIP_LEN_OFFSET) & Self::DESC_SKIP_LEN_MASK)
        });
    }

    /// **DMA_ARB_SCH** `[1]` (RW)
    ///
    /// DMA_ARB_SCH
    #[inline(always)]
    pub fn dma_arb_sch(&self) -> bool {
        (self.value.read_bits() & Self::DMA_ARB_SCH_MASK) != 0
    }

    /// Set DMA_ARB_SCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dma_arb_sch(&self) {
        self.value.modify_bits(|r| r | Self::DMA_ARB_SCH_MASK);
    }

    /// Clear DMA_ARB_SCH bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dma_arb_sch(&self) {
        self.value.modify_bits(|r| r & !Self::DMA_ARB_SCH_MASK);
    }

    /// **SW_RST** `[0]` (RW)
    ///
    /// SW_RST
    #[inline(always)]
    pub fn sw_rst(&self) -> bool {
        (self.value.read_bits() & Self::SW_RST_MASK) != 0
    }

    /// Set SW_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_sw_rst(&self) {
        self.value.modify_bits(|r| r | Self::SW_RST_MASK);
    }

    /// Clear SW_RST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_sw_rst(&self) {
        self.value.modify_bits(|r| r & !Self::SW_RST_MASK);
    }
}

/// DMABUSMODE value type for batch read/modify/write operations
///
/// Use `dmabusmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmabusmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmabusmodeVal {
    bits: u32,
}

impl DmabusmodeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00020101 }
    }

    /// Read **DMAREBUILDINCRBURST** `[31:31]`
    ///
    /// DMAREBUILDINCRBURST
    #[inline(always)]
    pub const fn dmarebuildincrburst(&self) -> bool {
        (self.bits & Dmabusmode::DMAREBUILDINCRBURST_MASK) != 0
    }

    /// Read **DMATRANSMITPRIORITY** `[27:27]`
    ///
    /// DMATRANSMITPRIORITY
    #[inline(always)]
    pub const fn dmatransmitpriority(&self) -> bool {
        (self.bits & Dmabusmode::DMATRANSMITPRIORITY_MASK) != 0
    }

    /// Read **DMAMIXEDBURST** `[26:26]`
    ///
    /// DMAMIXEDBURST
    #[inline(always)]
    pub const fn dmamixedburst(&self) -> bool {
        (self.bits & Dmabusmode::DMAMIXEDBURST_MASK) != 0
    }

    /// Read **DMAADDRALIBEA** `[25:25]`
    ///
    /// DMAADDRALIBEA
    #[inline(always)]
    pub const fn dmaaddralibea(&self) -> bool {
        (self.bits & Dmabusmode::DMAADDRALIBEA_MASK) != 0
    }

    /// Read **PBLX8_MODE** `[24:24]`
    ///
    /// PBLX8_MODE
    #[inline(always)]
    pub const fn pblx8_mode(&self) -> bool {
        (self.bits & Dmabusmode::PBLX8_MODE_MASK) != 0
    }

    /// Read **USE_SEP_PBL** `[23:23]`
    ///
    /// USE_SEP_PBL
    #[inline(always)]
    pub const fn use_sep_pbl(&self) -> bool {
        (self.bits & Dmabusmode::USE_SEP_PBL_MASK) != 0
    }

    /// Read **RX_DMA_PBL** `[22:17]`
    ///
    /// RX_DMA_PBL
    #[inline(always)]
    pub const fn rx_dma_pbl(&self) -> u32 {
        (self.bits & Dmabusmode::RX_DMA_PBL_MASK) >> Dmabusmode::RX_DMA_PBL_OFFSET
    }

    /// Read **FIXED_BURST** `[16:16]`
    ///
    /// FIXED_BURST
    #[inline(always)]
    pub const fn fixed_burst(&self) -> bool {
        (self.bits & Dmabusmode::FIXED_BURST_MASK) != 0
    }

    /// Read **PRI_RATIO** `[15:14]`
    ///
    /// PRI_RATIO
    #[inline(always)]
    pub const fn pri_ratio(&self) -> u32 {
        (self.bits & Dmabusmode::PRI_RATIO_MASK) >> Dmabusmode::PRI_RATIO_OFFSET
    }

    /// Read **PROG_BURST_LEN** `[13:8]`
    ///
    /// PROG_BURST_LEN
    #[inline(always)]
    pub const fn prog_burst_len(&self) -> u32 {
        (self.bits & Dmabusmode::PROG_BURST_LEN_MASK) >> Dmabusmode::PROG_BURST_LEN_OFFSET
    }

    /// Read **ALT_DESC_SIZE** `[7:7]`
    ///
    /// ALT_DESC_SIZE
    #[inline(always)]
    pub const fn alt_desc_size(&self) -> bool {
        (self.bits & Dmabusmode::ALT_DESC_SIZE_MASK) != 0
    }

    /// Read **DESC_SKIP_LEN** `[6:2]`
    ///
    /// DESC_SKIP_LEN
    #[inline(always)]
    pub const fn desc_skip_len(&self) -> u32 {
        (self.bits & Dmabusmode::DESC_SKIP_LEN_MASK) >> Dmabusmode::DESC_SKIP_LEN_OFFSET
    }

    /// Read **DMA_ARB_SCH** `[1:1]`
    ///
    /// DMA_ARB_SCH
    #[inline(always)]
    pub const fn dma_arb_sch(&self) -> bool {
        (self.bits & Dmabusmode::DMA_ARB_SCH_MASK) != 0
    }

    /// Read **SW_RST** `[0:0]`
    ///
    /// SW_RST
    #[inline(always)]
    pub const fn sw_rst(&self) -> bool {
        (self.bits & Dmabusmode::SW_RST_MASK) != 0
    }

    /// Set **DMAREBUILDINCRBURST** `[31:31]`
    ///
    /// DMAREBUILDINCRBURST
    #[inline(always)]
    pub const fn with_dmarebuildincrburst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::DMAREBUILDINCRBURST_MASK
            } else {
                self.bits & !Dmabusmode::DMAREBUILDINCRBURST_MASK
            },
        }
    }

    /// Set **DMATRANSMITPRIORITY** `[27:27]`
    ///
    /// DMATRANSMITPRIORITY
    #[inline(always)]
    pub const fn with_dmatransmitpriority(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::DMATRANSMITPRIORITY_MASK
            } else {
                self.bits & !Dmabusmode::DMATRANSMITPRIORITY_MASK
            },
        }
    }

    /// Set **DMAMIXEDBURST** `[26:26]`
    ///
    /// DMAMIXEDBURST
    #[inline(always)]
    pub const fn with_dmamixedburst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::DMAMIXEDBURST_MASK
            } else {
                self.bits & !Dmabusmode::DMAMIXEDBURST_MASK
            },
        }
    }

    /// Set **DMAADDRALIBEA** `[25:25]`
    ///
    /// DMAADDRALIBEA
    #[inline(always)]
    pub const fn with_dmaaddralibea(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::DMAADDRALIBEA_MASK
            } else {
                self.bits & !Dmabusmode::DMAADDRALIBEA_MASK
            },
        }
    }

    /// Set **PBLX8_MODE** `[24:24]`
    ///
    /// PBLX8_MODE
    #[inline(always)]
    pub const fn with_pblx8_mode(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::PBLX8_MODE_MASK
            } else {
                self.bits & !Dmabusmode::PBLX8_MODE_MASK
            },
        }
    }

    /// Set **USE_SEP_PBL** `[23:23]`
    ///
    /// USE_SEP_PBL
    #[inline(always)]
    pub const fn with_use_sep_pbl(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::USE_SEP_PBL_MASK
            } else {
                self.bits & !Dmabusmode::USE_SEP_PBL_MASK
            },
        }
    }

    /// Set **RX_DMA_PBL** `[22:17]`
    ///
    /// RX_DMA_PBL
    #[inline(always)]
    pub const fn with_rx_dma_pbl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmabusmode::RX_DMA_PBL_MASK)
                | ((value << Dmabusmode::RX_DMA_PBL_OFFSET) & Dmabusmode::RX_DMA_PBL_MASK),
        }
    }

    /// Set **FIXED_BURST** `[16:16]`
    ///
    /// FIXED_BURST
    #[inline(always)]
    pub const fn with_fixed_burst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::FIXED_BURST_MASK
            } else {
                self.bits & !Dmabusmode::FIXED_BURST_MASK
            },
        }
    }

    /// Set **PRI_RATIO** `[15:14]`
    ///
    /// PRI_RATIO
    #[inline(always)]
    pub const fn with_pri_ratio(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmabusmode::PRI_RATIO_MASK)
                | ((value << Dmabusmode::PRI_RATIO_OFFSET) & Dmabusmode::PRI_RATIO_MASK),
        }
    }

    /// Set **PROG_BURST_LEN** `[13:8]`
    ///
    /// PROG_BURST_LEN
    #[inline(always)]
    pub const fn with_prog_burst_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmabusmode::PROG_BURST_LEN_MASK)
                | ((value << Dmabusmode::PROG_BURST_LEN_OFFSET) & Dmabusmode::PROG_BURST_LEN_MASK),
        }
    }

    /// Set **ALT_DESC_SIZE** `[7:7]`
    ///
    /// ALT_DESC_SIZE
    #[inline(always)]
    pub const fn with_alt_desc_size(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::ALT_DESC_SIZE_MASK
            } else {
                self.bits & !Dmabusmode::ALT_DESC_SIZE_MASK
            },
        }
    }

    /// Set **DESC_SKIP_LEN** `[6:2]`
    ///
    /// DESC_SKIP_LEN
    #[inline(always)]
    pub const fn with_desc_skip_len(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmabusmode::DESC_SKIP_LEN_MASK)
                | ((value << Dmabusmode::DESC_SKIP_LEN_OFFSET) & Dmabusmode::DESC_SKIP_LEN_MASK),
        }
    }

    /// Set **DMA_ARB_SCH** `[1:1]`
    ///
    /// DMA_ARB_SCH
    #[inline(always)]
    pub const fn with_dma_arb_sch(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::DMA_ARB_SCH_MASK
            } else {
                self.bits & !Dmabusmode::DMA_ARB_SCH_MASK
            },
        }
    }

    /// Set **SW_RST** `[0:0]`
    ///
    /// SW_RST
    #[inline(always)]
    pub const fn with_sw_rst(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmabusmode::SW_RST_MASK
            } else {
                self.bits & !Dmabusmode::SW_RST_MASK
            },
        }
    }
}

impl Default for DmabusmodeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmabusmode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmabusmodeVal {
        DmabusmodeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmabusmodeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmabusmodeVal) -> DmabusmodeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMATXPOLLDEMAND Register
///
/// DMATXPOLLDEMAND register
///
/// Address offset: 0x1004
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **TRANS_POLL_DEMAND** `[31:0]` (RW)
///
///   TRANS_POLL_DEMAND
#[repr(transparent)]
pub struct Dmatxpolldemand {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmatxpolldemand {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmatxpolldemand {
    // ======== Field Constants ========

    /// **TRANS_POLL_DEMAND**
    ///
    /// TRANS_POLL_DEMAND
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const TRANS_POLL_DEMAND_OFFSET: u32 = 0;
    pub const TRANS_POLL_DEMAND_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **TRANS_POLL_DEMAND** `[31:0]` (RW)
    ///
    /// TRANS_POLL_DEMAND
    #[inline(always)]
    pub fn trans_poll_demand(&self) -> u32 {
        (self.value.read_bits() & Self::TRANS_POLL_DEMAND_MASK) >> Self::TRANS_POLL_DEMAND_OFFSET
    }

    /// Write **TRANS_POLL_DEMAND** `[31:0]` (RW)
    ///
    /// TRANS_POLL_DEMAND
    #[inline(always)]
    pub fn write_trans_poll_demand(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TRANS_POLL_DEMAND_MASK)
                | ((value << Self::TRANS_POLL_DEMAND_OFFSET) & Self::TRANS_POLL_DEMAND_MASK)
        });
    }
}

/// DMATXPOLLDEMAND value type for batch read/modify/write operations
///
/// Use `dmatxpolldemand.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmatxpolldemand.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmatxpolldemandVal {
    bits: u32,
}

impl DmatxpolldemandVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **TRANS_POLL_DEMAND** `[31:0]`
    ///
    /// TRANS_POLL_DEMAND
    #[inline(always)]
    pub const fn trans_poll_demand(&self) -> u32 {
        (self.bits & Dmatxpolldemand::TRANS_POLL_DEMAND_MASK)
            >> Dmatxpolldemand::TRANS_POLL_DEMAND_OFFSET
    }

    /// Set **TRANS_POLL_DEMAND** `[31:0]`
    ///
    /// TRANS_POLL_DEMAND
    #[inline(always)]
    pub const fn with_trans_poll_demand(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmatxpolldemand::TRANS_POLL_DEMAND_MASK)
                | ((value << Dmatxpolldemand::TRANS_POLL_DEMAND_OFFSET)
                    & Dmatxpolldemand::TRANS_POLL_DEMAND_MASK),
        }
    }
}

impl Default for DmatxpolldemandVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmatxpolldemand {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmatxpolldemandVal {
        DmatxpolldemandVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmatxpolldemandVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmatxpolldemandVal) -> DmatxpolldemandVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMARXPOLLDEMAND Register
///
/// DMARXPOLLDEMAND register
///
/// Address offset: 0x1008
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RECV_POLL_DEMAND** `[31:0]` (RW)
///
///   RECV_POLL_DEMAND
#[repr(transparent)]
pub struct Dmarxpolldemand {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmarxpolldemand {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmarxpolldemand {
    // ======== Field Constants ========

    /// **RECV_POLL_DEMAND**
    ///
    /// RECV_POLL_DEMAND
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const RECV_POLL_DEMAND_OFFSET: u32 = 0;
    pub const RECV_POLL_DEMAND_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **RECV_POLL_DEMAND** `[31:0]` (RW)
    ///
    /// RECV_POLL_DEMAND
    #[inline(always)]
    pub fn recv_poll_demand(&self) -> u32 {
        (self.value.read_bits() & Self::RECV_POLL_DEMAND_MASK) >> Self::RECV_POLL_DEMAND_OFFSET
    }

    /// Write **RECV_POLL_DEMAND** `[31:0]` (RW)
    ///
    /// RECV_POLL_DEMAND
    #[inline(always)]
    pub fn write_recv_poll_demand(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RECV_POLL_DEMAND_MASK)
                | ((value << Self::RECV_POLL_DEMAND_OFFSET) & Self::RECV_POLL_DEMAND_MASK)
        });
    }
}

/// DMARXPOLLDEMAND value type for batch read/modify/write operations
///
/// Use `dmarxpolldemand.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmarxpolldemand.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmarxpolldemandVal {
    bits: u32,
}

impl DmarxpolldemandVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **RECV_POLL_DEMAND** `[31:0]`
    ///
    /// RECV_POLL_DEMAND
    #[inline(always)]
    pub const fn recv_poll_demand(&self) -> u32 {
        (self.bits & Dmarxpolldemand::RECV_POLL_DEMAND_MASK)
            >> Dmarxpolldemand::RECV_POLL_DEMAND_OFFSET
    }

    /// Set **RECV_POLL_DEMAND** `[31:0]`
    ///
    /// RECV_POLL_DEMAND
    #[inline(always)]
    pub const fn with_recv_poll_demand(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmarxpolldemand::RECV_POLL_DEMAND_MASK)
                | ((value << Dmarxpolldemand::RECV_POLL_DEMAND_OFFSET)
                    & Dmarxpolldemand::RECV_POLL_DEMAND_MASK),
        }
    }
}

impl Default for DmarxpolldemandVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmarxpolldemand {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmarxpolldemandVal {
        DmarxpolldemandVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmarxpolldemandVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmarxpolldemandVal) -> DmarxpolldemandVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMARXBASEADDR Register
///
/// DMARXBASEADDR register
///
/// Address offset: 0x100c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **START_RECV_LIST** `[31:0]` (RW)
///
///   START_RECV_LIST
#[repr(transparent)]
pub struct Dmarxbaseaddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmarxbaseaddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmarxbaseaddr {
    // ======== Field Constants ========

    /// **START_RECV_LIST**
    ///
    /// START_RECV_LIST
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const START_RECV_LIST_OFFSET: u32 = 0;
    pub const START_RECV_LIST_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **START_RECV_LIST** `[31:0]` (RW)
    ///
    /// START_RECV_LIST
    #[inline(always)]
    pub fn start_recv_list(&self) -> u32 {
        (self.value.read_bits() & Self::START_RECV_LIST_MASK) >> Self::START_RECV_LIST_OFFSET
    }

    /// Write **START_RECV_LIST** `[31:0]` (RW)
    ///
    /// START_RECV_LIST
    #[inline(always)]
    pub fn write_start_recv_list(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::START_RECV_LIST_MASK)
                | ((value << Self::START_RECV_LIST_OFFSET) & Self::START_RECV_LIST_MASK)
        });
    }
}

/// DMARXBASEADDR value type for batch read/modify/write operations
///
/// Use `dmarxbaseaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmarxbaseaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmarxbaseaddrVal {
    bits: u32,
}

impl DmarxbaseaddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **START_RECV_LIST** `[31:0]`
    ///
    /// START_RECV_LIST
    #[inline(always)]
    pub const fn start_recv_list(&self) -> u32 {
        (self.bits & Dmarxbaseaddr::START_RECV_LIST_MASK) >> Dmarxbaseaddr::START_RECV_LIST_OFFSET
    }

    /// Set **START_RECV_LIST** `[31:0]`
    ///
    /// START_RECV_LIST
    #[inline(always)]
    pub const fn with_start_recv_list(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmarxbaseaddr::START_RECV_LIST_MASK)
                | ((value << Dmarxbaseaddr::START_RECV_LIST_OFFSET)
                    & Dmarxbaseaddr::START_RECV_LIST_MASK),
        }
    }
}

impl Default for DmarxbaseaddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmarxbaseaddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmarxbaseaddrVal {
        DmarxbaseaddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmarxbaseaddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmarxbaseaddrVal) -> DmarxbaseaddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMATXBASEADDR Register
///
/// DMATXBASEADDR register
///
/// Address offset: 0x1010
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **START_TRANS_LIST** `[31:0]` (RW)
///
///   START_TRANS_LIST
#[repr(transparent)]
pub struct Dmatxbaseaddr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmatxbaseaddr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmatxbaseaddr {
    // ======== Field Constants ========

    /// **START_TRANS_LIST**
    ///
    /// START_TRANS_LIST
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const START_TRANS_LIST_OFFSET: u32 = 0;
    pub const START_TRANS_LIST_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **START_TRANS_LIST** `[31:0]` (RW)
    ///
    /// START_TRANS_LIST
    #[inline(always)]
    pub fn start_trans_list(&self) -> u32 {
        (self.value.read_bits() & Self::START_TRANS_LIST_MASK) >> Self::START_TRANS_LIST_OFFSET
    }

    /// Write **START_TRANS_LIST** `[31:0]` (RW)
    ///
    /// START_TRANS_LIST
    #[inline(always)]
    pub fn write_start_trans_list(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::START_TRANS_LIST_MASK)
                | ((value << Self::START_TRANS_LIST_OFFSET) & Self::START_TRANS_LIST_MASK)
        });
    }
}

/// DMATXBASEADDR value type for batch read/modify/write operations
///
/// Use `dmatxbaseaddr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmatxbaseaddr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmatxbaseaddrVal {
    bits: u32,
}

impl DmatxbaseaddrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **START_TRANS_LIST** `[31:0]`
    ///
    /// START_TRANS_LIST
    #[inline(always)]
    pub const fn start_trans_list(&self) -> u32 {
        (self.bits & Dmatxbaseaddr::START_TRANS_LIST_MASK) >> Dmatxbaseaddr::START_TRANS_LIST_OFFSET
    }

    /// Set **START_TRANS_LIST** `[31:0]`
    ///
    /// START_TRANS_LIST
    #[inline(always)]
    pub const fn with_start_trans_list(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmatxbaseaddr::START_TRANS_LIST_MASK)
                | ((value << Dmatxbaseaddr::START_TRANS_LIST_OFFSET)
                    & Dmatxbaseaddr::START_TRANS_LIST_MASK),
        }
    }
}

impl Default for DmatxbaseaddrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmatxbaseaddr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmatxbaseaddrVal {
        DmatxbaseaddrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmatxbaseaddrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmatxbaseaddrVal) -> DmatxbaseaddrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMASTATUS Register
///
/// DMASTATUS register
///
/// Address offset: 0x1014
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **EMAC_LPI_INT** `[30]` (RW)
///
///   EMAC_LPI_INT
///
/// - **TS_TRI_INT** `[29]` (RW)
///
///   TS_TRI_INT
///
/// - **EMAC_PMT_INT** `[28]` (RW)
///
///   EMAC_PMT_INT
///
/// - **ERROR_BITS** `[25:23]` (RW)
///
///   ERROR_BITS
///
/// - **TRANS_PROC_STATE** `[22:20]` (RW)
///
///   TRANS_PROC_STATE
///
/// - **RECV_PROC_STATE** `[19:17]` (RW)
///
///   RECV_PROC_STATE
///
/// - **NORM_INT_SUMM** `[16]` (RW)
///
///   NORM_INT_SUMM
///
/// - **ABN_INT_SUMM** `[15]` (RW)
///
///   ABN_INT_SUMM
///
/// - **EARLY_RECV_INT** `[14]` (RW)
///
///   EARLY_RECV_INT
///
/// - **FATAL_BUS_ERR_INT** `[13]` (RW)
///
///   FATAL_BUS_ERR_INT
///
/// - **EARLY_TRANS_INT** `[10]` (RW)
///
///   EARLY_TRANS_INT
///
/// - **RECV_WDT_TO** `[9]` (RW)
///
///   RECV_WDT_TO
///
/// - **RECV_PROC_STOP** `[8]` (RW)
///
///   RECV_PROC_STOP
///
/// - **RECV_BUF_UNAVAIL** `[7]` (RW)
///
///   RECV_BUF_UNAVAIL
///
/// - **RECV_INT** `[6]` (RW)
///
///   RECV_INT
///
/// - **TRANS_UNDFLOW** `[5]` (RW)
///
///   TRANS_UNDFLOW
///
/// - **RECV_OVFLOW** `[4]` (RW)
///
///   RECV_OVFLOW
///
/// - **TRANS_JABBER_TO** `[3]` (RW)
///
///   TRANS_JABBER_TO
///
/// - **TRANS_BUF_UNAVAIL** `[2]` (RW)
///
///   TRANS_BUF_UNAVAIL
///
/// - **TRANS_PROC_STOP** `[1]` (RW)
///
///   TRANS_PROC_STOP
///
/// - **TRANS_INT** `[0]` (RW)
///
///   TRANS_INT
#[repr(transparent)]
pub struct Dmastatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmastatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmastatus {
    // ======== Field Constants ========

    /// **EMAC_LPI_INT**
    ///
    /// EMAC_LPI_INT
    ///
    /// Bits `[30:30]`, mask `0x40000000`
    pub const EMAC_LPI_INT_OFFSET: u32 = 30;
    pub const EMAC_LPI_INT_MASK: u32 = 0x40000000;
    pub const EMAC_LPI_INT: u32 = 1 << 30;

    /// **TS_TRI_INT**
    ///
    /// TS_TRI_INT
    ///
    /// Bits `[29:29]`, mask `0x20000000`
    pub const TS_TRI_INT_OFFSET: u32 = 29;
    pub const TS_TRI_INT_MASK: u32 = 0x20000000;
    pub const TS_TRI_INT: u32 = 1 << 29;

    /// **EMAC_PMT_INT**
    ///
    /// EMAC_PMT_INT
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const EMAC_PMT_INT_OFFSET: u32 = 28;
    pub const EMAC_PMT_INT_MASK: u32 = 0x10000000;
    pub const EMAC_PMT_INT: u32 = 1 << 28;

    /// **ERROR_BITS**
    ///
    /// ERROR_BITS
    ///
    /// Bits `[25:23]`, mask `0x03800000`
    pub const ERROR_BITS_OFFSET: u32 = 23;
    pub const ERROR_BITS_MASK: u32 = 0x03800000;

    /// **TRANS_PROC_STATE**
    ///
    /// TRANS_PROC_STATE
    ///
    /// Bits `[22:20]`, mask `0x00700000`
    pub const TRANS_PROC_STATE_OFFSET: u32 = 20;
    pub const TRANS_PROC_STATE_MASK: u32 = 0x00700000;

    /// **RECV_PROC_STATE**
    ///
    /// RECV_PROC_STATE
    ///
    /// Bits `[19:17]`, mask `0x000e0000`
    pub const RECV_PROC_STATE_OFFSET: u32 = 17;
    pub const RECV_PROC_STATE_MASK: u32 = 0x000e0000;

    /// **NORM_INT_SUMM**
    ///
    /// NORM_INT_SUMM
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const NORM_INT_SUMM_OFFSET: u32 = 16;
    pub const NORM_INT_SUMM_MASK: u32 = 0x00010000;
    pub const NORM_INT_SUMM: u32 = 1 << 16;

    /// **ABN_INT_SUMM**
    ///
    /// ABN_INT_SUMM
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const ABN_INT_SUMM_OFFSET: u32 = 15;
    pub const ABN_INT_SUMM_MASK: u32 = 0x00008000;
    pub const ABN_INT_SUMM: u32 = 1 << 15;

    /// **EARLY_RECV_INT**
    ///
    /// EARLY_RECV_INT
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const EARLY_RECV_INT_OFFSET: u32 = 14;
    pub const EARLY_RECV_INT_MASK: u32 = 0x00004000;
    pub const EARLY_RECV_INT: u32 = 1 << 14;

    /// **FATAL_BUS_ERR_INT**
    ///
    /// FATAL_BUS_ERR_INT
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const FATAL_BUS_ERR_INT_OFFSET: u32 = 13;
    pub const FATAL_BUS_ERR_INT_MASK: u32 = 0x00002000;
    pub const FATAL_BUS_ERR_INT: u32 = 1 << 13;

    /// **EARLY_TRANS_INT**
    ///
    /// EARLY_TRANS_INT
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const EARLY_TRANS_INT_OFFSET: u32 = 10;
    pub const EARLY_TRANS_INT_MASK: u32 = 0x00000400;
    pub const EARLY_TRANS_INT: u32 = 1 << 10;

    /// **RECV_WDT_TO**
    ///
    /// RECV_WDT_TO
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const RECV_WDT_TO_OFFSET: u32 = 9;
    pub const RECV_WDT_TO_MASK: u32 = 0x00000200;
    pub const RECV_WDT_TO: u32 = 1 << 9;

    /// **RECV_PROC_STOP**
    ///
    /// RECV_PROC_STOP
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const RECV_PROC_STOP_OFFSET: u32 = 8;
    pub const RECV_PROC_STOP_MASK: u32 = 0x00000100;
    pub const RECV_PROC_STOP: u32 = 1 << 8;

    /// **RECV_BUF_UNAVAIL**
    ///
    /// RECV_BUF_UNAVAIL
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const RECV_BUF_UNAVAIL_OFFSET: u32 = 7;
    pub const RECV_BUF_UNAVAIL_MASK: u32 = 0x00000080;
    pub const RECV_BUF_UNAVAIL: u32 = 1 << 7;

    /// **RECV_INT**
    ///
    /// RECV_INT
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const RECV_INT_OFFSET: u32 = 6;
    pub const RECV_INT_MASK: u32 = 0x00000040;
    pub const RECV_INT: u32 = 1 << 6;

    /// **TRANS_UNDFLOW**
    ///
    /// TRANS_UNDFLOW
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const TRANS_UNDFLOW_OFFSET: u32 = 5;
    pub const TRANS_UNDFLOW_MASK: u32 = 0x00000020;
    pub const TRANS_UNDFLOW: u32 = 1 << 5;

    /// **RECV_OVFLOW**
    ///
    /// RECV_OVFLOW
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const RECV_OVFLOW_OFFSET: u32 = 4;
    pub const RECV_OVFLOW_MASK: u32 = 0x00000010;
    pub const RECV_OVFLOW: u32 = 1 << 4;

    /// **TRANS_JABBER_TO**
    ///
    /// TRANS_JABBER_TO
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const TRANS_JABBER_TO_OFFSET: u32 = 3;
    pub const TRANS_JABBER_TO_MASK: u32 = 0x00000008;
    pub const TRANS_JABBER_TO: u32 = 1 << 3;

    /// **TRANS_BUF_UNAVAIL**
    ///
    /// TRANS_BUF_UNAVAIL
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const TRANS_BUF_UNAVAIL_OFFSET: u32 = 2;
    pub const TRANS_BUF_UNAVAIL_MASK: u32 = 0x00000004;
    pub const TRANS_BUF_UNAVAIL: u32 = 1 << 2;

    /// **TRANS_PROC_STOP**
    ///
    /// TRANS_PROC_STOP
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const TRANS_PROC_STOP_OFFSET: u32 = 1;
    pub const TRANS_PROC_STOP_MASK: u32 = 0x00000002;
    pub const TRANS_PROC_STOP: u32 = 1 << 1;

    /// **TRANS_INT**
    ///
    /// TRANS_INT
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const TRANS_INT_OFFSET: u32 = 0;
    pub const TRANS_INT_MASK: u32 = 0x00000001;
    pub const TRANS_INT: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **EMAC_LPI_INT** `[30]` (RW)
    ///
    /// EMAC_LPI_INT
    #[inline(always)]
    pub fn emac_lpi_int(&self) -> bool {
        (self.value.read_bits() & Self::EMAC_LPI_INT_MASK) != 0
    }

    /// Set EMAC_LPI_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emac_lpi_int(&self) {
        self.value.modify_bits(|r| r | Self::EMAC_LPI_INT_MASK);
    }

    /// Clear EMAC_LPI_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emac_lpi_int(&self) {
        self.value.modify_bits(|r| r & !Self::EMAC_LPI_INT_MASK);
    }

    /// **TS_TRI_INT** `[29]` (RW)
    ///
    /// TS_TRI_INT
    #[inline(always)]
    pub fn ts_tri_int(&self) -> bool {
        (self.value.read_bits() & Self::TS_TRI_INT_MASK) != 0
    }

    /// Set TS_TRI_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ts_tri_int(&self) {
        self.value.modify_bits(|r| r | Self::TS_TRI_INT_MASK);
    }

    /// Clear TS_TRI_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ts_tri_int(&self) {
        self.value.modify_bits(|r| r & !Self::TS_TRI_INT_MASK);
    }

    /// **EMAC_PMT_INT** `[28]` (RW)
    ///
    /// EMAC_PMT_INT
    #[inline(always)]
    pub fn emac_pmt_int(&self) -> bool {
        (self.value.read_bits() & Self::EMAC_PMT_INT_MASK) != 0
    }

    /// Set EMAC_PMT_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_emac_pmt_int(&self) {
        self.value.modify_bits(|r| r | Self::EMAC_PMT_INT_MASK);
    }

    /// Clear EMAC_PMT_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_emac_pmt_int(&self) {
        self.value.modify_bits(|r| r & !Self::EMAC_PMT_INT_MASK);
    }

    /// **ERROR_BITS** `[25:23]` (RW)
    ///
    /// ERROR_BITS
    #[inline(always)]
    pub fn error_bits(&self) -> u32 {
        (self.value.read_bits() & Self::ERROR_BITS_MASK) >> Self::ERROR_BITS_OFFSET
    }

    /// Write **ERROR_BITS** `[25:23]` (RW)
    ///
    /// ERROR_BITS
    #[inline(always)]
    pub fn write_error_bits(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ERROR_BITS_MASK)
                | ((value << Self::ERROR_BITS_OFFSET) & Self::ERROR_BITS_MASK)
        });
    }

    /// **TRANS_PROC_STATE** `[22:20]` (RW)
    ///
    /// TRANS_PROC_STATE
    #[inline(always)]
    pub fn trans_proc_state(&self) -> u32 {
        (self.value.read_bits() & Self::TRANS_PROC_STATE_MASK) >> Self::TRANS_PROC_STATE_OFFSET
    }

    /// Write **TRANS_PROC_STATE** `[22:20]` (RW)
    ///
    /// TRANS_PROC_STATE
    #[inline(always)]
    pub fn write_trans_proc_state(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TRANS_PROC_STATE_MASK)
                | ((value << Self::TRANS_PROC_STATE_OFFSET) & Self::TRANS_PROC_STATE_MASK)
        });
    }

    /// **RECV_PROC_STATE** `[19:17]` (RW)
    ///
    /// RECV_PROC_STATE
    #[inline(always)]
    pub fn recv_proc_state(&self) -> u32 {
        (self.value.read_bits() & Self::RECV_PROC_STATE_MASK) >> Self::RECV_PROC_STATE_OFFSET
    }

    /// Write **RECV_PROC_STATE** `[19:17]` (RW)
    ///
    /// RECV_PROC_STATE
    #[inline(always)]
    pub fn write_recv_proc_state(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RECV_PROC_STATE_MASK)
                | ((value << Self::RECV_PROC_STATE_OFFSET) & Self::RECV_PROC_STATE_MASK)
        });
    }

    /// **NORM_INT_SUMM** `[16]` (RW)
    ///
    /// NORM_INT_SUMM
    #[inline(always)]
    pub fn norm_int_summ(&self) -> bool {
        (self.value.read_bits() & Self::NORM_INT_SUMM_MASK) != 0
    }

    /// Set NORM_INT_SUMM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_norm_int_summ(&self) {
        self.value.modify_bits(|r| r | Self::NORM_INT_SUMM_MASK);
    }

    /// Clear NORM_INT_SUMM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_norm_int_summ(&self) {
        self.value.modify_bits(|r| r & !Self::NORM_INT_SUMM_MASK);
    }

    /// **ABN_INT_SUMM** `[15]` (RW)
    ///
    /// ABN_INT_SUMM
    #[inline(always)]
    pub fn abn_int_summ(&self) -> bool {
        (self.value.read_bits() & Self::ABN_INT_SUMM_MASK) != 0
    }

    /// Set ABN_INT_SUMM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_abn_int_summ(&self) {
        self.value.modify_bits(|r| r | Self::ABN_INT_SUMM_MASK);
    }

    /// Clear ABN_INT_SUMM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_abn_int_summ(&self) {
        self.value.modify_bits(|r| r & !Self::ABN_INT_SUMM_MASK);
    }

    /// **EARLY_RECV_INT** `[14]` (RW)
    ///
    /// EARLY_RECV_INT
    #[inline(always)]
    pub fn early_recv_int(&self) -> bool {
        (self.value.read_bits() & Self::EARLY_RECV_INT_MASK) != 0
    }

    /// Set EARLY_RECV_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_early_recv_int(&self) {
        self.value.modify_bits(|r| r | Self::EARLY_RECV_INT_MASK);
    }

    /// Clear EARLY_RECV_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_early_recv_int(&self) {
        self.value.modify_bits(|r| r & !Self::EARLY_RECV_INT_MASK);
    }

    /// **FATAL_BUS_ERR_INT** `[13]` (RW)
    ///
    /// FATAL_BUS_ERR_INT
    #[inline(always)]
    pub fn fatal_bus_err_int(&self) -> bool {
        (self.value.read_bits() & Self::FATAL_BUS_ERR_INT_MASK) != 0
    }

    /// Set FATAL_BUS_ERR_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fatal_bus_err_int(&self) {
        self.value.modify_bits(|r| r | Self::FATAL_BUS_ERR_INT_MASK);
    }

    /// Clear FATAL_BUS_ERR_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fatal_bus_err_int(&self) {
        self.value
            .modify_bits(|r| r & !Self::FATAL_BUS_ERR_INT_MASK);
    }

    /// **EARLY_TRANS_INT** `[10]` (RW)
    ///
    /// EARLY_TRANS_INT
    #[inline(always)]
    pub fn early_trans_int(&self) -> bool {
        (self.value.read_bits() & Self::EARLY_TRANS_INT_MASK) != 0
    }

    /// Set EARLY_TRANS_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_early_trans_int(&self) {
        self.value.modify_bits(|r| r | Self::EARLY_TRANS_INT_MASK);
    }

    /// Clear EARLY_TRANS_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_early_trans_int(&self) {
        self.value.modify_bits(|r| r & !Self::EARLY_TRANS_INT_MASK);
    }

    /// **RECV_WDT_TO** `[9]` (RW)
    ///
    /// RECV_WDT_TO
    #[inline(always)]
    pub fn recv_wdt_to(&self) -> bool {
        (self.value.read_bits() & Self::RECV_WDT_TO_MASK) != 0
    }

    /// Set RECV_WDT_TO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_recv_wdt_to(&self) {
        self.value.modify_bits(|r| r | Self::RECV_WDT_TO_MASK);
    }

    /// Clear RECV_WDT_TO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_recv_wdt_to(&self) {
        self.value.modify_bits(|r| r & !Self::RECV_WDT_TO_MASK);
    }

    /// **RECV_PROC_STOP** `[8]` (RW)
    ///
    /// RECV_PROC_STOP
    #[inline(always)]
    pub fn recv_proc_stop(&self) -> bool {
        (self.value.read_bits() & Self::RECV_PROC_STOP_MASK) != 0
    }

    /// Set RECV_PROC_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_recv_proc_stop(&self) {
        self.value.modify_bits(|r| r | Self::RECV_PROC_STOP_MASK);
    }

    /// Clear RECV_PROC_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_recv_proc_stop(&self) {
        self.value.modify_bits(|r| r & !Self::RECV_PROC_STOP_MASK);
    }

    /// **RECV_BUF_UNAVAIL** `[7]` (RW)
    ///
    /// RECV_BUF_UNAVAIL
    #[inline(always)]
    pub fn recv_buf_unavail(&self) -> bool {
        (self.value.read_bits() & Self::RECV_BUF_UNAVAIL_MASK) != 0
    }

    /// Set RECV_BUF_UNAVAIL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_recv_buf_unavail(&self) {
        self.value.modify_bits(|r| r | Self::RECV_BUF_UNAVAIL_MASK);
    }

    /// Clear RECV_BUF_UNAVAIL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_recv_buf_unavail(&self) {
        self.value.modify_bits(|r| r & !Self::RECV_BUF_UNAVAIL_MASK);
    }

    /// **RECV_INT** `[6]` (RW)
    ///
    /// RECV_INT
    #[inline(always)]
    pub fn recv_int(&self) -> bool {
        (self.value.read_bits() & Self::RECV_INT_MASK) != 0
    }

    /// Set RECV_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_recv_int(&self) {
        self.value.modify_bits(|r| r | Self::RECV_INT_MASK);
    }

    /// Clear RECV_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_recv_int(&self) {
        self.value.modify_bits(|r| r & !Self::RECV_INT_MASK);
    }

    /// **TRANS_UNDFLOW** `[5]` (RW)
    ///
    /// TRANS_UNDFLOW
    #[inline(always)]
    pub fn trans_undflow(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_UNDFLOW_MASK) != 0
    }

    /// Set TRANS_UNDFLOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_trans_undflow(&self) {
        self.value.modify_bits(|r| r | Self::TRANS_UNDFLOW_MASK);
    }

    /// Clear TRANS_UNDFLOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_trans_undflow(&self) {
        self.value.modify_bits(|r| r & !Self::TRANS_UNDFLOW_MASK);
    }

    /// **RECV_OVFLOW** `[4]` (RW)
    ///
    /// RECV_OVFLOW
    #[inline(always)]
    pub fn recv_ovflow(&self) -> bool {
        (self.value.read_bits() & Self::RECV_OVFLOW_MASK) != 0
    }

    /// Set RECV_OVFLOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_recv_ovflow(&self) {
        self.value.modify_bits(|r| r | Self::RECV_OVFLOW_MASK);
    }

    /// Clear RECV_OVFLOW bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_recv_ovflow(&self) {
        self.value.modify_bits(|r| r & !Self::RECV_OVFLOW_MASK);
    }

    /// **TRANS_JABBER_TO** `[3]` (RW)
    ///
    /// TRANS_JABBER_TO
    #[inline(always)]
    pub fn trans_jabber_to(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_JABBER_TO_MASK) != 0
    }

    /// Set TRANS_JABBER_TO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_trans_jabber_to(&self) {
        self.value.modify_bits(|r| r | Self::TRANS_JABBER_TO_MASK);
    }

    /// Clear TRANS_JABBER_TO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_trans_jabber_to(&self) {
        self.value.modify_bits(|r| r & !Self::TRANS_JABBER_TO_MASK);
    }

    /// **TRANS_BUF_UNAVAIL** `[2]` (RW)
    ///
    /// TRANS_BUF_UNAVAIL
    #[inline(always)]
    pub fn trans_buf_unavail(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_BUF_UNAVAIL_MASK) != 0
    }

    /// Set TRANS_BUF_UNAVAIL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_trans_buf_unavail(&self) {
        self.value.modify_bits(|r| r | Self::TRANS_BUF_UNAVAIL_MASK);
    }

    /// Clear TRANS_BUF_UNAVAIL bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_trans_buf_unavail(&self) {
        self.value
            .modify_bits(|r| r & !Self::TRANS_BUF_UNAVAIL_MASK);
    }

    /// **TRANS_PROC_STOP** `[1]` (RW)
    ///
    /// TRANS_PROC_STOP
    #[inline(always)]
    pub fn trans_proc_stop(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_PROC_STOP_MASK) != 0
    }

    /// Set TRANS_PROC_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_trans_proc_stop(&self) {
        self.value.modify_bits(|r| r | Self::TRANS_PROC_STOP_MASK);
    }

    /// Clear TRANS_PROC_STOP bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_trans_proc_stop(&self) {
        self.value.modify_bits(|r| r & !Self::TRANS_PROC_STOP_MASK);
    }

    /// **TRANS_INT** `[0]` (RW)
    ///
    /// TRANS_INT
    #[inline(always)]
    pub fn trans_int(&self) -> bool {
        (self.value.read_bits() & Self::TRANS_INT_MASK) != 0
    }

    /// Set TRANS_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_trans_int(&self) {
        self.value.modify_bits(|r| r | Self::TRANS_INT_MASK);
    }

    /// Clear TRANS_INT bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_trans_int(&self) {
        self.value.modify_bits(|r| r & !Self::TRANS_INT_MASK);
    }
}

/// DMASTATUS value type for batch read/modify/write operations
///
/// Use `dmastatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmastatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmastatusVal {
    bits: u32,
}

impl DmastatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **EMAC_LPI_INT** `[30:30]`
    ///
    /// EMAC_LPI_INT
    #[inline(always)]
    pub const fn emac_lpi_int(&self) -> bool {
        (self.bits & Dmastatus::EMAC_LPI_INT_MASK) != 0
    }

    /// Read **TS_TRI_INT** `[29:29]`
    ///
    /// TS_TRI_INT
    #[inline(always)]
    pub const fn ts_tri_int(&self) -> bool {
        (self.bits & Dmastatus::TS_TRI_INT_MASK) != 0
    }

    /// Read **EMAC_PMT_INT** `[28:28]`
    ///
    /// EMAC_PMT_INT
    #[inline(always)]
    pub const fn emac_pmt_int(&self) -> bool {
        (self.bits & Dmastatus::EMAC_PMT_INT_MASK) != 0
    }

    /// Read **ERROR_BITS** `[25:23]`
    ///
    /// ERROR_BITS
    #[inline(always)]
    pub const fn error_bits(&self) -> u32 {
        (self.bits & Dmastatus::ERROR_BITS_MASK) >> Dmastatus::ERROR_BITS_OFFSET
    }

    /// Read **TRANS_PROC_STATE** `[22:20]`
    ///
    /// TRANS_PROC_STATE
    #[inline(always)]
    pub const fn trans_proc_state(&self) -> u32 {
        (self.bits & Dmastatus::TRANS_PROC_STATE_MASK) >> Dmastatus::TRANS_PROC_STATE_OFFSET
    }

    /// Read **RECV_PROC_STATE** `[19:17]`
    ///
    /// RECV_PROC_STATE
    #[inline(always)]
    pub const fn recv_proc_state(&self) -> u32 {
        (self.bits & Dmastatus::RECV_PROC_STATE_MASK) >> Dmastatus::RECV_PROC_STATE_OFFSET
    }

    /// Read **NORM_INT_SUMM** `[16:16]`
    ///
    /// NORM_INT_SUMM
    #[inline(always)]
    pub const fn norm_int_summ(&self) -> bool {
        (self.bits & Dmastatus::NORM_INT_SUMM_MASK) != 0
    }

    /// Read **ABN_INT_SUMM** `[15:15]`
    ///
    /// ABN_INT_SUMM
    #[inline(always)]
    pub const fn abn_int_summ(&self) -> bool {
        (self.bits & Dmastatus::ABN_INT_SUMM_MASK) != 0
    }

    /// Read **EARLY_RECV_INT** `[14:14]`
    ///
    /// EARLY_RECV_INT
    #[inline(always)]
    pub const fn early_recv_int(&self) -> bool {
        (self.bits & Dmastatus::EARLY_RECV_INT_MASK) != 0
    }

    /// Read **FATAL_BUS_ERR_INT** `[13:13]`
    ///
    /// FATAL_BUS_ERR_INT
    #[inline(always)]
    pub const fn fatal_bus_err_int(&self) -> bool {
        (self.bits & Dmastatus::FATAL_BUS_ERR_INT_MASK) != 0
    }

    /// Read **EARLY_TRANS_INT** `[10:10]`
    ///
    /// EARLY_TRANS_INT
    #[inline(always)]
    pub const fn early_trans_int(&self) -> bool {
        (self.bits & Dmastatus::EARLY_TRANS_INT_MASK) != 0
    }

    /// Read **RECV_WDT_TO** `[9:9]`
    ///
    /// RECV_WDT_TO
    #[inline(always)]
    pub const fn recv_wdt_to(&self) -> bool {
        (self.bits & Dmastatus::RECV_WDT_TO_MASK) != 0
    }

    /// Read **RECV_PROC_STOP** `[8:8]`
    ///
    /// RECV_PROC_STOP
    #[inline(always)]
    pub const fn recv_proc_stop(&self) -> bool {
        (self.bits & Dmastatus::RECV_PROC_STOP_MASK) != 0
    }

    /// Read **RECV_BUF_UNAVAIL** `[7:7]`
    ///
    /// RECV_BUF_UNAVAIL
    #[inline(always)]
    pub const fn recv_buf_unavail(&self) -> bool {
        (self.bits & Dmastatus::RECV_BUF_UNAVAIL_MASK) != 0
    }

    /// Read **RECV_INT** `[6:6]`
    ///
    /// RECV_INT
    #[inline(always)]
    pub const fn recv_int(&self) -> bool {
        (self.bits & Dmastatus::RECV_INT_MASK) != 0
    }

    /// Read **TRANS_UNDFLOW** `[5:5]`
    ///
    /// TRANS_UNDFLOW
    #[inline(always)]
    pub const fn trans_undflow(&self) -> bool {
        (self.bits & Dmastatus::TRANS_UNDFLOW_MASK) != 0
    }

    /// Read **RECV_OVFLOW** `[4:4]`
    ///
    /// RECV_OVFLOW
    #[inline(always)]
    pub const fn recv_ovflow(&self) -> bool {
        (self.bits & Dmastatus::RECV_OVFLOW_MASK) != 0
    }

    /// Read **TRANS_JABBER_TO** `[3:3]`
    ///
    /// TRANS_JABBER_TO
    #[inline(always)]
    pub const fn trans_jabber_to(&self) -> bool {
        (self.bits & Dmastatus::TRANS_JABBER_TO_MASK) != 0
    }

    /// Read **TRANS_BUF_UNAVAIL** `[2:2]`
    ///
    /// TRANS_BUF_UNAVAIL
    #[inline(always)]
    pub const fn trans_buf_unavail(&self) -> bool {
        (self.bits & Dmastatus::TRANS_BUF_UNAVAIL_MASK) != 0
    }

    /// Read **TRANS_PROC_STOP** `[1:1]`
    ///
    /// TRANS_PROC_STOP
    #[inline(always)]
    pub const fn trans_proc_stop(&self) -> bool {
        (self.bits & Dmastatus::TRANS_PROC_STOP_MASK) != 0
    }

    /// Read **TRANS_INT** `[0:0]`
    ///
    /// TRANS_INT
    #[inline(always)]
    pub const fn trans_int(&self) -> bool {
        (self.bits & Dmastatus::TRANS_INT_MASK) != 0
    }

    /// Set **EMAC_LPI_INT** `[30:30]`
    ///
    /// EMAC_LPI_INT
    #[inline(always)]
    pub const fn with_emac_lpi_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::EMAC_LPI_INT_MASK
            } else {
                self.bits & !Dmastatus::EMAC_LPI_INT_MASK
            },
        }
    }

    /// Set **TS_TRI_INT** `[29:29]`
    ///
    /// TS_TRI_INT
    #[inline(always)]
    pub const fn with_ts_tri_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::TS_TRI_INT_MASK
            } else {
                self.bits & !Dmastatus::TS_TRI_INT_MASK
            },
        }
    }

    /// Set **EMAC_PMT_INT** `[28:28]`
    ///
    /// EMAC_PMT_INT
    #[inline(always)]
    pub const fn with_emac_pmt_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::EMAC_PMT_INT_MASK
            } else {
                self.bits & !Dmastatus::EMAC_PMT_INT_MASK
            },
        }
    }

    /// Set **ERROR_BITS** `[25:23]`
    ///
    /// ERROR_BITS
    #[inline(always)]
    pub const fn with_error_bits(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmastatus::ERROR_BITS_MASK)
                | ((value << Dmastatus::ERROR_BITS_OFFSET) & Dmastatus::ERROR_BITS_MASK),
        }
    }

    /// Set **TRANS_PROC_STATE** `[22:20]`
    ///
    /// TRANS_PROC_STATE
    #[inline(always)]
    pub const fn with_trans_proc_state(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmastatus::TRANS_PROC_STATE_MASK)
                | ((value << Dmastatus::TRANS_PROC_STATE_OFFSET)
                    & Dmastatus::TRANS_PROC_STATE_MASK),
        }
    }

    /// Set **RECV_PROC_STATE** `[19:17]`
    ///
    /// RECV_PROC_STATE
    #[inline(always)]
    pub const fn with_recv_proc_state(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmastatus::RECV_PROC_STATE_MASK)
                | ((value << Dmastatus::RECV_PROC_STATE_OFFSET) & Dmastatus::RECV_PROC_STATE_MASK),
        }
    }

    /// Set **NORM_INT_SUMM** `[16:16]`
    ///
    /// NORM_INT_SUMM
    #[inline(always)]
    pub const fn with_norm_int_summ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::NORM_INT_SUMM_MASK
            } else {
                self.bits & !Dmastatus::NORM_INT_SUMM_MASK
            },
        }
    }

    /// Set **ABN_INT_SUMM** `[15:15]`
    ///
    /// ABN_INT_SUMM
    #[inline(always)]
    pub const fn with_abn_int_summ(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::ABN_INT_SUMM_MASK
            } else {
                self.bits & !Dmastatus::ABN_INT_SUMM_MASK
            },
        }
    }

    /// Set **EARLY_RECV_INT** `[14:14]`
    ///
    /// EARLY_RECV_INT
    #[inline(always)]
    pub const fn with_early_recv_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::EARLY_RECV_INT_MASK
            } else {
                self.bits & !Dmastatus::EARLY_RECV_INT_MASK
            },
        }
    }

    /// Set **FATAL_BUS_ERR_INT** `[13:13]`
    ///
    /// FATAL_BUS_ERR_INT
    #[inline(always)]
    pub const fn with_fatal_bus_err_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::FATAL_BUS_ERR_INT_MASK
            } else {
                self.bits & !Dmastatus::FATAL_BUS_ERR_INT_MASK
            },
        }
    }

    /// Set **EARLY_TRANS_INT** `[10:10]`
    ///
    /// EARLY_TRANS_INT
    #[inline(always)]
    pub const fn with_early_trans_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::EARLY_TRANS_INT_MASK
            } else {
                self.bits & !Dmastatus::EARLY_TRANS_INT_MASK
            },
        }
    }

    /// Set **RECV_WDT_TO** `[9:9]`
    ///
    /// RECV_WDT_TO
    #[inline(always)]
    pub const fn with_recv_wdt_to(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::RECV_WDT_TO_MASK
            } else {
                self.bits & !Dmastatus::RECV_WDT_TO_MASK
            },
        }
    }

    /// Set **RECV_PROC_STOP** `[8:8]`
    ///
    /// RECV_PROC_STOP
    #[inline(always)]
    pub const fn with_recv_proc_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::RECV_PROC_STOP_MASK
            } else {
                self.bits & !Dmastatus::RECV_PROC_STOP_MASK
            },
        }
    }

    /// Set **RECV_BUF_UNAVAIL** `[7:7]`
    ///
    /// RECV_BUF_UNAVAIL
    #[inline(always)]
    pub const fn with_recv_buf_unavail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::RECV_BUF_UNAVAIL_MASK
            } else {
                self.bits & !Dmastatus::RECV_BUF_UNAVAIL_MASK
            },
        }
    }

    /// Set **RECV_INT** `[6:6]`
    ///
    /// RECV_INT
    #[inline(always)]
    pub const fn with_recv_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::RECV_INT_MASK
            } else {
                self.bits & !Dmastatus::RECV_INT_MASK
            },
        }
    }

    /// Set **TRANS_UNDFLOW** `[5:5]`
    ///
    /// TRANS_UNDFLOW
    #[inline(always)]
    pub const fn with_trans_undflow(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::TRANS_UNDFLOW_MASK
            } else {
                self.bits & !Dmastatus::TRANS_UNDFLOW_MASK
            },
        }
    }

    /// Set **RECV_OVFLOW** `[4:4]`
    ///
    /// RECV_OVFLOW
    #[inline(always)]
    pub const fn with_recv_ovflow(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::RECV_OVFLOW_MASK
            } else {
                self.bits & !Dmastatus::RECV_OVFLOW_MASK
            },
        }
    }

    /// Set **TRANS_JABBER_TO** `[3:3]`
    ///
    /// TRANS_JABBER_TO
    #[inline(always)]
    pub const fn with_trans_jabber_to(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::TRANS_JABBER_TO_MASK
            } else {
                self.bits & !Dmastatus::TRANS_JABBER_TO_MASK
            },
        }
    }

    /// Set **TRANS_BUF_UNAVAIL** `[2:2]`
    ///
    /// TRANS_BUF_UNAVAIL
    #[inline(always)]
    pub const fn with_trans_buf_unavail(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::TRANS_BUF_UNAVAIL_MASK
            } else {
                self.bits & !Dmastatus::TRANS_BUF_UNAVAIL_MASK
            },
        }
    }

    /// Set **TRANS_PROC_STOP** `[1:1]`
    ///
    /// TRANS_PROC_STOP
    #[inline(always)]
    pub const fn with_trans_proc_stop(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::TRANS_PROC_STOP_MASK
            } else {
                self.bits & !Dmastatus::TRANS_PROC_STOP_MASK
            },
        }
    }

    /// Set **TRANS_INT** `[0:0]`
    ///
    /// TRANS_INT
    #[inline(always)]
    pub const fn with_trans_int(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmastatus::TRANS_INT_MASK
            } else {
                self.bits & !Dmastatus::TRANS_INT_MASK
            },
        }
    }
}

impl Default for DmastatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmastatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmastatusVal {
        DmastatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmastatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmastatusVal) -> DmastatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMAOPERATION_MODE Register
///
/// DMAOPERATION_MODE register
///
/// Address offset: 0x1018
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DIS_DROP_TCPIP_ERR_FRAM** `[26]` (RW)
///
///   DIS_DROP_TCPIP_ERR_FRAM
///
/// - **DIS_FLUSH_RECV_FRAMES** `[24]` (RW)
///
///   DIS_FLUSH_RECV_FRAMES
///
/// - **FLUSH_TX_FIFO** `[20]` (RW)
///
///   FLUSH_TX_FIFO
///
/// - **TX_THRESH_CTRL** `[16:14]` (RW)
///
///   TX_THRESH_CTRL
///
/// - **START_STOP_TRANSMISSION_COMMAND** `[13]` (RW)
///
///   START_STOP_TRANSMISSION_COMMAND
///
/// - **FWD_ERR_FRAME** `[7]` (RW)
///
///   FWD_ERR_FRAME
///
/// - **FWD_UNDER_GF** `[6]` (RW)
///
///   FWD_UNDER_GF
///
/// - **DROP_GFRM** `[5]` (RW)
///
///   DROP_GFRM
///
/// - **RX_THRESH_CTRL** `[4:3]` (RW)
///
///   RX_THRESH_CTRL
///
/// - **OPT_SECOND_FRAME** `[2]` (RW)
///
///   OPT_SECOND_FRAME
///
/// - **START_STOP_RX** `[1]` (RW)
///
///   START_STOP_RX
#[repr(transparent)]
pub struct DmaoperationMode {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmaoperationMode {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmaoperationMode {
    // ======== Field Constants ========

    /// **DIS_DROP_TCPIP_ERR_FRAM**
    ///
    /// DIS_DROP_TCPIP_ERR_FRAM
    ///
    /// Bits `[26:26]`, mask `0x04000000`
    pub const DIS_DROP_TCPIP_ERR_FRAM_OFFSET: u32 = 26;
    pub const DIS_DROP_TCPIP_ERR_FRAM_MASK: u32 = 0x04000000;
    pub const DIS_DROP_TCPIP_ERR_FRAM: u32 = 1 << 26;

    /// **DIS_FLUSH_RECV_FRAMES**
    ///
    /// DIS_FLUSH_RECV_FRAMES
    ///
    /// Bits `[24:24]`, mask `0x01000000`
    pub const DIS_FLUSH_RECV_FRAMES_OFFSET: u32 = 24;
    pub const DIS_FLUSH_RECV_FRAMES_MASK: u32 = 0x01000000;
    pub const DIS_FLUSH_RECV_FRAMES: u32 = 1 << 24;

    /// **FLUSH_TX_FIFO**
    ///
    /// FLUSH_TX_FIFO
    ///
    /// Bits `[20:20]`, mask `0x00100000`
    pub const FLUSH_TX_FIFO_OFFSET: u32 = 20;
    pub const FLUSH_TX_FIFO_MASK: u32 = 0x00100000;
    pub const FLUSH_TX_FIFO: u32 = 1 << 20;

    /// **TX_THRESH_CTRL**
    ///
    /// TX_THRESH_CTRL
    ///
    /// Bits `[16:14]`, mask `0x0001c000`
    pub const TX_THRESH_CTRL_OFFSET: u32 = 14;
    pub const TX_THRESH_CTRL_MASK: u32 = 0x0001c000;

    /// **START_STOP_TRANSMISSION_COMMAND**
    ///
    /// START_STOP_TRANSMISSION_COMMAND
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const START_STOP_TRANSMISSION_COMMAND_OFFSET: u32 = 13;
    pub const START_STOP_TRANSMISSION_COMMAND_MASK: u32 = 0x00002000;
    pub const START_STOP_TRANSMISSION_COMMAND: u32 = 1 << 13;

    /// **FWD_ERR_FRAME**
    ///
    /// FWD_ERR_FRAME
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const FWD_ERR_FRAME_OFFSET: u32 = 7;
    pub const FWD_ERR_FRAME_MASK: u32 = 0x00000080;
    pub const FWD_ERR_FRAME: u32 = 1 << 7;

    /// **FWD_UNDER_GF**
    ///
    /// FWD_UNDER_GF
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const FWD_UNDER_GF_OFFSET: u32 = 6;
    pub const FWD_UNDER_GF_MASK: u32 = 0x00000040;
    pub const FWD_UNDER_GF: u32 = 1 << 6;

    /// **DROP_GFRM**
    ///
    /// DROP_GFRM
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DROP_GFRM_OFFSET: u32 = 5;
    pub const DROP_GFRM_MASK: u32 = 0x00000020;
    pub const DROP_GFRM: u32 = 1 << 5;

    /// **RX_THRESH_CTRL**
    ///
    /// RX_THRESH_CTRL
    ///
    /// Bits `[4:3]`, mask `0x00000018`
    pub const RX_THRESH_CTRL_OFFSET: u32 = 3;
    pub const RX_THRESH_CTRL_MASK: u32 = 0x00000018;

    /// **OPT_SECOND_FRAME**
    ///
    /// OPT_SECOND_FRAME
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const OPT_SECOND_FRAME_OFFSET: u32 = 2;
    pub const OPT_SECOND_FRAME_MASK: u32 = 0x00000004;
    pub const OPT_SECOND_FRAME: u32 = 1 << 2;

    /// **START_STOP_RX**
    ///
    /// START_STOP_RX
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const START_STOP_RX_OFFSET: u32 = 1;
    pub const START_STOP_RX_MASK: u32 = 0x00000002;
    pub const START_STOP_RX: u32 = 1 << 1;

    // ======== Field Accessors ========

    /// **DIS_DROP_TCPIP_ERR_FRAM** `[26]` (RW)
    ///
    /// DIS_DROP_TCPIP_ERR_FRAM
    #[inline(always)]
    pub fn dis_drop_tcpip_err_fram(&self) -> bool {
        (self.value.read_bits() & Self::DIS_DROP_TCPIP_ERR_FRAM_MASK) != 0
    }

    /// Set DIS_DROP_TCPIP_ERR_FRAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dis_drop_tcpip_err_fram(&self) {
        self.value
            .modify_bits(|r| r | Self::DIS_DROP_TCPIP_ERR_FRAM_MASK);
    }

    /// Clear DIS_DROP_TCPIP_ERR_FRAM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dis_drop_tcpip_err_fram(&self) {
        self.value
            .modify_bits(|r| r & !Self::DIS_DROP_TCPIP_ERR_FRAM_MASK);
    }

    /// **DIS_FLUSH_RECV_FRAMES** `[24]` (RW)
    ///
    /// DIS_FLUSH_RECV_FRAMES
    #[inline(always)]
    pub fn dis_flush_recv_frames(&self) -> bool {
        (self.value.read_bits() & Self::DIS_FLUSH_RECV_FRAMES_MASK) != 0
    }

    /// Set DIS_FLUSH_RECV_FRAMES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dis_flush_recv_frames(&self) {
        self.value
            .modify_bits(|r| r | Self::DIS_FLUSH_RECV_FRAMES_MASK);
    }

    /// Clear DIS_FLUSH_RECV_FRAMES bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dis_flush_recv_frames(&self) {
        self.value
            .modify_bits(|r| r & !Self::DIS_FLUSH_RECV_FRAMES_MASK);
    }

    /// **FLUSH_TX_FIFO** `[20]` (RW)
    ///
    /// FLUSH_TX_FIFO
    #[inline(always)]
    pub fn flush_tx_fifo(&self) -> bool {
        (self.value.read_bits() & Self::FLUSH_TX_FIFO_MASK) != 0
    }

    /// Set FLUSH_TX_FIFO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_flush_tx_fifo(&self) {
        self.value.modify_bits(|r| r | Self::FLUSH_TX_FIFO_MASK);
    }

    /// Clear FLUSH_TX_FIFO bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_flush_tx_fifo(&self) {
        self.value.modify_bits(|r| r & !Self::FLUSH_TX_FIFO_MASK);
    }

    /// **TX_THRESH_CTRL** `[16:14]` (RW)
    ///
    /// TX_THRESH_CTRL
    #[inline(always)]
    pub fn tx_thresh_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::TX_THRESH_CTRL_MASK) >> Self::TX_THRESH_CTRL_OFFSET
    }

    /// Write **TX_THRESH_CTRL** `[16:14]` (RW)
    ///
    /// TX_THRESH_CTRL
    #[inline(always)]
    pub fn write_tx_thresh_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::TX_THRESH_CTRL_MASK)
                | ((value << Self::TX_THRESH_CTRL_OFFSET) & Self::TX_THRESH_CTRL_MASK)
        });
    }

    /// **START_STOP_TRANSMISSION_COMMAND** `[13]` (RW)
    ///
    /// START_STOP_TRANSMISSION_COMMAND
    #[inline(always)]
    pub fn start_stop_transmission_command(&self) -> bool {
        (self.value.read_bits() & Self::START_STOP_TRANSMISSION_COMMAND_MASK) != 0
    }

    /// Set START_STOP_TRANSMISSION_COMMAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start_stop_transmission_command(&self) {
        self.value
            .modify_bits(|r| r | Self::START_STOP_TRANSMISSION_COMMAND_MASK);
    }

    /// Clear START_STOP_TRANSMISSION_COMMAND bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start_stop_transmission_command(&self) {
        self.value
            .modify_bits(|r| r & !Self::START_STOP_TRANSMISSION_COMMAND_MASK);
    }

    /// **FWD_ERR_FRAME** `[7]` (RW)
    ///
    /// FWD_ERR_FRAME
    #[inline(always)]
    pub fn fwd_err_frame(&self) -> bool {
        (self.value.read_bits() & Self::FWD_ERR_FRAME_MASK) != 0
    }

    /// Set FWD_ERR_FRAME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fwd_err_frame(&self) {
        self.value.modify_bits(|r| r | Self::FWD_ERR_FRAME_MASK);
    }

    /// Clear FWD_ERR_FRAME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fwd_err_frame(&self) {
        self.value.modify_bits(|r| r & !Self::FWD_ERR_FRAME_MASK);
    }

    /// **FWD_UNDER_GF** `[6]` (RW)
    ///
    /// FWD_UNDER_GF
    #[inline(always)]
    pub fn fwd_under_gf(&self) -> bool {
        (self.value.read_bits() & Self::FWD_UNDER_GF_MASK) != 0
    }

    /// Set FWD_UNDER_GF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_fwd_under_gf(&self) {
        self.value.modify_bits(|r| r | Self::FWD_UNDER_GF_MASK);
    }

    /// Clear FWD_UNDER_GF bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_fwd_under_gf(&self) {
        self.value.modify_bits(|r| r & !Self::FWD_UNDER_GF_MASK);
    }

    /// **DROP_GFRM** `[5]` (RW)
    ///
    /// DROP_GFRM
    #[inline(always)]
    pub fn drop_gfrm(&self) -> bool {
        (self.value.read_bits() & Self::DROP_GFRM_MASK) != 0
    }

    /// Set DROP_GFRM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_drop_gfrm(&self) {
        self.value.modify_bits(|r| r | Self::DROP_GFRM_MASK);
    }

    /// Clear DROP_GFRM bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_drop_gfrm(&self) {
        self.value.modify_bits(|r| r & !Self::DROP_GFRM_MASK);
    }

    /// **RX_THRESH_CTRL** `[4:3]` (RW)
    ///
    /// RX_THRESH_CTRL
    #[inline(always)]
    pub fn rx_thresh_ctrl(&self) -> u32 {
        (self.value.read_bits() & Self::RX_THRESH_CTRL_MASK) >> Self::RX_THRESH_CTRL_OFFSET
    }

    /// Write **RX_THRESH_CTRL** `[4:3]` (RW)
    ///
    /// RX_THRESH_CTRL
    #[inline(always)]
    pub fn write_rx_thresh_ctrl(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RX_THRESH_CTRL_MASK)
                | ((value << Self::RX_THRESH_CTRL_OFFSET) & Self::RX_THRESH_CTRL_MASK)
        });
    }

    /// **OPT_SECOND_FRAME** `[2]` (RW)
    ///
    /// OPT_SECOND_FRAME
    #[inline(always)]
    pub fn opt_second_frame(&self) -> bool {
        (self.value.read_bits() & Self::OPT_SECOND_FRAME_MASK) != 0
    }

    /// Set OPT_SECOND_FRAME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_opt_second_frame(&self) {
        self.value.modify_bits(|r| r | Self::OPT_SECOND_FRAME_MASK);
    }

    /// Clear OPT_SECOND_FRAME bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_opt_second_frame(&self) {
        self.value.modify_bits(|r| r & !Self::OPT_SECOND_FRAME_MASK);
    }

    /// **START_STOP_RX** `[1]` (RW)
    ///
    /// START_STOP_RX
    #[inline(always)]
    pub fn start_stop_rx(&self) -> bool {
        (self.value.read_bits() & Self::START_STOP_RX_MASK) != 0
    }

    /// Set START_STOP_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_start_stop_rx(&self) {
        self.value.modify_bits(|r| r | Self::START_STOP_RX_MASK);
    }

    /// Clear START_STOP_RX bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_start_stop_rx(&self) {
        self.value.modify_bits(|r| r & !Self::START_STOP_RX_MASK);
    }
}

/// DMAOPERATION_MODE value type for batch read/modify/write operations
///
/// Use `dmaoperationmode.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaoperationmode.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaoperationModeVal {
    bits: u32,
}

impl DmaoperationModeVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **DIS_DROP_TCPIP_ERR_FRAM** `[26:26]`
    ///
    /// DIS_DROP_TCPIP_ERR_FRAM
    #[inline(always)]
    pub const fn dis_drop_tcpip_err_fram(&self) -> bool {
        (self.bits & DmaoperationMode::DIS_DROP_TCPIP_ERR_FRAM_MASK) != 0
    }

    /// Read **DIS_FLUSH_RECV_FRAMES** `[24:24]`
    ///
    /// DIS_FLUSH_RECV_FRAMES
    #[inline(always)]
    pub const fn dis_flush_recv_frames(&self) -> bool {
        (self.bits & DmaoperationMode::DIS_FLUSH_RECV_FRAMES_MASK) != 0
    }

    /// Read **FLUSH_TX_FIFO** `[20:20]`
    ///
    /// FLUSH_TX_FIFO
    #[inline(always)]
    pub const fn flush_tx_fifo(&self) -> bool {
        (self.bits & DmaoperationMode::FLUSH_TX_FIFO_MASK) != 0
    }

    /// Read **TX_THRESH_CTRL** `[16:14]`
    ///
    /// TX_THRESH_CTRL
    #[inline(always)]
    pub const fn tx_thresh_ctrl(&self) -> u32 {
        (self.bits & DmaoperationMode::TX_THRESH_CTRL_MASK)
            >> DmaoperationMode::TX_THRESH_CTRL_OFFSET
    }

    /// Read **START_STOP_TRANSMISSION_COMMAND** `[13:13]`
    ///
    /// START_STOP_TRANSMISSION_COMMAND
    #[inline(always)]
    pub const fn start_stop_transmission_command(&self) -> bool {
        (self.bits & DmaoperationMode::START_STOP_TRANSMISSION_COMMAND_MASK) != 0
    }

    /// Read **FWD_ERR_FRAME** `[7:7]`
    ///
    /// FWD_ERR_FRAME
    #[inline(always)]
    pub const fn fwd_err_frame(&self) -> bool {
        (self.bits & DmaoperationMode::FWD_ERR_FRAME_MASK) != 0
    }

    /// Read **FWD_UNDER_GF** `[6:6]`
    ///
    /// FWD_UNDER_GF
    #[inline(always)]
    pub const fn fwd_under_gf(&self) -> bool {
        (self.bits & DmaoperationMode::FWD_UNDER_GF_MASK) != 0
    }

    /// Read **DROP_GFRM** `[5:5]`
    ///
    /// DROP_GFRM
    #[inline(always)]
    pub const fn drop_gfrm(&self) -> bool {
        (self.bits & DmaoperationMode::DROP_GFRM_MASK) != 0
    }

    /// Read **RX_THRESH_CTRL** `[4:3]`
    ///
    /// RX_THRESH_CTRL
    #[inline(always)]
    pub const fn rx_thresh_ctrl(&self) -> u32 {
        (self.bits & DmaoperationMode::RX_THRESH_CTRL_MASK)
            >> DmaoperationMode::RX_THRESH_CTRL_OFFSET
    }

    /// Read **OPT_SECOND_FRAME** `[2:2]`
    ///
    /// OPT_SECOND_FRAME
    #[inline(always)]
    pub const fn opt_second_frame(&self) -> bool {
        (self.bits & DmaoperationMode::OPT_SECOND_FRAME_MASK) != 0
    }

    /// Read **START_STOP_RX** `[1:1]`
    ///
    /// START_STOP_RX
    #[inline(always)]
    pub const fn start_stop_rx(&self) -> bool {
        (self.bits & DmaoperationMode::START_STOP_RX_MASK) != 0
    }

    /// Set **DIS_DROP_TCPIP_ERR_FRAM** `[26:26]`
    ///
    /// DIS_DROP_TCPIP_ERR_FRAM
    #[inline(always)]
    pub const fn with_dis_drop_tcpip_err_fram(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::DIS_DROP_TCPIP_ERR_FRAM_MASK
            } else {
                self.bits & !DmaoperationMode::DIS_DROP_TCPIP_ERR_FRAM_MASK
            },
        }
    }

    /// Set **DIS_FLUSH_RECV_FRAMES** `[24:24]`
    ///
    /// DIS_FLUSH_RECV_FRAMES
    #[inline(always)]
    pub const fn with_dis_flush_recv_frames(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::DIS_FLUSH_RECV_FRAMES_MASK
            } else {
                self.bits & !DmaoperationMode::DIS_FLUSH_RECV_FRAMES_MASK
            },
        }
    }

    /// Set **FLUSH_TX_FIFO** `[20:20]`
    ///
    /// FLUSH_TX_FIFO
    #[inline(always)]
    pub const fn with_flush_tx_fifo(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::FLUSH_TX_FIFO_MASK
            } else {
                self.bits & !DmaoperationMode::FLUSH_TX_FIFO_MASK
            },
        }
    }

    /// Set **TX_THRESH_CTRL** `[16:14]`
    ///
    /// TX_THRESH_CTRL
    #[inline(always)]
    pub const fn with_tx_thresh_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaoperationMode::TX_THRESH_CTRL_MASK)
                | ((value << DmaoperationMode::TX_THRESH_CTRL_OFFSET)
                    & DmaoperationMode::TX_THRESH_CTRL_MASK),
        }
    }

    /// Set **START_STOP_TRANSMISSION_COMMAND** `[13:13]`
    ///
    /// START_STOP_TRANSMISSION_COMMAND
    #[inline(always)]
    pub const fn with_start_stop_transmission_command(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::START_STOP_TRANSMISSION_COMMAND_MASK
            } else {
                self.bits & !DmaoperationMode::START_STOP_TRANSMISSION_COMMAND_MASK
            },
        }
    }

    /// Set **FWD_ERR_FRAME** `[7:7]`
    ///
    /// FWD_ERR_FRAME
    #[inline(always)]
    pub const fn with_fwd_err_frame(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::FWD_ERR_FRAME_MASK
            } else {
                self.bits & !DmaoperationMode::FWD_ERR_FRAME_MASK
            },
        }
    }

    /// Set **FWD_UNDER_GF** `[6:6]`
    ///
    /// FWD_UNDER_GF
    #[inline(always)]
    pub const fn with_fwd_under_gf(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::FWD_UNDER_GF_MASK
            } else {
                self.bits & !DmaoperationMode::FWD_UNDER_GF_MASK
            },
        }
    }

    /// Set **DROP_GFRM** `[5:5]`
    ///
    /// DROP_GFRM
    #[inline(always)]
    pub const fn with_drop_gfrm(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::DROP_GFRM_MASK
            } else {
                self.bits & !DmaoperationMode::DROP_GFRM_MASK
            },
        }
    }

    /// Set **RX_THRESH_CTRL** `[4:3]`
    ///
    /// RX_THRESH_CTRL
    #[inline(always)]
    pub const fn with_rx_thresh_ctrl(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmaoperationMode::RX_THRESH_CTRL_MASK)
                | ((value << DmaoperationMode::RX_THRESH_CTRL_OFFSET)
                    & DmaoperationMode::RX_THRESH_CTRL_MASK),
        }
    }

    /// Set **OPT_SECOND_FRAME** `[2:2]`
    ///
    /// OPT_SECOND_FRAME
    #[inline(always)]
    pub const fn with_opt_second_frame(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::OPT_SECOND_FRAME_MASK
            } else {
                self.bits & !DmaoperationMode::OPT_SECOND_FRAME_MASK
            },
        }
    }

    /// Set **START_STOP_RX** `[1:1]`
    ///
    /// START_STOP_RX
    #[inline(always)]
    pub const fn with_start_stop_rx(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmaoperationMode::START_STOP_RX_MASK
            } else {
                self.bits & !DmaoperationMode::START_STOP_RX_MASK
            },
        }
    }
}

impl Default for DmaoperationModeVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmaoperationMode {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaoperationModeVal {
        DmaoperationModeVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaoperationModeVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaoperationModeVal) -> DmaoperationModeVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMAIN_EN Register
///
/// DMAIN_EN register
///
/// Address offset: 0x101c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **DMAIN_NISE** `[16]` (RW)
///
///   DMAIN_NISE
///
/// - **DMAIN_AISE** `[15]` (RW)
///
///   DMAIN_AISE
///
/// - **DMAIN_ERIE** `[14]` (RW)
///
///   DMAIN_ERIE
///
/// - **DMAIN_FBEE** `[13]` (RW)
///
///   DMAIN_FBEE
///
/// - **DMAIN_ETIE** `[10]` (RW)
///
///   DMAIN_ETIE
///
/// - **DMAIN_RWTE** `[9]` (RW)
///
///   DMAIN_RWTE
///
/// - **DMAIN_RSE** `[8]` (RW)
///
///   DMAIN_RSE
///
/// - **DMAIN_RBUE** `[7]` (RW)
///
///   DMAIN_RBUE
///
/// - **DMAIN_RIE** `[6]` (RW)
///
///   DMAIN_RIE
///
/// - **DMAIN_UIE** `[5]` (RW)
///
///   DMAIN_UIE
///
/// - **DMAIN_OIE** `[4]` (RW)
///
///   DMAIN_OIE
///
/// - **DMAIN_TJTE** `[3]` (RW)
///
///   DMAIN_TJTE
///
/// - **DMAIN_TBUE** `[2]` (RW)
///
///   DMAIN_TBUE
///
/// - **DMAIN_TSE** `[1]` (RW)
///
///   DMAIN_TSE
///
/// - **DMAIN_TIE** `[0]` (RW)
///
///   DMAIN_TIE
#[repr(transparent)]
pub struct DmainEn {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmainEn {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmainEn {
    // ======== Field Constants ========

    /// **DMAIN_NISE**
    ///
    /// DMAIN_NISE
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const DMAIN_NISE_OFFSET: u32 = 16;
    pub const DMAIN_NISE_MASK: u32 = 0x00010000;
    pub const DMAIN_NISE: u32 = 1 << 16;

    /// **DMAIN_AISE**
    ///
    /// DMAIN_AISE
    ///
    /// Bits `[15:15]`, mask `0x00008000`
    pub const DMAIN_AISE_OFFSET: u32 = 15;
    pub const DMAIN_AISE_MASK: u32 = 0x00008000;
    pub const DMAIN_AISE: u32 = 1 << 15;

    /// **DMAIN_ERIE**
    ///
    /// DMAIN_ERIE
    ///
    /// Bits `[14:14]`, mask `0x00004000`
    pub const DMAIN_ERIE_OFFSET: u32 = 14;
    pub const DMAIN_ERIE_MASK: u32 = 0x00004000;
    pub const DMAIN_ERIE: u32 = 1 << 14;

    /// **DMAIN_FBEE**
    ///
    /// DMAIN_FBEE
    ///
    /// Bits `[13:13]`, mask `0x00002000`
    pub const DMAIN_FBEE_OFFSET: u32 = 13;
    pub const DMAIN_FBEE_MASK: u32 = 0x00002000;
    pub const DMAIN_FBEE: u32 = 1 << 13;

    /// **DMAIN_ETIE**
    ///
    /// DMAIN_ETIE
    ///
    /// Bits `[10:10]`, mask `0x00000400`
    pub const DMAIN_ETIE_OFFSET: u32 = 10;
    pub const DMAIN_ETIE_MASK: u32 = 0x00000400;
    pub const DMAIN_ETIE: u32 = 1 << 10;

    /// **DMAIN_RWTE**
    ///
    /// DMAIN_RWTE
    ///
    /// Bits `[9:9]`, mask `0x00000200`
    pub const DMAIN_RWTE_OFFSET: u32 = 9;
    pub const DMAIN_RWTE_MASK: u32 = 0x00000200;
    pub const DMAIN_RWTE: u32 = 1 << 9;

    /// **DMAIN_RSE**
    ///
    /// DMAIN_RSE
    ///
    /// Bits `[8:8]`, mask `0x00000100`
    pub const DMAIN_RSE_OFFSET: u32 = 8;
    pub const DMAIN_RSE_MASK: u32 = 0x00000100;
    pub const DMAIN_RSE: u32 = 1 << 8;

    /// **DMAIN_RBUE**
    ///
    /// DMAIN_RBUE
    ///
    /// Bits `[7:7]`, mask `0x00000080`
    pub const DMAIN_RBUE_OFFSET: u32 = 7;
    pub const DMAIN_RBUE_MASK: u32 = 0x00000080;
    pub const DMAIN_RBUE: u32 = 1 << 7;

    /// **DMAIN_RIE**
    ///
    /// DMAIN_RIE
    ///
    /// Bits `[6:6]`, mask `0x00000040`
    pub const DMAIN_RIE_OFFSET: u32 = 6;
    pub const DMAIN_RIE_MASK: u32 = 0x00000040;
    pub const DMAIN_RIE: u32 = 1 << 6;

    /// **DMAIN_UIE**
    ///
    /// DMAIN_UIE
    ///
    /// Bits `[5:5]`, mask `0x00000020`
    pub const DMAIN_UIE_OFFSET: u32 = 5;
    pub const DMAIN_UIE_MASK: u32 = 0x00000020;
    pub const DMAIN_UIE: u32 = 1 << 5;

    /// **DMAIN_OIE**
    ///
    /// DMAIN_OIE
    ///
    /// Bits `[4:4]`, mask `0x00000010`
    pub const DMAIN_OIE_OFFSET: u32 = 4;
    pub const DMAIN_OIE_MASK: u32 = 0x00000010;
    pub const DMAIN_OIE: u32 = 1 << 4;

    /// **DMAIN_TJTE**
    ///
    /// DMAIN_TJTE
    ///
    /// Bits `[3:3]`, mask `0x00000008`
    pub const DMAIN_TJTE_OFFSET: u32 = 3;
    pub const DMAIN_TJTE_MASK: u32 = 0x00000008;
    pub const DMAIN_TJTE: u32 = 1 << 3;

    /// **DMAIN_TBUE**
    ///
    /// DMAIN_TBUE
    ///
    /// Bits `[2:2]`, mask `0x00000004`
    pub const DMAIN_TBUE_OFFSET: u32 = 2;
    pub const DMAIN_TBUE_MASK: u32 = 0x00000004;
    pub const DMAIN_TBUE: u32 = 1 << 2;

    /// **DMAIN_TSE**
    ///
    /// DMAIN_TSE
    ///
    /// Bits `[1:1]`, mask `0x00000002`
    pub const DMAIN_TSE_OFFSET: u32 = 1;
    pub const DMAIN_TSE_MASK: u32 = 0x00000002;
    pub const DMAIN_TSE: u32 = 1 << 1;

    /// **DMAIN_TIE**
    ///
    /// DMAIN_TIE
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const DMAIN_TIE_OFFSET: u32 = 0;
    pub const DMAIN_TIE_MASK: u32 = 0x00000001;
    pub const DMAIN_TIE: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **DMAIN_NISE** `[16]` (RW)
    ///
    /// DMAIN_NISE
    #[inline(always)]
    pub fn dmain_nise(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_NISE_MASK) != 0
    }

    /// Set DMAIN_NISE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_nise(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_NISE_MASK);
    }

    /// Clear DMAIN_NISE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_nise(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_NISE_MASK);
    }

    /// **DMAIN_AISE** `[15]` (RW)
    ///
    /// DMAIN_AISE
    #[inline(always)]
    pub fn dmain_aise(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_AISE_MASK) != 0
    }

    /// Set DMAIN_AISE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_aise(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_AISE_MASK);
    }

    /// Clear DMAIN_AISE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_aise(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_AISE_MASK);
    }

    /// **DMAIN_ERIE** `[14]` (RW)
    ///
    /// DMAIN_ERIE
    #[inline(always)]
    pub fn dmain_erie(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_ERIE_MASK) != 0
    }

    /// Set DMAIN_ERIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_erie(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_ERIE_MASK);
    }

    /// Clear DMAIN_ERIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_erie(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_ERIE_MASK);
    }

    /// **DMAIN_FBEE** `[13]` (RW)
    ///
    /// DMAIN_FBEE
    #[inline(always)]
    pub fn dmain_fbee(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_FBEE_MASK) != 0
    }

    /// Set DMAIN_FBEE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_fbee(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_FBEE_MASK);
    }

    /// Clear DMAIN_FBEE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_fbee(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_FBEE_MASK);
    }

    /// **DMAIN_ETIE** `[10]` (RW)
    ///
    /// DMAIN_ETIE
    #[inline(always)]
    pub fn dmain_etie(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_ETIE_MASK) != 0
    }

    /// Set DMAIN_ETIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_etie(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_ETIE_MASK);
    }

    /// Clear DMAIN_ETIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_etie(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_ETIE_MASK);
    }

    /// **DMAIN_RWTE** `[9]` (RW)
    ///
    /// DMAIN_RWTE
    #[inline(always)]
    pub fn dmain_rwte(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_RWTE_MASK) != 0
    }

    /// Set DMAIN_RWTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_rwte(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_RWTE_MASK);
    }

    /// Clear DMAIN_RWTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_rwte(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_RWTE_MASK);
    }

    /// **DMAIN_RSE** `[8]` (RW)
    ///
    /// DMAIN_RSE
    #[inline(always)]
    pub fn dmain_rse(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_RSE_MASK) != 0
    }

    /// Set DMAIN_RSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_rse(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_RSE_MASK);
    }

    /// Clear DMAIN_RSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_rse(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_RSE_MASK);
    }

    /// **DMAIN_RBUE** `[7]` (RW)
    ///
    /// DMAIN_RBUE
    #[inline(always)]
    pub fn dmain_rbue(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_RBUE_MASK) != 0
    }

    /// Set DMAIN_RBUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_rbue(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_RBUE_MASK);
    }

    /// Clear DMAIN_RBUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_rbue(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_RBUE_MASK);
    }

    /// **DMAIN_RIE** `[6]` (RW)
    ///
    /// DMAIN_RIE
    #[inline(always)]
    pub fn dmain_rie(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_RIE_MASK) != 0
    }

    /// Set DMAIN_RIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_rie(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_RIE_MASK);
    }

    /// Clear DMAIN_RIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_rie(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_RIE_MASK);
    }

    /// **DMAIN_UIE** `[5]` (RW)
    ///
    /// DMAIN_UIE
    #[inline(always)]
    pub fn dmain_uie(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_UIE_MASK) != 0
    }

    /// Set DMAIN_UIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_uie(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_UIE_MASK);
    }

    /// Clear DMAIN_UIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_uie(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_UIE_MASK);
    }

    /// **DMAIN_OIE** `[4]` (RW)
    ///
    /// DMAIN_OIE
    #[inline(always)]
    pub fn dmain_oie(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_OIE_MASK) != 0
    }

    /// Set DMAIN_OIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_oie(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_OIE_MASK);
    }

    /// Clear DMAIN_OIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_oie(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_OIE_MASK);
    }

    /// **DMAIN_TJTE** `[3]` (RW)
    ///
    /// DMAIN_TJTE
    #[inline(always)]
    pub fn dmain_tjte(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_TJTE_MASK) != 0
    }

    /// Set DMAIN_TJTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_tjte(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_TJTE_MASK);
    }

    /// Clear DMAIN_TJTE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_tjte(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_TJTE_MASK);
    }

    /// **DMAIN_TBUE** `[2]` (RW)
    ///
    /// DMAIN_TBUE
    #[inline(always)]
    pub fn dmain_tbue(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_TBUE_MASK) != 0
    }

    /// Set DMAIN_TBUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_tbue(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_TBUE_MASK);
    }

    /// Clear DMAIN_TBUE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_tbue(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_TBUE_MASK);
    }

    /// **DMAIN_TSE** `[1]` (RW)
    ///
    /// DMAIN_TSE
    #[inline(always)]
    pub fn dmain_tse(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_TSE_MASK) != 0
    }

    /// Set DMAIN_TSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_tse(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_TSE_MASK);
    }

    /// Clear DMAIN_TSE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_tse(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_TSE_MASK);
    }

    /// **DMAIN_TIE** `[0]` (RW)
    ///
    /// DMAIN_TIE
    #[inline(always)]
    pub fn dmain_tie(&self) -> bool {
        (self.value.read_bits() & Self::DMAIN_TIE_MASK) != 0
    }

    /// Set DMAIN_TIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_dmain_tie(&self) {
        self.value.modify_bits(|r| r | Self::DMAIN_TIE_MASK);
    }

    /// Clear DMAIN_TIE bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_dmain_tie(&self) {
        self.value.modify_bits(|r| r & !Self::DMAIN_TIE_MASK);
    }
}

/// DMAIN_EN value type for batch read/modify/write operations
///
/// Use `dmainen.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmainen.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmainEnVal {
    bits: u32,
}

impl DmainEnVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **DMAIN_NISE** `[16:16]`
    ///
    /// DMAIN_NISE
    #[inline(always)]
    pub const fn dmain_nise(&self) -> bool {
        (self.bits & DmainEn::DMAIN_NISE_MASK) != 0
    }

    /// Read **DMAIN_AISE** `[15:15]`
    ///
    /// DMAIN_AISE
    #[inline(always)]
    pub const fn dmain_aise(&self) -> bool {
        (self.bits & DmainEn::DMAIN_AISE_MASK) != 0
    }

    /// Read **DMAIN_ERIE** `[14:14]`
    ///
    /// DMAIN_ERIE
    #[inline(always)]
    pub const fn dmain_erie(&self) -> bool {
        (self.bits & DmainEn::DMAIN_ERIE_MASK) != 0
    }

    /// Read **DMAIN_FBEE** `[13:13]`
    ///
    /// DMAIN_FBEE
    #[inline(always)]
    pub const fn dmain_fbee(&self) -> bool {
        (self.bits & DmainEn::DMAIN_FBEE_MASK) != 0
    }

    /// Read **DMAIN_ETIE** `[10:10]`
    ///
    /// DMAIN_ETIE
    #[inline(always)]
    pub const fn dmain_etie(&self) -> bool {
        (self.bits & DmainEn::DMAIN_ETIE_MASK) != 0
    }

    /// Read **DMAIN_RWTE** `[9:9]`
    ///
    /// DMAIN_RWTE
    #[inline(always)]
    pub const fn dmain_rwte(&self) -> bool {
        (self.bits & DmainEn::DMAIN_RWTE_MASK) != 0
    }

    /// Read **DMAIN_RSE** `[8:8]`
    ///
    /// DMAIN_RSE
    #[inline(always)]
    pub const fn dmain_rse(&self) -> bool {
        (self.bits & DmainEn::DMAIN_RSE_MASK) != 0
    }

    /// Read **DMAIN_RBUE** `[7:7]`
    ///
    /// DMAIN_RBUE
    #[inline(always)]
    pub const fn dmain_rbue(&self) -> bool {
        (self.bits & DmainEn::DMAIN_RBUE_MASK) != 0
    }

    /// Read **DMAIN_RIE** `[6:6]`
    ///
    /// DMAIN_RIE
    #[inline(always)]
    pub const fn dmain_rie(&self) -> bool {
        (self.bits & DmainEn::DMAIN_RIE_MASK) != 0
    }

    /// Read **DMAIN_UIE** `[5:5]`
    ///
    /// DMAIN_UIE
    #[inline(always)]
    pub const fn dmain_uie(&self) -> bool {
        (self.bits & DmainEn::DMAIN_UIE_MASK) != 0
    }

    /// Read **DMAIN_OIE** `[4:4]`
    ///
    /// DMAIN_OIE
    #[inline(always)]
    pub const fn dmain_oie(&self) -> bool {
        (self.bits & DmainEn::DMAIN_OIE_MASK) != 0
    }

    /// Read **DMAIN_TJTE** `[3:3]`
    ///
    /// DMAIN_TJTE
    #[inline(always)]
    pub const fn dmain_tjte(&self) -> bool {
        (self.bits & DmainEn::DMAIN_TJTE_MASK) != 0
    }

    /// Read **DMAIN_TBUE** `[2:2]`
    ///
    /// DMAIN_TBUE
    #[inline(always)]
    pub const fn dmain_tbue(&self) -> bool {
        (self.bits & DmainEn::DMAIN_TBUE_MASK) != 0
    }

    /// Read **DMAIN_TSE** `[1:1]`
    ///
    /// DMAIN_TSE
    #[inline(always)]
    pub const fn dmain_tse(&self) -> bool {
        (self.bits & DmainEn::DMAIN_TSE_MASK) != 0
    }

    /// Read **DMAIN_TIE** `[0:0]`
    ///
    /// DMAIN_TIE
    #[inline(always)]
    pub const fn dmain_tie(&self) -> bool {
        (self.bits & DmainEn::DMAIN_TIE_MASK) != 0
    }

    /// Set **DMAIN_NISE** `[16:16]`
    ///
    /// DMAIN_NISE
    #[inline(always)]
    pub const fn with_dmain_nise(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_NISE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_NISE_MASK
            },
        }
    }

    /// Set **DMAIN_AISE** `[15:15]`
    ///
    /// DMAIN_AISE
    #[inline(always)]
    pub const fn with_dmain_aise(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_AISE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_AISE_MASK
            },
        }
    }

    /// Set **DMAIN_ERIE** `[14:14]`
    ///
    /// DMAIN_ERIE
    #[inline(always)]
    pub const fn with_dmain_erie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_ERIE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_ERIE_MASK
            },
        }
    }

    /// Set **DMAIN_FBEE** `[13:13]`
    ///
    /// DMAIN_FBEE
    #[inline(always)]
    pub const fn with_dmain_fbee(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_FBEE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_FBEE_MASK
            },
        }
    }

    /// Set **DMAIN_ETIE** `[10:10]`
    ///
    /// DMAIN_ETIE
    #[inline(always)]
    pub const fn with_dmain_etie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_ETIE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_ETIE_MASK
            },
        }
    }

    /// Set **DMAIN_RWTE** `[9:9]`
    ///
    /// DMAIN_RWTE
    #[inline(always)]
    pub const fn with_dmain_rwte(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_RWTE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_RWTE_MASK
            },
        }
    }

    /// Set **DMAIN_RSE** `[8:8]`
    ///
    /// DMAIN_RSE
    #[inline(always)]
    pub const fn with_dmain_rse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_RSE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_RSE_MASK
            },
        }
    }

    /// Set **DMAIN_RBUE** `[7:7]`
    ///
    /// DMAIN_RBUE
    #[inline(always)]
    pub const fn with_dmain_rbue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_RBUE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_RBUE_MASK
            },
        }
    }

    /// Set **DMAIN_RIE** `[6:6]`
    ///
    /// DMAIN_RIE
    #[inline(always)]
    pub const fn with_dmain_rie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_RIE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_RIE_MASK
            },
        }
    }

    /// Set **DMAIN_UIE** `[5:5]`
    ///
    /// DMAIN_UIE
    #[inline(always)]
    pub const fn with_dmain_uie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_UIE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_UIE_MASK
            },
        }
    }

    /// Set **DMAIN_OIE** `[4:4]`
    ///
    /// DMAIN_OIE
    #[inline(always)]
    pub const fn with_dmain_oie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_OIE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_OIE_MASK
            },
        }
    }

    /// Set **DMAIN_TJTE** `[3:3]`
    ///
    /// DMAIN_TJTE
    #[inline(always)]
    pub const fn with_dmain_tjte(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_TJTE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_TJTE_MASK
            },
        }
    }

    /// Set **DMAIN_TBUE** `[2:2]`
    ///
    /// DMAIN_TBUE
    #[inline(always)]
    pub const fn with_dmain_tbue(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_TBUE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_TBUE_MASK
            },
        }
    }

    /// Set **DMAIN_TSE** `[1:1]`
    ///
    /// DMAIN_TSE
    #[inline(always)]
    pub const fn with_dmain_tse(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_TSE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_TSE_MASK
            },
        }
    }

    /// Set **DMAIN_TIE** `[0:0]`
    ///
    /// DMAIN_TIE
    #[inline(always)]
    pub const fn with_dmain_tie(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | DmainEn::DMAIN_TIE_MASK
            } else {
                self.bits & !DmainEn::DMAIN_TIE_MASK
            },
        }
    }
}

impl Default for DmainEnVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmainEn {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmainEnVal {
        DmainEnVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmainEnVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmainEnVal) -> DmainEnVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMAMISSEDFR Register
///
/// DMAMISSEDFR register
///
/// Address offset: 0x1020
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **Overflow_BFOC** `[28]` (RW)
///
///   Overflow_BFOC
///
/// - **Overflow_FC** `[27:17]` (RW)
///
///   Overflow_FC
///
/// - **Overflow_BMFC** `[16]` (RW)
///
///   Overflow_BMFC
///
/// - **Missed_FC** `[15:0]` (RW)
///
///   Missed_FC
#[repr(transparent)]
pub struct Dmamissedfr {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmamissedfr {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmamissedfr {
    // ======== Field Constants ========

    /// **Overflow_BFOC**
    ///
    /// Overflow_BFOC
    ///
    /// Bits `[28:28]`, mask `0x10000000`
    pub const OVERFLOW_BFOC_OFFSET: u32 = 28;
    pub const OVERFLOW_BFOC_MASK: u32 = 0x10000000;
    pub const OVERFLOW_BFOC: u32 = 1 << 28;

    /// **Overflow_FC**
    ///
    /// Overflow_FC
    ///
    /// Bits `[27:17]`, mask `0x0ffe0000`
    pub const OVERFLOW_FC_OFFSET: u32 = 17;
    pub const OVERFLOW_FC_MASK: u32 = 0x0ffe0000;

    /// **Overflow_BMFC**
    ///
    /// Overflow_BMFC
    ///
    /// Bits `[16:16]`, mask `0x00010000`
    pub const OVERFLOW_BMFC_OFFSET: u32 = 16;
    pub const OVERFLOW_BMFC_MASK: u32 = 0x00010000;
    pub const OVERFLOW_BMFC: u32 = 1 << 16;

    /// **Missed_FC**
    ///
    /// Missed_FC
    ///
    /// Bits `[15:0]`, mask `0x0000ffff`
    pub const MISSED_FC_OFFSET: u32 = 0;
    pub const MISSED_FC_MASK: u32 = 0x0000ffff;

    // ======== Field Accessors ========

    /// **Overflow_BFOC** `[28]` (RW)
    ///
    /// Overflow_BFOC
    #[inline(always)]
    pub fn overflow_bfoc(&self) -> bool {
        (self.value.read_bits() & Self::OVERFLOW_BFOC_MASK) != 0
    }

    /// Set Overflow_BFOC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_overflow_bfoc(&self) {
        self.value.modify_bits(|r| r | Self::OVERFLOW_BFOC_MASK);
    }

    /// Clear Overflow_BFOC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_overflow_bfoc(&self) {
        self.value.modify_bits(|r| r & !Self::OVERFLOW_BFOC_MASK);
    }

    /// **Overflow_FC** `[27:17]` (RW)
    ///
    /// Overflow_FC
    #[inline(always)]
    pub fn overflow_fc(&self) -> u32 {
        (self.value.read_bits() & Self::OVERFLOW_FC_MASK) >> Self::OVERFLOW_FC_OFFSET
    }

    /// Write **Overflow_FC** `[27:17]` (RW)
    ///
    /// Overflow_FC
    #[inline(always)]
    pub fn write_overflow_fc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::OVERFLOW_FC_MASK)
                | ((value << Self::OVERFLOW_FC_OFFSET) & Self::OVERFLOW_FC_MASK)
        });
    }

    /// **Overflow_BMFC** `[16]` (RW)
    ///
    /// Overflow_BMFC
    #[inline(always)]
    pub fn overflow_bmfc(&self) -> bool {
        (self.value.read_bits() & Self::OVERFLOW_BMFC_MASK) != 0
    }

    /// Set Overflow_BMFC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_overflow_bmfc(&self) {
        self.value.modify_bits(|r| r | Self::OVERFLOW_BMFC_MASK);
    }

    /// Clear Overflow_BMFC bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_overflow_bmfc(&self) {
        self.value.modify_bits(|r| r & !Self::OVERFLOW_BMFC_MASK);
    }

    /// **Missed_FC** `[15:0]` (RW)
    ///
    /// Missed_FC
    #[inline(always)]
    pub fn missed_fc(&self) -> u32 {
        (self.value.read_bits() & Self::MISSED_FC_MASK) >> Self::MISSED_FC_OFFSET
    }

    /// Write **Missed_FC** `[15:0]` (RW)
    ///
    /// Missed_FC
    #[inline(always)]
    pub fn write_missed_fc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::MISSED_FC_MASK) | ((value << Self::MISSED_FC_OFFSET) & Self::MISSED_FC_MASK)
        });
    }
}

/// DMAMISSEDFR value type for batch read/modify/write operations
///
/// Use `dmamissedfr.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmamissedfr.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmamissedfrVal {
    bits: u32,
}

impl DmamissedfrVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **Overflow_BFOC** `[28:28]`
    ///
    /// Overflow_BFOC
    #[inline(always)]
    pub const fn overflow_bfoc(&self) -> bool {
        (self.bits & Dmamissedfr::OVERFLOW_BFOC_MASK) != 0
    }

    /// Read **Overflow_FC** `[27:17]`
    ///
    /// Overflow_FC
    #[inline(always)]
    pub const fn overflow_fc(&self) -> u32 {
        (self.bits & Dmamissedfr::OVERFLOW_FC_MASK) >> Dmamissedfr::OVERFLOW_FC_OFFSET
    }

    /// Read **Overflow_BMFC** `[16:16]`
    ///
    /// Overflow_BMFC
    #[inline(always)]
    pub const fn overflow_bmfc(&self) -> bool {
        (self.bits & Dmamissedfr::OVERFLOW_BMFC_MASK) != 0
    }

    /// Read **Missed_FC** `[15:0]`
    ///
    /// Missed_FC
    #[inline(always)]
    pub const fn missed_fc(&self) -> u32 {
        (self.bits & Dmamissedfr::MISSED_FC_MASK) >> Dmamissedfr::MISSED_FC_OFFSET
    }

    /// Set **Overflow_BFOC** `[28:28]`
    ///
    /// Overflow_BFOC
    #[inline(always)]
    pub const fn with_overflow_bfoc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmamissedfr::OVERFLOW_BFOC_MASK
            } else {
                self.bits & !Dmamissedfr::OVERFLOW_BFOC_MASK
            },
        }
    }

    /// Set **Overflow_FC** `[27:17]`
    ///
    /// Overflow_FC
    #[inline(always)]
    pub const fn with_overflow_fc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmamissedfr::OVERFLOW_FC_MASK)
                | ((value << Dmamissedfr::OVERFLOW_FC_OFFSET) & Dmamissedfr::OVERFLOW_FC_MASK),
        }
    }

    /// Set **Overflow_BMFC** `[16:16]`
    ///
    /// Overflow_BMFC
    #[inline(always)]
    pub const fn with_overflow_bmfc(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmamissedfr::OVERFLOW_BMFC_MASK
            } else {
                self.bits & !Dmamissedfr::OVERFLOW_BMFC_MASK
            },
        }
    }

    /// Set **Missed_FC** `[15:0]`
    ///
    /// Missed_FC
    #[inline(always)]
    pub const fn with_missed_fc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmamissedfr::MISSED_FC_MASK)
                | ((value << Dmamissedfr::MISSED_FC_OFFSET) & Dmamissedfr::MISSED_FC_MASK),
        }
    }
}

impl Default for DmamissedfrVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmamissedfr {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmamissedfrVal {
        DmamissedfrVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmamissedfrVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmamissedfrVal) -> DmamissedfrVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMARINTWDTIMER Register
///
/// DMARINTWDTIMER register
///
/// Address offset: 0x1024
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **RIWTC** `[7:0]` (RW)
///
///   RIWTC
#[repr(transparent)]
pub struct Dmarintwdtimer {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmarintwdtimer {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmarintwdtimer {
    // ======== Field Constants ========

    /// **RIWTC**
    ///
    /// RIWTC
    ///
    /// Bits `[7:0]`, mask `0x000000ff`
    pub const RIWTC_OFFSET: u32 = 0;
    pub const RIWTC_MASK: u32 = 0x000000ff;

    // ======== Field Accessors ========

    /// **RIWTC** `[7:0]` (RW)
    ///
    /// RIWTC
    #[inline(always)]
    pub fn riwtc(&self) -> u32 {
        (self.value.read_bits() & Self::RIWTC_MASK) >> Self::RIWTC_OFFSET
    }

    /// Write **RIWTC** `[7:0]` (RW)
    ///
    /// RIWTC
    #[inline(always)]
    pub fn write_riwtc(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::RIWTC_MASK) | ((value << Self::RIWTC_OFFSET) & Self::RIWTC_MASK)
        });
    }
}

/// DMARINTWDTIMER value type for batch read/modify/write operations
///
/// Use `dmarintwdtimer.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmarintwdtimer.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmarintwdtimerVal {
    bits: u32,
}

impl DmarintwdtimerVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **RIWTC** `[7:0]`
    ///
    /// RIWTC
    #[inline(always)]
    pub const fn riwtc(&self) -> u32 {
        (self.bits & Dmarintwdtimer::RIWTC_MASK) >> Dmarintwdtimer::RIWTC_OFFSET
    }

    /// Set **RIWTC** `[7:0]`
    ///
    /// RIWTC
    #[inline(always)]
    pub const fn with_riwtc(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmarintwdtimer::RIWTC_MASK)
                | ((value << Dmarintwdtimer::RIWTC_OFFSET) & Dmarintwdtimer::RIWTC_MASK),
        }
    }
}

impl Default for DmarintwdtimerVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmarintwdtimer {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmarintwdtimerVal {
        DmarintwdtimerVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmarintwdtimerVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmarintwdtimerVal) -> DmarintwdtimerVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMAAHBSTATUS Register
///
/// DMAAHBSTATUS register
///
/// Address offset: 0x102c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **AHB_ST** `[0]` (RW)
///
///   AHB_ST
#[repr(transparent)]
pub struct Dmaahbstatus {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmaahbstatus {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmaahbstatus {
    // ======== Field Constants ========

    /// **AHB_ST**
    ///
    /// AHB_ST
    ///
    /// Bits `[0:0]`, mask `0x00000001`
    pub const AHB_ST_OFFSET: u32 = 0;
    pub const AHB_ST_MASK: u32 = 0x00000001;
    pub const AHB_ST: u32 = 1 << 0;

    // ======== Field Accessors ========

    /// **AHB_ST** `[0]` (RW)
    ///
    /// AHB_ST
    #[inline(always)]
    pub fn ahb_st(&self) -> bool {
        (self.value.read_bits() & Self::AHB_ST_MASK) != 0
    }

    /// Set AHB_ST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn set_ahb_st(&self) {
        self.value.modify_bits(|r| r | Self::AHB_ST_MASK);
    }

    /// Clear AHB_ST bit (RW) - NOT atomic
    #[inline(always)]
    pub fn clr_ahb_st(&self) {
        self.value.modify_bits(|r| r & !Self::AHB_ST_MASK);
    }
}

/// DMAAHBSTATUS value type for batch read/modify/write operations
///
/// Use `dmaahbstatus.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmaahbstatus.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmaahbstatusVal {
    bits: u32,
}

impl DmaahbstatusVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **AHB_ST** `[0:0]`
    ///
    /// AHB_ST
    #[inline(always)]
    pub const fn ahb_st(&self) -> bool {
        (self.bits & Dmaahbstatus::AHB_ST_MASK) != 0
    }

    /// Set **AHB_ST** `[0:0]`
    ///
    /// AHB_ST
    #[inline(always)]
    pub const fn with_ahb_st(self, value: bool) -> Self {
        Self {
            bits: if value {
                self.bits | Dmaahbstatus::AHB_ST_MASK
            } else {
                self.bits & !Dmaahbstatus::AHB_ST_MASK
            },
        }
    }
}

impl Default for DmaahbstatusVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmaahbstatus {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmaahbstatusVal {
        DmaahbstatusVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmaahbstatusVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmaahbstatusVal) -> DmaahbstatusVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMATXCURRDESC Register
///
/// DMATXCURRDESC register
///
/// Address offset: 0x1048
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Dmatxcurrdesc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmatxcurrdesc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmatxcurrdesc {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// DMATXCURRDESC value type for batch read/modify/write operations
///
/// Use `dmatxcurrdesc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmatxcurrdesc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmatxcurrdescVal {
    bits: u32,
}

impl DmatxcurrdescVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Dmatxcurrdesc::ADDRESS_LOW_MASK) >> Dmatxcurrdesc::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmatxcurrdesc::ADDRESS_LOW_MASK)
                | ((value << Dmatxcurrdesc::ADDRESS_LOW_OFFSET) & Dmatxcurrdesc::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for DmatxcurrdescVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmatxcurrdesc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmatxcurrdescVal {
        DmatxcurrdescVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmatxcurrdescVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmatxcurrdescVal) -> DmatxcurrdescVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMARXCURRDESC Register
///
/// DMARXCURRDESC register
///
/// Address offset: 0x104c
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct Dmarxcurrdesc {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for Dmarxcurrdesc {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Dmarxcurrdesc {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// DMARXCURRDESC value type for batch read/modify/write operations
///
/// Use `dmarxcurrdesc.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmarxcurrdesc.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmarxcurrdescVal {
    bits: u32,
}

impl DmarxcurrdescVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & Dmarxcurrdesc::ADDRESS_LOW_MASK) >> Dmarxcurrdesc::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !Dmarxcurrdesc::ADDRESS_LOW_MASK)
                | ((value << Dmarxcurrdesc::ADDRESS_LOW_OFFSET) & Dmarxcurrdesc::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for DmarxcurrdescVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl Dmarxcurrdesc {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmarxcurrdescVal {
        DmarxcurrdescVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmarxcurrdescVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmarxcurrdescVal) -> DmarxcurrdescVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMATXCURRADDR_BUF Register
///
/// DMATXCURRADDR_BUF register
///
/// Address offset: 0x1050
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct DmatxcurraddrBuf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmatxcurraddrBuf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmatxcurraddrBuf {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// DMATXCURRADDR_BUF value type for batch read/modify/write operations
///
/// Use `dmatxcurraddrbuf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmatxcurraddrbuf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmatxcurraddrBufVal {
    bits: u32,
}

impl DmatxcurraddrBufVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & DmatxcurraddrBuf::ADDRESS_LOW_MASK) >> DmatxcurraddrBuf::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmatxcurraddrBuf::ADDRESS_LOW_MASK)
                | ((value << DmatxcurraddrBuf::ADDRESS_LOW_OFFSET)
                    & DmatxcurraddrBuf::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for DmatxcurraddrBufVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmatxcurraddrBuf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmatxcurraddrBufVal {
        DmatxcurraddrBufVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmatxcurraddrBufVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmatxcurraddrBufVal) -> DmatxcurraddrBufVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// DMARXCURRADDR_BUF Register
///
/// DMARXCURRADDR_BUF register
///
/// Address offset: 0x1054
/// Reset value: 0x00000000
///
/// ## Fields
///
/// - **ADDRESS_LOW** `[31:0]` (RW)
///
///   ADDRESS_LOW
#[repr(transparent)]
pub struct DmarxcurraddrBuf {
    value: crate::common::VolatileCell<u32>,
}

impl core::ops::Deref for DmarxcurraddrBuf {
    type Target = crate::common::VolatileCell<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl DmarxcurraddrBuf {
    // ======== Field Constants ========

    /// **ADDRESS_LOW**
    ///
    /// ADDRESS_LOW
    ///
    /// Bits `[31:0]`, mask `0xffffffff`
    pub const ADDRESS_LOW_OFFSET: u32 = 0;
    pub const ADDRESS_LOW_MASK: u32 = 0xffffffff;

    // ======== Field Accessors ========

    /// **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn address_low(&self) -> u32 {
        (self.value.read_bits() & Self::ADDRESS_LOW_MASK) >> Self::ADDRESS_LOW_OFFSET
    }

    /// Write **ADDRESS_LOW** `[31:0]` (RW)
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub fn write_address_low(&self, value: u32) {
        self.value.modify_bits(|r| {
            (r & !Self::ADDRESS_LOW_MASK)
                | ((value << Self::ADDRESS_LOW_OFFSET) & Self::ADDRESS_LOW_MASK)
        });
    }
}

/// DMARXCURRADDR_BUF value type for batch read/modify/write operations
///
/// Use `dmarxcurraddrbuf.read_val()` to read the register into this struct,
/// modify fields with builder methods, then write back with `dmarxcurraddrbuf.write_val(val)`.
#[derive(Clone, Copy)]
pub struct DmarxcurraddrBufVal {
    bits: u32,
}

impl DmarxcurraddrBufVal {
    /// Create from raw bits
    #[inline(always)]
    pub const fn from_bits(bits: u32) -> Self {
        Self { bits }
    }

    /// Get the raw bits
    #[inline(always)]
    pub const fn bits(&self) -> u32 {
        self.bits
    }

    /// Create with reset value
    #[inline(always)]
    pub const fn reset_val() -> Self {
        Self { bits: 0x00000000 }
    }

    /// Read **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn address_low(&self) -> u32 {
        (self.bits & DmarxcurraddrBuf::ADDRESS_LOW_MASK) >> DmarxcurraddrBuf::ADDRESS_LOW_OFFSET
    }

    /// Set **ADDRESS_LOW** `[31:0]`
    ///
    /// ADDRESS_LOW
    #[inline(always)]
    pub const fn with_address_low(self, value: u32) -> Self {
        Self {
            bits: (self.bits & !DmarxcurraddrBuf::ADDRESS_LOW_MASK)
                | ((value << DmarxcurraddrBuf::ADDRESS_LOW_OFFSET)
                    & DmarxcurraddrBuf::ADDRESS_LOW_MASK),
        }
    }
}

impl Default for DmarxcurraddrBufVal {
    fn default() -> Self {
        Self::reset_val()
    }
}

impl DmarxcurraddrBuf {
    /// Read the register into a value struct
    #[inline(always)]
    pub fn read_val(&self) -> DmarxcurraddrBufVal {
        DmarxcurraddrBufVal::from_bits(self.value.read_bits())
    }

    /// Write a value struct to the register
    #[inline(always)]
    pub fn write_val(&self, val: DmarxcurraddrBufVal) {
        self.value.write_bits(val.bits())
    }

    /// Modify the register using a value struct
    #[inline(always)]
    pub fn modify_val<F: FnOnce(DmarxcurraddrBufVal) -> DmarxcurraddrBufVal>(&self, f: F) {
        self.write_val(f(self.read_val()))
    }
}

/// Register block
#[repr(C)]
pub struct RegisterBlock {
    /// 0x0000
    /// EMACCONFIG register
    pub emacconfig: Emacconfig,
    /// 0x0004
    /// EMACFF register
    pub emacff: Emacff,
    _reserved0: [u8; 8],
    /// 0x0010
    /// EMACMIIADDR register
    pub emacmiiaddr: Emacmiiaddr,
    /// 0x0014
    /// EMACMIIDATA register
    pub emacmiidata: Emacmiidata,
    /// 0x0018
    /// EMACFC register
    pub emacfc: Emacfc,
    /// 0x001c
    /// EMACVLANTAG register
    pub emacvlantag: Emacvlantag,
    _reserved1: [u8; 4],
    /// 0x0024
    /// EMACDEBUG register
    pub emacdebug: Emacdebug,
    /// 0x0028
    /// PMT_RWUFFR register
    pub pmt_rwuffr: PmtRwuffr,
    /// 0x002c
    /// PMT_CSR register
    pub pmt_csr: PmtCsr,
    /// 0x0030
    /// EMACLPI_CSR register
    pub emaclpi_csr: EmaclpiCsr,
    /// 0x0034
    /// EMACLPITIMERSCONTROL register
    pub emaclpitimerscontrol: Emaclpitimerscontrol,
    /// 0x0038
    /// EMACINTS register
    pub emacints: Emacints,
    /// 0x003c
    /// EMACINTMASK register
    pub emacintmask: Emacintmask,
    /// 0x0040
    /// EMACADDR0HIGH register
    pub emacaddr0high: Emacaddr0high,
    /// 0x0044
    /// EMACADDR0LOW register
    pub emacaddr0low: Emacaddr0low,
    /// 0x0048
    /// EMACADDR1HIGH register
    pub emacaddr1high: Emacaddr1high,
    /// 0x004c
    /// EMACADDR1LOW register
    pub emacaddr1low: Emacaddr1low,
    /// 0x0050
    /// EMACADDR2HIGH register
    pub emacaddr2high: Emacaddr2high,
    /// 0x0054
    /// EMACADDR2LOW register
    pub emacaddr2low: Emacaddr2low,
    /// 0x0058
    /// EMACADDR3HIGH register
    pub emacaddr3high: Emacaddr3high,
    /// 0x005c
    /// EMACADDR3LOW register
    pub emacaddr3low: Emacaddr3low,
    /// 0x0060
    /// EMACADDR4HIGH register
    pub emacaddr4high: Emacaddr4high,
    /// 0x0064
    /// EMACADDR4LOW register
    pub emacaddr4low: Emacaddr4low,
    /// 0x0068
    /// EMACADDR5HIGH register
    pub emacaddr5high: Emacaddr5high,
    /// 0x006c
    /// EMACADDR5LOW register
    pub emacaddr5low: Emacaddr5low,
    /// 0x0070
    /// EMACADDR6HIGH register
    pub emacaddr6high: Emacaddr6high,
    /// 0x0074
    /// EMACADDR6LOW register
    pub emacaddr6low: Emacaddr6low,
    /// 0x0078
    /// EMACADDR7HIGH register
    pub emacaddr7high: Emacaddr7high,
    /// 0x007c
    /// EMACADDR7LOW register
    pub emacaddr7low: Emacaddr7low,
    /// 0x0080
    /// EMACADDR8HIGH register
    pub emacaddr8high: Emacaddr8high,
    /// 0x0084
    /// EMACADDR8LOW register
    pub emacaddr8low: Emacaddr8low,
    _reserved2: [u8; 80],
    /// 0x00d8
    /// EMACCSTATUS register
    pub emaccstatus: Emaccstatus,
    /// 0x00dc
    /// EMACWDOGTO register
    pub emacwdogto: Emacwdogto,
    _reserved3: [u8; 1188],
    /// 0x0584
    /// EMACTXVLTCTRL register
    pub emactxvltctrl: Emactxvltctrl,
    _reserved4: [u8; 376],
    /// 0x0700
    /// EMACTSTPCTRL register
    pub emactstpctrl: Emactstpctrl,
    /// 0x0704
    /// EMACSUB2NDINCR register
    pub emacsub2ndincr: Emacsub2ndincr,
    /// 0x0708
    /// EMACSYSTIM2ND register
    pub emacsystim2nd: Emacsystim2nd,
    /// 0x070c
    /// EMACSYSTIMNS register
    pub emacsystimns: Emacsystimns,
    /// 0x0710
    /// EMAC2NDUPT register
    pub emac2ndupt: Emac2ndupt,
    /// 0x0714
    /// EMACNSUPT register
    pub emacnsupt: Emacnsupt,
    /// 0x0718
    /// EMACTSTPADDEND register
    pub emactstpaddend: Emactstpaddend,
    /// 0x071c
    /// EMACTGTIME2ND register
    pub emactgtime2nd: Emactgtime2nd,
    /// 0x0720
    /// EMACTGTIMENS register
    pub emactgtimens: Emactgtimens,
    /// 0x0724
    /// EMACSYSTIMHIGHWORD2ND register
    pub emacsystimhighword2nd: Emacsystimhighword2nd,
    /// 0x0728
    /// EMACTSTPSTATUS register
    pub emactstpstatus: Emactstpstatus,
    _reserved5: [u8; 2260],
    /// 0x1000
    /// DMABUSMODE register
    pub dmabusmode: Dmabusmode,
    /// 0x1004
    /// DMATXPOLLDEMAND register
    pub dmatxpolldemand: Dmatxpolldemand,
    /// 0x1008
    /// DMARXPOLLDEMAND register
    pub dmarxpolldemand: Dmarxpolldemand,
    /// 0x100c
    /// DMARXBASEADDR register
    pub dmarxbaseaddr: Dmarxbaseaddr,
    /// 0x1010
    /// DMATXBASEADDR register
    pub dmatxbaseaddr: Dmatxbaseaddr,
    /// 0x1014
    /// DMASTATUS register
    pub dmastatus: Dmastatus,
    /// 0x1018
    /// DMAOPERATION_MODE register
    pub dmaoperation_mode: DmaoperationMode,
    /// 0x101c
    /// DMAIN_EN register
    pub dmain_en: DmainEn,
    /// 0x1020
    /// DMAMISSEDFR register
    pub dmamissedfr: Dmamissedfr,
    /// 0x1024
    /// DMARINTWDTIMER register
    pub dmarintwdtimer: Dmarintwdtimer,
    _reserved6: [u8; 4],
    /// 0x102c
    /// DMAAHBSTATUS register
    pub dmaahbstatus: Dmaahbstatus,
    _reserved7: [u8; 24],
    /// 0x1048
    /// DMATXCURRDESC register
    pub dmatxcurrdesc: Dmatxcurrdesc,
    /// 0x104c
    /// DMARXCURRDESC register
    pub dmarxcurrdesc: Dmarxcurrdesc,
    /// 0x1050
    /// DMATXCURRADDR_BUF register
    pub dmatxcurraddr_buf: DmatxcurraddrBuf,
    /// 0x1054
    /// DMARXCURRADDR_BUF register
    pub dmarxcurraddr_buf: DmarxcurraddrBuf,
}
