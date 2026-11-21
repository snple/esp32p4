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
| [`esp32p4-hal`](esp32p4-hal/) | **HAL**：时钟树、GPIO / IO MUX、UART、I2C 主机、SPI 主机/从机（GPSPI2/3）、SD 卡主机（SDHOST）、ADC、温度传感器、GDMA（GDMA-AHB）与描述符链、SYSTIMER 延时/单调时钟/闹钟、定时器组与看门狗、复位、封装内 PSRAM、USB Serial/JTAG、以太网 MAC、同步 TCP/IP |
| [`esp32p4-usb`](esp32p4-usb/) | **USB 栈**：DWC OTG 全速/高速控制器的设备模式与主机模式驱动（slave 模式、轮询）；协议层全部在跨仓库共享的 [`usb-stack`](../../rp2350/usb-stack)（设备状态机、标准请求、class、主机枚举与控制/bulk/interrupt 传输、HID 键盘） |
| [`esp32p4-examples`](esp32p4-examples/) | 示例：GPIO2 blink、UART 回环、I2C 扫描、SPI 主机/从机自检、SD 卡读写、ADC 单次与扫描采样、GDMA 存储到存储拷贝、PSRAM 初始化与分区、USB Serial/JTAG、USB CDC、USB HID 主机 |
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
| `usb_host_hid` | 全速 OTG 作主机：枚举 USB 键盘，打印按键的 HID usage code |
| `usb_host_hub` | 全速 OTG 作主机：枚举 HUB，端口上电/复位后枚举下游键盘并读按键 |
| `timer` | `TIMG0` 的两路通用定时器：100 ms 周期中断闪灯，外加 1 s 的倒计时闹钟 |
| `alarm` | `SYSTIMER` 比较器：250 ms 周期闹钟 + 5 s 一次性 deadline，主循环用 `sleep_until` 睡到下一个检查点 |
| `watchdog` | `TIMG0` 的 MWDT：1 s 超时复位系统，主循环每 100 ms `feed()` 一次 |
| `emac` | 初始化 RMII 以太网（P4 Function-EV 板的 IP101）、扫描 MDIO、自动协商、打印链路状态并收发 ARP 帧 |
| `net_tcp_echo` | DHCP + TCP echo 服务器（需 `--features net`，无堆、无 async） |
| `uart` | `UART1` 内部回环自检：三组数据原样收发，并打印波特率与帧格式 |
| `i2c_scan` | `I2C0` 扫描 7 位地址空间，逐个探测并打印应答的从机 |
| `adc` | 报告 eFuse 校准是否可用，并在四档衰减下采样 13 个 ADC 焊盘 |
| `dma_mem2mem` | GDMA-AHB 的存储到存储拷贝：单描述符 64 B 与四描述符 16 KB 两种链路，逐字节校验并打印吞吐 |
| `spi` | GP-SPI2 主机对 GP-SPI3 从机（四根飞线）：时钟模式 0–3 × 1/4/32/64 字节双向校验 |
| `sd` | SDHOST 上的 SD 卡：识别、读 CID/CSD 与容量、读第 0 块、双块写入后回读比对 |
| `adc_scan` | ADC1 样式表扫描：两个通道经 GDMA-AHB 交替采样 32 个样本，打印样本数与读数 |
| `tsens` | 温度传感器：每秒打印一次芯片温度，自动选择测量档位 |
| `ledc` | LEDC：GPIO2 上扫 0/25/50/75/100 % 占空比并回读波形，再用 100 步渐变拉到满占空比 |
| `rmt` | RMT：GPIO2 上自发自收一段 WS2812 波形，逐脉冲比对电平与时长 |
| `pcnt` | PCNT：用 LEDC 的 10 kHz 方波计数，验证限值回绕、标志锁存与暂停/恢复 |
| `mcpwm` / `mcpwm_extras` | MCPWM0：前者查计数器/强制电平/三档占空比/互补加死区，后者查同步、捕获、故障跳闸、载波、ETM、事件比较器与中断 |
| `twai` | TWAI0（CAN 2.0B）：把 TX/RX 路由到同一个焊盘做回环，验证标准/扩展/远程帧往返、过滤器正负向、收发中断、位时序与非法输入 |
| `i2s` | I2S0 标准模式：TX 主 / RX 从 + 内部 BCK/WS 共享、SD 单焊盘回环，验证时钟树、立体声与 mono 逐字节往返、BCK 计数、中断与非法配置 |

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
> 所有 `info!`/`debug!` 会直接在编译期消失，看起来就像"日志没工作"。它配在
> `esp32p4-examples/.cargo/config.toml` 里，所以**要在那个目录下 cargo build/run**；
> 从仓库根目录构建会绕开这份配置，日志同样会消失。

如果 `espflash monitor` 把日志打成一堆二进制乱码，说明它没认出 `defmt` 帧——4.7.0
的 `monitor` 不会自己从 flash 里找 `LOG_FORMAT` 标记，把 ELF 递给它就行：

```bash
espflash monitor --chip esp32p4 --log-format defmt \
  --elf target/riscv32imafc-unknown-none-elf/release/mcpwm
```

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

## USB OTG

P4 有两个独立的 DesignWare OTG 控制器，`esp32p4-usb` 用同一套寄存器定义
（`src/dwc.rs`）驱动它们，**slave 模式**（CPU 搬 FIFO，无 DMA）、**轮询**
（无中断、无 async），既能当设备也能当主机：

| 控制器 | 基地址 | 速率 | PHY | 驱动 |
|--------|--------|------|-----|------|
| `USB_OTG11` | `0x5004_0000` | 全速 12 Mbit/s | 内部全速 PHY，D+/D- 在 GPIO27/26 | `UsbFs` / `UsbHostFs` |
| `USB_OTG20` | `0x5000_0000` | 高速 480 Mbit/s | 内部 UTMI+ PHY，不占 GPIO | `UsbHs` / `UsbHostHs` |

同一时刻一个核只能归一侧所有：设备驱动与主机驱动共用「核是否被占用」的标记，
重复初始化会 panic。

### 设备模式

```rust
use esp32p4_usb::class::cdc_acm::CdcAcm;
use esp32p4_usb::{FsResources, UsbFs};
use esp32p4_usb::{UsbBusAllocator, UsbDeviceBuilder};

let allocator = UsbBusAllocator::new(UsbFs::new(FsResources {
    clkrst: &dp.HP_SYS_CLKRST,
    lp_aon_clkrst: &dp.LP_AON_CLKRST,
    usb_wrap: &dp.USB_WRAP,
    gpio: &dp.GPIO,
    io_mux: &dp.IO_MUX,
}));
let mut serial = CdcAcm::new(&allocator);              // bulk 包大小取 UsbBus::MAX_PACKET_SIZE
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

层次（上三层来自共享 crate `usb-stack`，本仓库只提供最底层驱动）：

```text
class::cdc_acm::CdcAcm        usb-stack：接口描述符 + 端点回调
device::UsbDevice             usb-stack：枚举状态机、标准请求、EP0 控制传输
bus::UsbBus                   usb-stack：allocate / poll / read / write
UsbFs / UsbHs                 esp32p4-usb：DWC OTG 寄存器、FIFO 分配、复位与枚举
```

`UsbFs`/`UsbHs` 通过 `UsbBus::MAX_PACKET_SIZE`（64 / 512）告诉 class 该用多大的
bulk 包，因此同一个 `CdcAcm` 在全速和高速控制器上都能直接用。

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

### 主机模式

主机侧同样是 slave + 轮询，也分两层，且协议层与设备侧一样放在共享 crate
`usb-stack` 的 `host` 模块里，本仓库只提供控制器实现：

```text
host::class::hid::Keyboard     HID boot 键盘（其余 class 自备）
host::Device / host::Pipe      枚举、控制传输、bulk / interrupt 传输
host::HostController           trait：port_status / reset_port / alloc_channel / configure / setup / transfer_in / transfer_out
UsbHostFs / UsbHostHs     DWC OTG 主机模式驱动（本仓库）
```

`usb-stack` 本身按方向分层：`device`（总线 trait、控制管道、状态机、class）与
`host`（枚举、控制传输、管道、HID），两端共用的词汇表放在根模块
`endpoint` / `descriptor` / `request`——其中 `request::Request` 既解析设备收到的
8 字节 SETUP 包，也负责主机侧构造与编码。

```rust
use esp32p4_usb::host::Host;
use esp32p4_usb::host::class::hid::Keyboard;
use esp32p4_usb::{FsResources, UsbHostFs};

