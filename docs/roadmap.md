# ESP32-P4 HAL 驱动路线图

本文件记录 HAL 还缺哪些模块、为什么缺、准备按什么顺序补。范围限定为**非图形图像**
功能：`LCD_CAM`、MIPI DSI/CSI（bridge 与 host）、`JPEG`、`ISP`、`H264`、`PPA`、
`DMA2D` 及这些模块的 DMA 不在此列。

判定标准只有一条：**没有它，就写不出一份像样的固件**。每个新模块的写法遵循
[`style-guide.md`](style-guide.md)（第 2、4、9、11、13 节），骨架、所有权模型和验证
流程都与既有模块保持一致。

## 1. 现状

已实现且可用的部分：

| 模块 | 文件 | 说明 |
|------|------|------|
| `time` / `delay` | [time.rs](../esp32p4-hal/src/time.rs) / [delay.rs](../esp32p4-hal/src/delay.rs) | SYSTIMER 单调时钟、阻塞延时与比较器闹钟 |
| `clock` | [clock.rs](../esp32p4-hal/src/clock.rs) | ROOT_CLK 选源、CPU/MEM/SYS/APB 分频、PLL_FxxM、LP 时钟、CPU 变频 |
| `reset` | [reset.rs](../esp32p4-hal/src/reset.rs) | `HP_RST_EN0/1/2` 上的外设复位 |
| `timer` | [timer/mod.rs](../esp32p4-hal/src/timer/mod.rs) | TIMG0/1 四路计数器与两个 MWDT，中断归 group |
| `systimer` | [systimer.rs](../esp32p4-hal/src/systimer.rs) | COMP1/COMP2 闹钟，COMP0 归 `Delay` |
| `gpio` | [gpio/](../esp32p4-hal/src/gpio) | 引脚电气配置、GPIO 读写、四路中断、完整的 HP GPIO 交换矩阵信号路由，以及每个焊盘的直接 IO MUX 功能表（`pins`） |
| `uart` | [uart.rs](../esp32p4-hal/src/uart.rs) | UART0–4：轮询收发、帧格式、流控、按源中断、GPIO 矩阵引脚路由 |
| `i2c` | [i2c.rs](../esp32p4-hal/src/i2c.rs) | I2C0/1 主机：任意 SCL、7 位地址、写/读/写读、总线恢复、按源中断 |
| `spin` | [spin.rs](../esp32p4-hal/src/spin.rs) | 有界自旋助手（`spin_wait` / `try_spin_wait`），驱动轮询的统一上界 |
| `interrupt` | [interrupt.rs](../esp32p4-hal/src/interrupt.rs) | CLIC 线与矩阵、`claim`/`InterruptHandler` |
| `emac` / `net` | [emac/](../esp32p4-hal/src/emac) / [net.rs](../esp32p4-hal/src/net.rs) | RMII + MDIO + DMA，smoltcp 同步栈 |
| `psram` | [psram/](../esp32p4-hal/src/psram) | MPLL、MSPI、MMU 映射、分配器 |
| `usb` | [esp32p4-usb](../esp32p4-usb/) | USB OTG 设备与主机两侧的 DWC 驱动（slave、轮询）；协议层在共享 crate `usb-stack`（device：CDC-ACM 等 class；host：枚举 + HID 键盘） |
| `usb_serial_jtag` | [usb_serial_jtag.rs](../esp32p4-hal/src/usb_serial_jtag.rs) | USB Serial/JTAG 控制台 |
| `system` / `startup` | [system.rs](../esp32p4-hal/src/system.rs) / [startup.rs](../esp32p4-hal/src/startup.rs) | 软复位、复位原因、看门狗与核 1 的启动清理 |
| `cache` | [cache.rs](../esp32p4-hal/src/cache.rs) | writeback / invalidate / writeback_invalidate，L1 I/D + L2 挂起与恢复 |
| `efuse` | [efuse.rs](../esp32p4-hal/src/efuse.rs) | 芯片/块版本、MAC、Flash/PSRAM 参数的只读访问 |
| `mmu` | [mmu.rs](../esp32p4-hal/src/mmu.rs) | 外部内存页的 map/unmap、cached/uncached 窗口与占用位图 |
| `tcm` | [tcm.rs](../esp32p4-hal/src/tcm.rs) | 8 KB HP SPM（寄存器里叫 HP TCM）：`#[ramfunc]` / `#[tcmfunc]` / `#[tcmdata]` 放置、范围查询、奇偶校验初始化 |

