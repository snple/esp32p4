/* ESP32-P4 application memory layout.
 *
 * ROM is the flash window mapped by the 2nd-stage bootloader. Offset 0x20
 * skips the app image header; the first segment (app descriptor + rodata)
 * starts there.
 *
 * RAM is the 768 KB HP L2MEM minus what the L2 cache and the mask ROM claim.
 * Where those holes fall depends on the chip revision, and espflash flashes a
 * bootloader built for the revision it finds. This board reports v1.3
 * (revision 103), a pre-v3.0 die, where
 *
 *   - the L2 cache takes the *top* 128 KB, 0x4FFA_0000..0x4FFC_0000, and
 *   - the ROM keeps its .data/.bss in the middle, at 0x4FF3_FBA4..0x4FF4_0000.
 *
 * Both were measured on the board (see the `ram_map` example), and both break
 * a single contiguous region: zeroing the ROM's window makes the next ROM call
 * hang, and writes into cache memory do not survive cache traffic. What is
 * left is two windows:
 *
 *   RAM       0x4FF0_0000..0x4FF2_BBD0  below the 2nd-stage bootloader
 *   RAM_HIGH  0x4FF4_0000..0x4FFA_0000  between the ROM window and the cache
 *
 * `.bss`, `.data` and the stack live in RAM; a buffer too big for it (up to
 * 384 KB) goes in RAM_HIGH by naming the `.ram.high` section.
 *
 * A v3.0+ die moves both holes - cache to the bottom, ROM data to the top - so
 * these numbers are only right for the die they were measured on. See
 * docs/boot.md before changing them.
 */
MEMORY
{
    ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
    RAM : ORIGIN = 0x4FF00000, LENGTH = 0x2BBD0
    RAM_HIGH : ORIGIN = 0x4FF40000, LENGTH = 0x60000
    /* 8 KB HP scratchpad ("TCM"): direct, uncached code and data. */
    SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
}