let controller = UsbHostFs::new(FsResources {          // 与设备模式共用资源结构体
    clkrst: &dp.HP_SYS_CLKRST,
    lp_aon_clkrst: &dp.LP_AON_CLKRST,
    usb_wrap: &dp.USB_WRAP,
    gpio: &dp.GPIO,
    io_mux: &dp.IO_MUX,
});
let host = Host::new(&controller);

let device = host.enumerate()?;                       // 复位端口 + 枚举（地址 1）
device.set_configuration(1)?;
let keyboard = Keyboard::open(&device)?;               // SET_PROTOCOL / SET_IDLE + 打开中断端点

let mut previous = Default::default();
loop {
    if let Ok(report) = keyboard.read() {             // 设备没变化时返回 Timeout
        for key in report.new_keys(&previous) {
            defmt::info!("key {}", key);
        }
        previous = report;
    }
}
```

`Host::enumerate` 做的是一套固定序列：复位端口 → `GET_DESCRIPTOR(DEVICE, 8)`
拿到 `bMaxPacketSize0` → `SET_ADDRESS` → 重新读设备描述符 → 读配置描述符
（先读 9 字节头拿 `wTotalLength`）→ 返回 `Device`。之后 `Device::control`
可以发任意控制请求，`Device::open` 把某个端点变成 `Pipe`。

要点：

- **通道与 PID**：每个端点占一个 host channel（`HCCHAR`），驱动按事务编程
  `HCTSIZ` 并置 `ChEna`，等核心 `ChHltd` 后再读 `HCINT`。数据翻转由协议层自己
  记账（`Toggle`），不依赖核心跨传输保存 PID，因此换地址、重配通道都不会错位。
- **NAK 与错误**：NAK 在驱动内重试（`NAK_RETRIES`，约 100 ms，HID 轮询就靠它），
  事务错误重试 3 次；STALL 直接返回 `HostError::Stall`，由调用方发
  `CLEAR_FEATURE(ENDPOINT_HALT)`（`Device::clear_halt`）并重置翻转。
- **FIFO 划分**：RX = 2 个最大包 + 8、非周期 TX = 1 个最大包 + 8、周期 TX =
  1 个最大包 + 8；HS 上共 536 ≤ 896 words。每个 OUT 事务前先 flush 对应 TX
  FIFO，避免上一次失败残留的包被当成新数据。
- **全速口的主机配置**：OTG 输入改接成 A 设备电平（`IDDIG=0`、`AVALID=1`、
  `VBUSVALID=1`、`BVALID=0`、`SESSEND=0`），并打开 `USB_WRAP` 的
  `srp_sessend_override`，让 PHY 认为 VBUS 正常；`GOTGCTL` 用 `AVALIDOV` /
  `VBVALIDOV` 固定会话，`GUSBCFG.FHMOD=1` 强制主机模式。
  低速设备接在端口上时把 `HCFG.FSLSPclkSel`/`HFIR.FrInt` 切换到 6 MHz。
- **高速口的主机配置**：`GUSBCFG` 选 UTMI+（`PHYIF=1`、`TOCAL=5`），并且必须
  把 `LP_SYS + 0x204` 的 bit1/2 **置位**——主机要 15 kΩ 下拉，设备模式则清零。
- **VBUS 由板子提供**：`HPRT.PrtPwr` 只是告诉核心「端口已上电」，芯片本身没有
  5 V 开关；板上没有 VBUS 开关时请用带电源的 HUB。
- **HUB 支持**：`usb-stack` 的 `host::class::hub` 负责 HUB 类请求（读 HUB 描述符、
  端口上电、端口状态/复位、子设备枚举），`host::AddressPool` 分配 1–126 的地址。
  全速口直接支持 FS HUB 及其 FS/LS 下游设备（LS 下游由驱动置
  `HCCHAR.LSPDDev`）。高速口没有实现 TT 拆分事务（`HCSPLT` 在 P4 的寄存器定义里
  也未公开），所以 FS/LS 设备接在高速 HUB 后面会得到 `HostError::Unsupported`；
  改用 `UsbHostHs::new_fsls_only` 让高速核以 FS/LS-only 主机工作，HUB 就会以全速
  枚举，下游 FS/LS 设备无需拆分事务（与 ESP-IDF 的 `fsls_only` 相同）。HS HUB 后面
  的 HS 设备仍可用普通 `UsbHostHs::new`。
- **限制**：HUB 可级联（全速链路上）；等时传输、多包流水（transfer level）未
  实现；主机口不支持 SRP/HNP（芯片的 ID/VBUS 悬空）。

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

[`adc`](esp32p4-hal/src/adc.rs) 是两路 12 位 SAR ADC 的驱动，两种采法都在：RTC/LP 控制器
的单次转换，以及 HP 数字控制器的样式表扫描（数据经 GDMA-AHB 直接落内存）：

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

### 多通道扫描

`Adc::scan` 走 HP 数字控制器：把 `Pattern` 列表写进样式表，控制器按 `timer_cycles` 的
周期逐项采样、循环，每一路转换结果由 GDMA-AHB 写成 4 字节样本（`Sample`：低 12 位是结果，
bit 13–15 是通道号），采满帧后自动停下。

```rust
let patterns = [
    Pattern::new(input_a.channel(), Attenuation::_12dB),
    Pattern::new(input_b.channel(), Attenuation::_12dB),
];
let mut buffer = [Sample::ZERO; 32];
let mut descriptors = [const { Descriptor::new() }; 1];
adc.scan(Unit::Adc1, &patterns, &dma.rx_channel::<1>(), &mut buffer, &mut descriptors, 4_000)?;
```

示例 [`adc_scan`](esp32p4-examples/src/adc_scan.rs) 每轮采 32 个样本、两个通道交替，
打印各自样本数与最后一次读数。四个只在硬件上才暴露的点：

- **`SAR1_DONE`/`SAR2_DONE` 每转换一次就置位**：拿它判断「整帧采完」会立刻返回，
  缓冲区还是旧内容。整帧结束要看 GDMA 的 `SucEof`（描述符上的 EOF 标志）；
- **停下来的次数由 `CTRL2.MAX_MEAS_NUM` + `MEAS_NUM_LIMIT` 决定**（8 位，所以一次扫描
  最多 256 个样本）；`DMA_CONF.APB_ADC_EOF_NUM` 是给 GDMA 的帧长，不是停止条件；
- **ADC2 接管给数字控制器的极性和 ADC1 相反**：ADC1 是 `SAR1_DIG_FORCE=1`，ADC2 是
  `SAR2_RTC_FORCE=0`；
- **让 FSM 自己管电源**：`XPD_SARn_FORCE=0`，否则 one-shot 留下的「软件强制上电」会让
  FSM 的上下电时序失效。

### 滤波器与阈值监控器

HP 数字控制器自带两级测量滤波器（`FILTER_CTRL0/1`）与两个阈值监控器
（`THRES0/1_CTRL`），扫描与 one-shot 都能用：

```rust
adc.set_filter(
    FilterIndex::Zero,
    Unit::Adc1,
    Some(Filter { channel: input_a.channel(), coefficient: FilterCoefficient::_4 }),
);
adc.set_monitor(
    MonitorIndex::Zero,
    Unit::Adc1,
    Some(Monitor { channel: input_a.channel(), low: 0, high: 1000 }),
);

