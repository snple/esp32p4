# esp32p4-hal

ESP32-P4 的最小硬件抽象层。

## 模块

| 模块 | 内容 |
|------|------|
| `startup` | `#[main]` 自动调用的核级初始化：看门狗、core 1、分支预测器 |
| `gpio` | `Pins` 控制器、`Pin<N>` 句柄（输入/输出/上拉/驱动强度）；GPIO0–GPIO53 |
| `delay` | 基于 `SYSTIMER` unit 0 的 `delay_us` / `delay_ms` |
| `system` | `Cpu`/`ResetReason` 枚举、`reset_reason`、`software_reset`、`software_reset_cpu` |
| `regi2c` | 模拟 I2C 主控：用 `Register`/`Field` 描述 PLL/PHY/ADC 校准寄存器（含 `mpll` 等 block 常量、时钟管理、临界区） |
| `psram` | 封装内 PSRAM 初始化（MPLL 250 MHz、16/32 MB 自动检测、MMU 映射）与 `PsramAllocator` |

`lib.rs` 统一重导出：

```rust
pub use esp32p4_macro::main;
pub use esp32p4_pac as pac;
pub use esp32p4_pac::{Interrupt, Peripherals};
pub use esp32p4_rt as rt;
```

## 启动初始化

`#[main]` 会在用户函数体之前插入 `startup::init()`：

1. **关闭 RTC 看门狗**：ESP-IDF 2nd-stage bootloader 默认启用它（9 s）并留给应用，
   应用不关就会在几秒后被复位；
2. **关闭 TIMG0/TIMG1 MWDT**：防御性，避免上一个程序留下的 WDT 在 CPU 复位后继续跑；
3. **停放 core 1**：P4 的 ROM 只启动 core 0，core 1 的时钟/复位/停顿状态在 LP 域，
   软件复位后可能继续执行旧代码；
4. **打开分支预测器**：ROM 在 cache 完全配置前关掉它，只影响性能。

`startup::init()` 用 `steal()` 直接配置寄存器，不需要 `Peripherals`；调用两次也安全。

## 复位与复位原因

ESP32-P4 没有 Cortex-M 的 `SYSRESETREQ`，复位由 always-on 的 **LP 域**寄存器触发，
但**全部在 PAC 里，不需要 ROM 符号**：

| 函数 | 触发的寄存器（TRM 名） | 复位类型 / 复位码 |
|------|------------------------|-------------------|
| `system::software_reset()` | `LP_SYSTEM_SYS_CTRL_REG.SYS_SW_RST`（PAC：`LP_SYS.SYS_CTRL`） | HP core reset / `0x03` |
| `system::software_reset_cpu(cpu)` | `LP_CLKRST_HPCOREx_SW_RESET`（PAC：`LP_AON_CLKRST.HPCPU_RESET_CTRL0`） | CPU reset / `0x0c` |
| `system::reset_reason(cpu)` | 读 `LP_CLKRST_HPCOREx_RESET_CAUSE`（PAC：`LP_AON_CLKRST.RESET_CAUSE`） | — |

**为什么复位 HP 核却在 LP 域？** 核不能复位自己：复位请求必须由“目标被复位期间仍在运行”
的逻辑锁存，且 HP 时钟/总线要活到复位释放，所以 P4 的所有复位触发位都在 always-on 域。
`LP_SYSTEM`（PAC/SVD 里叫 `LP_SYS`，`0x5011_0000`）持有 HP **内核**复位位，
`LP_CLKRST`（PAC 里叫 `LP_AON_CLKRST`，`0x5011_1000`）持有**单核**复位位与复位原因，
而 `HP_SYS`（`0x500E_5000`）根本没有复位触发位。
另外 `LP_SYSTEM_SYS_CTRL` 必须读-改-写（同寄存器还有 `DIG_FIB`/`ANA_FIB`/`IO_MUX_RESET_DISABLE`
等默认非零字段），PAC 生成的 `trigger_sys_sw_rst()` 已经是读-改-写。

```rust
use esp32p4_hal::system::{reset_reason, software_reset, Cpu, ResetReason};

if reset_reason(Cpu::ProCpu) == Some(ResetReason::CpuSoftware) {
    // 上一次复位来自 software_reset_cpu()
}
software_reset(); // 不复返
```

