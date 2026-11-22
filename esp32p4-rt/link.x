/*
 * ESP32-P4 application linker script.
 *
 * The ESP-IDF 2nd-stage bootloader maps two flash segments (DROM + IROM) and
 * copies every RAM segment to its virtual address before jumping to `_start`.
 *
 * Layout:
 *   segment 0 (R)  : `.flash.appdesc` + `.rodata`  (DROM, mapped from flash)
 *   segment 1 (RX) : `.init` + `.text` + `.trap`   (IROM, mapped from flash)
 *   RAM segments   : `.ramfunc` and `.tcm.text` / `.tcm.data` (copied by the
 *                    bootloader), `.data` (copied by the bootloader),
 *                    `.bss` (zeroed here)
 *
 * The bootloader copies a segment to whatever load address the image names, so
 * `.ramfunc` (in `RAM`) and `.tcm.text` (in `SPM`) are ordinary RAM segments:
 * `#[ramfunc]` and `#[tcmfunc]` functions live in flash in the image and run
 * from their RAM address.
 *
 * The `SPM` region is the 8 KB HP scratchpad at 0x3010_0000, which the register
 * set calls the HP TCM. `memory.x` has to define it:
 *
 *     MEMORY
 *     {
 *         ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
 *         RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
 *         SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
 *     }
 *
 * The L2 cache and the mask ROM's own `.data`/`.bss` both live inside L2MEM, at
 * addresses that depend on the chip revision, so `memory.x` — not this script —
 * is what has to keep those two out of `RAM`. See `docs/boot.md` §3 and §8.
 */

ENTRY(_start)
EXTERN(main)

INCLUDE memory.x

SECTIONS
{
    /* Segment 0: the app descriptor must come first, followed by rodata so
     * that both merge into the DROM segment. */
    .flash.appdesc : ALIGN(4)
    {
        KEEP(*(.flash.appdesc))
    } > ROM

    .rodata : ALIGN(4)
    {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    } > ROM

    /* Force a gap so the DROM and IROM segments stay separate; the ESP32-P4
     * bootloader expects exactly two flash segments. This NOLOAD section is
     * not emitted into the app image, but it advances the location counter
     * and keeps espflash from merging the two flash segments (its merge
     * tolerance is 4 bytes). */
    .drom_gap (NOLOAD) : ALIGN(0x100)
    {
        . += 0x10;
    } > ROM

    .text : ALIGN(0x100)
    {
        KEEP(*(.init))
        *(.text .text.*)
        *(.trap .trap.*)
    } > ROM

    /* Functions marked `#[ramfunc]`: copied to SRAM by the bootloader, so they
     * keep running while the flash cache is suspended. */
    .ramfunc : ALIGN(4)
    {
        _sramfunc = .;
        KEEP(*(.ramfunc .ramfunc.*))
        . = ALIGN(4);
        _eramfunc = .;
    } > RAM

    /* Functions and data marked `#[tcmfunc]` / `#[tcmdata]`: the 8 KB HP
     * scratchpad, reachable over the CPU's own instruction and data buses with
     * a two-cycle latency and no cache in the way. */
    .tcm.text : ALIGN(4)
    {
        _stcmtext = .;
        KEEP(*(.tcm.text .tcm.text.*))
        . = ALIGN(4);
        _etcmtext = .;
    } > SPM

    .tcm.data : ALIGN(4)
    {
        _stcmdata = .;
        KEEP(*(.tcm.data .tcm.data.*))
        . = ALIGN(4);
        _etcmdata = .;
    } > SPM

    /* Everything past this point is not looked at by the TCM code. */

    .data : ALIGN(4)
    {
        _sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4);
        _edata = .;
    } > RAM

    .bss (NOLOAD) : ALIGN(4)
    {
        _sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .stack (NOLOAD) : ALIGN(16)
    {
        _stack_end = .;
        . += 0x4000;
        . = ALIGN(16);
        _stack_start = .;
    } > RAM

    ASSERT(_stack_start <= ORIGIN(RAM) + LENGTH(RAM), "esp32p4-rt: stack overflows RAM")
    ASSERT(_etcmdata <= ORIGIN(SPM) + LENGTH(SPM),
           "esp32p4-rt: #[tcmfunc]/#[tcmdata] overflows the 8 KB HP scratchpad")

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.comment)
    }

    /* Metadata espflash reads from the ELF rather than from the image: the
     * `espflash.LOG_FORMAT` symbol produced by the `defmt` USB logger lives
     * here and tells espflash to decode the serial stream as `defmt` frames.
     * Non-allocated, so it is not part of the flashed application. */
    .espressif.metadata 0 (INFO) :
    {
        KEEP(*(.espressif.metadata));
    }
}

PROVIDE(__global_pointer$ = MIN(_sdata + 0x800, _edata));

/* Mask ROM entry points used by the PSRAM bring-up and cache maintenance
 * (addresses from ESP-IDF's `esp32p4.rom.ld`; the ROM lives at 0x4FC0_0000). */
PROVIDE(esp_rom_spi_cmd_config = 0x4fc00108);
PROVIDE(esp_rom_spi_set_op_mode = 0x4fc00110);
PROVIDE(Cache_Invalidate_Addr = 0x4fc003e4);
PROVIDE(Cache_WriteBack_Addr = 0x4fc003f4);
PROVIDE(Cache_WriteBack_Invalidate_Addr = 0x4fc003fc);
PROVIDE(Cache_Suspend_L1_CORE0_ICache = 0x4fc004d8);
PROVIDE(Cache_Resume_L1_CORE0_ICache = 0x4fc004dc);
PROVIDE(Cache_Suspend_L1_CORE1_ICache = 0x4fc004e8);
PROVIDE(Cache_Resume_L1_CORE1_ICache = 0x4fc004ec);
PROVIDE(Cache_Suspend_L1_DCache = 0x4fc004f8);
PROVIDE(Cache_Resume_L1_DCache = 0x4fc004fc);
PROVIDE(Cache_Suspend_L2_Cache = 0x4fc00508);
PROVIDE(Cache_Resume_L2_Cache = 0x4fc0050c);
