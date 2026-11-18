# ESP32-P4 启动与镜像布局

本文记录 `esp32p4-rt` 为什么这样写 `link.x`，以及 `espflash` 生成的镜像
在 P4 上如何被 2nd-stage bootloader 接受。内容来自 ESP-IDF 源码
（`components/bootloader_support/src/bootloader_utility.c`、
`components/esp_image_verify/src/esp_image_format.c`）和 TRM。

## 1. 谁加载应用

ESP32-P4 的 ROM 先运行 bootloader，bootloader 再从 app 分区加载应用。
它不是 “ROM 直接跳到 ELF entry” 的裸机流程，因此应用镜像必须符合
ESP-IDF 的 app image 格式。

`espflash` 负责：

- 生成 bootloader + 分区表 + app image；
- 写 app image header、segment header、checksum、SHA256；
- 在 `.flash.appdesc` 存在时，把它所在的段排到第一位并回填
  `app_elf_sha256`。

## 2. bootloader 对 P4 应用的两个硬性要求

### 2.1 恰好两个 flash 段

`unpack_load_app()` 在 `SOC_MMU_DI_VADDR_SHARED`（P4）下：

```c
for (每个 segment) {
    if (load_addr 在 DROM/IROM 范围) rom_index++;
}
assert(rom_index == 2);
```

所以镜像必须有 **两个** load address 位于 `0x4000_0000..0x43FF_FFFF` 的段：

- DROM：app descriptor + `.rodata`
- IROM：`.init` + `.text` + `.trap`

只有 RAM 段的镜像会在 bootloader 里触发 assert。

`espflash` 的 `merge_adjacent_segments()` 会把地址连续的段合并（间隙
≤ 4 字节也会补 0 后合并）。因此 `link.x` 在两个 flash 段之间放了一个
**NOLOAD** 的 `.drom_gap` 段：它不进入镜像，但会把 `.text` 推到下一个
`0x100` 边界，保证 DROM 和 IROM 不会被合并。

### 2.2 第一个段必须有 app descriptor

`verify_segment_header()` 对 segment 0 检查：

- `data_len >= sizeof(esp_app_desc_t)`（256 字节）；
- 段数据开头的 `magic_word == 0xABCD5432`；
- 如果 `mmu_page_size` 字段非 0，使用该值；为 0 时使用传统默认页大小；
- `(segment_data_offset % page_size) == (load_addr % page_size)`。

`esp32p4-rt/src/app_desc.rs` 提供 256 字节的 `esp_app_desc_t`，通过
`#[unsafe(link_section = ".flash.appdesc")]` 放到第一个段的最前面。
`espflash` 还会把 `app_elf_sha256` 回填到这个结构里。

## 3. 当前布局

`esp32p4-examples/memory.x`：

```ld
MEMORY
{
    ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
    RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
}
```

`0x40000020` 跳过 24 字节 image header + 8 字节第一个 segment header。
`0x4FF20000` 是 bootloader 默认保留 128 KB L2 cache 后的 L2 RAM 起点；
`0x4FFADFC0` 是可用 RAM 顶端（上方约 72 KB 留给 ROM BSS/stack）。

输出段顺序：

```text
.flash.appdesc   ROM  0x40000020   (256 B)
.rodata          ROM  紧随其后
.drom_gap        ROM  NOLOAD，仅推进地址
.text            ROM  下一个 0x100 边界
.data            RAM  bootloader 拷贝
.bss             RAM  _start 清零
.stack           RAM  NOLOAD
```

debug 构建的典型镜像（`espflash save-image` 解析结果）：

```text
seg0: addr=0x40000020 len=0x600   # appdesc + rodata
seg1: addr=0x00000000 len=0x1d0   # espflash 填充段，bootloader 忽略
seg2: addr=0x40000800 len=0x1120  # text
```

`unpack_load_app` 只会数到 seg0 和 seg2，满足 `rom_index == 2`。
seg1 的 `addr=0` 不在 RAM/flash 范围，`should_load/should_map` 都为 false。

## 4. `.data` 和 `.bss`

- `.data`：VMA 在 RAM，espflash 把它作为 RAM segment 写进 app image；
  bootloader 在验证阶段 `memcpy` 到 VMA。因此 `_start` **不**需要从 flash
  拷贝 `.data`。
