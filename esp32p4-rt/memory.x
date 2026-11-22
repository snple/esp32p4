/* Default ESP32-P4 memory layout for a v3.0+ die: 128 KB of L2MEM reserved for
 * the L2 cache at the bottom, the mask ROM's data above 0x4FFADFC0.
 * Applications may copy this file next to their build.rs and adjust it; a
 * pre-v3.0 die needs the two-window layout in docs/boot.md §8 instead. */
MEMORY
{
    RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
    /* 8 KB HP scratchpad ("TCM"): direct, uncached code and data. */
    SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
}