if let Some(bound) = adc.monitor_state(MonitorIndex::Zero) {
    adc.clear_monitor(MonitorIndex::Zero); // 状态是锁存的，读后要手动清
}
```

滤波器是一阶 IIR（`value = ((k-1) * previous + new) / k + 0.5`），`k` 取 2/4/8/16/64。
两个只在硬件上才暴露的点：

- **两套通道编码不一样**：滤波器的 `FILTER_CHANNELn` 是 `(unit + 1) << 3 | channel`
  （ADC1 ch0 写 8，与 ESP-IDF 一致），监控器的 `THRESn_CHANNEL` 是 `unit << 3 | channel`
  （ADC1 ch0 写 0）。按监控器的写法配滤波器、或反过来，寄存器不报错但功能静默失效；
- **监控器不需要滤波器**：两者是并行的独立通路，没配 `Filter` 时监控器照样收到样本。
  阈值是 13 位原始码（0–8191），比较结果锁存在 `int_raw` 里，`monitor_state` 读它，
  越界后必须 `clear_monitor` 才能看到下一次状态。

中断经 `ADC_INTR` 进中断矩阵，`enable_monitor_interrupt` 只负责打开传感器自己这一路的
使能。示例 [`adc_scan`](esp32p4-examples/src/adc_scan.rs) 在扫描里同时配了滤波器和监控器，
每轮打印监控状态（实测高压时 `Some(High)`、窗口放宽后 `None`）。滤波的平滑效果没有做
定量验证：浮空焊盘采到的是一段缓慢漂移，产生不了可比较的阶跃。

## GDMA

[`dma`](esp32p4-hal/src/dma/mod.rs) 是 GDMA-AHB（PAC 里的 `AHB_DMA`，基址 `0x5008_5000`）。
P4 上还有两套 DMA 不在本模块范围：GDMA-AXI（`AXI_DMA`，LCD/CAM、SPI2/3、PARLIO、AES、SHA）
和 DesignWare 的 `DMA`（JPEG/摄像头用的纯存储搬运引擎）。

- 3 组发送/接收通道：`TxChannel<0..2>` / `RxChannel<0..2>`，每个通道可选接
  I3C / UHCI（UART0/1 的 DMA 前端）/ I2S0–2 / ADC / RMT，或用 `Dummy-n` 与配对的另一个
  通道做存储到存储；
- 描述符 3 个字（`size`/`length`/EOF/owner + buffer + next）；[`Chain`] 把一个缓冲区按
  4095 字节切成描述符链或环（`mount` / `mount_eof` / `mount_ring`），
  `Descriptor::length()` 读回引擎写入的有效字节数；
- `AhbDma::copy` 在通道对 1 或 2 上做阻塞式存储到存储拷贝，自带 cache 维护与截止时间；
- 中断按源单独 `enable_interrupt`/`interrupt_pending`/`clear_interrupt`，另有优先级、
  数据/描述符突发、ETM 启动与 owner 检查。

```rust
let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
let descriptors: [Descriptor; 4] = [const { Descriptor::new() }; 4];

let src = [0x5au8; 4096];
let mut dst = [0u8; 4096];
dma.copy(MemChannel::Ch1, &src, &mut dst, &descriptors).unwrap();
```

示例 [`dma_mem2mem`](esp32p4-examples/src/dma_mem2mem.rs) 每秒跑两遍：单描述符 64 B 与
四描述符 16380 B，逐字节校验。硬件实测 200 轮无描述符错误、无数据不符，未开突发时约
23 MB/s。

两个只在硬件上才暴露的点：

- **描述符要用非缓存别名访问**：P4 的片内 SRAM（`0x4FF0_0000`）和 PSRAM
  （`0x4800_0000`）都过 cache，CPU 写进缓存里的描述符引擎读不到 —— 实测引擎直接回
  `DescriptorError`。`Chain::mount` 因此用 `+0x4000_0000` 的直通别名（
  [`cache::uncached_alias`](esp32p4-hal/src/cache.rs)）读写描述符的字，并在挂载前清一次
  对应的缓存行；写进描述符的 `next` 仍是普通地址，因为引擎只认访问窗口内的地址；
- **访问窗口**：`AhbDma::init` 把可访问区间配成 `0x4000_0000..0x4FFC_0000`（与 ESP-IDF
  一致），描述符与缓冲区都要落在里面。直通别名（`0x8FF0_0000`、`0x8800_0000`）在窗口外，
  引擎会拒绝，所以 DMA 缓冲区放片内 SRAM 或 cached PSRAM 窗口。片内 SRAM 的读写
  `copy` 已经处理好 cache；cached PSRAM 虽然引擎能写对，但本仓库的 `cache` 维护还回读不到
  引擎写的数据（见 [roadmap](docs/roadmap.md)），暂按片内 SRAM 使用。

## SPI

[`spi`](esp32p4-hal/src/spi.rs) 覆盖 GP-SPI2 与 GP-SPI3 的 **CPU 传输**：CPU 填/读控制器
里 64 字节的数据缓冲（`SPI_W0..W15`），自己触发每次事务。没有任何 DMA 依赖
（SPI2/3 的 DMA 走 GDMA-AXI，见 [roadmap](docs/roadmap.md)），所以它也是本仓库里唯一
不碰描述符的高速外设。

- 主机 `Spi2`/`Spi3`：`init` 里配好时钟分频（源为 XTAL 40 MHz）、时钟模式 0–3、位序、
  CS 建立/保持时间；`transfer` 做全双工，`write`/`read` 是它的两个退化情形
  （读的时候 MOSI 输出 `0xFF`，因为时钟只在有数据可发时才转）；
- 从机 `SpiSlave2`/`SpiSlave3`：`start` 先武装（填 TX、设长度），`finish` 等主机把时钟
  打完后取回数据，并回报主机实际打了多少位；
- 引脚全部经 GPIO 矩阵，任意焊盘都能当 SCK/MOSI/MISO/CS；`set_cs` 可选，用硬件 CS 时
  一次传输的多段 64 字节之间 CS 保持低电平。

```rust
let spi = Spi2::init(
    dp.SPI2,
    &Config::default().frequency(1_000_000).mode(Mode::_0),
    &dp.HP_SYS_CLKRST,
);
spi.set_pins(pins.pin::<20>(), pins.pin::<21>(), pins.pin::<22>());
spi.set_cs(pins.pin::<23>());

