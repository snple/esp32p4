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
| [`esp32p4-hal`](esp32p4-hal/) | **HAL**：时钟树、GPIO / IO MUX、UART、I2C 主机、ADC、SYSTIMER 延时/单调时钟/闹钟、定时器组与看门狗、复位、封装内 PSRAM、USB Serial/JTAG、以太网 MAC、同步 TCP/IP |
| [`esp32p4-usb`](esp32p4-usb/) | **USB 设备栈**：DWC OTG 全速/高速控制器（slave 模式、轮询）、标准请求与描述符、CDC-ACM 虚拟串口、自定义类 |
| [`esp32p4-examples`](esp32p4-examples/) | 示例：GPIO2 blink、UART 回环、I2C 扫描、ADC 采样、PSRAM 初始化与分区、USB Serial/JTAG、USB CDC |
| [`xtask`](xtask/) | 构建辅助：`cargo xtask gen-pac` |

依赖方向是单向的：`macro` / `rt` → `pac` → `esp32p4` / `esp32p4-usb` → `examples`。

## 示例

| 二进制 | 内容 |
|--------|------|
| `blink` | GPIO2 每 500 ms 翻转一次 |
| `gpio_interrupt` | GPIO3 下降沿中断（`GPIO_INTR0` + CPU 线 21）翻转 GPIO2 |
| `gpio_pads` | 打印若干焊盘的直接 IO MUX 功能，演示 `set_mux_function`/`mux_function` 往返 |
| `efuse` | 打印芯片版本、MAC、Flash/PSRAM 参数，并用版本号配置时钟树 |
| `mmu` | 把 PSRAM 物理页 0 映射到 MMU 表项 512，验证 cached/uncached 别名 |
| `clock` | 打印时钟树的根/CPU/MEM/SYS/APB 频率与参考时钟状态 |
| `tcm` | 用 `#[tcmfunc]`/`#[ramfunc]` 把代码放进 8 KB scratchpad / SRAM，并打印落点 |
| `psram` | 初始化封装内 PSRAM，整体作为全局堆使用 |
| `psram_split` | 把 PSRAM 分成「cached 全局堆 + uncached DMA 池」两部分 |
| `defmt_log` | 用 `defmt-rtt` 输出日志（需 `--no-default-features --features defmt-rtt`，JTAG 读取） |
| `usb_serial` | 通过 USB Serial/JTAG 打印并回显，同时翻转 GPIO2（两个驱动共享 `HP_SYS_CLKRST`） |
| `usb_cdc` | 全速 OTG（GPIO26/27）上的 CDC-ACM 虚拟串口，回显主机发来的字节 |
| `usb_cdc_hs` | 高速 OTG（内部 UTMI+ PHY）上的 CDC-ACM，512 字节 bulk 包 |
| `timer` | `TIMG0` 的两路通用定时器：100 ms 周期中断闪灯，外加 1 s 的倒计时闹钟 |
| `alarm` | `SYSTIMER` 比较器：250 ms 周期闹钟 + 5 s 一次性 deadline，主循环用 `sleep_until` 睡到下一个检查点 |
| `watchdog` | `TIMG0` 的 MWDT：1 s 超时复位系统，主循环每 100 ms `feed()` 一次 |
| `emac` | 初始化 RMII 以太网（P4 Function-EV 板的 IP101）、扫描 MDIO、自动协商、打印链路状态并收发 ARP 帧 |
| `net_tcp_echo` | DHCP + TCP echo 服务器（需 `--features net`，无堆、无 async） |
| `uart` | `UART1` 内部回环自检：三组数据原样收发，并打印波特率与帧格式 |
| `i2c_scan` | `I2C0` 扫描 7 位地址空间，逐个探测并打印应答的从机 |
| `adc` | 报告 eFuse 校准是否可用，并在四档衰减下采样 13 个 ADC 焊盘 |

## 快速开始

`esp32p4-examples/src/blink.rs` 每 500 ms 翻转一次 GPIO2：

```rust
#![no_std]
#![no_main]

use esp32p4_hal::delay::Delay;
use esp32p4_hal::gpio::Pins;
use esp32p4_hal::main;
use esp32p4_hal::pac::Peripherals;
use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

#[main]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let _serial = UsbSerialJtag::init(dp.USB_DEVICE, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
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

### 日志怎么才能看见（选一个 logger）

`defmt` 的日志语句本身不带输出通道：只有接上一个 **logger** 才会真的发出去，
而可用的 logger 有两个、彼此互斥，所以要按你用什么工具读日志来选：

| 读日志的工具 | feature | runner | 通道 |
|---|---|---|---|
| `espflash monitor`（只插 USB 线） | `defmt-usb` | `espflash flash --chip esp32p4 --monitor` | USB Serial/JTAG（CDC-ACM），espflash 直接解码 |
| `probe-rs run`（需要 JTAG 调试器） | `defmt-rtt` | `probe-rs run --chip esp32p4` | RAM 里的 RTT 缓冲，只有调试器能读 |

panic 消息也走同一个 logger：默认 panic handler 先关中断，再用 `defmt` 输出
`PanicInfo`，最后执行 `ebreak`。接了调试器就停在 panic 现场；没接调试器时
breakpoint 异常进入 trap 入口，把 `mcause`/`mepc` 存进 `TRAP_CAUSE`/`TRAP_EPC`
后停住核。

两个 logger 各定义一份全局 `defmt` 符号，同时启用会链接失败（build script 会
直接报 `mutually exclusive`）。`esp32p4-examples/.cargo/config.toml` 里的 runner
也要跟着配对切换：

```toml
[target.riscv32imafc-unknown-none-elf]
# espflash + defmt-usb
runner = "espflash flash --chip esp32p4 --monitor"
# probe-rs + defmt-rtt
# runner = "probe-rs run --chip esp32p4"
```

```bash
cd esp32p4-examples

# A. 只插 USB 线：日志走 espflash（默认 features 就是 defmt-usb，
#    帧前缀和 ELF 里的 LOG_FORMAT 标记让 espflash 自动按 defmt 解码）
cargo run --bin efuse          # HAL 和示例自己的日志都可见

# B. 接了 JTAG 调试器：日志走 RTT（runner 也要换成 probe-rs）
cargo run --bin defmt_log --no-default-features --features defmt-rtt
```

默认 features 是 `defmt-usb` + `net`（`defmt` 由它隐含，走 espflash 工作流）。
除 `defmt_log` 外，所有示例都用默认的 `defmt-usb` transport，并在 `main` 开头
调用 `UsbSerialJtag::init` 打开端口——logger 在时钟未打开时会直接丢帧，panic
消息也一样。只有 `defmt_log` 声明了 `required-features = ["defmt-rtt"]`，
是保留的 RTT 示例。

想默认改用 probe-rs/RTT，把 [esp32p4-examples/Cargo.toml](esp32p4-examples/Cargo.toml)
里的 `default` 改成 `["defmt", "defmt-rtt", "net"]`；各示例开头的
`#[cfg(feature = "defmt-rtt")] use defmt_rtt as _;` 会让它们照常走 RTT。

> `DEFMT_LOG` 必须保留（仓库已配成 `debug`）。defmt 1.x 在没有它时只编译 `error!`，
> 所有 `info!`/`debug!` 会直接在编译期消失，看起来就像"日志没工作"。

## USB Serial/JTAG

P4 内置的全速 USB 设备控制器同时实现了 CDC-ACM 串口和 JTAG 桥，也就是
`espflash` 用来烧写的那个口。一根 USB 线同时提供烧写、日志和调试，不需要
USB 转串口芯片。

