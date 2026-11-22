/* Default ESP32-P4 memory layout for a pre-v3.0 die (v1.x, revision < 300).
 * Applications may copy this file next to their build.rs and adjust it; see
 * docs/boot.md for the v3.0+ die, which keeps no hole between the two windows.
 *
 * Internal SRAM is not contiguous: the L2 cache holds the top 128 KB and the
 * mask ROM keeps its own .data/.bss at 0x4FF3_FBA4..0x4FF4_0000, so `RAM` stops
 * below the 2nd-stage bootloader and `RAM_HIGH` starts past the ROM's window. */
MEMORY
{
    RAM : ORIGIN = 0x4FF00000, LENGTH = 0x2BBD0
    RAM_HIGH : ORIGIN = 0x4FF40000, LENGTH = 0x60000
    /* 8 KB HP scratchpad ("TCM"): direct, uncached code and data. */
    SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
}