let mut id = [0u8; 3];
spi.transfer(&[0x9f, 0x00, 0x00], &mut id);
```

示例 [`spi`](esp32p4-examples/src/spi.rs) 把 SPI2 主机和 SPI3 从机用四根线连起来
（`SCK 20↔16`、`MOSI 21↔17`、`MISO 22↔18`、`CS 23↔19`），跑 4 种时钟模式 × 4 种长度，
两个方向各自校验，上板全部通过。

三个只在硬件上才暴露的点：

- **每次传输前必须写 `SPI_CMD.SPI_UPDATE`**：`SPI_MS_DLEN` 等寄存器有影子副本，不触发
  更新的话从机只收到 1 bit；写完要等 `SPI_UPDATE` 自己清零；
- **数据缓冲是 AFIFO，指针跨事务保留**：`SPI_DMA_CONF.SPI_BUF_AFIFO_RST`（CPU 发送）与
  `SPI_RX_AFIFO_RST`（接收）必须在每次事务前各打一拍。否则第二次事务的读写指针停在上一次
  结束的位置 —— 表现是主机读回自己的 TX（RX 落到了下一个字），从机干脆不再驱动 MISO；
- **`SPI_USR_MOSI` 同时管发送和时钟**：只置 `SPI_USR_MISO` 的话 SCK 不动，读操作也必须
  把 `SPI_USR_MOSI` 打开（用 `0xFF` 填充）。

## SD 卡

[`sdhost`](esp32p4-hal/src/sdhost.rs) 驱动 SD/MMC 主机控制器（`SDHOST`，Synopsys
DesignWare MSHC）：[`SdHost`](esp32p4-hal/src/sdhost.rs) 负责控制器（时钟、总线位宽、命令、块传输），
[`SdCard`](esp32p4-hal/src/sdhost.rs) 负责卡（SD 规范的识别序列、从 CSD 解出的容量、
512 字节块读写）。

- 命令支持无响应、48 位（R1/R3/R6/R7）与 136 位（R2）响应；
- 块读写走控制器的内部 DMA（IDMAC）：驱动里有一块 2 KB 的暂存区，调用者的切片不需要对齐；
- 多块传输用 CMD18/CMD25 加一次手发的 CMD12（原因见下）；
- 位宽 1/4 位，卡时钟由 160 MHz PLL 分频得到（识别 400 kHz，之后 25 MHz）。

```rust
// 卡座电源：低电平通电（板上有 10K 下拉，这里显式驱动以免依赖复位默认值）。
let mut power = pins.pin::<45>();
power.set_function_gpio();
power.set_output_enabled(true);
power.set_low();

let host = SdHost::init(dp.SDHOST, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST, &dp.PMU);
host.set_pins(
    pins.pin::<43>(),
    pins.pin::<44>(),
    (pins.pin::<39>(), pins.pin::<40>(), pins.pin::<41>(), pins.pin::<42>()),
);

let card = SdCard::init(&host)?;
let mut block = [0u8; 512];
card.read_blocks(&host, 0, &mut block)?;
```

示例 [`sd`](esp32p4-examples/src/sd.rs) 识别卡（32 GB SDHC，容量 62 521 344 块）、读出
CID/CSD、读第 0 块（`0x55AA` 引导签名），再在卡尾写两个块并回读逐字节比对，上板通过。

四个只在硬件上才暴露的点：

- **槽位的引脚走 IO MUX，不是 GPIO 矩阵**：P4 的两个卡槽路径不同，槽 0（片上 SD 焊盘）
  用固定的 `SD1_*` 焊盘功能，槽 1 才走矩阵。命令里的 `CARD_NUMBER` 也要选对（这里是 0）。
  走错路径时命令发得出去、卡却不回响应；
- **供电分两路，缺一路都不行**：卡座本身的电源由板上的开关管给，栅极接 GPIO45 并带
  10K 下拉，所以**低电平才是通电**、高电平切断（下拉让复位后的默认状态就是通电）；
  SD 接口的电平域则由片上 LDO 通道 4 供，`SdHost::init` 里打开
  `PMU.EXT_LDO_P1_0P2A`（3.3 V 直连）。缺任意一路，所有命令都超时，看起来像接线错误。
  断电再上电后卡会重新上电复位，需要重新走一遍识别；
- **`COMMAND_DONE` 比响应早到**：中断位置起来时 `RESP0..3` 还是上一条命令的内容，必须等到
  `STATUS.RESPONSE_INDEX` 变成刚发的命令号再读，否则每条响应都会晚一条命令；
- **多块写不能用硬件的自动 CMD12**：`SEND_AUTO_STOP` 置位时数据阶段永远等不到 `DATA_OVER`
  （卡总线已经传完 1024 字节），改成数据阶段结束后手发 CMD12 就正常。另外 IDMAC 的描述符
  与缓冲区都过 cache，必须用 `cache::writeback`/`invalidate` 维护（直通别名在这条 DMA 上
  读不到）。

## 温度传感器

[`tsens`](esp32p4-hal/src/tsens.rs) 驱动低功耗域里的温度传感器（`LP_TSENS`）：

- `init` 打开 `LP_PERI` 的时钟、脉冲复位，给传感器上电，等稳定后就能连续读数；
- 传感器有 5 个测量档位（50~125 / 20~100 / -10~80 / -30~50 / -40~20 °C），档位越窄
  精度越高。档位通过模拟块 `I2C_SAR_ADC` 的 `TSENS_DAC` 字段选择（走 `regi2c`）；
- `update()` 读一次，若结果不在当前档位内就换档、等稳定再读，返回毫摄氏度；
  想固定档位就用 `set_range()` + `millidegrees()`；
- 硬件自动监测：`set_sample_rate()` / `set_thresholds()` / `set_monitoring_delta()` /
  `enable_monitoring()` + `enable_sampling()`，越界后 `interrupt_pending()` 置位，
  `monitoring_reason()` 告诉你越的是上限还是下限。

换算用参考手册的定点公式 `T = 0.4386 × value − 27.88 × offset − 20.52`，`offset` 由档位
决定，不需要浮点。

```rust
let mut sensor = TemperatureSensor::init(dp.LP_TSENS, &dp.LP_PERI);

loop {
    defmt::info!("{} m°C", sensor.update());
    delay.delay_ms(1_000);
}
```

示例 [`tsens`](esp32p4-examples/src/tsens.rs) 每秒打印一次温度。上板实测室温约 23.8 °C，
并且五个档位各自换算出的温度互相吻合（24.0 / 24.0 / 23.8 / 24.4 / 24.6 °C），说明档位
的 DAC 值与 offset 表是对的；把窗口设成永远越界时监测中断能正确置位并报告是「超过上限」。

## LED PWM（LEDC）

[`ledc`](esp32p4-hal/src/ledc.rs) 是 LEDC：4 个定时器（`XTAL_40M` / `FOSC_20M` / `REF_80M`
时钟、8 位小数分频、1–20 位分辨率）驱动 8 个通道，通道可选任意定时器，支持相位偏移、
空闲电平和 16 段占空比渐变。

```rust
let ledc = Ledc::init(dp.LEDC, &dp.HP_SYS_CLKRST);
ledc.configure_timer(TimerIndex::Zero, TimerConfig::new(5_000, 13)).unwrap();

let mut pin = pins.pin::<2>();
pin.route_output_signal(OutputSignal::LEDC_LS_SIG_OUT_PAD_OUT0);

