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
| [`esp32p4`](esp32p4/) | **HAL**：GPIO / IO MUX、SYSTIMER 延时/单调时钟/闹钟、通用定时器、复位、封装内 PSRAM、USB Serial/JTAG、以太网 MAC、同步 TCP/IP |
| [`esp32p4-usb`](esp32p4-usb/) | **USB 设备栈**：DWC OTG 全速/高速控制器（slave 模式、轮询）、标准请求与描述符、CDC-ACM 虚拟串口、自定义类 |
| [`esp32p4-examples`](esp32p4-examples/) | 示例：GPIO2 blink、PSRAM 初始化与分区、USB Serial/JTAG、USB CDC |
| [`xtask`](xtask/) | 构建辅助：`cargo xtask gen-pac` |

依赖方向是单向的：`macro` / `rt` → `pac` → `esp32p4` / `esp32p4-usb` → `examples`。

## 示例

| 二进制 | 内容 |
|--------|------|
| `blink` | GPIO2 每 500 ms 翻转一次 |
| `psram` | 初始化封装内 PSRAM，整体作为全局堆使用 |
| `psram_split` | 把 PSRAM 分成「cached 全局堆 + uncached DMA 池」两部分 |
| `defmt_log` | 用 `defmt-rtt` 输出日志（需 `--features defmt`，JTAG 读取） |
| `usb_serial` | 通过 USB Serial/JTAG 打印并回显，同时翻转 GPIO2（两个驱动共享 `HP_SYS_CLKRST`） |
| `usb_cdc` | 全速 OTG（GPIO26/27）上的 CDC-ACM 虚拟串口，回显主机发来的字节 |
| `usb_cdc_hs` | 高速 OTG（内部 UTMI+ PHY）上的 CDC-ACM，512 字节 bulk 包 |
| `timer` | `TIMG0` 的两路通用定时器：100 ms 周期中断闪灯，外加 1 s 的倒计时闹钟 |
| `alarm` | `SYSTIMER` 比较器：250 ms 周期闹钟 + 5 s 一次性 deadline，主循环用 `sleep_until` 睡到下一个检查点 |
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
- **中断驱动**：`Emac::enable_interrupts(lines::EMAC)` 把 MAC 的 DMA 完成中断接到 CPU，
  `Net::poll_wait()` 用 `wfi` 把核心停住，等 MAC 中断或 smoltcp 的下一个定时器
  （`SYSTIMER` target 0）唤醒。详见下节。

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
- 处理器是普通 `fn()`，注册到按 CPU 中断线索引的表里；处理期间 `MIE = 0`，不支持嵌套。

HAL 固定占用两条线（见 [`interrupt::lines`](esp32p4/src/interrupt.rs)）：

| 线 | 用途 |
|----|------|
| 19 | `SYSTIMER` target 0：`Delay::sleep_us` / `sleep_ms` 的定时唤醒 |
| 20 | EMAC DMA 完成（`ETH_MAC_INTR`） |

驱动侧接口：

```rust
// 打开 MAC 中断，处理器只做两件事：清 DMASTATUS、置一个标志
emac.enable_interrupts(esp32p4::interrupt::lines::EMAC);
// 全局开关（启动最后一步）
unsafe { esp32p4::interrupt::enable() };

loop {
    net.poll_wait(&mut emac, 20);   // 有活干就处理，没活就 wfi 睡 20 ms 或睡到定时器
}
```

ISR 里不碰描述符环：环的所有权仍然只在主循环里，中断只负责把核心叫醒。这样既没有
数据竞争，也不需要临界区。

### HAL 中断接口

[`esp32p4::interrupt`](esp32p4/src/interrupt.rs) 把矩阵、CLIC 和处理器表包成一套带所有权的 API：

