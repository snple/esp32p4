/* ESP32-P4 application memory layout.
 *
 * ROM is the flash window mapped by the 2nd-stage bootloader. Offset 0x20
 * skips the app image header; the first segment (app descriptor + rodata)
 * starts there.
 *
 * RAM starts after the 128 KB L2 cache that the bootloader configures by
 * default and ends at 0x4FFADFC0, leaving the ROM BSS/stack area untouched.
 */
MEMORY
{
    ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
    RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
}