ledc.configure_channel(ChannelIndex::Zero, ChannelConfig::new(TimerIndex::Zero));
ledc.set_duty_fraction(ChannelIndex::Zero, 1, 4); // 25 %
ledc.start(TimerIndex::Zero);
```

两个只有上板才暴露的位：

- **`TIMER_RST` 复位后是「复位中」**：只清 `TIMER_PAUSE` 不会让计数器跑，`start` 必须同时
  清 `TIMER_RST`；
- **`DUTY_START_CHn` 是占空比生效的闸门**：只写 `DUTY_CHn` 不会到达比较器，静态占空比
  也必须置位它（配渐变时它同时是渐变开关）。

示例 [`ledc`](esp32p4-examples/src/ledc.rs) 在 GPIO2 上把 PWM 打回自己的输入缓冲：1 kHz、
12 位，实测 25/50/75/100 % 的高电平占比是 24.93/49.97/74.47/99.98 %；100 步、每步一个周期
的渐变在约 100 ms 内把占空比从 0 拉到 4000/4095，关掉输出后引脚停在 idle 电平。

## RMT（红外/WS2812 波形）

[`rmt`](esp32p4-hal/src/rmt.rs) 管 4 个发送通道（0–3）与 4 个接收通道（4–7）。384 字的
脉冲 RAM 存 16 位编码（bit15 电平、bit14:0 时钟数），duration 为 0 表示结束。

```rust
let rmt = Rmt::init(dp.RMT, &dp.HP_SYS_CLKRST);
rmt.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, 2)?; // 20 MHz tick

rmt.configure_tx(TxChannelIndex::Zero, &TxConfig::new(1))?;
rmt.configure_rx(RxChannelIndex::Zero, &RxConfig::new(1, 1_000))?;
rmt.send(TxChannelIndex::Zero, &pulses, 10_000)?;
```

驱动按 ESP-IDF 的做法直接读写脉冲 RAM（NONFIFO，`SYS_CONF.APB_FIFO_MASK=1`），不走 APB
FIFO。上板踩到的坑（都已写进注释）：

- **SVD 里 RMT 在 `0x500D_4000`，TRM 说那是保留地址**，实际在 `0x500A_2000`；同时脉冲
  RAM 完全没有寄存器，补了 `TX_CHn_MEM`/`RX_CHn_MEM` 之后才能访问；
- **`SYS_CONF` 前面还有一级时钟 mux**：它复位在 80 MHz 节点上，只改 PCR 的
  `RMT_CLK_SRC_SEL` 会让 tick 频率和配置完全对不上，两处都要写；
- **接收载波解调复位时是开的**：默认阈值会把所有输入脉冲当载波滤掉，接收器只报 idle 超时，
  `configure_rx` 必须显式关掉；
- **接收期间 RAM 归接收器**（`MEM_OWNER`），读完要交回总线，否则读写互相干扰；
- **一段波形的最后一个脉冲通常读不到**：idle 间隔结束接收时那个电平还在计数。

示例 [`rmt`](esp32p4-examples/src/rmt.rs) 让 TX 通道 0 与 RX 通道 0 共用 GPIO2，发一段
48 个编码的 WS2812 数据，回读的 47 个完整脉冲电平与时长全部一致。

## 脉冲计数（PCNT）

[`pcnt`](esp32p4-hal/src/pcnt.rs) 是 4 个独立单元，每个单元两个通道加一个 16 位有符号
计数器：脉冲边沿动作、控制电平动作、单元级毛刺滤波、上下限回绕、两个阈值与零点观察点。

```rust
let pcnt = Pcnt::init(dp.PCNT, &dp.HP_SYS_CLKRST);
pcnt.configure_channel(UnitIndex::Zero, pcnt::ChannelIndex::Zero, ChannelConfig::rising_edge());
pcnt.set_limits(UnitIndex::Zero, 0, 800);
pcnt.enable(UnitIndex::Zero, Trigger::HIGH_LIMIT);
```

- **脉冲模式与控制模式的三档编码不同**：脉冲是 `0=保持/1=加/2=减`，控制是
  `0=加/1=减/2=保持`，所以分成 `EdgeAction` 与 `ControlMode` 两个枚举；
- 上下限是**回绕**而不是饱和，限值窗太窄会把计数变成锯齿；
- `PULSE_CNT_RST` 是普通读写位，清零要「置位再清除」，否则计数器会一直被按在 0。

示例 [`pcnt`](esp32p4-examples/src/pcnt.rs) 用 LEDC 的 10 kHz 方波在 GPIO2 上计数：100 ms
数到 1000 个边沿，800 步回绕后计数停在 201；暂停时保持在 0，恢复后 10 ms 内再数 100 个。

## 电机控制 PWM（MCPWM）

[`mcpwm`](esp32p4-hal/src/mcpwm.rs) 覆盖两个控制器（`Mcpwm0`/`Mcpwm1`）的定时器、操作器
与死区：时钟源与群分频、每定时器预分频与计数模式、比较器 A/B、按事件的输出动作
（保持/低/高/翻转）、死区、强制输出电平，以及 `GeneratorConfig::edge_aligned` /
`::complementary` 两个常用组合。群时钟由 `Timer::configure` 从 `PERI_CLK_CTRL20` 回读，
不需要再手工告诉它时钟频率。

配置走**按实例类型化的句柄**：`mcpwm.timer::<N>()` / `operator::<N>()` / `capture::<N>()` /
`fault::<N>()` / `capture_timer()` 返回该实例的句柄（`mc0::Timer<'_, N>` 等，MCPWM1 对应
`mc1`），`N` 在编译期检查范围。每个句柄的 inherent impl 由宏按实例号生成一次，寄存器名在
展开时替换，所以**寄存器访问只有一份代码，运行期零实例分发**——模块里没有 trait、没有
`dyn`，剩下的 `match` 全部是在解码一个**值**（计数模式、跳闸形状、时钟源），没有一处按实例
选寄存器。和 RP2350 的 `pwm` 模块是同一个形状。

事件与任务也是句柄给的，调用方不需要（也无法）写「第几号」：

```rust
let timer = mcpwm.timer::<0>();
let capture = mcpwm.capture::<1>();
mcpwm.enable_event(timer.zero_event(), true);        // EVT_EN bit 3
mcpwm.enable_task(capture.capture_task(), true);     // TASK_EN bit 20
channel.connect(timer.zero_event(), capture.capture_task());
```

`mc0::Event` / `mc0::Task`（MCPWM1 是 `mc1::Event` / `mc1::Task`）是 `#[repr(u8)]` 枚举，
判别值**就是**位表偏移（`EVT_EN`、`TASK_EN`、`INT_*` 与 `SOC_ETM` id 每控制器一张连续位表），
所以信号身份、位号和名字只有一处定义，搬动变体会在 `const` 断言和 `match` 处立刻报错。两类
信号**按控制器分成两个类型**：两个控制器位表布局完全相同，共用类型的话把 MCPWM0 的事件喂给
`Mcpwm1::enable_event` 会静默写到 MCPWM1 的同名位，现在那样写是编译错误。

调试与批量操作：`raw()` 取回偏移，`name()` 给出信号名（`timer0-zero`、`capture1-capture` …），
`Event::ALL` / `Task::ALL` 是整个位表，`into()` 直接转成 `etm::EventId` / `etm::TaskId`
（`Channel::connect` 收 `impl Into<…>`，不用再经过控制器方法）。

```rust
let mcpwm = Mcpwm0::init(dp.MCPWM0, &dp.HP_SYS_CLKRST);
mcpwm.set_clock_source(&dp.HP_SYS_CLKRST, ClockSource::Xtal40m, 1)?;

let timer = mcpwm.timer::<0>();
timer.configure(&dp.HP_SYS_CLKRST, TimerConfig::new(20_000))?;

mcpwm
    .operator::<0>()
    .configure(&timer, GeneratorConfig::complementary(1000, DeadTime::active_high(10, 30)))?;
timer.start();
```

