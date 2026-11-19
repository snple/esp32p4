/* Default ESP32-P4 memory layout: 128 KB reserved for L2 cache (IDF default).
 * Applications may copy this file next to their build.rs and adjust it. */
MEMORY
{
    RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
    /* 8 KB HP scratchpad ("TCM"): direct, uncached code and data. */
    SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
}
