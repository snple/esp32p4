# esp32p4-rt

ESP32-P4 高性能 RISC-V 核的最小运行时。

## 代码组织

`_start`、`_start_trap` 是 `src/rt.rs` 里的 `global_asm!` 字符串，没有单独的
`.S` 文件；它们必须在任何 Rust 代码用到栈之前执行，用汇编写最直接。

`asm.rs` 里的内联汇编写数字标签时从 `2` 开始：`1b` 会被当成二进制字面量，
`binary_asm_labels` 是 deny 级 lint。

## 启动流程

ESP-IDF 2nd-stage bootloader 完成：

1. 映射两个 flash 段（DROM + IROM）；
2. 把 RAM 段拷贝到各自虚拟地址；
3. 跳到 ELF entry（`_start`）。

`_start` 然后：

1. 设置 `gp`、`sp`；
2. 清零 `.bss`；
3. 安装 direct-mode `mtvec`，关闭 `MIE`、清零 `mie`；
4. 设置 `mstatus.FS = Dirty`；
5. `call main`，返回后原地循环。

## Trap

`_start_trap` 保存 `ra/t0-t6/a0-a7`，把栈帧指针传给
`_start_trap_rust`。最小实现不返回：把 `mcause` / `mepc` 写入
`TRAP_CAUSE` / `TRAP_EPC` 后 `wfi` 停住，方便调试器查看。

当前 trap 入口只保存整数寄存器，不保存 FPU 寄存器；运行时也不使能中断。
要支持可恢复中断/FPU 上下文时，需要扩展 trap frame 并保存 `f0-f31`/`fcsr`。

P4 的中断控制器是 **CLIC**（48 条中断线、8 级优先级）。`_start` 把 `mtvec` 设为
direct 模式（`MODE=0`），只用于异常；以后要开中断需要：

- 配置 `mtvt` 指向 64 字节对齐的中断向量表，并把 `mtvec.MODE` 设为 3；
- 通过中断矩阵（`INTERRUPT_CORE0/1`）把源号映射到 CLIC 线；
- 在 `_start_trap` 里保存/恢复更多上下文（含 FPU）。

## 链接脚本

`build.rs` 把 `link.x` 放到 `OUT_DIR` 并加入链接搜索路径。`link.x`：

- `ENTRY(_start)`、`EXTERN(main)`；
- `INCLUDE memory.x`（由应用自己的 `build.rs` 提供）；
- 段 0：`.flash.appdesc` + `.rodata`；
- 一个 NOLOAD 间隙段（bootloader 要求 P4 镜像恰好两个 flash 段）；
- 段 1：`.init` + `.text` + `.trap`；
- RAM：`.data`（bootloader 拷贝）、`.bss`（`_start` 清零）、`.stack`；
- RAM_HIGH：`.ram.high`（NOLOAD，`_start` 清零），给放不进 RAM 的大缓冲区。

应用侧 `memory.x` 模板（pre-v3.0 die，即 ESP32-P4 v1.3 / revision 103）：

```ld
MEMORY
{
    ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
    RAM : ORIGIN = 0x4FF00000, LENGTH = 0x2BBD0
    RAM_HIGH : ORIGIN = 0x4FF40000, LENGTH = 0x60000
}
```

内部 SRAM 里嵌着 L2 cache 和 mask ROM 自己的 `.data`/`.bss`，位置随芯片版本
变化，所以不能当成一整块：这块 die 上 cache 在顶部 128 KB
（`0x4FFA_0000`..`0x4FFC_0000`），ROM 数据在 `0x4FF3_FBA4`..`0x4FF4_0000`。
`RAM` 停在 2nd-stage bootloader 之下，`RAM_HIGH` 夹在 ROM 窗口与 cache 之间。
超大缓冲区：

```rust
#[unsafe(link_section = ".ram.high")]
static mut FRAME: [u8; 256 * 1024] = [0; 256 * 1024];
```

v3.0+ 的 die 换成单区（cache 在底部），见 `docs/boot.md` §8。

## 应用描述符

`src/app_desc.rs` 提供一个 256 字节的 `esp_app_desc_t`，放在
`.flash.appdesc`。这是 `espflash` 生成 ESP-IDF 应用镜像时要求的；
普通启动不依赖其中的版本/项目名字段。

## 模块

- `asm`：`nop` / `wfi` / `fence` / `ebreak` / `delay_cycles`
- `register`：`mstatus` / `mtvec` / `mcause` / `mepc` / `mhartid` /
  `mcycle` / `minstret` / `misa`
- `interrupt`：`disable` / `enable` / `restore` / `free`
