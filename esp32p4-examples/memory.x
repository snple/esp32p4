/* ESP32-P4 application memory layout, v3.0+ die.
 *
 * ROM is the flash window mapped by the 2nd-stage bootloader. Offset 0x20
 * skips the app image header; the first segment (app descriptor + rodata)
 * starts there.
 *
 * RAM starts after the 128 KB L2 cache the bootloader carves off the bottom of
 * L2MEM and ends at 0x4FFADFC0, the 2nd-stage bootloader's iram_loader_seg.
 * Above that sits the mask ROM's own .data/.bss at 0x4FFBFBB0..0x4FFC0000, so
 * 0x4FFADFC0 is also where the ROM window starts: nothing of ours may cross it.
 *
 * This is the v3.0+ (ECO5 and later) layout. A pre-v3.0 die - v1.x, revision
 * below 300, which is the board this repository was developed on - puts both
 * holes elsewhere: the L2 cache takes the TOP 128 KB (0x4FFA_0000..0x4FFC_0000)
 * and the ROM keeps its data at 0x4FF3_FBA4..0x4FF4_0000, in the middle of the
 * range below. On such a die a `.bss` larger than about 130 KB is zeroed by
 * `_start` straight through the ROM's data, and the next ROM call never comes
 * back. The values for that die are in docs/boot.md §8.
 */
MEMORY
{
    ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
    RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
    /* 8 KB HP scratchpad ("TCM"): direct, uncached code and data. */
    SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
}
