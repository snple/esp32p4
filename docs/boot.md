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
    RAM : ORIGIN = 0x4FF00000, LENGTH = 0x2BBD0
    RAM_HIGH : ORIGIN = 0x4FF40000, LENGTH = 0x60000
    SPM : ORIGIN = 0x30100000, LENGTH = 0x2000
}
```

`0x40000020` 跳过 24 字节 image header + 8 字节第一个 segment header。

`0x30100000` 是 8 KB HP SPM（寄存器里叫 HP TCM），同时也被 CPU 复位向量
占用；ESP-IDF 的 `spm_idram_seg` 把整块交给应用，bootloader 的
`verify_load_addresses()` 有专门的 `esp_ptr_in_spm()` 分支接受它。
`link.x` 用它放 `#[tcmfunc]` / `#[tcmdata]` 的代码和数据。

内部 SRAM 不是一块连续可用的窗口：768 KB HP L2MEM 里既有 L2 cache，也有
**mask ROM 自己的 `.data`/`.bss`**，而它们的位置随芯片版本变化。这块板子是
v1.3（revision 103，pre-v3.0），实测（见 `ram_map` 例程）：

| 地址 | 用途 | 能否给应用 |
|------|------|-----------|
| `0x4FF0_0000`..`0x4FF2_BBD0` | L2 RAM，2nd-stage bootloader 之下 | 可以 → `RAM` |
| `0x4FF2_BBD0`..`0x4FF3_FBA4` | bootloader 镜像与 ROM 共享缓冲（运行期已废弃） | 可以，但 .data 的**装载**地址不能落进 bootloader 自己的段（`0x4FF28ED0`..`0x4FF35088`） |
| `0x4FF3_FBA4`..`0x4FF4_0000` | **ROM 的 `.data`/`.bss`（活的）** | 不可以，清零它之后任何 ROM 调用都会挂死 |
| `0x4FF4_0000`..`0x4FFA_0000` | L2 RAM，ROM 窗口与 cache 之间 | 可以 → `RAM_HIGH` |
| `0x4FFA_0000`..`0x4FFC_0000` | **L2 cache（128 KB）** | 不可以，写进去的内容会被 cache 覆盖 |

因此 `.data`/`.bss`/栈放在 `RAM`，放不下的大缓冲区放在 `RAM_HIGH`（`.ram.high`
段，`NOLOAD`，由 `_start` 清零）：

```rust
#[unsafe(link_section = ".ram.high")]
static mut FRAME: [u8; 256 * 1024] = [0; 256 * 1024];
```

`.bss` 溢出 `RAM` 不会静默踩到 ROM 窗口，而是链接期报错
（`esp32p4-rt: stack overflows RAM` / `section '.bss' will not fit in region 'RAM'`）。