句柄之间不混用：`Operator::configure(&timer, …)` 的定时器号来自句柄的**类型**（`T` 直接
作为常量写进 `OPERATORn_TIMERSEL`），所以操作器绑哪个定时器只有一个来源，也不存在写错索引
的可能。同一实例号的句柄不是同一个类型（`Timer<'_, 0>` ≠ `Timer<'_, 1>`），因此
`for timer in [t0, t1]` 这种写法不成立——需要各自写，或者把循环体抽成函数用宏展开。

比较器与死区延迟都以定时器 tick 计，必须在周期内：超过周期的比较器永远不会命中，输出就是
平的。这个约束没有运行期校验（校验就要按定时器号去查周期，正是要消灭的那类分派），
`configure` 拿到的句柄就是这些数值的对齐对象。

几个容易踩的点：

- **`TIMER_START` 是 3 位命令而不是运行标志**：`2` 才是「启动并持续运行」，`0`/`1` 是
  「到下一个零点/周期点停」；
- **零点事件是 `UTEZ`/`DTEZ`，不是 `UT0`/`DT0`**：后者对应 `event_t0`，只由故障或同步
  输入触发。把「零点置高」写进 `UT0`，发生器就再没有把输出拉高的动作——定时器照常计数、
  寄存器回读全对，焊盘却是一条平线。这个驱动最初的「输出不出来」就是这个错配；
- 发生器的 `UT*`/`DT*` 动作按**计数方向**区分（向上/向下），不是信号边沿；边沿对齐只需要
  写 `UT*`；
- 所有配置寄存器都用**立即更新**（`0`）写入。影子寄存器的按事件更新在这里既没用、也有
  风险：定时器还没启动时按 `TEZ` 更新，有效周期会停在复位值；
- 死区发生器复位时对 A/B 都是旁路的，配置 `DeadTime` 才会打开，延迟按**定时器 tick**
  （TRM 的 `PT_clk`）计；死区模块是用输出 A 生成 B 的，所以配了死区之后 `comparator_b`
  与 `actions_b` 不会到达焊盘；
- 延迟单元也是计数器：**定时器停着时延迟永远不会走完**，此时用 `force` 改一个要跨过
  延迟沿的电平，焊盘上看不到——先旁路死区（或让定时器跑起来）再强制；
- 预分频只在定时器（重新）启动时生效，改变频率要 `timer.stop()` → `timer.configure()` → `timer.start()`。

**当前状态**：下表是这套驱动在板上逐层验证的结果（40 MHz 晶振，20 kHz），句柄化重写
之后重新跑过一遍，数值与重写前一致。

| 检查 | 结果 |
|------|------|
| 群时钟回读 / 周期 | 40 MHz → period 1999 |
| 计数器 | 采样到 104…1938，落在 0…1999 内 |
| 强制电平 | 拉高时 16 次读回全为高，拉低时全为低 |
| 占空比 25/50/75 % | 比较值 499/999/1499，RMT 实测高电平 499/999/1499 tick |
| 互补 + 死区（RED 10 / FED 30） | 实测高电平 988 = 999 − 10，说明 RED 加在上升沿 |

示例 [`mcpwm`](esp32p4-examples/src/mcpwm.rs) 每秒重跑一遍这套检查，每层单独打印
`ok`/`FAIL`。

### 同步、捕获、故障、载波、ETM、事件比较器与中断

[`mcpwm`](esp32p4-hal/src/mcpwm.rs) 还覆盖了 MCPWM 的其余子模块：

- **同步**：`Timer::sync_input` / `enable_sync_input` / `sync_output` / `set_sync_phase` /
  `software_sync`。同步事件把计数器重载成相位寄存器里的值并复位预分频，多个定时器因此
  可以锁相；`SyncOutSource::Zero` / `Period` 让一个定时器成为另一个的时基源。
- **故障检测**：`mcpwm.fault::<N>()` 的 `enable(active_high)` / `disable` / `active` 配三个
  GPIO 故障输入的极性与使能，`mask()` 给出该输入的 `FaultMask` 供 `TripConfig::sources`
  使用；`Operator::configure_trip` 按模式（逐周期 / 一次性）设置每个输出的动作、触发源、
  软件触发和刷新点；`trip_status` / `release_trip` / `software_trip` 负责状态与恢复。
- **捕获**：`Capture::configure` / `enable` / `value` / `last_rising` / `capture_now`，加
  `mcpwm.capture_timer()` 句柄上的自由运行捕获定时器（`start` / `stop` / `set_phase` /
  `sync_input` / `software_sync`）。
- **载波**：`Operator::configure_carrier` / `disable_carrier`。
- **ETM**：`enable_event` / `enable_task` 打开 MCPWM 内部的事件与任务，句柄给出的
  `mc0::Event` / `mc0::Task` 用 `into()` 换成 `SOC_ETM` 的信号 id（`Channel::connect`
  直接收 `impl Into<…>`）；配合 [`etm`](esp32p4-hal/src/etm.rs) 模块（50 个通道，
  `connect` + `enable`）就能让任意事件触发任意任务。
- **事件比较器**：`Operator::set_event_comparator` / `enable_event_comparator` /
  `event_comparator_id` 操作每操作器多出来的两个比较点 `OPn_TSTMP_E1/E2`
  （`EventComparator::E1` / `E2`，与寄存器同名）。它们不碰输出，只产生 ETM 事件，用来在
  PWM 周期的固定位置启动一次转换、一次捕获或别的外设。
- **中断**：`mc0::Event` 的每一位同时也是中断位，`enable_interrupt` / `interrupt_pending` /
  `clear_interrupt` / `clear_interrupts` 直接对应 `INT_ENA` / `INT_ST` / `INT_CLR`
  （事件仍由句柄给出，如 `timer.zero_event()`）；
  `Mcpwm0::SOURCE` / `Mcpwm1::SOURCE` 是 `interrupt::claim` 要的中断源。

```rust
// 每个定时器零点触发一次 CAP1 捕获，全程不经 CPU
let timer = mcpwm.timer::<0>();
let capture = mcpwm.capture::<1>();
mcpwm.enable_event(timer.zero_event(), true);
mcpwm.enable_task(capture.capture_task(), true);
let mut channel = etm.free_channel().unwrap();
channel.connect(timer.zero_event(), capture.capture_task());
channel.enable(true);
```

几个在板上踩过的坑：

- **载波周期是 `8 × (prescale + 1)` 个 PWM_clk**，多出来的 8 来自三位的占空比字段；按
  `prescale + 1` 算会差 8 倍；
- **逐周期跳闸靠 TEZ/TEP 刷新**，定时器真的在跑才会自动恢复；一次性跳闸只能 `release_trip`
  手动清；
- 跳闸与软件同步的触发位是**边沿触发**，驱动里统一「先写 0 再写 1」，否则隔一次就丢；
- 同一路故障同时配了 OST 和 CBC 时，OST 会先锁住，看起来像「CBC 不恢复」——测试时要把另
  一种模式的触发源清空；
- **ETM 要三个时钟一起开**：`SOC_CLK_CTRL3.reg_etm_apb_clk_en`（不开则寄存器读回全 0）、
  `SOC_CLK_CTRL1.reg_etm_sys_clk_en` 与 `SOC_ETM.CLK_EN`；MCPWM 侧还要先置 `EVT_EN` /
  `TASK_EN` 的对应位，事件/任务才会真正进出矩阵。

