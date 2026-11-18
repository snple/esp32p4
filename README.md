# esp32p4

> ESP32-P4 的 Rust `no_std` 嵌入式支持：从 SVD 生成的 PAC、RISC-V 运行时、
> 最小可用 HAL，以及可以直接用 `espflash` 烧写的 blink 示例。

项目结构参考 [`rp2350`](../../rp2350/rp2350) 的分层方式：PAC 只做寄存器，
rt 只做启动，HAL 是唯一手写的用户接口层。

## Crate 结构

| Crate | 说明 |
|-------|------|
| [`esp32p4-pac`](esp32p4-pac/) | **PAC**：由 `esp32p4.svd` 生成，提供寄存器块、字段常量和访问器。勿手改 |
| [`esp32p4-rt`](esp32p4-rt/) | **运行时**：`_start`、trap 入口、`link.x`、CSR/汇编封装 |
| [`esp32p4-macro`](esp32p4-macro/) | **过程宏**：`#[main]` |
| [`esp32p4`](esp32p4/) | **HAL**：GPIO / IO MUX、SYSTIMER 延时、复位、封装内 PSRAM、USB Serial/JTAG |
| [`esp32p4-examples`](esp32p4-examples/) | 示例：GPIO2 blink、PSRAM 初始化与分区、USB Serial/JTAG |
| [`xtask`](xtask/) | 构建辅助：`cargo xtask gen-pac` |

依赖方向是单向的：`macro` / `rt` → `pac` → `esp32p4` → `examples`。

## 示例

| 二进制 | 内容 |
|--------|------|
| `blink` | GPIO2 每 500 ms 翻转一次 |
| `psram` | 初始化封装内 PSRAM，整体作为全局堆使用 |
| `psram_split` | 把 PSRAM 分成「cached 全局堆 + uncached DMA 池」两部分 |
| `defmt_log` | 用 `defmt-rtt` 输出日志（需 `--features defmt`，JTAG 读取） |
| `usb_serial` | 通过 USB Serial/JTAG 打印，并回显主机发送的字节 |
| `defmt_usb` | `defmt` 日志直接走 USB Serial/JTAG（需 `--features defmt-usb`，`espflash monitor` 可见） |

## 快速开始

`esp32p4-examples/src/blink.rs` 每 500 ms 翻转一次 GPIO2：

```rust
#![no_std]
#![no_main]

use esp32p4::delay::Delay;
use esp32p4::gpio::Pins;
use esp32p4::main;
use esp32p4::pac::Peripherals;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, dp.HP_SYS_CLKRST);
    let mut led = pins.pin::<2>();
    led.into_output();
    let delay = Delay::init(dp.SYSTIMER);

    loop {
        led.toggle();
        delay.delay_ms(500);
    }
}
```

### 构建与烧写