espflash 按芯片版本选 bootloader（`esp32p4-v0` / `esp32p4-v3`），所以内存图
必须和它一致；v3.0+ 的 die 把 cache 挪到底部、ROM 数据挪到顶部，参数见
[§8](#8-换-bootloader--cache-配置)。

输出段顺序：

```text
.flash.appdesc   ROM  0x40000020   (256 B)
.rodata          ROM  紧随其后
.drom_gap        ROM  NOLOAD，仅推进地址
.text            ROM  下一个 0x100 边界
.ramfunc         RAM  bootloader 拷贝（#[ramfunc]）
.tcm.text        SPM  bootloader 拷贝（#[tcmfunc]）
.tcm.data        SPM  bootloader 拷贝（#[tcmdata]）
.data            RAM  bootloader 拷贝
.bss             RAM  _start 清零
.stack           RAM  NOLOAD
.ram.high        RAM_HIGH  NOLOAD，_start 清零
```

debug 构建的 `blink` 镜像（`espflash save-image` 解析结果）：

```text
seg0: addr=0x40000020 len=0x1108   # appdesc + rodata
seg1: addr=0x00000000 len=0x1c8    # espflash 填充段，bootloader 忽略
seg2: addr=0x40001300 len=0x3400   # text
```

`unpack_load_app` 只数 load address 落在 flash 窗口的段，这里正好是 seg0 和
seg2，满足 `rom_index == 2`。seg1 的 `addr=0` 不在 RAM/flash 范围，
`should_load/should_map` 都为 false。

带 RAM 段的镜像（例如 `tcm` 例程）在这些之外再加 RAM/SPM 段，flash 段仍然只有
两个：

```text
seg0: addr=0x40000020 len=6544    # flash: appdesc + rodata
seg1: addr=0x30100000 len=120     # SPM: .tcm.text + .tcm.data
seg2: addr=0x4ff20000 len=192     # RAM: .ramfunc + .data
seg3: addr=0x40001b00 len=16944   # flash: text
seg4: addr=0x4ff200c0 len=976     # RAM: 未初始化但仍需加载的数据
```

## 4. `.data`、`.ramfunc`、`.tcm.text` 和 `.bss`

- `.data`：VMA 在 RAM，espflash 把它作为 RAM segment 写进 app image；
  bootloader 在验证阶段 `memcpy` 到 VMA。因此 `_start` **不**需要从 flash
  拷贝 `.data`。
- `.bss`：NOLOAD，不进入镜像；`_start` 用 `sw zero` 清零。
- `.stack`：NOLOAD，`_stack_start` 在段尾。
- `.ramfunc` / `.tcm.text` / `.tcm.data`：与 `.data` 同一个机制——VMA 在 RAM
  （或 SPM），espflash 把它们作为 RAM segment 写进镜像，bootloader 拷贝。
  `#[ramfunc]` 的函数在 flash cache 挂起时仍可执行，`#[tcmfunc]` 的走 CPU
  自己的总线、不经 cache；两者都只影响被标记的那个函数，它调用的东西仍在
  flash。

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

### `mtvec` 必须 64 字节对齐

P4 的 HP 核只运行在 CLIC 模式下：`mtvec` 的低位读回总是 `0b11`（MODE=3），
trap 向量基地址取 `mtvec & !0x3f`。因此 `_start_trap` 必须 **64 字节对齐**，
否则基地址落在函数体中间：任何异常都会跳到错误的指令上，异常处理循环里
再出错，表现为**整个核挂死**（`TRAP_CAUSE` 永远写不进去），约 9 s 后被 RTC
看门狗复位——看起来就像“程序没运行、设备每 10 s 重连一次”。

### `mintthresh` / `mintstatus` 不可用

`csrr`/`csrw` 访问 CLIC 的 `mintthresh`（CSR `0x347`）和 `mintstatus`
（CSR `0xFB1`）在 P4（v1.3 实测）上会触发 **illegal instruction** 异常，
尽管 TRM 把它们列为已实现。运行时不碰这两个 CSR：阈值保持复位值 0
（接受所有级别），处理中状态由软件记录。

> 这两条都是“启动即挂死”的坑：`interrupt::init()` 曾写过 `mintthresh`，
> 而 trap 入口只有 4 字节对齐，于是任何示例都到不了 `main`。

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

内存图和 bootloader 是一套的，两条都要一起改。

**bootloader 按版本选**。espflash 内置两套 P4 bootloader，按 eFuse 里的
revision 挑选：`esp32p4-v0`（revision < 300）和 `esp32p4-v3`（≥ 300），
它们的段地址完全不同（`esp32p4-v0` 从 `0x4FF28ED0` 起，`esp32p4-v3` 从
`0x4FFAB2C0` 起）。想确认实际用的是哪套，看启动日志里
`segment N: ... vaddr=0x4FF...` 的地址，或者 `espflash board-info` 的
`Chip type / revision`。

**cache 位置按版本定**。768 KB L2MEM 由 L2 RAM 和 L2 cache 共享：

| die | L2 cache 位置 | ROM `.data`/`.bss` | 可用窗口 |
|-----|--------------|--------------------|---------|
| pre-v3.0（v1.3 / revision 103） | 顶部 `0x4FFC_0000 - size` | `0x4FF3_FBA4`..`0x4FF4_0000` | `0x4FF0_0000`..`0x4FF2_BBD0` + `0x4FF4_0000`..`0x4FFC_0000 - size` |
| v3.0+ | 底部 `0x4FF0_0000` | 顶部 `0x4FFB_FBB0`..`0x4FFC_0000` | `0x4FF0_0000 + size`..`0x4FFA_DFC0` |

pre-v3.0 就是本仓库现在这套（`RAM` + `RAM_HIGH`）；v3.0+ 时内部 SRAM 又变回
一整块，改回单区即可，例如 128 KB cache（IDF 默认）：

```ld
RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
```

那时 `RAM_HIGH` 可以指到同一范围（`.ram.high` 段留空即可），`RAM` 也不再
需要停在 `0x4FF2BBD0`。cache 大小改过时（256/0 KB），`ORIGIN` 相应改成
`0x4FF40000` / `0x4FF00000`，顶端不变。

两个区的划分不是推出来的，是量出来的：`ram_map` 例程在每个边界附近写一个
标记、用 1 MB PSRAM 流量刷一遍 L2、再读回来；落在 cache 里的页会丢掉内容。
