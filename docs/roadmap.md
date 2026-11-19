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
| `time` / `delay` | [time.rs](../esp32p4/src/time.rs) / [delay.rs](../esp32p4/src/delay.rs) | SYSTIMER 单调时钟、阻塞延时与比较器闹钟 |
| `clock` | [clock.rs](../esp32p4/src/clock.rs) | ROOT_CLK 选源、CPU/MEM/SYS/APB 分频、PLL_FxxM、LP 时钟、CPU 变频 |
| `reset` | [reset.rs](../esp32p4/src/reset.rs) | `HP_RST_EN0/1/2` 上的外设复位 |
| `timer` | [timer/mod.rs](../esp32p4/src/timer/mod.rs) | TIMG0/1 四路计数器与两个 MWDT，中断归 group |
| `systimer` | [systimer.rs](../esp32p4/src/systimer.rs) | COMP1/COMP2 闹钟，COMP0 归 `Delay` |
| `gpio` | [gpio.rs](../esp32p4/src/gpio.rs) | 引脚电气配置与 GPIO 读写（**无中断、无外设信号路由**） |
| `interrupt` | [interrupt.rs](../esp32p4/src/interrupt.rs) | CLIC 线与矩阵、`claim`/`InterruptHandler` |
| `emac` / `net` | [emac/](../esp32p4/src/emac) / [net.rs](../esp32p4/src/net.rs) | RMII + MDIO + DMA，smoltcp 同步栈 |
| `psram` | [psram/](../esp32p4/src/psram) | MPLL、MSPI、MMU 映射、分配器 |
| `usb` | [esp32p4-usb](../esp32p4-usb/) | USB OTG **设备（slave）** 侧：CDC-ACM 等 |
| `usb_serial_jtag` | [usb_serial_jtag.rs](../esp32p4/src/usb_serial_jtag.rs) | USB Serial/JTAG 控制台 |
| `system` / `startup` | [system.rs](../esp32p4/src/system.rs) / [startup.rs](../esp32p4/src/startup.rs) | 软复位、复位原因、看门狗与核 1 的启动清理 |
| `cache` | [cache.rs](../esp32p4/src/cache.rs) | 只有 writeback/invalidate |

## 2. 第 0 层：先补现有模块的缺口

后面所有驱动都建立在这一层上。

| 缺口 | 现状 | 需要补什么 | 为什么是前置 |
|------|------|------------|--------------|
| **GPIO 中断 + 外设信号矩阵** | `gpio.rs` 只有引脚电气配置和 GPIO 读写，`FUNCx_OUT_SEL` 固定指向 GPIO | GPIO_INTR 的边沿/电平配置、状态清除与 CPU 线；`FUNCx_OUT_SEL/IN_SEL` 的外设信号路由 API | UART/SPI/I2S/LEDC/RMT 想用引脚都要经过 GPIO matrix；按键与传感器中断也在这里 |
| **eFuse / 芯片版本** | 没有 | 芯片版本、MAC、Flash/PSRAM 参数的只读访问 | CPLL 是 360 还是 400 MHz、PSRAM 时序、USB PHY 都取决于版本；`clock::Sources` 目前只能由用户手填 |
| **MMU / cache 公共 API** | region 映射藏在 `psram` 里，`cache` 只有 writeback/invalidate | 外部内存 region 的 map/unmap、DMA 一致性维护、切换频率前挂起 ext-mem cache | 接外部 PSRAM 以外的映射、DMA 缓冲、`set_cpu_frequency` 的前置操作都要它 |

## 3. 第 1 层：通信与常用外设

按依赖关系排序，同一档内的可以并行。

1. **UART0–4 + LP UART**：先轮询，再中断（回调放进 owner），高速路径可选 DMA。
   依赖第 0 层的引脚路由。做完之后 HAL 才有不依赖 `defmt` 的日志/控制台。
2. **I2C0/1 + LP I2C**：主机模式（7/10 bit 地址、时钟拉伸、超时），轮询与中断。
   依赖第 0 层。
3. **GDMA（`DMA`，即 GDMA；另有 `AHB_DMA`/`AXI_DMA`）**：描述符环、外设握手、
   字节序/位宽配置。UART/SPI/I2S/ADC 的“能用”与“好用”之间就差这一块；
   `emac` 有自己的 DMA，不必依赖它。
4. **GPSPI2/3（SPI master/slave）+ SDHOST**：SD 卡、传感器、外部模块。
5. **ADC + TSENS**：电压/电池/温度采样。
6. **PWM 家族：LEDC、MCPWM、RMT、PCNT**：调光、电机、WS2812/红外、编码器。
7. **TWAI（CAN-FD ×3）、I2S0/1/2（音频）、SDIO slave、USB OTG host**：
   `esp32p4-usb` 现在只有设备侧，host 侧缺失。

## 4. 第 2 层：系统与电源

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
| 1 | GPIO 信号矩阵 + GPIO 中断 | 引脚可以路由到任一外设信号；按键中断示例可编译 |
| 2 | UART（含 LP UART） | 轮询与中断收发示例，README 与模块文档更新 |
| 3 | eFuse / 芯片版本 | `clock` 与 `psram` 按版本判断，不再需要手工 `Sources` |
| 4 | I2C（HP + LP） | 扫描/读写示例 |
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
  `cargo doc -p esp32p4 --no-deps`（无新断链）、`rustfmt --check` 全绿。
- **交付物**（第 13 节）：`src/<name>.rs` + `lib.rs` 注册 + 至少一个
  `esp32p4-examples` 示例 + README 对应小节。

## 8. 非目标

- 图形与图像：`LCD_CAM`、MIPI DSI/CSI、`JPEG`、`ISP`、`H264`、`PPA`、`DMA2D`。
- 无线：ESP32-P4 没有 WiFi/蓝牙射频，不需要 Wi-Fi/BLE 驱动。
- 安全启动/Flash 加密的完整流程：本仓库的镜像布局见 [`boot.md`](boot.md)，
  相关 eFuse 编程排在第三档。
