/*
 * ESP32-P4 application linker script.
 *
 * The ESP-IDF 2nd-stage bootloader maps two flash segments (DROM + IROM) and
 * copies every RAM segment to its virtual address before jumping to `_start`.
 *
 * Layout:
 *   segment 0 (R)  : `.flash.appdesc` + `.rodata`  (DROM, mapped from flash)
 *   segment 1 (RX) : `.init` + `.text` + `.trap`   (IROM, mapped from flash)
 *   RAM segments   : `.data` (copied by the bootloader), `.bss` (zeroed here)
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

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.eh_frame_hdr)
        *(.comment)
    }
}

PROVIDE(__global_pointer$ = MIN(_sdata + 0x800, _edata));
