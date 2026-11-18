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
| [`esp32p4`](esp32p4/) | **HAL**：GPIO / IO MUX、SYSTIMER 延时、复位、封装内 PSRAM、USB Serial/JTAG、以太网 MAC、同步 TCP/IP |
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
| `usb_serial` | 通过 USB Serial/JTAG 打印并回显，同时翻转 GPIO2（两个驱动共享 `HP_SYS_CLKRST`） |
| `emac` | 初始化 RMII 以太网、自动协商、打印链路状态并收发 ARP 帧 |
| `net_tcp_echo` | DHCP + TCP echo 服务器（需 `--features net`，无堆、无 async） |
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
    let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
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
- `HP_SYS_CLKRST` 是借用而不是独占的，所以 USB Serial、GPIO（`Pins::init`）等驱动
  可以同时初始化。
- USB Serial/JTAG 与 `USB_WRAP` 的 OTG FS 共享两个全速 PHY，默认占用 PHY 0；
  两个口同时使用时才需要 `UsbSerialJtag::select_phy`。

`defmt` 也可以直接走这个端口（示例 `defmt_usb`）：

```bash
cargo run --bin defmt_usb --features defmt-usb
```

HAL 的 `defmt-usb-logger` feature 会注册成全局 logger，因此不能和 `defmt-rtt`
同时启用。时钟未打开时 logger 丢弃日志而不是访问未上时钟的寄存器。

## 以太网 MAC

P4 内置 DesignWare 10/100 MAC + DMA，通过 RMII 连接外部 PHY
（LAN8720 / DP83848 之类）。SVD 里没有这个外设，所以寄存器定义来自 TRM
第 53 章：`xtask/gen_emac.py` 从 LaTeX 源生成根目录下的 `emac.svd` 片段，
`cargo xtask gen-pac` 再把它和 `esp32p4.svd` 合并。片段已提交，正常构建不需要
TRM 源或 Python。

```rust
static mut BUFFERS: EmacBuffers<8, 4> = EmacBuffers::new();

let pins = Pins::init(dp.GPIO, dp.IO_MUX, &dp.HP_SYS_CLKRST);
let rmii = RmiiPins::new(
    pins.pin::<33>(), // TXEN
    pins.pin::<34>(), // TXD0
    pins.pin::<35>(), // TXD1
    pins.pin::<30>(), // RXD1
    pins.pin::<29>(), // RXD0
    pins.pin::<28>(), // CRS_DV
    pins.pin::<32>(), // REF_CLK
    pins.pin::<20>(), // MDC
    pins.pin::<21>(), // MDIO
);
let (gpio, io_mux) = pins.release();

// SAFETY: `BUFFERS` is only handed to the driver, once.
let mut emac = Emac::init(
    dp.EMAC,
    rmii,
    EmacConfig::default(),
    unsafe { &mut *core::ptr::addr_of_mut!(BUFFERS) },
    EmacResources {
        gpio: &gpio,
        io_mux: &io_mux,
        hp_sys: &dp.HP_SYS,
        clkrst: &dp.HP_SYS_CLKRST,
        lp_clkrst: &dp.LP_AON_CLKRST,
    },
);

let link = emac.autonegotiate();   // 5 s 超时，返回协商结果
emac.transmit(&frame)?;            // 帧不带 FCS
if let Some(rx) = emac.receive() {
    // ...
    emac.release_frame();
}
```

要点：

- **引脚**：RMII 数据/时钟是 **IO MUX 的专用 GMAC 功能（Function 3）**，不是普通 GPIO，
  也不走 GPIO matrix —— 选中功能后 pad 直接连到 MAC，连输出使能都由 MAC 控制，
  `GPIO_ENABLE` 对它无效（IDF `gpio_hal_iomux_out` 的注释："as long as the func sel
  is not GPIO, the oe can only be controlled by the peripheral"）。可用的 pad 分组是
  GPIO28-36 / 40-48 / 49-54。MDC/MDIO 没有专用功能，走 GPIO matrix，可以放在任意引脚，
  但 `OEN_SEL` 仍留给 MAC：MDIO 是双向的，MAC 必须在 PHY 应答前释放总线。
  `RmiiPins::new` 用 `Pin<N>` 参数做类型检查，引脚不带 GMAC 功能会编译失败。
- **时钟**：`RefClock::External` 由 PHY 提供 50 MHz 到 `REF_CLK`（推荐）；
  `RefClock::InternalMpll` 用 MPLL 500 MHz / 10 输出到 `REF_50M_CLK`，需板上回环到
  `REF_CLK`。RMII 模式下 EMAC 时钟树由 `HP_SYS_CLKRST.PERI_CLK_CTRL00/01` 配置，
  复位在 `LP_AON_CLKRST.HP_SDMMC_EMAC_RST_CTRL`（不在 HP 复位门里）。