`software_reset()` 复位整个 HP 域（两个核、外设、HP 内存），LP/RTC 域保留状态，
随后 ROM bootloader 重新从 flash 加载应用；`software_reset_cpu()` 只复位单个核，
外设状态保留，需要调用者自己清理。

## PSRAM

封装内的 16 MB / 32 MB PSRAM 上电后不可见，`psram::Psram::init` 会完成全部初始化
（PHY LDO → 时钟 → 复位 → 引脚 → 控制器 → Mode Register → MMU 映射）：

```rust
use esp32p4_hal::psram::{Psram, PsramAllocator, PsramConfig};

#[global_allocator]
static HEAP: PsramAllocator = PsramAllocator::empty();

let psram = Psram::init(PsramConfig::default()).unwrap(); // 250 MHz, 自动检测容量
HEAP.init(&psram);

let boxed = alloc::boxed::Box::new(42u32);
```

Rust 1.100 起 `core::alloc::Allocator` 已稳定，`PsramAllocator` 同时实现
`GlobalAlloc` 和 `Allocator`，可以只把部分集合放到 PSRAM：

```rust
let mut v: Vec<u32, &PsramAllocator> = Vec::new_in(&HEAP);
```

| 窗口 | 地址 | 用途 |
|------|------|------|
| cached | `0x4800_0000` 起 | 普通读写，经 L2 cache |
| uncached | `0x8800_0000` 起 | DMA / 自检，`Psram::uncached_base()` |

`PsramAllocator::init_raw` 可以在窗口内开多个互不重叠的子堆，例如把尾部一段留给
DMA（示例见 `esp32p4-examples/src/psram_split.rs`）：

```rust
unsafe {
    HEAP.init_raw(psram.base(), heap_len);                             // cached
    DMA_POOL.init_raw(psram.uncached_base().add(heap_len), dma_len);   // uncached
}
let mut buf: Vec<u8, &PsramAllocator> = Vec::new_in(&DMA_POOL);        // 无需 cache 维护
```

- 时钟：MPLL 500 MHz → 总线 250 MHz（`psram_clk_src_sel = MPLL`）。MPLL 的倍频写在
  模拟寄存器里，由 [`regi2c`](src/regi2c.rs)（`LP_I2C_ANA_MST`）写入，不依赖 ROM。
- Mode Register（MR0/MR2/MR4/MR8）读写走 `SPI1` 直接命令，需要 ROM 的
  `esp_rom_spi_set_op_mode` / `esp_rom_spi_cmd_config`（在 `link.x` 里 `PROVIDE` 为固定地址）。
- `PsramConfig::rev3_workaround`：ESP32-P4 v3.0（ECO5）需要，默认关闭。
- `PsramConfig::verify`（默认开）会用 uncached 别名写读校验，失败返回 `Error::VerifyFailed`。

## GPIO

```rust
let pins = Pins::init(dp.GPIO, dp.IO_MUX, dp.HP_SYS_CLKRST);

let mut led = pins.pin::<2>();
led.into_output();      // 推挽输出，初始低
led.set_high();
led.toggle();

let mut button = pins.pin::<3>();
button.into_input_pullup();
if button.get() { /* 未按下 */ }
```

和 rp2350 一样，`Pin<N>` 自身就是句柄：配置方法取 `&mut self`，读写方法取 `&self`，
`into_output()` / `into_input()` 返回 `&Self` 便于链式调用。ISR 里可用
`unsafe { Pin::<N>::steal() }`（返回 `Steal<Pin<N>>`，只能调 `&self` 方法）。

`init_pad()`（`into_*` 会先调用）做这些事：

1. `IO_MUX_GPIOx.MCU_SEL = 1`（GPIO 功能）；
2. 关闭输入缓冲与上下拉；
3. `FUN_DRV = 2`（约 20 mA）；
4. `GPIO_FUNCn_OUT_SEL_CFG.OUT_SEL = 256`（选择 GPIO 自己的输出寄存器）；
5. `OEN_SEL = 1`（输出使能由 `GPIO_ENABLE` 控制）。

`into_output()` 之后再通过 `GPIO_ENABLE_W1TS` 打开输出。输出使用 `OUT_W1TS/W1TC`，
GPIO32–53 使用 `OUT1_W1TS/W1TC`；GPIO 没有 XOR 寄存器，`toggle()` 是读输出寄存器后置位/清零。

