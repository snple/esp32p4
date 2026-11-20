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
| `adc` | [adc.rs](../esp32p4-hal/src/adc.rs) | ADC1/2 单次转换与 eFuse 校准、HP 数字控制器样式表扫描（定时器 + GDMA-AHB）、IIR 滤波器与阈值监控器 |
| `tsens` | [tsens.rs](../esp32p4-hal/src/tsens.rs) | `LP_TSENS` 温度传感器：上电与时钟、5 档自动选择、毫摄氏度读数、硬件监测阈值与中断 |
| `sdhost` | [sdhost.rs](../esp32p4-hal/src/sdhost.rs) | SD 卡主机（`SDHOST`）：识别（CMD0/8/2/3/9/7、ACMD41/6）、CSD 容量解码、1/4 位总线、内部 DMA 的块读写 |
| `spi` | [spi.rs](../esp32p4-hal/src/spi.rs) | GP-SPI2/3 主机与从机：CPU 传输（64 B 缓冲）、时钟模式 0–3、位序、硬件 CS 时序、任意引脚路由 |
| `dma` | [dma/](../esp32p4-hal/src/dma) | GDMA-AHB：3 组收发通道、描述符链/环、外设选择、存储到存储拷贝、按源中断、优先级与突发 |
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
3. ~~**GDMA**：描述符环、外设握手、字节序/位宽配置。~~
   已完成 GDMA-AHB（PAC 的 `AHB_DMA`）：`dma` 模块的通道句柄、`Chain` 描述符链/环、
   `AhbDma::copy` 的存储到存储路径，以及中断、优先级、突发与 ETM 开关；上板 200 轮
   SRAM→SRAM 校验通过，未开 burst 时约 23 MB/s。剩下：
   - **GDMA-AXI**（`AXI_DMA`）：LCD/CAM、SPI2/3、PARLIO、AES、SHA 用的是它，
     描述符 8 字节对齐、多一段外部存储窗口、突发长度可选；
   - **`DMA`（`DW_GDMA`）**：JPEG/摄像头所用的纯存储搬运引擎，与 GDMA 无关；
   - **外设握手驱动**：UART 的 DMA 经 UHCI（UART0/1），另有 I2S、ADC 连续采样、RMT；
     通道与 `Peripheral` 选择已经就位，缺的是这些外设自己的驱动；
   - **cached PSRAM 缓冲区**：引擎能写对，但 `cache` 侧的失效目前对 PSRAM 窗口无效，
     CPU 回读取不到 DMA 写入的数据（片内 SRAM 没有这个问题）。
4. ~~**GPSPI2/3（SPI master/slave）**~~：已完成 `Spi2`/`Spi3` 主机与 `SpiSlave2`/`SpiSlave3`
   从机（CPU 传输、64 字节缓冲、时钟模式 0–3、位序、硬件 CS 时序），上板 4 种模式 ×
   4 种长度双向校验通过。剩下：
   - **DMA 传输**：SPI2/3 挂在 GDMA-AXI 上，需要先实现 `AXI_DMA`；
   - **2/4/8 线模式、半双工（`SPI_DOUTDIN=0`）、SCT 分段传输**；
   - **从机 HD 模式**（`SPI_SLAVE_HD_VER2`，DMA + 分段）。

5. ~~**SDHOST（SD 卡主机，Synopsys DesignWare MSHC，TRM 第 57 章）**~~：已完成
   `SdHost` + `SdCard`（识别、CSD 容量解码、1/4 位总线、内部 DMA 的块读写、多块
   传输手发 CMD12），上板在 32 GB SDHC 上读第 0 块并写两块回读、逐字节校验通过。
   剩下：
   - **SDIO 功能访问**（CMD52/53）与卡中断；
   - **UHS-I（1.8 V 切换、SDR50/104、DLL 调相）**与高速/DDR 模式；
   - **MMC 8 位总线**（P4 封装只引出 4 根数据线）；
   - **卡检测/写保护引脚**（矩阵信号已就位，驱动侧未接）。
6. ~~**ADC**：电压/电池采样。~~ 已完成 `ADC1`/`ADC2` 的 one-shot 读取（每通道衰减、
   eFuse 工厂校准与毫伏换算），HP 数字控制器的样式表扫描（`Pattern` 表、定时器触发、
   帧长/转换次数限制、GDMA-AHB 落内存，上板两通道交替采样通过），以及 IIR 测量滤波器
   与阈值监控器（上板越界判 High、窗口内判 None 均正确）。剩下：
   - **LP 核侧的 LP ADC**：LP CPU / ULP 在睡眠中采样的路径；
   - **DAC 与 PWDET** 共用 SAR 的配置。
   **TSENS** 已实现（见上面 `tsens` 一行）：5 个档位的自动选择、毫摄氏度换算、硬件
   自动监测与越界中断；ETM 任务/事件与 LP 核侧的唤醒路径未接。
7. **PWM 家族：LEDC、MCPWM、RMT、PCNT**：调光、电机、WS2812/红外、编码器。
8. **TWAI（CAN-FD ×3）、I2S0/1/2（音频）、SDIO slave**。
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
| 5 | ~~GDMA-AHB~~ | 已完成：`dma_mem2mem` 示例逐字节校验通过；AXI/DW_GDMA 与外设握手驱动见第 3 节 |
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