需要 `riscv32imafc-unknown-none-elf` target 和 [`espflash`](https://github.com/esp-rs/espflash)：

```bash
rustup target add riscv32imafc-unknown-none-elf
cargo install espflash

cd esp32p4-examples
cargo run --bin blink
```

`esp32p4-examples/.cargo/config.toml` 已把 runner 配成
`espflash flash --chip esp32p4 --monitor`。只生成镜像、不烧写：

```bash
cd esp32p4-examples
cargo build --bin blink --release
espflash save-image --chip esp32p4 \
  ../target/riscv32imafc-unknown-none-elf/release/blink /tmp/blink.bin
```

## USB Serial/JTAG

P4 内置的全速 USB 设备控制器同时实现了 CDC-ACM 串口和 JTAG 桥，也就是
`espflash` 用来烧写的那个口。一根 USB 线同时提供烧写、日志和调试，不需要
USB 转串口芯片。

```rust
use core::fmt::Write;

use esp32p4::usb_serial_jtag::UsbSerialJtag;

let mut serial = UsbSerialJtag::init(
    dp.USB_DEVICE,
    &dp.HP_SYS_CLKRST,
    &dp.LP_AON_CLKRST,
);

writeln!(serial, "hello").ok();   // 写入并 flush
let mut buf = [0u8; 64];
let n = serial.read(&mut buf);    // 主机发来的字节；没有就返回 0
```

- `init` 只打开 APB 时钟、48 MHz PHY 时钟和内部 PHY，**不复位模块**：复位会让
  设备从主机上消失几百毫秒，`espflash monitor` 得重开串口。
- 写入 `EP1` 的数据留在 64 字节 TX FIFO 里，`flush()` 置 `EP1_CONF.WR_DONE`
  后才作为一包发给主机。
- 无法区分「主机很忙」和「没有插线」，因此除 `write_byte` 外的接口都带约
  10 ms 超时：`write` 返回实际写入字节数，`flush` 返回这一包是否被取走。
  `write_byte` 在 FIFO 满且无主机时会一直自旋（文档已注明）。
- USB Serial/JTAG 与 `USB_WRAP` 的 OTG FS 共享两个全速 PHY，默认占用 PHY 0；
  两个口同时使用时才需要 `UsbSerialJtag::select_phy`。

`defmt` 也可以直接走这个端口（示例 `defmt_usb`）：

```bash
cargo run --bin defmt_usb --features defmt-usb
```

HAL 的 `defmt-usb-logger` feature 会注册成全局 logger，因此不能和 `defmt-rtt`
同时启用。时钟未打开时 logger 丢弃日志而不是访问未上时钟的寄存器。

## 内存布局与启动

ESP32-P4 不是“ROM 直接跳到应用”的芯片：镜像由 ESP-IDF 2nd-stage bootloader
加载。当前布局与 bootloader 的期望一致：

| 段 | 地址 | 内容 |
|----|------|------|
| flash segment 0 (DROM) | `0x4000_0020` | `esp_app_desc_t` + `.rodata` |
| flash segment 1 (IROM) | `0x4000_0xxx` | `.init` + `.text` + `.trap` |
| RAM | `0x4FF2_0000` | `.data` / `.bss` / stack |

- `0x4FF0_0000..0x4FF1_FFFF` 预留给 bootloader 配置的 128 KB L2 cache。
- RAM 顶端在 `0x4FFA_DFC0`，避开 ROM 的 BSS/stack。
- bootloader 会拷贝 RAM 段；`_start` 只需要清零 `.bss`。
- bootloader 在 P4 上要求恰好两个 flash 段，因此 `link.x` 用一个 NOLOAD
  间隙段把 DROM 和 IROM 分开。

细节见 [`docs/boot.md`](docs/boot.md)。

## 重新生成 PAC

`esp32p4.svd` 更新后：

```bash
cargo xtask gen-pac            # 重新生成 esp32p4-pac/src/generated
cargo xtask gen-pac --check    # 仅校验生成代码是否与 SVD 一致
```

生成器会展开 ESP SVD 的 `<dim>` 寄存器数组（`GPIO%s`、`IV_MEM[%s]` 等），
并处理数组元素与普通寄存器重名的情况。生成代码已自动 `rustfmt`。

## 工具链

- Rust **1.100+**（`workspace.package.rust-version = "1.100"`）：用到了 1.100
  稳定的 `core::alloc::Allocator`（`Vec::new_in` / `Box::new_in` 指定分配器）。
- 默认 target：`riscv32imafc-unknown-none-elf`。

## 验证

```bash
cargo check -p esp32p4-pac --target riscv32imafc-unknown-none-elf
cargo check -p esp32p4 --target riscv32imafc-unknown-none-elf
cargo clippy -p esp32p4 -p esp32p4-rt -p esp32p4-macro -p esp32p4-examples \
  --target riscv32imafc-unknown-none-elf -- -D warnings
cargo fmt --all -- --check
cargo xtask gen-pac --check

cd esp32p4-examples
cargo build --bin blink
espflash save-image --chip esp32p4 \
  ../target/riscv32imafc-unknown-none-elf/debug/blink /tmp/blink.bin
```

## 当前范围

已实现：

- 从 SVD 生成完整 PAC（89 个外设、约 5600 个展开寄存器）
- RISC-V 启动、trap 入口、CSR/汇编封装、ESP-IDF 兼容镜像布局
- GPIO0–GPIO53 的简单推挽输出/输入、IO MUX 配置
- SYSTIMER 阻塞延时
- 复位控制与复位原因（`system::software_reset` / `software_reset_cpu` / `reset_reason`，纯寄存器实现）
- 封装内 PSRAM 初始化（MPLL 250 MHz、16/32 MB 自动检测、MMU 映射、`PsramAllocator`）
- USB Serial/JTAG（CDC-ACM）收发：`UsbSerialJtag`，以及可选的 `defmt` 全局 logger
- `defmt` 日志支持（`Format` 实现 + 关键路径日志 + 基于 `mstatus.MIE` 的
  `critical-section` 实现）
- `#[main]`（注入 `startup::init()`：关 bootloader 遗留的 RTC/MWDT 看门狗、停放 core 1、
  启用分支预测器）、默认 panic handler
- stable `riscv32imafc-unknown-none-elf`（RV32IMAFC、ilp32f 硬浮点 ABI），启动时使能 FPU

尚未实现：

- CLIC / 中断矩阵绑定、外设中断
- 完整时钟树（PLL/分频）配置
- UART、SPI、I2C、DMA、RMT、WS2812 等驱动
- USB OTG FS/HS 主机与设备栈
- LP 核与低功耗
- trap 入口尚未保存 FPU 寄存器上下文（当前不使能可恢复中断）

## 许可证

MIT OR Apache-2.0