可用方法：`set_function` / `function` / `set_function_gpio`、`set_pull` /
`set_pull_up` / `set_pull_down` / `set_pull_none`、`set_drive_strength`、
`enable_input` / `disable_input`、`set_direction_out` / `set_direction_in` /
`is_output`、`get` / `set_high` / `set_low` / `set` / `toggle` / `output_level`、
`into_output` / `into_input` / `into_input_pullup` / `into_input_pulldown`。

## 延时

`SYSTIMER` 的 `CNT_CLK` 平均 16 MHz（`XTAL_CLK / 2.5`），因此 1 µs = 16 tick。
`Delay` 通过 `UNIT0` 的 update/valid 握手读取 52 位计数器：

```rust
let delay = Delay::init(dp.SYSTIMER);
delay.delay_ms(500);
```

## Features

- `rt`（默认）：链接 `esp32p4-rt`。
- `panic-handler`（默认）：提供停核式 `#[panic_handler]`；应用要自定义 panic
  处理时关闭它。
- `critical-section-impl`（默认）：基于 `mstatus.MIE` 的 `critical-section` 实现，
  `defmt-rtt` 等需要它的库可以直接用。
- `defmt`：为公开类型实现 `defmt::Format`，并在启动、复位、PSRAM 初始化、regi2c
  关键路径输出 `debug`/`info`/`warn`/`error` 日志。日志传输由最终二进制提供
  （`defmt-rtt`，或下面的 `defmt-usb`）；不开启时所有日志语句展开为空。
- `defmt-usb`：在 `defmt` 之上把 USB Serial/JTAG 注册成全局 logger，日志直接从
  `espflash monitor` 那根线出来，不需要调试探针。和任何其它全局 logger
  （`defmt-rtt`）互斥。

## 日志（defmt）

日志语句本身不带输出通道，要由最终二进制接上一个 **logger**。两个现成的选择互斥：

| 读日志的工具 | 在二进制里装哪个 logger | 备注 |
|---|---|---|
| `espflash monitor`（只用 USB 线） | `esp32p4-hal` 的 `defmt-usb` feature | 帧走 USB Serial/JTAG；ELF 里的 `espflash.LOG_FORMAT` 标记让 espflash 自动按 defmt 解码 |
| `probe-rs run`（JTAG 调试器） | `defmt-rtt` crate 的 logger | 帧写进 RAM 的 RTT 缓冲，`espflash` 看不到 |

```rust
// A. probe-rs：自己装 RTT logger（只能在一个二进制里出现一次）
use defmt_rtt as _;

defmt::timestamp!("{=u64:us}", esp32p4_hal::delay::micros());

#[esp32p4_hal::main]
fn main() -> ! {
    defmt::info!("hello from ESP32-P4");   // HAL 自己的日志走同一通道
    loop {}
}
```

```toml
# B. espflash：HAL 自己注册 USB logger，二进制里不要再装别的
[dependencies]
esp32p4-hal = { path = "../esp32p4-hal", features = ["defmt", "defmt-usb"] }
```

```bash
# A. RTT
probe-rs run --chip esp32p4 target/riscv32imafc-unknown-none-elf/debug/defmt_log
# B. USB Serial/JTAG（espflash monitor 直接解码）
cargo run --features defmt-usb --bin my_app
```

两个 logger 只能二选一，同时启用会重复定义 `_defmt_acquire` 等符号。另外 `DEFMT_LOG`
要设（例如 `debug`）：defmt 1.x 没有它只编译 `error!`。

在自己的项目里开启 `defmt` 时，还要把 defmt 的链接脚本挂上，否则格式字符串会散落在
多个独立段里，主机端无法解码：

```rust
// build.rs
println!("cargo:rustc-link-arg=-Tdefmt.x");
```

`defmt.x` 由 `defmt` crate 的 build script 生成，并自动加入链接搜索路径。

## 当前限制

- 只支持简单 GPIO 输出/输入；GPIO 中断、滤波、同步阶段尚未封装。
- 没有时钟树 API；依赖 bootloader 留下的默认时钟。
- 没有 UART/SPI/I2C/DMA/RMT 等驱动。
- `system` 的复位路径未经硬件验证（无板可测）；复位码表按 TRM 12-RESCLK 与
  ESP-IDF `soc/reset_reasons.h` 对齐。
- PSRAM 初始化同样未上板验证：寄存器序列对齐 ESP-IDF/esp-hal，容量检测与自检都
  在 `Psram::init` 内完成；v3.0 (ECO5) 芯片需要打开 `PsramConfig::rev3_workaround`。
