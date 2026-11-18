//! Minimal ESP-IDF application descriptor.
//!
//! `espflash` looks for a symbol named `esp_app_desc` or a `.rodata_desc` /
//! `.flash.appdesc` section before it will build an ESP-IDF application image.
//! The descriptor lives in `.flash.appdesc`, which the linker script places in
//! the first flash segment so the ESP-IDF bootloader can find it.
//! The 2nd-stage bootloader does not otherwise require the descriptor unless
//! secure boot or anti-rollback is enabled, so a valid magic word and zeroed
//! fields are enough for a bare-metal application.

/// `ESP_APP_DESC_MAGIC_WORD`.
const ESP_APP_DESC_MAGIC_WORD: u32 = 0xABCD_5432;

/// Layout of `esp_app_desc_t`.
#[repr(C, packed)]
pub struct AppDescriptor {
    magic_word: u32,
    secure_version: u32,
    reserv1: [u32; 2],
    version: [u8; 32],
    project_name: [u8; 32],
    time: [u8; 16],
    date: [u8; 16],
    idf_ver: [u8; 32],
    app_elf_sha256: [u8; 32],
    min_efuse_blk_rev_full: u16,
    max_efuse_blk_rev_full: u16,
    mmu_page_size: u8,
    reserv3: [u8; 3],
    reserv2: [u32; 18],
}

/// Application descriptor recognised by espflash.
#[used]
#[unsafe(no_mangle)]
#[unsafe(link_section = ".flash.appdesc")]
pub static esp_app_desc: AppDescriptor = AppDescriptor {
    magic_word: ESP_APP_DESC_MAGIC_WORD,
    secure_version: 0,
    reserv1: [0; 2],
    version: *b"0.1.0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    project_name: *b"esp32p4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    time: *b"00:00:00\0\0\0\0\0\0\0\0",
    date: *b"Jan 01 2026\0\0\0\0\0",
    idf_ver: *b"esp32p4-hal\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
    app_elf_sha256: [0; 32],
    min_efuse_blk_rev_full: 0,
    max_efuse_blk_rev_full: 0xffff,
    mmu_page_size: 0,
    reserv3: [0; 3],
    reserv2: [0; 18],
};