```rust
use core::fmt::Write;

use esp32p4_hal::usb_serial_jtag::UsbSerialJtag;

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

`defmt` 默认就走这个端口（`defmt-usb`），日志用 `espflash monitor` 就能看到，
不需要调试器——详见[日志怎么才能看见](#日志怎么才能看见选一个-logger)。示例都在
`main` 开头调用 `UsbSerialJtag::init`；时钟未打开时 logger 丢弃日志，而不是去访问
未上时钟的寄存器。

## USB OTG（设备模式）

P4 有两个独立的 DesignWare OTG 控制器，`esp32p4-usb` 把它们都做成**设备模式**、
**slave 模式**（CPU 搬 FIFO，无 DMA）、**轮询**（无中断、无 async）的驱动：

| 控制器 | 基地址 | 速率 | PHY |
|--------|--------|------|-----|
| `USB_OTG11` | `0x5004_0000` | 全速 12 Mbit/s | 内部全速 PHY，D+/D- 在 GPIO27/26 |
| `USB_OTG20` | `0x5000_0000` | 高速 480 Mbit/s | 内部 UTMI+ PHY，不占 GPIO |

```rust
use esp32p4_usb::class::cdc_acm::CdcAcm;
use esp32p4_usb::p4::{FsResources, UsbFs};
use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};

let allocator = UsbBusAllocator::new(UsbFs::new(FsResources {
    clkrst: &dp.HP_SYS_CLKRST,
    lp_aon_clkrst: &dp.LP_AON_CLKRST,
    usb_wrap: &dp.USB_WRAP,
    gpio: &dp.GPIO,
    io_mux: &dp.IO_MUX,
}));
let mut serial = CdcAcm::new(&allocator, 64);          // 全速 bulk 包 64 字节
let mut device = UsbDeviceBuilder::new(&allocator, 0x303a, 0x1001)
    .manufacturer("Espressif")
    .product("ESP32-P4 CDC")
    .build();

loop {
    device.poll(&mut [&mut serial]);                    // 收发一帧，建议 ≥ 每 5 ms 一次
    let mut buffer = [0u8; 64];
    if let Ok(n) = serial.read(&mut buffer) {
        serial.write_all(&buffer[..n], 1_000_000);      // 预算用完就丢弃，不死等主机
    }
}
```

层次：

```text
class::cdc_acm::CdcAcm        你的 UsbClass：接口描述符 + 端点回调
device::UsbDevice             枚举状态机、标准请求、EP0 控制传输
bus::UsbBus                   allocate / poll / read / write
p4::UsbFs / p4::UsbHs         DWC OTG 寄存器、FIFO 分配、复位与枚举
```

要点：

- **寄存器不来自 SVD**：DWC 的寄存器手册由 IP 厂商 NDA 限制，SVD 里没有
  `USB_OTG11`/`USB_OTG20`，因此 `src/dwc.rs` 按 ESP-IDF 的
  `soc/usb_dwc_struct.h` 手写了设备模式子集（偏移 + 位域常量），其余寄存器一律
  仍走 PAC。
- **FIFO**：共享数据 FIFO 全速 200 words、高速 896 words（运行时从
  `GHWCFG3.DFIFODEPTH` 读）。`alloc_ep` 按 MPS 预留：每个 IN 端点至少 16 words
  的专用 TX FIFO，OUT 端点占 RX FIFO，剩下的都归 RX；装不下直接返回
  `EndpointMemoryOverflow`。
- **OUT 暂存**：轮询把收到的包搬进 1 KiB 的暂存池（`OUT_STAGING_BYTES`），
  `read()` 再从池里取；因此 `UsbBusAllocator` 只有 ~1.3 KiB，可以安全放在
  `main` 的栈上（栈 16 KiB）。
- **控制传输**：EP0 支持多包数据阶段、ZLP 规则和状态阶段；控制端点的 MPS
  必须是 8/16/32/64，描述符与寄存器保持一致。
- **地址**：DWC 要求 `SET_ADDRESS` 的地址在状态阶段之前写入 `DCFG.DAD`，
  所以 `UsbBus::SET_ADDRESS_BEFORE_STATUS = true`。
- **全速口的 OTG 输入**：`USB_OTG11` 通过 GPIO matrix 取 `IDDIG`/`AVALID`/
  `BVALID`/`VBUSVALID`/`SESSEND`，驱动把它们固定成「已上电的总线上的外设」
  电平；同时用 `GOTGCTL.BVALIDOVEN` 强制 B 会话有效（板子不把 VBUS 引到 PHY，
  总线供电的设备掉电后本来也无法工作）。
- **高速口的 PHY**：需要 `HP_SYS.OTG_SUSPENDM=1` 才能检测拔插；芯片 3.0 起
  D+/D- 的 15 kΩ 下拉由软件控制（`LP_SYS + 0x204` 的 bit1/2 清零），更早的
  版本由 OTG 核自己管，写入被忽略。
- **与 USB Serial/JTAG 共存**：两个全速控制器分时复用两个内部 PHY，默认
  OTG FS 用 PHY2（GPIO26/27），USB Serial/JTAG 用 PHY1（GPIO24/25）；需要交换时
  调 `UsbSerialJtag::select_phy`，OTG 驱动不碰这个选择。
- **限制**：事务级传输（一次 `read`/`write` 一个包），等时端点不支持
  （分配时返回 `Unsupported`）——等时还需要按帧翻转 PID、丢弃未发出的包。

## 以太网 MAC

P4 内置 DesignWare 10/100 MAC + DMA，通过 RMII 连接外部 PHY
（LAN8720 / DP83848 / IP101 之类）。SVD 里没有这个外设，所以寄存器定义来自 TRM
第 53 章：`xtask/gen_emac.py` 从 LaTeX 源生成根目录下的 `emac.svd` 片段，
`cargo xtask gen-pac` 再把它和 `esp32p4.svd` 合并。片段已提交，正常构建不需要
TRM 源或 Python。

```rust
static mut BUFFERS: EmacBuffers<8, 4> = EmacBuffers::new();

// 官方 P4 Function-EV 板：IP101GRI，PHY_AD0 上拉 -> MDIO 地址 1，
// nRST 接 GPIO51（拉高前 PHY 不响应 MDIO）。
let mut phy_reset = pins.pin::<51>();
phy_reset.into_output().set_low();
delay.delay_ms(100);
phy_reset.set_high();
delay.delay_ms(300);

let rmii = RmiiPins::new(
    pins.pin::<49>(), // TXEN
    pins.pin::<34>(), // TXD0
    pins.pin::<35>(), // TXD1
    pins.pin::<30>(), // RXD1
    pins.pin::<29>(), // RXD0
    pins.pin::<28>(), // CRS_DV
    pins.pin::<50>(), // REF_CLK（PHY 输出的 50 MHz）
    pins.pin::<31>(), // MDC
    pins.pin::<52>(), // MDIO
);
let (gpio, io_mux) = pins.release();