## 2. 第 0 层：现有模块的缺口（已完成）

后面所有驱动都建立在这一层上。三项缺口已经补齐：

| 项 | 实现 | 为什么是前置 |
|----|------|--------------|
| **GPIO 中断 + 外设信号矩阵** | `gpio`：`InterruptType`/`InterruptLine`、`STATUS`/`INTRn` 状态与清除、`signal` 下完整的 `OUT_SEL`/`IN_SEL` 路由 API | UART/SPI/I2S/LEDC/RMT 想用引脚都要经过 GPIO matrix；按键与传感器中断也在这里 |
| **eFuse / 芯片版本** | `efuse`：芯片/块版本、MAC、Flash/PSRAM 参数；`Sources::from_efuse`、`ClockTree::init_from_efuse`、`PsramConfig::from_efuse` | CPLL 是 360 还是 400 MHz、PSRAM 时序、USB PHY 都取决于版本；`clock::Sources` 不再需要手工填写 |
| **MMU / cache 公共 API** | `mmu::Region` 的 map/unmap 与占用位图；`cache::writeback_invalidate`、`cache::suspend`/`resume` | 接外部 PSRAM 以外的映射、DMA 缓冲、`set_cpu_frequency` 的前置操作都要它 |

## 3. 第 1 层：通信与常用外设

按依赖关系排序，同一档内的可以并行。

1. ~~**UART0–4 + LP UART**：先轮询，再中断（回调放进 owner），高速路径可选 DMA。~~
   已完成 `UART0–4`；LP UART 依赖 LP GPIO 矩阵/焊盘模型，见第 2 层。
2. ~~**I2C0/1 + LP I2C**：主机模式（7/10 bit 地址、时钟拉伸、超时），轮询与中断。~~
   已完成 `I2C0/1` 主机（7 位地址、时钟拉伸由硬件处理 + 软件截止时间）；10 位地址、
   从机模式与 LP I2C 待补。
3. **GDMA（`DMA`，即 GDMA；另有 `AHB_DMA`/`AXI_DMA`）**：描述符环、外设握手、
   字节序/位宽配置。UART/SPI/I2S/ADC 的“能用”与“好用”之间就差这一块；
   `emac` 有自己的 DMA，不必依赖它。
4. **GPSPI2/3（SPI master/slave）+ SDHOST**：SD 卡、传感器、外部模块。
5. ~~**ADC**：电压/电池采样。~~ 已完成 `ADC1`/`ADC2` 的 one-shot 读取（每通道衰减、
   eFuse 工厂校准与毫伏换算）；HP 数字控制器（pattern 表 + DMA）与 LP 核侧的 LP ADC 待补。
   **TSENS**（`LP_TSENS`，独立的温度传感器）仍未实现。
6. **PWM 家族：LEDC、MCPWM、RMT、PCNT**：调光、电机、WS2812/红外、编码器。
7. **TWAI（CAN-FD ×3）、I2S0/1/2（音频）、SDIO slave**。
   ~~USB OTG host~~ 已完成：`esp32p4-usb` 的 `host` 模块（枚举、控制/bulk/interrupt
   传输、HID boot 键盘、HUB）加 `esp32p4-usb` 的 DWC 主机模式驱动；高速 HUB
   后面的 FS/LS 拆分事务（TT）与等时传输仍是缺口，可用 `UsbHostHs::new_fsls_only`
   规避 TT。

## 4. 第 2 层：系统与电源