```rust
use esp32p4::interrupt::{self, CpuInterrupt, Interrupt, Level};

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

## 定时器

按用途分成三层：

| 模块 | 硬件 | 计数单位 | 用途 |
|------|------|----------|------|
| [`time`](esp32p4/src/time.rs) | `SYSTIMER` unit 0 | 16 MHz tick（62.5 ns） | 单调时钟：`Instant` / `Duration`；52 位，约 8.9 年回绕 |
| [`systimer`](esp32p4/src/systimer.rs) | `SYSTIMER` COMP1 / COMP2 | 同上 | 绝对 deadline（一次性）与固定周期闹钟，带中断 |
| [`timer`](esp32p4/src/timer/mod.rs) | `TIMG0` / `TIMG1`，各 2 路 | 可配预分频，默认 1 µs | 可重载的通用计数器：一次性 / 周期性闹钟，带中断 |

`time` 只读计数器，永远自由运行；`Delay` 在它上面借 comparator 0 睡到指定 tick（`sleep_ms` 睡一段时间，`sleep_until(Instant)` 睡到某个时刻）。

```rust
let start = Instant::now();
do_work();
let elapsed: Duration = start.elapsed();   // 与 delay::micros() 同一个时间源
```

`timer` 的 4 路计数器是完整的通用定时器：

- 时钟源可选 `XTAL_CLK` 40 MHz、`RC_FAST_CLK`（约 17.5 MHz）、`PLL_F80M_CLK` 80 MHz，
  16 位预分频（2..=65536）；默认 40 → 1 tick = 1 µs。
- 54 位计数器，可增可减；重载值与闹钟比较值也是 54 位。
- 闹钟触发后硬件会清掉 `ALARM_EN`。开启 auto-reload 时计数器从重载值重新开始，HAL 的
  中断壳清状态后自动重新武装，因此周期性闹钟不需要用户干预；一次性闹钟保持关闭。
- 每路有独立的矩阵源（`TG0_T0` / `TG0_T1` / `TG1_T0` / `TG1_T1`），CPU 线由调用者传入，
  `enable_interrupt(line, handler)` 内部走 `interrupt::claim`，线被占用会 panic。

```rust
let (fast, slow) = TimerGroup0::new(dp.TIMG0, &dp.HP_SYS_CLKRST).split();

fast.enable_interrupt(CpuInterrupt::new(21), tick);      // CPU 线 21
fast.start_periodic(Duration::from_millis(100));

slow.set_direction(Direction::Down);
slow.enable_interrupt(CpuInterrupt::new(22), slow_tick); // CPU 线 22
slow.start_periodic(Duration::from_secs(1));

unsafe { interrupt::enable() };
```

### `SYSTIMER` 比较器闹钟

`SYSTIMER` 的 3 个比较器里，COMP0 归 `Delay`，另外两个由 [`systimer`](esp32p4/src/systimer.rs)
以 `Alarm` 的形式交出（`Alarm::new(Comparator::One)`，重复创建会 panic）：

- **target 模式**：一次中断，比较值是完整的 52 位计数器值，也就是一个 `Instant`
  （`alarm.set_target(deadline)` / `start_in(Duration)`）。
- **period 模式**：每 δt 一次中断，δt 存在 26 位寄存器里，因此上限约 67 秒
  （`alarm.set_period(Duration)`）。周期模式由硬件自己重新武装，handler 只要应答。
- 中断是电平型，HAL 的中断壳负责清 `int_clr`；`enable_interrupt(line, handler)` 同样走
  `interrupt::claim`，CPU 线由调用者选。
- `AlarmHandle` 与 `TimerHandle` 同构：`Copy`、方法取 `&self`、`Alarm` 通过 `Deref` 复用，
  `AlarmHandle::steal(comparator)` 可在中断里重建句柄。

```rust
let alarm = Alarm::new(Comparator::One);
alarm.enable_interrupt(CpuInterrupt::new(21), tick);
alarm.set_period(Duration::from_millis(10));   // 10 ms 一次

