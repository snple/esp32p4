# ESP32-P4 HAL 驱动路线图

本文件回答三个问题：**已经实现了什么**、**还缺什么**、**按什么顺序补**。范围限定为
**非图形图像**功能（排除清单见[第 6 节](#6-非目标)）。判定标准只有一条：**没有它，就
写不出一份像样的固件**。新模块遵循 [`style-guide.md`](style-guide.md)（第 2、4、6、9、
11、13 节），骨架、所有权模型与验证流程和既有模块一致（见[第 5 节](#5-完成标准)）。

## 1. 现状

已实现且上板验证的模块，按「基础 → 通信 → 外设 → 系统」排列：

| 模块 | 文件 | 状态 |
|------|------|------|
| `time` / `delay` | [time.rs](../esp32p4-hal/src/time.rs) · [delay.rs](../esp32p4-hal/src/delay.rs) | 单调时钟、自旋延时、停核睡眠 |
| `clock` | [clock.rs](../esp32p4-hal/src/clock.rs) | ROOT_CLK 选源、各级分频、PLL_FxxM、LP 时钟、CPU 变频、CPLL/SPLL 模拟侧分频与标定 |
| `reset` / `system` / `startup` | [reset.rs](../esp32p4-hal/src/reset.rs) · [system.rs](../esp32p4-hal/src/system.rs) · [startup.rs](../esp32p4-hal/src/startup.rs) | 外设复位、软复位、复位原因、看门狗清理 |
| `timer` | [timer/mod.rs](../esp32p4-hal/src/timer/mod.rs) | TIMG0/1 四路计数器与两个 MWDT |
| `systimer` | [systimer.rs](../esp32p4-hal/src/systimer.rs) | COMP1/COMP2 闹钟 |
| `machine_timer` | [machine_timer.rs](../esp32p4-hal/src/machine_timer.rs) | 每核 CLINT 64 位比较器 + CLIC id 7 |
| `interrupt` | [interrupt.rs](../esp32p4-hal/src/interrupt.rs) | CLIC 线与矩阵、`claim`/`InterruptHandler` |
| `gpio` | [gpio/](../esp32p4-hal/src/gpio) | 电气配置、四路中断、完整 HP 交换矩阵路由、IO MUX 功能表 |
| `cache` / `mmu` / `tcm` | [cache.rs](../esp32p4-hal/src/cache.rs) · [mmu.rs](../esp32p4-hal/src/mmu.rs) · [tcm.rs](../esp32p4-hal/src/tcm.rs) | cache 维护、外部内存映射、8 KB HP SPM 代码放置 |
| `efuse` | [efuse.rs](../esp32p4-hal/src/efuse.rs) | 只读：芯片/块版本、MAC、Flash/PSRAM 参数 |
| `psram` | [psram/](../esp32p4-hal/src/psram) | MPLL、MSPI、MMU 映射、分配器（DMA 一致性语义与双向拷贝限制见 2.1） |
| `uart` | [uart.rs](../esp32p4-hal/src/uart.rs) | UART0–4 轮询 + 按源中断、流控、GPIO 矩阵路由、UHCI + GDMA 的 DMA 收发 |
| `i2c` | [i2c.rs](../esp32p4-hal/src/i2c.rs) | I2C0/1 主机与从机 |
| `spi` | [spi.rs](../esp32p4-hal/src/spi.rs) | SPI2/3 主机与从机（CPU 传输 64 B + GDMA-AXI 的 DMA 与半双工） |
| `sdhost` | [sdhost.rs](../esp32p4-hal/src/sdhost.rs) | SD 卡识别、容量解码、1/4 位总线块读写 |
| `twai` | [twai.rs](../esp32p4-hal/src/twai.rs) | TWAI0/1/2 经典 CAN 2.0B（P4 无 CAN-FD） |
| `emac` / `net` | [emac/](../esp32p4-hal/src/emac) · [net.rs](../esp32p4-hal/src/net.rs) | RMII + MDIO + DMA，smoltcp 同步栈 |
| `usb` | [esp32p4-usb](../esp32p4-usb/) | OTG 设备/主机 DWC 驱动（slave、轮询） |
| `usb_serial_jtag` | [usb_serial_jtag.rs](../esp32p4-hal/src/usb_serial_jtag.rs) | USB Serial/JTAG 控制台 |
| `adc` | [adc.rs](../esp32p4-hal/src/adc.rs) | ADC1/2 one-shot + 样式表扫描 + ADC1 连续采样（GDMA 环）+ IIR + 阈值监控 |
| `tsens` | [tsens.rs](../esp32p4-hal/src/tsens.rs) | LP_TSENS 温度传感器 |
| `i2s` | [i2s.rs](../esp32p4-hal/src/i2s.rs) | 标准双槽/TDM + PDM 发送（PDM 接收未落地，见 2.1） |
| `apll` | [apll.rs](../esp32p4-hal/src/apll.rs) | 音频 PLL：系数搜索、regi2c、三步标定 |
| `ledc` / `rmt` / `pcnt` / `mcpwm` | [ledc.rs](../esp32p4-hal/src/ledc.rs) · [rmt.rs](../esp32p4-hal/src/rmt.rs) · [pcnt.rs](../esp32p4-hal/src/pcnt.rs) · [mcpwm.rs](../esp32p4-hal/src/mcpwm.rs) | PWM 家族全功能，RMT 通道 3/7 支持 GDMA，MCPWM 无剩余 |
| `etm` | [etm.rs](../esp32p4-hal/src/etm.rs) | SOC_ETM 50 通道事件/任务连接 |
| `dma` | [dma/](../esp32p4-hal/src/dma) | GDMA-AHB + GDMA-AXI：通道句柄、描述符链/环、mem2mem、突发宽度、硬件 CRC；DW_GDMA（`DMA`）：4 通道、LLI 块传输、mem2mem（它的 CSI/DSI/ISP 外设角色属第 6 节排除项，不做） |
| `multicore` | [multicore.rs](../esp32p4-hal/src/multicore.rs) | 双 HP 核启动、跨核软中断、软件 Mutex |
| `spin` | [spin.rs](../esp32p4-hal/src/spin.rs) | 有界自旋助手 |

## 2. 缺口

### 2.1 已有驱动，但功能不全

| 模块 | 缺什么 | 前置 |
|------|--------|------|
| `uart` | LP UART | LP 引脚 |
| `i2c` | 10 位地址、双地址模式；LP I2C | —；LP 引脚 |
| `spi` | 2/4/8 线、SCT 分段、从机 HD（DMA 与主机半双工已完成） | 2/4/8 线需先补 QIO/OCT 引脚信号 |
| `dma` | GDMA-AXI 的加权仲裁 | — |
| `sdhost` | SDIO（CMD52/53 + 卡中断）、UHS-I（1.8 V/SDR50/104）、MMC 8 位、卡检测/写保护引脚 | — |
| `adc` | LP ADC、DAC 与 PWDET；ADC2 连续模式（errata 禁用） | LP 引脚 |
| `psram` | AXI 同时读源与写目的（PSRAM→PSRAM 拷贝）时每 4 KB 稳定丢 32–128 字节，单向传输正常。已排除：读采样相位（`EXTRA_DUMMY_CYCLELEN` 0–7）、DQS 相位（4 档）× 每引脚 delayline（0–15）全部 64 个组合（偶发 0 错、复测又不稳定）、burst 宽度（B8–B64）、通道优先级（0/0、5/0、0/5、5/5）、长度 64 字节对齐；方向指向 AXI↔MSPI 的读写并发本身，不是采样点问题。`AxiDma::copy` 现在对两端都在 PSRAM 的传输返回 `Error::ExternalMemoryCopy`，一端在内部 RAM 的双向传输照常可用 | — |
| `tsens` | ETM 事件/任务、LP 核唤醒路径 | — |
| `i2s` | PDM2PCM 接收（自发自收帧边界问题，需真麦克风/同步时钟验证） | — |
| `emac` | MII 接口（现只 RMII）、TX/RX 校验和卸载、PTP 时间戳 | — |
| `usb` | 主机 TT（高速 HUB 后 FS/LS 拆分事务）、等时传输 | — |
| `multicore` | 抢占式调度、Systimer 跨核共享、`msip`、核间队列、跨核 cache 维护 | — |
| `machine_timer` | `delay`/`micros` 默认按 `BOOT_CPU_HZ`=100 MHz 计数，而启动后 CPU 是 90 MHz：没有 `MachineTimer::init` 的示例里延时偏长、时间偏短 11 %（`i2s` 的 `stream` 计数检查因此 FAIL） | — |
| `interrupt` | 中断嵌套与优先级分级 | — |

### 2.2 完全没有驱动

**低功耗与 LP 域**（电池设备必需，也是 P4 上仅次于时钟树的复杂模块）：

| 模块 | PAC 块 | 备注 |
|------|--------|------|
| PMU / 低功耗管理 | `PMU` | light/deep sleep、HP 电源域、唤醒源 |
| LP 核（ULP RISC-V） | `LP_SYS` / `LP_PERI` / `LP_INTR` | 固件加载、启动、共享内存 |
| LP 引脚模型 | `LP_IO_MUX` / `LP_GPIO` | **全部 LP 外设的总前置** |
| LP 外设 | `LP_UART` / `LP_I2C0` / `LP_I2S0` / `LP_ADC` / `LP_TIMER` / `LP_WDT` / `LP_TOUCH` | 寄存器布局与 HP 版兼容，补引脚后可复用驱动 |
| 掉电检测 / PSDET / 模拟比较器 | `LP_ANA_PERI` | brownout 等 |
| 睡眠寄存器备份 | `PAU` | — |
| TRNG | `LP_SYS`（RNG 寄存器） | 非独立外设块 |
| VAD（语音活动检测） | `LP_I2S0`（VAD 寄存器） | 属语音处理，不在图形图像排除范围内 |

**安全与加密**（TLS、安全启动的前提）：

| 模块 | PAC 块 |
|------|--------|
| AES（含 XTS_AES）、SHA | `AES` / `SHA` |
| RSA、DS、ECC、ECDSA、HMAC | `RSA` / `DS` / `ECC` / `ECDSA` / `HMAC` |
| key manager | `LP_HUK` |

**通信与存储**：

| 模块 | PAC 块 | 备注 |
|------|--------|------|
| I3C | `I3C_MST` / `I3C_SLV` | — |
| UHCI | `UHCI0` | UART0/1 的 DMA 前端 |
| PARL_IO | `PARL_IO` | 通用并行 IO，挂 GDMA-AXI |
| GPIO_SD | `GPIO_SD` | Sigma-Delta 调制 ×4 |
| BITSCRAMBLER | `BITSCRAMBLER` | 片外存储器数据扰码 |
| 运行时 Flash 读写 | `SPI0` / `SPI1` | 现在烧录完全依赖 `espflash` |
| eFuse 编程 | `EFUSE` | 现在只读 |

**DMA 与调试监测**：

| 模块 | PAC 块 | 备注 |
|------|--------|------|
| GDMA-AXI | `AXI_DMA` | SPI2/3、PARLIO、AES、SHA 用；见 2.1 |
| DW_GDMA | `DMA` | JPEG/摄像头的纯存储搬运引擎，与 GDMA 无关 |
| AXI_ICM | `AXI_ICM` | AXI 互连/QoS 配置 |
| ASSIST_DEBUG | `ASSIST_DEBUG` | 总线/栈/区域监控 |
| TRACE | `TRACE0` / `TRACE1` | RISC-V 追踪编码器 |
| PVT | `PVT` | 工艺/电压/温度监测 |

### 2.3 SVD/PAC 缺块，暂无法驱动

TRM 有专门章节，但 `esp32p4.svd` 没有对应 peripheral，PAC 里没有寄存器块：

- **AXI 性能监测器**（TRM 第 23 章）：9 个监测端口、2 个监测单元。
- **LP 信箱控制器**（TRM 第 24 章）：16 个 32 位消息寄存器，LP CPU ↔ HP CPU 通信。
- **权限控制 PMS/APM**（TRM 第 21 章）：机器/用户模式访问控制。

补法：等 SVD 补块，或按 `emac.svd` 先例手写寄存器定义合并进 PAC。TRNG 与 VAD 不属于
此类（寄存器分别在 `LP_SYS` 与 `LP_I2S0` 内，等 LP 引脚与 LP I2S 驱动落地即可）。

## 3. 关键依赖

- **LP 引脚模型** → 所有 LP 外设（LP UART/I2C/I2S/ADC/TIMER/TOUCH）与 VAD。
- ~~**AXI_DMA** → SPI 的 DMA、PARL_IO、AES/SHA 的 DMA 路径。~~ 已完成（`AxiDma`），
  SPI 的 DMA 已用上，PARL_IO / AES / SHA 需要各自的驱动。
- **UHCI** → UART0/1 高速收发。
- **PMU + PAU** → 低功耗与 LP 核唤醒。
- **加密套件 + TRNG** → TLS 与安全启动。

## 4. 推荐节奏

| 阶段 | 内容 | 验收 |
|------|------|------|
| 0 | ~~eFuse / 芯片版本、MMU/cache 公共 API~~ | 已完成：`clock`/`psram` 不再需要手工参数 |
| 1 | ~~GPIO 信号矩阵 + GPIO 中断~~ | 已完成：引脚可路由到任一外设信号 |
| 2 | ~~UART（UART0–4）~~ | 已完成：回环示例通过 |
| 3 | ~~I2C（I2C0/1）~~ | 已完成：`i2c_scan` 示例通过 |
| 4 | ~~GDMA-AHB~~ | 已完成：mem2mem 逐字节校验通过 |
| 5 | ~~SPI/SD、ADC/TSENS、PWM 家族~~ | 已完成：各自示例通过 |
| 6 | ~~双 HP 核~~ | 已完成：`multicore` 示例跨核互踢与加锁计数通过 |
| 7 | LP 引脚模型 | LP 外设可复用现有驱动 |
| 8 | 低功耗（PMU / LP 核 / LP WDT / brownout / PAU / LP 信箱） | 睡眠-唤醒示例 |
| 9 | 加密套件 + TRNG | TLS / 安全启动前提 |
| 10 | ~~AXI_DMA~~ → ~~SPI DMA~~ / PARL_IO / AES / SHA | SPI DMA 已完成；其余各自示例 |
| 11 | 调试与监测（TRACE / PVT / ASSIST_DEBUG / AXI_ICM / BITSCRAMBLER） | 按需补齐 |

阶段 0–6 完成后，HAL 具备「接串口就能调试、接按键就能交互」的最小闭环；7–8 补齐低功耗
路径；9 起是安全与高速外设；10–11 按需求推进。

## 5. 完成标准

沿用 [`style-guide.md`](style-guide.md)：

- **骨架**（第 2 节）：owner 类型持有 PAC 外设，子部件各自持有句柄，宏生成重复类型；
  中断线归 owner，`unsafe fn steal()` 只给寄存器访问。
- **所有权与子句柄**（第 4 节）：`init`/`release`、`Deref` 到寄存器块（如 `ClockTree`）。
- **错误处理**（第 6 节）：`try_*` 返回 `Result`，无法恢复的配置错误用 `panic`。
- **宏约定**（第 9 节）：重复类型用宏生成，避免 trait/`dyn` 的实例分发。
- **验证**（第 11 节）：`cargo check --workspace`、`cargo clippy --workspace`、
  `cargo doc -p esp32p4-hal --no-deps`（无新断链）、`rustfmt --check` 全绿。
- **交付物**（第 13 节）：`src/<name>.rs` + `lib.rs` 注册 + 至少一个
  `esp32p4-examples` 示例 + README 对应小节。

## 6. 非目标

- 图形与图像：`LCD_CAM`、MIPI DSI/CSI、`JPEG`、`ISP`、`H264`、`PPA`、VDMA（TRM 第 7
  章）、2D-DMA（TRM 第 8 章）。
- 无线：ESP32-P4 没有 WiFi/蓝牙射频。
- 安全启动 / Flash 加密的完整流程：镜像布局见 [`boot.md`](boot.md)，相关 eFuse 编程排
  在第三档。