- `.bss`：NOLOAD，不进入镜像；`_start` 用 `sw zero` 清零。
- `.stack`：NOLOAD，`_stack_start` 在段尾。

如果以后把 `.data` 的 LMA 放到 flash，需要自己加 `AT > ROM` 并在 `_start`
里拷贝；同时注意 espflash 的 `load_address()` 会把 LMA 在 flash 的段按 VMA
（RAM）作为 load address，bootloader 仍会拷贝一次。

## 5. 启动汇编

```text
_start:
    gp = __global_pointer$
    sp = _stack_start
    zero .bss
    mtvec = _start_trap
    mie = 0; mstatus.MIE = 0
    mstatus.FS = Dirty
    call main
    loop
```

P4 的 CLIC 支持 direct mode，`mtvec.MODE=0` 是合法的。当前不使用中断，
所以一个 trap 入口足够。

## 6. bootloader 交接到应用时的状态

应用入口不是“干净复位”，有几处 bootloader 留下的状态必须处理（`startup::init()`
在 `#[main]` 里做）：

| 状态 | bootloader 行为 | 应用必须做什么 |
|------|-----------------|----------------|
| RTC 看门狗（`LP_WDT`） | `CONFIG_BOOTLOADER_WDT_ENABLE` 默认 `y`，以 9 s 超时启动（`CONFIG_BOOTLOADER_WDT_TIME_MS`）后**不关闭** | 解锁 `WPROTECT` → 清 `CONFIG0.WDT_EN` → 重新加锁。否则约 9 s 后被复位 |
| SWD（超级看门狗） | `bootloader_super_wdt_auto_feed()` 已设置 `SWD_AUTO_FEED_EN` | 无需处理 |
| TIMG0/TIMG1 MWDT | 只关闭 flashboot 保护，不保证定时器停止 | 防御性关闭（先开 `PERI_CLK_CTRL{20,21}` 的 WDT 时钟） |
| core 1 | 只启动 core 0 | 停放 core 1：`PMU.cpu_sw_stall=0x86`、清 `CORE1_CPU_CLK_EN`、置 `RST_EN_CORE1_GLOBAL` |
| 分支预测器 | 保持关闭（cache 配置期间不能用） | 可选：`csrs 0x7c1` 打开，只影响性能 |
| `.data` 段 | 已 `memcpy` 到 VMA | 无需拷贝 |
| `mtvec`/`mie`/`mstatus.FS` | 未替应用设置 | `_start` 设置 |

`CONFIG_BOOTLOADER_WDT_DISABLE_IN_USER_CODE` 默认 `n`，含义是“由 ESP-IDF 应用关闭”，
不是“bootloader 已关闭”——所以裸机程序必须自己关。

## 7. 验证镜像

```bash
cd esp32p4-examples
cargo build --bin blink
espflash save-image --chip esp32p4 \
  ../target/riscv32imafc-unknown-none-elf/debug/blink /tmp/blink.bin
```

可以用下面的脚本检查段数和 entry：

```python
import struct
b = open('/tmp/blink.bin', 'rb').read()
magic, segcount, _, _, entry = struct.unpack_from('<BBBBI', b, 0)
print(f"magic={magic:#x} segments={segcount} entry={entry:#x}")
off = 24
for i in range(segcount):
    addr, length = struct.unpack_from('<II', b, off)
    off += 8
    print(f"  seg{i}: addr={addr:#x} len={length:#x}")
    off += (length + 3) & ~3
```

期望：

- `magic == 0xe9`；
- flash 段（`addr` 在 `0x4000_0000..0x43FF_FFFF`）恰好 2 个；
- 第一个 flash 段以 `32 54 cd ab`（`0xABCD5432` 小端）开头；
- `entry` 落在第二个 flash 段内。

## 8. 换 bootloader / cache 配置

如果使用自定义 ESP-IDF bootloader，L2 cache 划分可能不是 128 KB。此时要
同步修改 `memory.x` 的 `RAM.ORIGIN`：

| L2 cache | L2 RAM ORIGIN |
|----------|---------------|
| 512 KB | `0x4FF80000` |
| 256 KB | `0x4FF40000` |
| 128 KB（IDF 默认） | `0x4FF20000` |
| 0 KB | `0x4FF00000` |

RAM 顶端仍建议不超过 `0x4FFADFC0`。