上板结果（GPIO2 回环到 CAP0/CAP1 与 RMT，GPIO3 驱动 FAULT0）：

| 检查 | 结果 |
|------|------|
| 同步 | 定时器 1 自由计数到 13553 tick，打开同步输入后回到相位附近（1401），采样跨度 1952 < 一个 PWM 周期 |
| 捕获 | 94 个上升沿，相邻捕获值之差恰为 2000 tick |
| 故障 | OST：故障拉高后输出 0/16 高、状态置位，`release_trip` 后恢复；CBC：999 → 全低 → 恢复 999 |
| 载波 | prescale 16 / duty 4/8 实测高 63、低 59（周期 122 ≈ 128 tick），关掉后回到 999 |
| ETM | 通道 0 接 TEZ(63) 与 CAP1 任务(133)，95 次捕获、相邻差 2000 tick |
| 事件比较器 | E1=500 / E2=1200，经 ETM 触发捕获，实测落在零点之后 500 / 1200 tick |
| 中断 | 10 ms 收到 200/200 个 TEZ 中断；屏蔽 CPU 线后 `INT_ST` 置位，`clear_interrupts` 后清零 |

示例 [`mcpwm_extras`](esp32p4-examples/src/mcpwm_extras.rs) 每秒重跑这七项检查（同步、捕获、
故障、载波、ETM、事件比较器、中断）。

## CAN（TWAI）

[`twai`](esp32p4-hal/src/twai.rs) 覆盖三个 TWAI 控制器（`Twai0`/`Twai1`/`Twai2`）。先把硬件
事实说清楚：**P4 的 TWAI 是经典 CAN 2.0B，没有 CAN-FD**（`soc_caps.h` 里没有
`SOC_TWAI_SUPPORTS_FD`，TRM 第 54 章也没有 FD 的帧格式），所以这里只有标准帧/扩展帧、
数据帧/远程帧、最多 8 字节载荷。路线图早先写的「CAN-FD ×3」是错的，已更正。

```rust
let twai = Twai0::init(dp.TWAI0, &dp.HP_SYS_CLKRST);
twai.configure(&Config {
    timing: Timing::for_bitrate(XTAL_HZ, 500_000, 875).unwrap(),
    mode: Mode::SelfTest,
    filter: Filter::Standard { id: 0x123, mask: 0x7FF },
});
twai.start();

let mut frame = Frame::data(Id::Extended(0x1AB_CDEF), &[1, 2, 3]).unwrap();
frame.self_reception = true;   // 发送的同时收进接收缓冲
frame.single_shot = true;      // 失败不重试
twai.transmit(&frame).unwrap();
twai.wait_transmitted();
let received = twai.receive().unwrap();
```

- **位时序**：`Timing::for_bitrate(clock, bitrate, 采样点千分比)` 只在
  `clock / (bitrate × quanta)` 整除时接受（`quanta = 1 + tseg1 + tseg2`），再在合法的
  tseg1/tseg2 里挑采样点最接近的组合。一个量子是 `prescaler` 个时钟周期，而寄存器里只放
  `prescaler / 2 − 1`，所以预分频只能是偶数。40 MHz 晶振下 125k/250k/500k 都拿到 87.5 %，
  1 Mbit/s 最接近的是 90 %。
- **过滤器**：硬件只有一个过滤器（`acceptance_filter_mode` 恒为单过滤器），32 位 code/mask
  与**帧缓冲共用 `0x40`–`0x70`**，只有在复位模式里才是 ACR/AMR。`Filter::{AcceptAll, Raw,
  Standard, Extended}` 把这些编码收在里面。
- **收发**：`transmit` 先把 13 字节写进发送缓冲，再用**一次**命令写入触发（`TX_REQUEST` 是
  自清位，单次发送要 `TX_REQUEST | ABORT_TX` 同时写）；`receive` 读完缓冲再 `RELEASE_BUFFER`
  让 FIFO 前进；`frames_pending()` 是 FIFO 深度。
- **状态与中断**：`status()`、`bus_state()`（Active/Passive/Off）、`error_counters()`；
  `Interrupts` 是一组位，配 `enable_interrupts` / `interrupts()`，`Twai0::SOURCE` 直接喂给
  `interrupt::claim`。**读中断寄存器会清掉除 RI 以外的全部标志**，所以 `interrupts()` 一次
  读完再解码，本身就是「读即应答」。

没有收发器也能自测，但要两件事同时成立：`Mode::SelfTest`（不因缺少应答报错）+
`self_reception`（帧同时进接收缓冲）。而且**发送电平必须回到 RX 输入**——自接收不是外设内部的
数字回环，位流仍然是从 RX 引脚进来的。省掉收发器与杜邦线的办法是把 `TWAI0_TX` 与
`TWAI0_RX` 路由到**同一个焊盘**：焊盘的输入缓冲读到的就是它自己被驱动的电平。示例就是这么
接的。

踩过的坑：

- 只把 RX 上拉、TX 悬空时，控制器看到的是浮空/带噪声的总线：`STATUS` 里 RS/TS 常亮、
  发送请求永远不完成、错误计数一直是 0。发不出去的后果是**卡住而不是报错**，很容易误判成
  「时钟没开」；
- 帧缓冲那段地址**读的是接收缓冲、写的是发送缓冲**，所以回读校验拿到的是上一次收到的帧，
  不是刚写进去的那一帧；
- 复位模式下同一段地址是 ACR/AMR（`data0`–`data7`），`data8`–`data12` 在复位模式里不存在，
  读回 0；
- 时钟：`soc_clk_ctrl2.reg_twaiN_apb_clk_en` + `peri_clk_ctrl115.reg_twaiN_clk_en`，源固定
  40 MHz 晶振（`clk_src_sel` 只有一位，0 = XTAL；RC_FAST 精度不够，不做），`clock_divider`
  只影响 CLKOUT、与位率无关。**复位位在时钟块里叫 `REG_RST_EN_CAN0/1/2`**（bit 26/27/28），
  PAC 里就是 `reg_rst_en_can0`——名字不同，位置和 IDF 的 `reg_rst_en_twai0` 一致；
- 中断使能寄存器的字段名带 `ext_` 前缀（`ext_receive_int_ena` 等），位置与中断寄存器相同。

**当前状态**（40 MHz 晶振，500 kbit/s，87.5 %，自测模式 + 同焊盘回环）：

| 检查 | 结果 |
|------|------|
| 标准帧往返 | ID `0x123`、3 字节载荷原样回来 |
| 扩展帧往返 | ID `0x1ABCDEF`、8 字节载荷原样回来 |
| 远程帧 | `remote` 置位、DLC 4、无载荷 |
| 过滤器 | `0x123` 放行、`0x124` 丢弃（`Filter::Standard { id: 0x123, mask: 0x7FF }`） |
| 中断 | 1 次发送中断 + 1 次接收中断（处理器把帧读走），事后无残留标志 |
| 位时序 | 125k/250k/500k/1M 精确命中，采样点 87.5/87.5/87.5/90 % |
| 非法输入 | 9 字节载荷、标准 ID `0x800`、扩展 ID `0x20000000`、1234567 bit/s 全部被拒 |

示例 [`twai`](esp32p4-examples/src/twai.rs) 每秒重跑一遍这套检查，每项单独打印 `ok`/`FAIL`。

## I2S（音频）