- **DMA**：32 字节增强描述符、`ALT_DESC_SIZE = 1`，描述符和缓冲区必须常驻
  `static`。驱动通过 [`cache`](esp32p4/src/cache.rs) 在交给 MAC 前 writeback、
  在读回前 invalidate；放在 PSRAM uncached 窗口则不需要维护。
- **中断**：当前是轮询驱动，`DMAIN_EN` 和 `EMACINTMASK` 全部屏蔽，
  `transmit`/`receive` 都不阻塞等待。
- **PHY**：`mdio_read`/`mdio_write` 是 clause-22 原始寄存器访问，
  `poll_link` 读标准 BMSR/BMCR/ANLPAR 推断速率与双工并同步到 MAC，
  `autonegotiate` 重新发起协商。厂商专有寄存器（例如 LAN8720 的 SCSR）需自行读写。

## TCP/IP（smoltcp）

网络栈适配 `smoltcp 0.14`，**同步轮询、无 async、无执行器**，并且默认不依赖堆。
`Emac` 直接实现 `smoltcp::phy::Device`，`Net` 把 interface 和 socket 表打包，
由应用自己的主循环驱动：

```rust
let mut net = Net::new(
    &mut emac,
    SocketSet::new(&mut SOCKET_STORAGE[..]),   // 或用 Net::new_alloc
    mac_address,
    delay::micros(),                            // TCP ISN 随机种子
);
net.start_dhcp();
net.wait_dhcp(&mut emac, 10_000);               // 忙轮询，仅用于启动阶段

let socket = smoltcp::socket::tcp::Socket::new(rx_buf, tx_buf);
let handle = net.sockets_mut().add(socket);

loop {
    net.poll(&mut emac);                        // 收发所有就绪帧 + 跑 timers
    // 从 net.sockets_mut() 取 socket 做业务
    delay.delay_ms(net.poll_delay().map(|d| d.total_millis().min(20) as u32).unwrap_or(5));
}
```

| feature | 作用 |
|---------|------|
| `net` | 以太网 + IPv4 + TCP/UDP/ICMP + DHCP 客户端 + DNS 客户端，socket 表由用户提供 `&mut [SocketStorage]` |
| `net-alloc` | 同上，但 `Net::new_alloc` 用全局分配器（PSRAM 堆）持有 socket 表 |
| `net-ipv6` | 追加 IPv6 / SLAAC / 分片 |

要点：

- **堆是可选的**：`smoltcp` 自身在关闭 `alloc` 时不分配；只有 socket 表的容器可以放到堆上。
  TCP 的收发窗口永远是用户提供的静态缓冲（示例用 2 KiB × 2）。
- **不用 async**：`Net::poll()` 一次把所有就绪帧走完就返回；`poll_delay()` 给出下一次需要
  运行的时间（smoltcp 的重传/延迟 ACK 定时器），应用据此 sleep。
- **`Emac` 的数据路径改成 `&self`**：环索引在 `Cell` 里、缓冲区通过裸指针访问，这样
  网络栈可以同时持有接收 token 和发送 token（`Device::receive` 的签名要求）。
- **校验和走软件**（`ChecksumCapabilities::default()`）。MAC 支持硬件校验和，
  但需要额外的描述符位，暂未启用。
- **中断仍然关闭**：`DMAIN_EN = 0`，全部靠轮询。等 `rt` 接上中断后再考虑异步/唤醒。

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
- 以太网 MAC（RMII + MDIO + DMA 描述符环），含 L1/L2 cache 维护
- 同步 TCP/IP（smoltcp）：DHCP 客户端、TCP/UDP/ICMP、可选 IPv6，无 async、堆可选
- `defmt` 日志支持（`Format` 实现 + 关键路径日志 + 基于 `mstatus.MIE` 的
  `critical-section` 实现）
- `#[main]`（注入 `startup::init()`：关 bootloader 遗留的 RTC/MWDT 看门狗、停放 core 1、
  启用分支预测器）、默认 panic handler
- stable `riscv32imafc-unknown-none-elf`（RV32IMAFC、ilp32f 硬浮点 ABI），启动时使能 FPU

尚未实现：

- CLIC / 中断矩阵绑定、外设中断
- 完整时钟树（PLL/分频）配置
- UART、SPI、I2C、DMA、RMT、WS2812 等驱动
- EMAC 中断驱动、TX/RX 校验和卸载、PTP 时间戳、MII/RGMII 接口
- 阻塞式 socket API（当前是 smoltcp 的事件/轮询风格）、DHCP 服务端
- USB OTG FS/HS 主机与设备栈
- LP 核与低功耗
- trap 入口尚未保存 FPU 寄存器上下文（当前不使能可恢复中断）

## 许可证

MIT OR Apache-2.0