// SAFETY: `BUFFERS` is only handed to the driver, once.
let mut emac = Emac::init(
    dp.EMAC,
    rmii,
    EmacConfig {
        phy_address: 1, // PHY 的 strapping 决定的 MDIO 地址
        ..EmacConfig::default()
    },
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
  另外 `HP_SYS_CLKRST.SOC_CLK_CTRL1.REG_EMAC_SYS_CLK_EN` 才是外设自身的时钟门控，
  没打开时**第一次访问 MAC 寄存器就会 fault**（现象是程序在 `Emac::init` 里静默挂死），
  驱动在钟树配置前先把它置位。
- **DMA**：32 字节增强描述符、`ALT_DESC_SIZE = 1`，描述符和缓冲区必须常驻
  `static`。驱动通过 [`cache`](esp32p4-hal/src/cache.rs) 在交给 MAC 前 writeback、
  在读回前 invalidate；放在 PSRAM uncached 窗口则不需要维护。
- **描述符和缓冲区各自独占一个 cache 行**（`#[repr(C, align(64))]`）。cache 维护以整行
  为单位，而描述符只有 32 字节：两个描述符共用一行时，写回当前描述符会把它邻居里 DMA 刚
  写好的 `OWN`/状态字一起写回成旧值——那一帧就凭空消失，而**所有错误计数器仍然是 0**
  （MAC 没丢，是驱动把它"写没"了）。症状极隐蔽：跑一阵流量后空闲 ping 从 3 ms 变成
  1–4 s、`rx` 计数每秒为 0 然后一次跳满整个环、TCP 连接大面积超时，重启板子又好一阵。
  诊断方法是把环的"待处理描述符数"（`Emac::rx_pending()`）打出来：卡住时它停在几十个，
  说明帧在环里、是驱动没读。
- **中断**：当前是轮询驱动，`DMAIN_EN` 和 `EMACINTMASK` 全部屏蔽，
  `transmit`/`receive` 都不阻塞等待。
- **统计**：这块 MAC 没有 Synopsys 的 MMC 计数器块，真计数只有 `DMAMISSEDFR` 里的两个:
  `Missed_FC`（没有空闲描述符而丢的帧）和 `Overflow_FC`（RX FIFO 溢出丢的帧）。该寄存器
  **读清**，所以 [`Emac::stats`](esp32p4-hal/src/emac/mod.rs) 每次采样先累加，返回的是自
  `init` 以来的累计值。描述符状态位和 `DMASTATUS` 都是"看一眼就没了"的一次性状态，不为了
  凑一张 MMC 表去软件复刻。

  ```rust
  let stats = emac.stats();          // rx_missed / rx_fifo_overflows，单调递增
  let pending = emac.rx_pending();   // 环里已收到、但还没被读走的帧数
  ```

  两个数字就能定位方向：`rx_pending` 卡在高位 = 帧在环里、驱动没读；`rx_missed` 在涨 =
  驱动来不及取；两者都是 0 而外部丢包 = 丢在 MAC 之外。
- **PHY**：`mdio_read`/`mdio_write` 是 clause-22 原始寄存器访问，
  `poll_link` 读标准 BMSR/BMCR/ANLPAR 推断速率与双工并同步到 MAC，
  `autonegotiate` 重新发起协商。厂商专有寄存器（例如 LAN8720 的 SCSR）需自行读写。
  IP101 的 ID 是 `0x0243:0x0c54`，`phy_address` 必须与其 strapping 一致
  （`PHY_AD0` 上拉即地址 1），否则 MDIO 全部读回 `0xffff`。
- **站地址过滤**：地址写入 `ADDR0HIGH`（字节 5-4）/`ADDR0LOW`（字节 3-0，字节 0 在 bit7:0）。
  字节序写反时广播/组播照收，但发给本站的单播帧会被 MAC 静默丢弃。
- **用 eFuse 里的出厂 MAC**，不要用 `EmacConfig::default()` 的 `02:00:00:00:00:01`：本地
  管理地址会让部分 AP/网桥的转发行为异常（实测换回真实 MAC 后，宿主机到板子的 ICMP 丢包
  从 10–80% 降到 0%）。`Efuse::mac()` 负责把 `RD_MAC_SYS1`（前两个字节）和
  `RD_MAC_SYS0`（后四个字节）拼成传输顺序，两个寄存器都是"首字节在最高位"。

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
  TCP 的收发窗口永远是用户提供的静态缓冲（`TcpBuffers` 默认 8 KiB × 2）。接收缓冲同时
  是 smoltcp 的重组缓冲：它若只有一个窗口深，一旦一段丢失、后续乱序段就会把它填满，
  通告窗口被钉在 0 —— 空洞永远补不上，连接会一直卡到对端放弃。窗口开大几段后这种情况
  才有余量。
- **不用 async**：`Net::poll()` 一次把所有就绪帧走完就返回；`poll_delay()` 给出下一次需要
  运行的时间（smoltcp 的重传/延迟 ACK 定时器），应用据此 sleep。
- **`Emac` 的数据路径改成 `&self`**：环索引在 `Cell` 里、缓冲区通过裸指针访问，这样
  网络栈可以同时持有接收 token 和发送 token（`Device::receive` 的签名要求）。
- **校验和走软件**（`ChecksumCapabilities::default()`）。MAC 支持硬件校验和，
  但需要额外的描述符位，暂未启用。
- **中断驱动**：`Emac::enable_interrupts(lines::EMAC)` 把 MAC 的 DMA 完成中断接到 CPU，
  `Net::poll_wait()` 用 `wfi` 把核心停住，等 MAC 中断或 smoltcp 的下一个定时器
  （`SYSTIMER` target 0）唤醒。注意 `sleep_ticks` 必须以**任意**中断结束睡眠，否则收到的
  帧要等到比较器到期才被处理（曾经因此多出最多一个睡眠周期的延迟）；写数据到 socket 后
  也要再 `poll()` 一次，否则 echo 要等下一轮唤醒才发出。
- **echo 要先看发送窗口**：从接收缓冲取走的字节已经被 ACK 过，若因为发送窗口满而没能写回
  socket，这些字节就永久丢了（对端不会重传）。所以每次只取 `send_capacity() - send_queue()`
  能容纳的量。
- **监听池**：一个 socket 在 `LastAck` 里等对端最后一个 ACK 时不能接受新连接，对端此时连
  过来会直接吃 RST（`Connection refused`）。示例在 7 端口上挂两个 socket，把关闭窗口遮住。
- **空闲连接要回收**：对端不发 FIN 就消失时 socket 会永远停在 `Established`，两个监听槽都被
  占住后端口就再也接不了连接。示例对 10 s 没有任何数据流动的连接直接 `abort()`；握手/拆链
  中途的状态（`SynSent`/`SynReceived`/`FinWait1`/`FinWait2`/`Closing`）3 s 没进展也直接
  丢掉 —— 半开连接会一直重传，而这些重传会把整条链路拖慢（实测几个半死连接之后，空闲板子的
  ping RTT 从 3 ms 涨到 800 ms）。
- **ICMP echo 回复**已随 `smoltcp/auto-icmp-echo-reply` 打开，板子可以被 `ping`，这是现场
  判断"链路还在不在"最省事的工具。

## 中断

P4 没有 PLIC，用的是 Espressif 自己的 CLIC（TRM 第 1 章）：

```text
外设 → INTERRUPT_CORE0 矩阵 → 32 条 CPU 中断线 → CLIC → 核心
                                     mcause = 0x8000_0000 | (线号 + 16)
```

`esp32p4-rt` 提供最小可用的中断框架：

- `_start_trap` 保存整数寄存器 + 被调用者保存的 FPU 寄存器（`fsw/flw`，只在带 F 的
  target 上生成），调用 Rust 侧分发后 `mret` 返回。
- CLIC 配置成 **direct 模式**（`SHV = 0`），所有中断都进同一个入口，按 `mcause` 分发，
  不需要 RAM 里的向量表。
- **矩阵里的目标写的是 CLIC ID**（线号 + 16），不是线号本身：`CORE0_xxx_INT_MAP_REG`
  写成 `19` 会让中断落到 CLINT 的 id 19 上，永远不会被取走。`interrupt::route` 会自动加
  偏移，用 `route_of` 读回的是线号。
- `sleep_ticks` 在 arm 比较器前会先清掉可能残留的 `INT_CLR` 请求：上一次睡眠若是通过读计数
  器结束的（而不是比较器中断），残留的请求会让下一次 `int_ena` 置位时立刻触发，实测表现为
  "每隔一次睡眠立刻返回"（20 ms 的目标平均只睡 10 ms）。
- `interrupt::epoch()` 在每次中断分发时自增，`sleep_ticks` 用它判断"这轮睡眠是否被别的中断
  打断"，从而把 CPU 让给调用者。实测有 200 pkt/s 广播流量时，20 ms 的睡眠平均 4.2 ms 就被
  MAC 中断打断，wfi 与忙轮询收到的帧数相同（1195 vs 1199 帧/5 s）。
- 处理器是普通 `fn()`，注册到按 CPU 中断线索引的表里；处理期间 `MIE = 0`，不支持嵌套。

HAL 固定占用两条线（见 [`interrupt::lines`](esp32p4-hal/src/interrupt.rs)）：

| 线 | 用途 |
|----|------|
| 19 | `SYSTIMER` target 0：`Delay::sleep_us` / `sleep_ms` 的定时唤醒 |
| 20 | EMAC DMA 完成（`ETH_MAC_INTR`） |

驱动侧接口：

```rust
// 打开 MAC 中断，处理器只做两件事：清 DMASTATUS、置一个标志
emac.enable_interrupts(esp32p4_hal::interrupt::lines::EMAC);
// 全局开关（启动最后一步）
unsafe { esp32p4_hal::interrupt::enable() };

loop {
    net.poll_wait(&mut emac, 20);   // 有活干就处理，没活就 wfi 睡 20 ms 或睡到定时器
    // ... 处理 socket，往 socket 写数据 ...
    net.poll(&mut emac);            // 把刚写进 socket 的帧立刻发出去
}
```

ISR 里不碰描述符环：环的所有权仍然只在主循环里，中断只负责把核心叫醒。这样既没有
数据竞争，也不需要临界区。

### HAL 中断接口

[`esp32p4_hal::interrupt`](esp32p4-hal/src/interrupt.rs) 把矩阵、CLIC 和处理器表包成一套带所有权的 API：

```rust
use esp32p4_hal::interrupt::{self, CpuInterrupt, Interrupt, Level};

fn isr() { /* 清外设状态，置一个标志 */ }

// 1. 占用一条线：检查是否已被占用、路由中断源、设定级别、注册处理器。
//    线保持 masked，外设准备好后再 enable。
let handler = unsafe {
    interrupt::claim(Interrupt::TG0_T0, CpuInterrupt::new(21), Level::LOW, isr)
};
handler.enable();

// 2. 多个源共线时，用矩阵状态区分来源
if interrupt::source_pending(Interrupt::LP_GPIO) { /* ... */ }

// 3. 临界区与 ISR 共享数据
interrupt::free(|cs| COUNTER.borrow(cs).set(0));
```

- **类型安全的源与线**：`CpuInterrupt` 是 0..=31 的 CPU 线（`new` 越界即编译期/运行期
  panic），`Source` 覆盖矩阵全部 128 个源。厂商 SVD 只命名了其中 95 个（例如 EMAC 的
  `SBD_INTR` 就缺失），所以路由接口接收 `impl Into<Source>`：具名的 `Interrupt`
  自动转换，缺失的用 `Source::SBD` 或 `Source::new(id)`。
- **所有权**：`claim` 用原子位图记录已占用的线，第二个驱动再抢同一条线会 panic；
  `InterruptHandler` 在 drop 时 mask、注销处理器并归还线，`leak()` 表示“这条线归程序
  所有”（`Delay` 就是这么做的）。
- **级别**：`mcliccfg.MNLBITS = 3`，每级编码成 8 位字节（级别 0 = `0x1f`，级别 7 =
  `0xff`，低 5 位硬连线为 1）。级别只决定**同时挂起时谁先被取走**；trap 入口里
  `MIE = 0`，所以不会发生嵌套抢占。
- **临界区**：`free(|cs| ...)` 把 `CriticalSection` 令牌传给闭包，`Mutex<T>` 只有拿到
  令牌才能借数据；`disable_all()` 返回 RAII guard，drop 时恢复 `MIE`。
- **矩阵状态**：`source_pending(source)` 读 `CORE0_INTR_STATUS_REG_n`，共享中断线的
  处理器据此判断是自己的哪个源。

### GPIO 中断与外设信号路由

[`gpio`](esp32p4-hal/src/gpio/mod.rs) 现在覆盖 GPIO 交换矩阵的两件事：

- **中断**：`GPIO_PINn.INT_TYPE` 选边沿/电平，`INT_ENA` 的 bit 13/14/16/17 分别对应
  `GPIO_INTR0..3`；`GPIO_STATUS_INTERRUPT` 是四个线共用的锁存状态，`clear_interrupt()`
  走 `STATUS_W1TC`，`interrupt_pending(line)` 读对应线的 `GPIO_INTRn`。
  `InterruptLine::source()` 直接给出 `Interrupt::GPIO_INTn`，交给 `interrupt::claim`。
- **信号路由**：[`gpio::signal`](esp32p4-hal/src/gpio/signal.rs) 按 TRM 表 10.12-1 生成了全部
  输入/输出信号的具名常量。发送方向用 `Pin::set_output_signal`（输出使能可来自
  `GPIO_ENABLE` 或外设），接收方向用 `Pin::set_input_signal`；同一个引脚可以同时喂给
  多个输入信号。

```rust
use esp32p4_hal::gpio::signal::{InputSignal, OutputSignal};

tx.set_output_signal(OutputSignal::UART0_TXD_PAD_OUT);
rx.set_input_signal(InputSignal::UART0_RXD_PAD_IN);

button.set_interrupt_type(InterruptType::NegativeEdge);
button.enable_interrupt(InterruptLine::Line0);
let handler = unsafe {
    interrupt::claim(InterruptLine::Line0.source(), CpuInterrupt::new(21), Level::LOW, isr)
};
handler.enable();
```

### 焊盘直接功能（`pins`）

「HP GPIO 矩阵能接任意外设」只对**在矩阵里有信号**的外设成立（UART、I2C、SPI、I2S、
TWAI、PWM、PCNT、RMT、SD、USB 控制信号……）。以下几类是例外，只有固定焊盘：

| 功能 | 焊盘 | 选择方式 |
|------|------|----------|
| **GMAC PHY/RMII** 数据与时钟（TXD0/1、TXEN、TXER、RXD0/1、RXDV、RXER、RMII_CLK） | GPIO28–53 中带 GMAC 功能的那些 | IO MUX 功能 3，**矩阵里没有这些信号** |
| **USB 数据线** USB Serial/JTAG（D-/D+） | GPIO24 / GPIO25 | 焊盘功能 0（模拟通路） |
| **USB 数据线** OTG 全速（D-/D+） | GPIO26 / GPIO27 | 焊盘功能 0（模拟通路） |
| **模拟功能** ADC1 / ADC2 / 触摸 / 比较器 / 32 kHz 晶振 | GPIO16–23 / GPIO49–54 / GPIO2–15 / GPIO51–54 / GPIO0–1 | 焊盘保持浮空，交给模拟模块 |
| **LP 外设**（LP UART、LP I2C、LP SPI……） | 仅 GPIO0–15 | 独立的 LP GPIO 矩阵 |

每个焊盘另有最多 4 个**直接** IO MUX 功能（`IO_MUX_GPIOx.MCU_SEL`）：功能 1 在所有焊盘上
都是 GPIO（走矩阵），功能 0/2/3 是焊盘专有的 —— GPIO2–5 的 JTAG、GPIO8–13 与 37–38 的
UART、GPIO6–11/28–38 的 SPI2 flash、GPIO39–48 的 SD1、GPIO28–53 的 GMAC PHY/RMII。

[`gpio::pins`](esp32p4-hal/src/gpio/pins.rs) 把 TRM 的「HP IO MUX Pin Functions」和
「GPIO Pin Analog Functions」两张表编成 `PAD_TABLE`：

```rust
use esp32p4_hal::gpio::MuxFunction;

let mut sd_clk = pins.pin::<43>();
sd_clk.set_mux_function(MuxFunction::Sd1Cclk);   // 写 MCU_SEL = 0
assert_eq!(sd_clk.mux_function(), Some(MuxFunction::Sd1Cclk));
sd_clk.set_mux_function(MuxFunction::Gpio);      // 回到矩阵

for pad in esp32p4_hal::gpio::pins::pads_with(MuxFunction::Uart0Txd) {
    // GPIO37
}
```

`PadDesc` 还记录每个焊盘的复位驱动强度、模拟功能（`AnalogFunction`：ADC/触摸/比较器/
32 kHz 晶振/USB PHY）、以及它是否属于 LP 域（GPIO0–15）。`PadDesc::is_usb_phy()` 与
`PadDesc::is_gmac_phy()` 用来判断某个焊盘是不是那两类专用引脚；USB 两对焊盘另有常量
`USB_SERIAL_JTAG_PADS` (24, 25) 与 `USB_OTG_FS_PADS` (26, 27)。

## UART

[`uart`](esp32p4-hal/src/uart.rs) 覆盖 `UART0..UART4`（LP UART 的寄存器布局相同，但走 LP GPIO
矩阵，等 LP 引脚模型补上再接）：

- 轮询收发：`write`/`write_byte`/`flush`、`read`/`read_byte`/`read_byte_blocking`、
  带截止时间的 `read_timeout`；
- 5–8 数据位、1/1.5/2 停止位、奇/偶校验，信号反相、内部回环与 break；
- 时钟源可选 XTAL(40 MHz) / RC_FAST(17.5 MHz) / PLL_F80M(80 MHz)，默认 PLL_F80M；
  波特率用「整数预分频 + 16 分频小数」计算，`set_baudrate` 返回实际值；
- CTS/RTS 硬件流控（RTS 阈值可配）；
- 每个中断源单独 `enable_interrupt`/`interrupt_pending`/`clear_interrupt`；
- 引脚走 GPIO 矩阵（`set_pins(tx, rx)`），任何 GPIO 都能当 TX/RX/CTS/RTS ——
  UART 的信号都在矩阵里；USB、EMAC PHY 这类没有矩阵信号的请见上一节的例外表。

```rust
let uart = Uart1::init(dp.UART1, &UartConfig::default().baudrate(115_200), &dp.HP_SYS_CLKRST);
uart.set_pins(pins.pin::<20>(), pins.pin::<21>());
uart.write(b"hello\r\n");
uart.flush();
```

示例 [`uart`](esp32p4-examples/src/uart.rs) 用内部回环自检，三组数据（含 64 字节递增序列）
原样收回；实测 115200 配置下硬件给出 115201，属正常的分频舍入。

## I2C

[`i2c`](esp32p4-hal/src/i2c.rs) 是 `I2C0`/`I2C1` 的主机模式驱动：

- 100 kHz / 400 kHz 及介于其间的任意 SCL 频率，时钟源 XTAL 或 RC_FAST；
  时序按 ESP-IDF `i2c_ll_master_cal_bus_clk` 的公式算（预分频 + 低/高/等待高/保持/建立）；
- `write`、`read`、`write_read`（重复起始）与 `probe`；每个等待都有以
  `delay::micros()` 为准的截止时间，从机拉伸时钟不会误判超时；
- NACK / 仲裁丢失 / 总线忙 / 超时都有独立错误值；`clear_bus` 发 9 个 SCL 脉冲救总线，
  `reset_fsm` 复位控制器状态机；
- 一条事务最多 8 个命令寄存器、一 FIFO（32 字节，含地址字节）；写 31 字节、读 32 字节、
  写读组合写 30 字节，超出返回 `TooManyCommands`/`TooMuchData`；
- SCL/SDA 配成开漏、输出使能交给外设，需要外部上拉。

```rust
let i2c = I2c0::init(dp.I2C0, &I2cConfig::default(), &dp.HP_SYS_CLKRST);
i2c.set_pins(pins.pin::<8>(), pins.pin::<9>());

let mut value = [0u8; 1];
i2c.write_read(0x48, &[0x0f], &mut value).unwrap();
```

示例 [`i2c_scan`](esp32p4-examples/src/i2c_scan.rs) 扫描 7 位地址空间；空总线上每个探测
立刻返回 `NoAcknowledge`（实测 112 个地址约 5 ms）。

## ADC

[`adc`](esp32p4-hal/src/adc.rs) 是两路 12 位 SAR ADC 的 one-shot 驱动（RTC/LP 控制器；
HP 数字控制器 + DMA 的连续采样模式还没做）：

- `ADC1` 八通道（GPIO16–23）、`ADC2` 六通道（GPIO49–54）；ADC 通道不是 GPIO 矩阵信号，
  `AdcInput::new` 从焊盘功能表（[`gpio::pins`](esp32p4-hal/src/gpio/pins.rs)）反查通道号，
  并把焊盘设成浮空（关输入缓冲、关上下拉、关输出）；
- 每个通道独立衰减：0 / 2.5 / 6 / 12 dB，随 `AdcInput` 携带；
- 阻塞读取 `read_raw`（0–4095）与 `read_millivolts`，也可以 `start` / `is_done` /
  `read_result` 自己轮询；
- 工厂校准来自 eFuse：每次转换前把该单元/衰减对应的 offset 码写进 SAR（模拟 I2C
  寄存器 `ADC_SARn_DREF` + `INITIAL_CODE`），`read_millivolts` 再用参考点、误差多项式
  与逐通道补偿换算成毫伏；eFuse 未烧录时退回标称满量程并如实报告。

```rust
let adc = Adc::init(dp.ADC, dp.LP_ADC, &dp.HP_SYS_CLKRST, &dp.LP_PERI);
let mut sense = pins.pin::<16>();
let input = AdcInput::new(&mut sense, Attenuation::_12dB);
let millivolts = adc.read_millivolts(&input);
```

示例 [`adc`](esp32p4-examples/src/adc.rs) 先报告校准是否可用，再把 13 个 ADC 焊盘在四档
衰减下各采两次。实测同一浮空焊盘在四档衰减下换算出的电压互相吻合（例如 743/716/668/606
mV，差异来自采样把焊盘电容一点点放掉），说明衰减与校准系数是对得上的。

## eFuse 与芯片版本

[`efuse`](esp32p4-hal/src/efuse.rs) 只读 BLOCK0/1 的寄存器快照（bootloader 已在复位时加载）：

- `chip_version()`：主/次版本，`revision()` 采用 ESP-IDF 的 `major*100+minor`；
  `cpll_hz()` 在 v3.0 之前返回 360 MHz，v3.0 起返回 400 MHz。
- `mac()`：出厂 48 位 MAC。
- `flash_type()` / `flash_page_size_code()` / `flash_ecc_enabled()` /
  `flash_power_up_delay_ms()`。
- `psram_capacity_code()` / `psram_vendor()` / `package_version()` /
  `temperature_code()`。

需要版本相关默认值的模块现在直接吃这个句柄：

```rust
let efuse = Efuse::init(dp.EFUSE);
let clocks = ClockTree::init_from_efuse(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST, &efuse);
let psram = Psram::init(PsramConfig::from_efuse(&efuse))?;
```

## 外部内存 MMU 与 cache

[`mmu`](esp32p4-hal/src/mmu.rs) 把 PSRAM 控制器的 1024 个 64 KB 表项做成带所有权的
`Region`：

```rust
let alias = Region::map(0, 512, 1).unwrap();   // 物理页 0 → 表项 512
let cached = alias.cached_ptr();               // 0x4A00_0000
let uncached = alias.uncached_ptr();           // 0x8A00_0000
alias.unmap();
```

映射表用一个全局位图记录占用，重叠映射返回 `Error::AlreadyMapped`；`map`/`unmap` 会自动
失效对应窗口的 cache 行。`Psram` 现在内部就持有这个 `Region`。

[`cache`](esp32p4-hal/src/cache.rs) 补齐了 DMA 一致性和现场保护：

- `writeback`（只清脏位）、`invalidate`（丢弃副本）、`writeback_invalidate`（交给 DMA
  后还要读回）；
- `suspend()` / `resume()` 暂停 L1 I/D 与 L2，用于改 Flash/PSRAM 时序或重映射 MMU；
  这两个包装函数放在 `.ramfunc`，调用方的临界区也必须从 RAM 执行。

## 定时器

按用途分成三层：

| 模块 | 硬件 | 计数单位 | 用途 |
|------|------|----------|------|
| [`time`](esp32p4-hal/src/time.rs) | `SYSTIMER` unit 0 | 16 MHz tick（62.5 ns） | 单调时钟：`Instant` / `Duration`；52 位，约 8.9 年回绕 |
| [`systimer`](esp32p4-hal/src/systimer.rs) | `SYSTIMER` COMP1 / COMP2 | 同上 | 绝对 deadline（一次性）与固定周期闹钟，带中断 |
| [`timer`](esp32p4-hal/src/timer/mod.rs) | `TIMG0` / `TIMG1`，各 2 路 + 1 个 MWDT | 可配预分频，默认 1 µs | `TimerGroup0/1` → `Timer00/01/10/11` 通用计数器 + `Watchdog0/1` 看门狗 |

`time` 只读计数器，永远自由运行；`Delay` 在它上面借 comparator 0 睡到指定 tick（`sleep_ms` 睡一段时间，`sleep_until(Instant)` 睡到某个时刻）。

```rust
let start = Instant::now();
do_work();
let elapsed: Duration = start.elapsed();   // 与 delay::micros() 同一个时间源
```

### 定时器组：`TimerGroup0/1`、`Timer00/01/10/11` 与 `Watchdog0/1`

```text
TimerGroup0 ─┬─ Timer00      矩阵源 TG0_T0
             ├─ Timer01      TG0_T1
             └─ Watchdog0    TG0_WDT
TimerGroup1 ─┬─ Timer10      TG1_T0
             ├─ Timer11      TG1_T1
             └─ Watchdog1    TG1_WDT
```

`TimerGroup0::init(dp.TIMG0, &clkrst)` 接管 `TIMG0` 的所有权，打开时钟门、把两路计数器和
看门狗复位到已知状态，然后像硬件那样把部件作为**公开字段**持有——`group.timer0`、
`group.timer1`、`group.wdt`。每个部件是独立类型，各持有一个指向本组寄存器的 `TIMG` 句柄
（不是全局指针、也没有类型转换）；CPU 线由 group 拥有，`group.enable_timer0_interrupt(...)` /
`enable_timer1_interrupt(...)` 负责申请、路由、释放。计时器的方法全部取 `&self`：

- 时钟源可选 `XTAL_CLK` 40 MHz、`RC_FAST_CLK`（约 17.5 MHz）、`PLL_F80M_CLK` 80 MHz，
  16 位预分频（2..=65536）；默认 40 → 1 tick = 1 µs。`timer.ticks(Duration)` 负责把时间段
  换算成这个定时器的 tick。
- 54 位计数器，可增可减；重载值（`set_reload_value`）决定 `start()` 从哪儿开始，
  闹钟值（`set_alarm`）是绝对 tick 数。
- **没有东西会自己启动**：配置完再 `timer.start()`——它先停计数器、从重载值重新装入，
  然后才开始计数，所以重启一定是确定性的。`stop()` / `is_running()` / `counter()` 配套。
- 闹钟触发后硬件会清掉 `ALARM_EN`。开启 auto-reload 时计数器从重载值重新开始，HAL 的
  中断壳清状态后自动重新武装，周期性闹钟因此不需要用户干预；一次性闹钟保持关闭。
- 每路有独立的矩阵源，CPU 线由调用者传入；`group.enable_timer0_interrupt(line, handler)`
  （或 `..._timer1_...`）内部走 `interrupt::claim`，线被占用会 panic，drop group 或调用
  `disable_timer0_interrupt()` 时自动屏蔽并交还该线。

```rust
let group = TimerGroup0::init(dp.TIMG0, &dp.HP_SYS_CLKRST);
let fast = &group.timer0;
let slow = &group.timer1;

// 100 ms 周期中断
group.enable_timer0_interrupt(CpuInterrupt::new(21), tick);
fast.set_reload_value(0);
fast.set_alarm(fast.ticks(Duration::from_millis(100)));
fast.set_auto_reload(true);
fast.start();

// 1 s，倒计数
slow.set_direction(Direction::Down);
group.enable_timer1_interrupt(CpuInterrupt::new(22), slow_tick);
slow.set_reload_value(slow.ticks(Duration::from_secs(1)));
slow.set_alarm(0);
slow.set_auto_reload(true);
slow.start();

unsafe { interrupt::enable() };
```

中断里没有 `&Timer00`，用 `unsafe { Timer00::steal() }` 拿到**同一个类型**的副本
（和 `Pin::<N>::steal()` 一个模式）：一个寄存器都不写，不启动计数、不持有 CPU 线，
drop 它也不会影响 owner。四路计数器和两个看门狗都能这样 `steal()`；中断的申请与交还则
始终留在 group 上。

`Watchdog0` / `Watchdog1` 是 MWDT，就是 `group.wdt`：`set_timeout(MwdtStage::Stage0,
Duration::from_secs(1))` 配合 `set_stage_action(stage, MwdtStageAction::ResetSystem)`，
然后 `enable()`；之后必须在超时前 `feed()`，否则系统复位。它也有 `steal()`，因此在别处
喂狗不需要持有 group。`startup::init` 已经把 bootloader 留下的看门狗关掉了，所以不碰它的
程序不会被 reset。

### 时钟树：`ClockTree`

`ClockTree::init(dp.HP_SYS_CLKRST, dp.LP_AON_CLKRST)` 拥有两个时钟/复位控制器，并 `Deref`
到 `HP_SYS_CLKRST` 的寄存器块，所以可以直接传给其它驱动（`TimerGroup0::init(dp.TIMG0, &clocks)`）：

- **ROOT_CLK 选源**：`XTAL_CLK` 40 MHz / `CPLL_CLK`（360 或 400 MHz）/ `RC_FAST_CLK`（约
  17.5 MHz）。`root_source()` / `root_hz()` 读取，`set_root_source()`（unsafe）切换。
- **SoC 分频链**：`CPU → MEM → SYS → APB` 串联。`dividers()`、`cpu_hz()`（含分数分频）、
  `mem_hz()`、`sys_hz()`、`apb_hz()`；`set_dividers()` 按硬件要求的顺序写入每级并用
  `SOC_CLK_DIV_UPDATE` 锁存（升频先 APB 后 CPU，降频反之），超出
  `CPU/MEM/SYS/APB` 上限会 panic。
- **CPU 变频**：`set_cpu_frequency(mhz)` 支持 360/180/90（360 MHz CPLL）或
  400/200/100（400 MHz CPLL），自动配 MEM/APB 分频，最后才切换源；标记为 unsafe，
  因为 flash/PSRAM 的 MSPI 时序重调不在驱动里（ESP-IDF 会挂起外部 cache 再切）。
- **参考时钟 `PLL_FxxM`**：`RefClock::{F20M,F25M,F50M,F80M,F120M,F160M,F240M}`，
  `ref_hz()` / `ref_enabled()` / `set_ref_divider()` / `enable_ref()`（F50/F25 来自 MPLL，
  其余来自 SPLL）。
- **LP 时钟**：`lp_slow_hz()`（RC_SLOW / XTAL32K / OSC_SLOW）、`lp_fast_hz`（RC_FAST / XTAL /
  PLL_LP）、`lp_peri_hz()` 及对应设置函数。
- **源门控**：`source_enabled()` / `set_source_enabled()`（XTAL / RC_FAST / CPLL / SPLL / MPLL）。

PLL 的模拟配置（分频、校准）不在这里：CPLL 由 bootloader 拉起，MPLL 由 PSRAM 初始化提供。
树按 [`Sources`](esp32p4-hal/src/clock.rs) 表报告频率，`init_with(..., Sources { cpll_hz: CPLL_400M_HZ, .. })`
可以覆盖 400 MHz 的情况。

### `SYSTIMER` 比较器闹钟

`SYSTIMER` 的 3 个比较器里，COMP0 归 `Delay`，另外两个是
[`systimer::Systimer`](esp32p4-hal/src/systimer.rs) 的部件：`systimer.alarm1`（COMP1）与
`systimer.alarm2`（COMP2）。`Systimer::init(dp.SYSTIMER)` 接管外设所有权，每个 alarm 各持有
一个指向同一寄存器的句柄；CPU 线由 `Systimer` 拥有，`enable_alarm1_interrupt(line, handler)` /
`enable_alarm2_interrupt(...)` 负责申请与释放，drop 时自动屏蔽并交还。

- **target 模式**：一次中断，比较值是完整的 52 位计数器值，也就是一个 `Instant`
  （`alarm.set_target(Instant::now() + Duration::from_secs(5))`）。
- **period 模式**：每 δt 一次中断，δt 存在 26 位寄存器里，因此上限约 67 秒
  （`alarm.set_period(Duration::from_millis(10))`）。周期模式由硬件自己重新武装，
  handler 只要应答。
- 中断是电平型，HAL 的中断壳负责清 `int_clr`；`enable_alarmN_interrupt` 内部走
  `interrupt::claim`，线被占用会 panic。`unsafe { Alarm1::steal() }` 同样给中断上下文
  一份只访问寄存器的副本。

```rust
let systimer = Systimer::init(dp.SYSTIMER);
let delay = systimer.delay();          // COMP0，独占的 Delay 驱动

let periodic = &systimer.alarm1;
systimer.enable_alarm1_interrupt(CpuInterrupt::new(21), tick);
periodic.set_period(Duration::from_millis(250));

let one_shot = &systimer.alarm2;
one_shot.set_target(Instant::now() + Duration::from_secs(5));
```

厂商 SVD 只描述每个 TIMG 的 timer 0，timer 1 落在保留区里。这里没有手写寄存器层：
[`xtask/gen_timg.py`](xtask/gen_timg.py) 按 TRM 布局（两路相隔 0x24，与 ESP-IDF 的
`timg_hwtimer_reg_t hw_timer[2]` 一致）补出完整的两组，生成 [`timg.svd`](timg.svd)，
`cargo xtask gen-pac` 再用它**替换**厂商的 TIMG0/TIMG1。于是 `T1CONFIG`…`T1LOAD`
和 `WDTx` 一样都是 PAC 的寄存器类型与访问器，HAL 里没有裸指针读写。

## 内存布局与启动

ESP32-P4 不是“ROM 直接跳到应用”的芯片：镜像由 ESP-IDF 2nd-stage bootloader
加载。当前布局与 bootloader 的期望一致：

| 段 | 地址 | 内容 |
|----|------|------|
| flash segment 0 (DROM) | `0x4000_0020` | `esp_app_desc_t` + `.rodata` |
| flash segment 1 (IROM) | `0x4000_0xxx` | `.init` + `.text` + `.trap` |
| RAM | `0x4FF2_0000` | `.ramfunc` / `.data` / `.bss` / stack |
| SPM ("TCM") | `0x3010_0000` | `.tcm.text` / `.tcm.data` |

- `0x4FF0_0000..0x4FF1_FFFF` 预留给 bootloader 配置的 128 KB L2 cache。
- RAM 顶端在 `0x4FFA_DFC0`，避开 ROM 的 BSS/stack。
- bootloader 会拷贝每个 RAM 段到镜像里写的加载地址；`_start` 只需要清零 `.bss`。
- bootloader 在 P4 上要求恰好两个 **flash** 段，因此 `link.x` 用一个 NOLOAD
  间隙段把 DROM 和 IROM 分开；RAM 段数量不受这个限制（idf 的
  `verify_load_addresses` 明确接受 SPM 段）。

### 把代码放进 RAM / TCM

两个属性宏对应两种 RAM，都是“镜像里在 flash、启动时被 bootloader 拷到 RAM”：

```rust
#[esp32p4_hal::ramfunc]           // .ramfunc → SRAM，flash cache 挂起时仍可运行
fn mspi_retune() { /* ... */ }

#[esp32p4_hal::tcmfunc]           // .tcm.text → 0x3010_0000，CPU 自己的总线，无 cache
fn fast_isr() { /* ... */ }

#[esp32p4_hal::tcmdata]           // .tcm.data → 0x3010_0000
static SAMPLES: [u32; 64] = [0; 64];
```

- **TCM 就是 HP SPM**（8 KB，`0x3010_0000..0x3010_1FFF`）：寄存器里叫 `HP_TCM`，
  文档里叫 SPM。只有它不经过 cache，访问延迟两个周期，也是 CPU 复位向量地址。
- `#[ramfunc]` 标记的函数会被补上 `#[inline(never)]`（否则内联副本仍在 flash），
  它调用的函数不跟着走，需要一起标。
- 初始化值随镜像在 flash，`link.x` 把 `.tcm.text` / `.tcm.data` 放进 `SPM` 区，
  `memory.x` 提供该区（本仓库两份 `memory.x` 已包含）。
- [`tcm`](esp32p4-hal/src/tcm.rs) 模块提供 `contains()`、`used_range()` 以及
  `init_parity()`（SPM 的奇偶校验初始化，bootloader 不做）。
- 例程见 [`esp32p4-examples/src/tcm.rs`](esp32p4-examples/src/tcm.rs)，它会打印每个
  函数实际落在哪个地址。

细节见 [`docs/boot.md`](docs/boot.md)。

## 重新生成 PAC

`esp32p4.svd` 更新后：

```bash
cargo xtask gen-pac            # 重新生成 esp32p4-pac/src/generated
cargo xtask gen-pac --check    # 仅校验生成代码是否与 SVD 一致
```

生成器会展开 ESP SVD 的 `<dim>` 寄存器数组（`GPIO%s`、`IV_MEM[%s]` 等），
并处理数组元素与普通寄存器重名的情况。生成代码已自动 `rustfmt`。

厂商 SVD 缺失或残缺的外设由 fragment 补全，按名字**替换**厂商的定义：
[`emac.svd`](emac.svd)（整块缺失，脚本见 `xtask/gen_emac.py`）、
[`timg.svd`](timg.svd)（两组各缺 timer 1，脚本见 `xtask/gen_timg.py`，重新生成用
`python3 xtask/gen_timg.py`），以及 [`psram_mspi.svd`](psram_mspi.svd)
（`PSRAM_MSPI0`/`PSRAM_MSPI1` 两个 PSRAM 控制器在 `0x5008_E000`/`0x5008_F000`
整块缺失，用 `derivedFrom` 继承同为 MSPI IP 的 `SPI0`/`SPI1` 寄存器定义）。

## 工具链

- Rust **1.100+**（`workspace.package.rust-version = "1.100"`）：用到了 1.100
  稳定的 `core::alloc::Allocator`（`Vec::new_in` / `Box::new_in` 指定分配器）。
- 默认 target：`riscv32imafc-unknown-none-elf`。

## 验证

```bash
cargo check -p esp32p4-pac --target riscv32imafc-unknown-none-elf
cargo check -p esp32p4-hal --target riscv32imafc-unknown-none-elf
cargo clippy -p esp32p4-hal -p esp32p4-rt -p esp32p4-macro -p esp32p4-examples \
  --target riscv32imafc-unknown-none-elf -- -D warnings
cargo fmt --all -- --check
cargo xtask gen-pac --check

cd esp32p4-examples
cargo build --bin blink
espflash save-image --chip esp32p4 \
  ../target/riscv32imafc-unknown-none-elf/debug/blink /tmp/blink.bin
```

## 当前范围

路线图（还缺哪些模块、按什么顺序补、每个模块的完成标准）见
[`docs/roadmap.md`](docs/roadmap.md)。

已实现：

- 从 SVD 生成完整 PAC（89 个外设、约 5600 个展开寄存器）
- RISC-V 启动、trap 入口、CSR/汇编封装、ESP-IDF 兼容镜像布局
- GPIO0–GPIO53 的推挽/开漏输出与输入、IO MUX 配置、四路中断（边沿/电平、按线使能与状态清除）、
  完整的 HP GPIO 交换矩阵信号路由（`gpio::signal`）
- SYSTIMER 阻塞延时（`sleep_ms` / `sleep_until`）与单调时钟（`time::Instant` / `Duration`）
- SYSTIMER 比较器闹钟（`Systimer` → `Alarm1` / `Alarm2`：绝对 deadline 与周期模式，带中断）
- 定时器组（`TimerGroup0/1` → `Timer00/01/10/11`：4 路 54 位计数器，时钟源与预分频、增/减计数、
  软件重载、一次性与周期性闹钟中断）与主系统看门狗（`Watchdog0/1`：分阶段超时、
  中断/复位动作、`feed()`）
- 代码放置（`#[ramfunc]` / `#[tcmfunc]` / `#[tcmdata]`：SRAM 与 8 KB HP SPM，`tcm` 模块提供范围与奇偶校验）
- 时钟树（`clock::ClockTree`：ROOT_CLK 选源、CPU/MEM/SYS/APB 分频与 CPU 变频、
  PLL_FxxM 参考时钟、LP 慢/快时钟、源门控）
- 复位控制与复位原因（`system::software_reset` / `software_reset_cpu` / `reset_reason`，纯寄存器实现）；
  外设复位（`reset::Resettable`：`HP_SYS_CLKRST.HP_RST_EN0/1/2` 上的 HP 外设位，
  `init()` / `reset()` / `unreset()`，掩码可用 `|` 组合）
- 封装内 PSRAM 初始化（MPLL 250 MHz、16/32 MB 自动检测、MMU 映射、`PsramAllocator`）；
  `mmu::Region` 提供外部内存页的 map/unmap 与 cached/uncached 地址
- eFuse 只读访问（芯片/块版本、MAC、Flash 与 PSRAM 参数），`clock`/`psram` 可按版本取默认值
- cache 维护补齐 `writeback_invalidate` 与 L1 I/D + L2 的 `suspend`/`resume`
- USB Serial/JTAG（CDC-ACM）收发：`UsbSerialJtag`，以及可选的 `defmt` 全局 logger
- 以太网 MAC（RMII + MDIO + DMA 描述符环），含 L1/L2 cache 维护
- 同步 TCP/IP（smoltcp）：DHCP 客户端、TCP/UDP/ICMP、可选 IPv6，无 async、堆可选
- UART0–UART4：轮询收发、5–8 数据位/奇偶/停止位、信号反相与回环、CTS/RTS 流控、
  可选中钟源、按源中断；引脚经 GPIO 矩阵任意路由
- I2C0/I2C1 主机：100/400 kHz 及任意 SCL、7 位地址、写/读/写读、总线恢复、
  NACK/仲裁/超时错误与按源中断
- CLIC 中断框架（direct 模式、按 CPU 线分发、trap 保存 FPU 上下文）与 EMAC/SYSTIMER
  中断驱动
- `defmt` 日志支持（`Format` 实现 + 关键路径日志 + 基于 `mstatus.MIE` 的
  `critical-section` 实现）
- `#[main]`（注入 `startup::init()`：关 bootloader 遗留的 RTC/MWDT 看门狗、停放 core 1、
  启用分支预测器）、默认 panic handler（`defmt` 输出 + `ebreak`，无调试器时停在 trap）
- stable `riscv32imafc-unknown-none-elf`（RV32IMAFC、ilp32f 硬浮点 ABI），启动时使能 FPU

尚未实现：

- 中断嵌套与优先级分级、其它外设中断
- PLL 的模拟配置（CPLL/SPLL/APLL 的分频与校准；时钟树本身见 `clock::ClockTree`）
- LP UART 与 LP I2C（需先补 LP GPIO 矩阵/焊盘模型）、SPI、DMA、RMT、WS2812 等驱动
- EMAC 中断驱动、TX/RX 校验和卸载、PTP 时间戳、MII/RGMII 接口
- 阻塞式 socket API（当前是 smoltcp 的事件/轮询风格）、DHCP 服务端
- USB OTG FS/HS 主机与设备栈
- LP 核与低功耗
- trap 入口尚未保存 FPU 寄存器上下文（当前不使能可恢复中断）

## 许可证

MIT OR Apache-2.0

每个可发布 crate 目录里的 `LICENSE-APACHE` / `LICENSE-MIT` 是指向仓库根同名文件的软链接，
`cargo package` 会跟随链接、把实际内容打进 `.crate`（不是把链接本身打进去）。