let one_shot = Alarm::new(Comparator::Two);
one_shot.start_in(Duration::from_secs(5));     // 5 s 后一次
```

### 两个层次：`Timer` 与 `TimerHandle`

`Timer` 是拥有者：它借用 `HP_SYS_CLKRST`、记住时钟源与预分频、在开中断时持有 CPU 线。
所有寄存器级操作放在 `TimerHandle` 上——`Copy` 值、方法全取 `&self`，`Timer` 通过 `Deref`
把它暴露出来，所以两者是同一套 API（`timer.counter()` 编译成对 handle 的调用）。

中断里没有 `&Timer`，用 `TimerHandle::steal` 重建一个（和 `Pin::<N>::steal()` 同一个模式）：

```rust
fn overrun() {
    // SAFETY: main 里的 Timer 一直活着，且这里只读计数器、重设闹钟。
    let timer = unsafe { TimerHandle::steal(Interrupt::TG0_T0) };
    OVERRUNS.fetch_add(1, Ordering::Relaxed);
    timer.set_alarm(timer.counter() + PERIOD_TICKS);
}
```

`Timer::handle()` 是安全版本：拿到副本后可以放进 `static`（`TimerHandle::steal` 是
`const unsafe fn`，`Steal<TimerHandle>` 因此能直接 const 初始化），主循环和中断各用各的副本。

厂商 SVD 只描述每个 TIMG 的 timer 0，timer 1 落在保留区里，所以这组寄存器没有走 PAC：
[`timer/regs.rs`](esp32p4/src/timer/regs.rs) 按 TRM 第 18 章手写了偏移与位域
（两路相隔 0x24，与 ESP-IDF 的 `timg_hwtimer_reg_t hw_timer[2]` 一致）。

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
- SYSTIMER 阻塞延时（`sleep_ms` / `sleep_until`）与单调时钟（`time::Instant` / `Duration`）
- SYSTIMER 比较器闹钟（COMP1 / COMP2：绝对 deadline 与周期模式，带中断与 `AlarmHandle`）
- 通用定时器（`TIMG0` / `TIMG1` 的 4 路 54 位计数器：时钟源与预分频、增/减计数、
  软件重载、一次性与周期性闹钟中断）
- 复位控制与复位原因（`system::software_reset` / `software_reset_cpu` / `reset_reason`，纯寄存器实现）
- 封装内 PSRAM 初始化（MPLL 250 MHz、16/32 MB 自动检测、MMU 映射、`PsramAllocator`）
- USB Serial/JTAG（CDC-ACM）收发：`UsbSerialJtag`，以及可选的 `defmt` 全局 logger
- 以太网 MAC（RMII + MDIO + DMA 描述符环），含 L1/L2 cache 维护
- 同步 TCP/IP（smoltcp）：DHCP 客户端、TCP/UDP/ICMP、可选 IPv6，无 async、堆可选
- CLIC 中断框架（direct 模式、按 CPU 线分发、trap 保存 FPU 上下文）与 EMAC/SYSTIMER
  中断驱动
- `defmt` 日志支持（`Format` 实现 + 关键路径日志 + 基于 `mstatus.MIE` 的
  `critical-section` 实现）
- `#[main]`（注入 `startup::init()`：关 bootloader 遗留的 RTC/MWDT 看门狗、停放 core 1、
  启用分支预测器）、默认 panic handler
- stable `riscv32imafc-unknown-none-elf`（RV32IMAFC、ilp32f 硬浮点 ABI），启动时使能 FPU

尚未实现：

- 中断嵌套与优先级分级、其它外设中断
- 完整时钟树（PLL/分频）配置
- UART、SPI、I2C、DMA、RMT、WS2812 等驱动
- EMAC 中断驱动、TX/RX 校验和卸载、PTP 时间戳、MII/RGMII 接口
- 阻塞式 socket API（当前是 smoltcp 的事件/轮询风格）、DHCP 服务端
- USB OTG FS/HS 主机与设备栈
- LP 核与低功耗
- trap 入口尚未保存 FPU 寄存器上下文（当前不使能可恢复中断）

## 许可证

MIT OR Apache-2.0