[`i2s`](esp32p4-hal/src/i2s.rs) 覆盖三个 I2S 控制器（`I2s0`/`I2s1`/`I2s2`）的**标准双槽模式**
（Philips 与 MSB 对齐）、主/从角色、8–32 bit 槽宽，数据通路全部走 GDMA-AHB。和 MCPWM 一样，
每个控制器的寄存器实现由宏展开一次，所以没有 trait、没有实例分派；差别只在于 I2S1/2 的时钟
分频字段散在不同的 `PERI_CLK_CTRL` 寄存器里，宏参数把寄存器名和字段名都带上。

```rust
let i2s = I2s0::init(dp.I2S0, &dp.HP_SYS_CLKRST, &dp.LP_AON_CLKRST);
let clock = i2s.tx_configure(&StandardConfig {
    sample_rate: 48_000,
    data_bits: 16,
    slot_bits: 16,
    slot: Slot::Both,
    standard: Standard::Philips,
    ..StandardConfig::default()
})?;                                  // 40 MHz / (3 + 131/512) = 12.288 MHz, BCK 1/8

let dma = AhbDma::init(dp.AHB_DMA, &dp.HP_SYS_CLKRST);
i2s.tx_write(&dma.tx_channel::<0>(), &samples, &descriptors)?;
```

- **时钟树不在外设里**：源选择、模块门控和分数分频器都在 `HP_SYS_CLKRST`
  （`f = f_src / (N + b/a)`，`2 ≤ N ≤ 256`，`a ≤ 512`），外设里的 `TX/RX_BCK_DIV_NUM` 再把
  它分到 BCK。`StandardConfig` 进，`Clock` 出：MCLK、BCK、分频比、以及实际能拿到的采样点都
  能打出来。分频只在能整除时才接受，40 MHz 晶振下 48 kHz × 256 得到 12.2855 MHz
  （误差 −0.02 %），BCK 1.536 MHz 精确。
- **标准模式就是双槽 TDM**：`TDM_EN` 必须置一、`PDM_EN` 必须清零，`MSB_SHIFT` 选 Philips
  （WS 比数据早一个 BCK）还是 MSB 对齐。`StandardConfig` 用 `Slot::{Both, Left, Right}` 决定哪
  一槽带数据；mono 只是少搬一半数据，帧宽（也就是 BCK）不变。`TdmConfig` 把它推广到最多 16
  槽：`channels` 是每帧槽数，`active` 是带数据的槽，没被选中的槽既不占 DMA 数据、也不改变帧宽
  （BCK 仍是 `采样率 × channels × slot_bits`）。
- **收发**：`tx_begin`/`tx_wait`（或合并的 `tx_write`）与 `rx_begin`/`rx_wait`（或
  `rx_read`）负责描述符链、cache 维护、`RX_EOF_NUM` 和起停顺序；`tx_stream_begin` /
  `rx_stream_begin` 把描述符挂成**环**，同一段缓冲反复播放（或反复覆盖），是连续播放/采集
  的形态，用 `tx_stream_stop` / `rx_stream_stop` 停下。`tx_bck_count()` 是硬件 BCK 计数器，
  用来量真实位时钟；`share_bck_ws(true)` 让同一控制器的两个半区共用 BCK/WS，全双工不需要把
  时钟引出芯片。
- **中断**：`Interrupts::{RX_DONE, TX_DONE, RX_HUNG, TX_HUNG}`，配
  `enable_interrupts` / `interrupts()` / `clear_interrupts()`，`I2s0::SOURCE` 给
  `interrupt::claim`。`set_hung_timeout` 调整从机等不到 BCK/WS 的判定时间。

踩过的坑：

- **没有 CPU 数据通路**：P4 的 I2S 没有 `TXFIFO`/`RXFIFO` 寄存器，FIFO 只由 GDMA 填/排，
  所以每个传输都必须先接上 DMA 通道（`Peripheral::I2s0..2`）；
- **起停顺序**：先复位半区与 FIFO，再启动 DMA，最后置 `TX_START`。反过来（先启动外设）会让
  发送半区在没有数据时饿死，`TX_HUNG` 锁存，之后 FIFO 只进不出——HAL 现在自己按正确顺序
  做这件事；
- **`TX/RX_UPDATE` 是自清位**，要把格式寄存器压进数据通路，写完要等它归零再 `START`；
- **从机的模块时钟**：从机靠自己的模块时钟采样，TRM 要求 `f(I2S_TX/RX_CLK) ≥ 8 × f(BCK)`。
  只把 BCK 分频寄存器留给主机、从机不写分频器时，它的模块时钟停在复位值（40 MHz/256 ≈
  156 kHz），现象是**收不到任何数据、也不报错**——这一条花了最多时间。现在从机也会写自己的
  分频器，`mclk_multiple` 不够 8 倍时直接返回 `Error::SlaveClockTooSlow`；
- **`RX_EOF_NUM` 必须等于描述符长度**，它是外设告诉 DMA「这一帧结束」的字节数，所以单次读
  不超过一个描述符（4095 字节）；
- `BCK_DIV_NUM` 不能为 1，也就是 BCK 分频至少 2；`mclk_multiple` 用 256 时 24 bit 槽要取 3
  的倍数（384）才能整除；
- **`TX_TDM_SKIP_MSK_EN` 会缩短帧**：置一之后硬件只发送被选中的槽，帧宽随之变小，而接收侧没有
  对应位——两端对 BCK 的理解就差一个槽数倍，现象是接收侧永远等不到一帧结束。这个驱动把这个位
  一直清零，槽的空隙靠「不占 DMA 数据」实现，帧宽保持不变。

**当前状态**（40 MHz 晶振，48 kHz、16 bit、Philips、256× MCLK，TX 主 / RX 从 + 内部
BCK/WS 共享，SD 单焊盘回环）：

| 检查 | 结果 |
|------|------|
| 时钟 | 40 MHz /(3 + 131/512) = 12.2855 MHz MCLK，/8 = 1.536 MHz BCK，误差 −0.02 % |
| 位时钟读数 | 1024 字节（立体声）= 8193 个 BCK（期望 8192），mono = 16385（期望 16384） |
| 立体声回环 | 1024 字节逐字节相同 |
| mono（仅左槽）回环 | 1024 字节逐字节相同，BCK 不变 |
| TDM 4 槽 | 4 × 8 bit，BCK 1.536 MHz，8193/8192 个 BCK，数据逐字节相同 |
| TDM 有空洞（`active = 0b1010`） | 每帧仍 4 槽（16393/16384 BCK），但 DMA 每帧只搬 2 字节 |
| 环形流 | TX 环（32 字节模式）与 RX 环（40 字节窗口）各跑 10 ms：15359/15360 个 BCK，窗口内容是该模式的某个相位，无中断残留 |
| 中断 | `TX_DONE` 触发、读取后清零 |
| 非法配置 | 数据位宽 > 槽宽、0 bit、5 MHz、100 Hz、从机 128× 全部被拒 |

示例 [`i2s`](esp32p4-examples/src/i2s.rs) 每秒重跑一遍这套检查，每项单独打印 `ok`/`FAIL`。

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
- SPI2/SPI3 主机与从机（四种时钟模式、64 字节缓冲）、SD 卡主机（识别、容量、DMA 块读写）、
  ADC1/2 单次转换与 HP 数字控制器扫描（含 IIR 滤波与阈值监控器）、温度传感器
- PWM 家族：LEDC（4 定时器 × 8 通道、渐变）、RMT（4 发 4 收脉冲波形）、PCNT（4 单元计数）、
  MCPWM（定时器/比较器/发生器/死区/强制电平/同步/故障/捕获/载波/事件比较器/中断/ETM，上板验证）
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