- **LP 引脚与 LP 外设**：`LP_IO_MUX` 的焊盘配置 + `LP_GPIO` 交换矩阵（LP UART / LP I2C /
  LP SPI / LP I2S 的引脚都在这里），是 LP UART、LP I2C 的前置；LP 外设本身与 HP 版共用
  寄存器布局，补上引脚后可直接复用现有驱动。`gpio::pins` 已经标出 LP 域可达的
  GPIO0–15，以及 GPIO14/15 的 `LP_UART_TXD/RXD` 直接功能。
- **PMU / 低功耗**：light sleep、deep sleep、HP 核心电源域、唤醒源（GPIO / LP timer /
  LP UART）。电池设备必需，也是 P4 上仅次于时钟树的复杂模块。
- **LP 核（ULP RISC-V）**：固件加载、启动、与 HP 核共享内存。
- **LP WDT / 掉电检测（brownout）**：目前 `startup` 只是把看门狗关掉，没有可用驱动。
- **TRNG + 硬件加密（AES/SHA/RSA/ECC/HMAC/DS）**：TLS、安全启动的前提，
  否则全部落在软件实现上。

## 5. 第 3 层：锦上添花

I3C、UHCI、SOC_ETM（事件矩阵）、APM/访问权限、eFuse 编程、运行时 Flash 读写
（现在烧录完全依赖 `espflash`）。

## 6. 推荐节奏

| 阶段 | 内容 | 验收 |
|------|------|------|
| 0 | ~~eFuse / 芯片版本、MMU/cache 公共 API~~ | 已完成：`clock`/`psram` 不再需要手工参数 |
| 1 | ~~GPIO 信号矩阵 + GPIO 中断~~ | 已完成：引脚可以路由到任一外设信号，`gpio_interrupt` 示例可编译 |
| 2 | ~~UART（UART0–4）~~ | 已完成：`uart` 回环示例通过，README 与模块文档更新 |
| 3 | ~~eFuse / 芯片版本~~ | 已完成：`clock` 与 `psram` 按版本判断，不再需要手工 `Sources` |
| 4 | ~~I2C（I2C0/1）~~ | 已完成：`i2c_scan` 示例通过；LP I2C 待 LP 引脚模型 |
| 5 | GDMA | 描述符环 + UART/SPI 的 DMA 路径 |
| 6 | SPI/SD、ADC/TSENS、PWM 家族 | 各自示例 |
| 7 | 低功耗（PMU / LP 核 / WDT / brownout） | 睡眠-唤醒示例 |

第 0 层与阶段 1、2 完成后，HAL 具备“接串口就能调试、接按键就能交互”的最小闭环，
之后再向外设广度与低功耗推进。

## 7. 每个模块的完成标准

沿用 [`style-guide.md`](style-guide.md)：

- **骨架**（第 2 节）：owner 类型持有 PAC 外设，子部件各自持有句柄，宏生成重复类型；
  中断线归 owner，`unsafe fn steal()` 只给寄存器访问。
- **所有权与子句柄**（第 4 节）：`init`/`release`、`Deref` 到寄存器块（如 `ClockTree`）。
- **错误处理**（第 6 节）：`try_*` 返回 `Result`，无法恢复的配置错误用 `panic`。
- **验证**（第 11 节）：`cargo check --workspace`、`cargo clippy --workspace`、
  `cargo doc -p esp32p4-hal --no-deps`（无新断链）、`rustfmt --check` 全绿。
- **交付物**（第 13 节）：`src/<name>.rs` + `lib.rs` 注册 + 至少一个
  `esp32p4-examples` 示例 + README 对应小节。

## 8. 非目标

- 图形与图像：`LCD_CAM`、MIPI DSI/CSI、`JPEG`、`ISP`、`H264`、`PPA`、`DMA2D`。
- 无线：ESP32-P4 没有 WiFi/蓝牙射频，不需要 Wi-Fi/BLE 驱动。
- 安全启动/Flash 加密的完整流程：本仓库的镜像布局见 [`boot.md`](boot.md)，
  相关 eFuse 编程排在第三档。
