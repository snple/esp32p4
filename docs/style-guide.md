# esp32p4 项目设计风格指南

本文件整理本仓库的设计约定，目标是「每个新模块都长得和既有模块一样」：
一致的骨架、一致的所有权模型、一致的错误处理，以及一致的验证流程。

本仓库位于 `/home/danc/code/esp32/esp32p4`，HAL crate 名为 `esp32p4-hal`，
CPU 是 RISC-V RV32IMAFC（stable 目标使用 `riscv32imafc-unknown-none-elf`）。

---

## 1. 工作区结构（Crate 分层）

```
esp32p4/                 # 工作区根
├── esp32p4-hal/         # HAL：本指南的主角（手写，面向用户）
├── esp32p4-pac/         # 由 SVD 生成的 PAC（勿手改 src/generated）
├── esp32p4-rt/          # 运行时：_start、trap、link.x、CSR/asm
├── esp32p4-macro/       # 过程宏：#[main]
├── esp32p4-examples/    # 独立二进制示例（blink）
├── docs/                # 设计/启动说明（本文件、boot.md）
└── xtask/               # 构建辅助（cargo xtask gen-pac）
```

分层原则：

- **PAC 只做寄存器**：`esp32p4-pac` 由 SVD 生成，提供 `RegisterBlock`、
  字段常量、访问器和 `peripheral!` 宏注入的 ZST 外设类型。HAL 不复制这些，
  只做语义封装。
- **rt 只做启动**：栈、`.bss` 清零、`mtvec`、FPU 位、然后调 `main`。
- **macro 只做代码生成**：目前只有 `#[main]`。
- **HAL 是唯一的手写用户接口层**：每个外设对应一个 `src/<name>.rs` 模块，
  在 `lib.rs` 里 `pub mod` 并 `pub use` 常用类型。

`lib.rs` 统一重导出入口：

```rust
pub use esp32p4_macro::main;
pub use esp32p4_pac as pac;
pub use esp32p4_pac::{Interrupt, Peripherals};
pub use esp32p4_rt as rt;
```

---

## 2. HAL 模块骨架

典型外设模块：

```rust
//! 模块文档：一句话说清外设，列出 Features，给一个 Example（` ```ignore`）。

use esp32p4_pac::{FOO, foo};

/// 外设包装结构体。拥有 PAC ZST，强制所有权语义。
pub struct Foo {
    foo: FOO,
}

impl Foo {
    /// 从 PAC 拿所有权。必要时在这里做时钟/复位配置。
    pub fn init(foo: FOO) -> Self {
        Self { foo }
    }

    /// 无所有权检查地拿句柄（ISR / 已初始化场景）。
    /// # Safety：调用者保证已初始化、且无数据竞争。
    pub unsafe fn steal() -> Steal<Self> { /* ... */ }

    /// 逃生口：返回底层寄存器块。
    pub fn regs(&self) -> &foo::RegisterBlock { &self.foo }

    /// 归还 PAC 外设。
    pub fn release(self) -> FOO { self.foo }
}
```

要点：

- **每模块一个 `pub struct`，字段是 PAC 外设 ZST**。
- 方法几乎都取 `&self`；`&mut self` 只用于真正需要内部可变状态的地方。
- `#[inline]` 标在热路径/转发方法上。
- 错误/状态枚举用
  `#[derive(Clone, Copy, PartialEq, Eq, Debug)]`。

---

## 3. PAC 关键事实（别踩的坑）

- PAC 外设是非 `Copy` 的零大小类型（ZST）：
  `pub struct FOO { _marker: PhantomData<*const ()> }` + `peripheral!` 注入
  `Deref<Target = RegisterBlock>` 和 `PTR`。
- 寄存器访问统一走 `VolatileCell<u32>`：
  - `read_bits` / `write_bits` / `modify_bits`；
  - **没有** RP2350 的 `+0x1000/+0x2000/+0x3000` 原子别名；
  - 生成的 `set_xxx()` / `clr_xxx()` 是读改写，**不是原子操作**。
- 有 W1TS/W1TC 的外设（GPIO、中断状态等）直接写对应寄存器：
  `gpio.out_w1ts.write_out_w1ts(mask)`、`gpio.enable_w1tc...`。
- 字段命名统一：`fn field()` 读、`fn write_field(v)` 写多位数、
  `fn set_field()` / `fn clr_field()` 操作单比特位、
  `fn trigger_field()` 触发 WO 单比特。
- **禁止裸指针 + 魔法数访问寄存器**：一律用 PAC 的命名寄存器/字段。
  例外是 HAL 内部为 ZST 句柄访问全局寄存器块（`unsafe { &*FOO::PTR }`），
  且必须在 `Pins::init` / `init` 之后。

### SVD 数组与命名

ESP SVD 用 `<dim>` + `%s` / `[%s]` 表示寄存器数组。生成器会：

- 展开为 `GPIO0`、`GPIO1`、`IV_MEM0`…；
- 替换描述里的 `%s`；
- 对「数组元素 0」与同名普通寄存器（如 `OUT_CONF0_CH0`）做类型名去重，
  冲突时追加数字后缀；
- 字段名/类型名以数字开头或撞 Rust 关键字时加 `_` / `r#` 处理。

### derivedFrom

`UART1..4`、`I2C1`、`I2S1/2`、`MCPWM1`、`TIMG1`、`TWAI1/2` 等外设
在 SVD 里没有寄存器，生成器会复制基础外设的寄存器定义，生成独立的
`RegisterBlock`。

---

## 4. 所有权模型与子句柄

```
控制器（拥有 PAC ZST）──steal()──► Steal<Self>
        │
        └── pin::<N>() / sub(&self) ──► 零大小句柄 / 借用子句柄
```

- **控制器拥有 ZST**：`init(pac)` 或 `steal() -> Steal<Self>`。
- **`Pin<N>` 是零大小令牌**：`Pins::init` 之后由 `pins.pin::<N>()` 取得；
  方法直接通过 `GPIO::PTR` / `IO_MUX::PTR` 访问寄存器。
- `Steal<T>` 实现 `Deref<T>` 但**不**实现 `DerefMut`。
- 需要借用的子句柄写成 `struct X<'a> { regs: &'a pac::FOO }`。

---

## 5. 驱动 trait 模式

HAL **不内置** `I2cBus` / `SpiBus` / `UartBus` 等抽象 trait：需要抽象时由
调用者（driver crate 或最终应用）自行定义并实现。HAL 只提供具体外设类型。

---

## 6. 错误处理约定

- **每个模块定义自己的 `Error` 枚举**，只列本模块真正返回的错误。
- **阻塞方法在超时时 `panic`**：用 [`spin`](../esp32p4-hal/src/spin.rs) 的有界自旋。
- **`try_*` 方法返回 `Result<_, Error>`**。
- `spin` 模块提供三个助手：
  - `spin_wait(what, cond)` — 有界自旋，超时 panic（hang-guard）。
  - `try_spin_wait(cond) -> bool` — 有界自旋，不 panic。
  - `spin_wait_max(max, what, cond)` — 自定义上界。
- 上界按**迭代次数**计，只用来兜住「永远等不到」；需要真实时限（例如 I2C 从机拉伸
  时钟）时用 `delay::micros()` 做截止时间，参见 `i2c`。

当前 HAL 的 GPIO/Delay 都是无错误路径的简单操作，因此还没有 `Error` 枚举；
新增可能失败的模块时按上面的模式补。

---

## 7. 配置结构体与构建器

- **成组的配置抽成结构体**，带 `Default`（对应硬件复位值，并注明）。
- 复位默认值以 SVD 的 `resetValue` 为准，注释里写明十六进制原值。
- 需要多步配置的用**构建器**，提供 `const fn` 链式方法。

---

## 8. 启动、复位与内存

### 启动

ESP32-P4 由 ESP-IDF 2nd-stage bootloader 加载，不是 ROM 直跳。关键约束：

- **恰好两个 flash 段**（DROM + IROM），否则 P4 bootloader 的
  `unpack_load_app()` assert 失败；
- **第一个段必须以 `esp_app_desc_t` 开头**（magic `0xABCD5432`，≥256 字节）；
- bootloader 拷贝 RAM 段；`_start` 只清零 `.bss`。

详见 [`boot.md`](boot.md)。

### 内存

默认 `memory.x`：

```ld
MEMORY
{
    ROM : ORIGIN = 0x40000020, LENGTH = 0x4000000 - 0x20
    RAM : ORIGIN = 0x4FF20000, LENGTH = 0x8DFC0
}
```

- `0x40000020` 跳过 app image header（24 B）+ 第一个 segment header（8 B）。
- `0x4FF20000` = L2MEM 起点 + 128 KB L2 cache。
- `0x4FFADFC0` = 可用 RAM 顶端，避开 ROM BSS/stack。
- 换 bootloader 的 L2 cache 划分时必须同步改 `ORIGIN`。

### 启动初始化（`startup::init`）

`#[main]` 在用户函数体前注入 `esp32p4_hal::startup::init()`，处理 bootloader 留下的状态：

- **RTC 看门狗**：bootloader 默认 `CONFIG_BOOTLOADER_WDT_ENABLE=y`，以 9 s 超时启动后
  直接交给应用；ESP-IDF 应用在 `esp_system` 启动时关闭它，裸机必须自己做，否则周期性复位。
- **TIMG0/TIMG1 MWDT**：防御性关闭；写之前要先打开
  `PERI_CLK_CTRL{20,21}.REG_TIMERGRPx_WDT_CLK_EN`。
- **core 1**：ROM 只启动 core 0；core 1 的 `PMU.cpu_sw_stall`、
  `SOC_CLK_CTRL0.CORE1_CPU_CLK_EN`、`HP_RST_EN0.RST_EN_CORE1_GLOBAL` 在 LP 域，
  软件复位不清除，需要主动停放。
- **SWD**：不用管，bootloader 已经设置 `SWD_AUTO_FEED_EN`。
- **分支预测器**：`csrs 0x7c1`，只影响性能。

### 时钟与复位

其余 HAL 依赖 bootloader 留下的默认时钟：

- GPIO `CLOCK_GATE.CLK_EN` 复位为 1；
- `HP_SYS_CLKRST.PERI_CLK_CTRL26.IOMUX_CLK_EN` 复位为 1；
- `PERI_CLK_CTRL21.SYSTIMER_CLK_EN` 复位为 1，`SYSTIMER.CONF.UNIT0_WORK_EN`
  复位为 1。

`Pins::init` 仍会显式打开 GPIO/IOMUX 时钟并释放 IOMUX 复位，避免依赖
bootloader 的具体版本。新增外设时，如果它有自己的时钟门控/复位位，在
`init` 里显式配置，并写清复位值。

---

## 9. 宏约定

- **过程宏在 `esp32p4-macro`**：`#[main]` 把 `fn main() -> !` 展开为
  `#[unsafe(no_mangle)] pub extern "C" fn main() -> !`。
- **放置类宏**：`#[ramfunc]` / `#[tcmfunc]` / `#[tcmdata]` 只改 `link_section`
  （`.ramfunc` / `.tcm.text` / `.tcm.data`）并补 `#[inline(never)]`。
  函数类宏拒绝 `async fn`、`#[inline]` 和重复的 `link_section`，因为这三种情况
  都会让函数在无提示的情况下仍然从 flash 运行。
- 实例化宏用 `macro_rules!`：GPIO 的 54 个 pin 用 `impl_pin!` 展开，
  每个 pin 生成 `Pin<N>` 的寄存器操作和 `PinOps` 实现。
- 宏内部引用的寄存器字段名必须由宏参数传入，避免卫生问题。

---

## 10. 命名与文档风格

- 模块名与外设缩写一致：`gpio`、`delay`、`systimer`、`io_mux`；跨外设的系统级功能放
  `system`（复位、复位原因、核间信息）。带子部件的模块用目录形式（`psram/` 下
  `mod.rs` + `regi2c.rs`/`mpll.rs`/`pads.rs`/`mspi.rs`/`alloc.rs`；`gpio/` 下
  `mod.rs`（共享类型与 `Pins`）+ `pin.rs`（`Pin<N>` 与 `impl_pin!`）+ `pins.rs`
  （焊盘功能表）+ `signal.rs`（矩阵信号））。
- **每引脚实现宏 + 引脚能力表**：`Pin<N>` 的寄存器操作由 `impl_pin!` 逐脚展开，
  所有驱动通过 `PinOps` 泛型接收引脚；焊盘专有的直接功能（IO MUX 的 `MCU_SEL`）与
  模拟功能放在 `gpio::pins::PAD_TABLE`，不要把它们写成 `Function` 的裸数字。
- **矩阵不是万能的**：写「任意引脚都能接」之前先查信号表 —— GMAC PHY/RMII 数据与时钟、
  USB 数据线、模拟功能都只有专用焊盘，LP 外设只能用 GPIO0–15。这类驱动的引脚检查
  用 `PadDesc`（`is_gmac_phy` / `is_usb_phy` / `function_index`），别假设矩阵能救。
- 类型：`Pins`（控制器）、`Pin<N>`（零大小令牌，配置 `&mut self`、读写 `&self`，
  `into_output()` 返回 `&Self`，与 rp2350 一致）。
- 模块文档统一结构：一段概述 → `## Features` 列表 → `## Example`
  （` ```ignore`），必要时加 `## Architecture` / `## Pin Mapping`。
- 跨模块复用的概念放回源头再引用，避免重复定义。

---

## 11. 验证流程（每个改动都跑）

```bash
cargo check -p esp32p4-pac --target riscv32imafc-unknown-none-elf
cargo check -p esp32p4-hal --target riscv32imafc-unknown-none-elf
cargo clippy -p esp32p4-hal -p esp32p4-rt -p esp32p4-macro -p esp32p4-examples \
  --target riscv32imafc-unknown-none-elf -- -D warnings
cargo fmt --all -- --check
```

生成代码漂移检查（改过 SVD / 生成器后）：

```bash
cargo xtask gen-pac --check
```

镜像验证：

```bash
cd esp32p4-examples
cargo build --bin blink
espflash save-image --chip esp32p4 \
  ../target/riscv32imafc-unknown-none-elf/debug/blink /tmp/blink.bin
```

### 工具链

- **stable**，默认 target `riscv32imafc-unknown-none-elf`（RV32IMAFC，
  `+f`，`ilp32f` 硬浮点 ABI）。`_start` 在 `call main` 前设置
  `mstatus.FS = Dirty`。
- 无 F 的构建仍可显式使用 `riscv32imac-unknown-none-elf`；根
  `.cargo/config.toml` 同时为两个 target 配置了 `-Tlink.x`。
- **生成代码不要手改**：`esp32p4-pac/src/generated/` 是
  `cargo xtask gen-pac` 产物；改动走 `xtask/src/generator.rs` 再重新生成
  （生成后已自动 `rustfmt`）。

---

## 12. 常见 bug 模式（历史教训）

- **P4 bootloader 要求两个 flash 段**：只有 RAM 段的镜像会 assert。
  不要删掉 `.drom_gap`。
- **segment 0 必须是 app descriptor**：`espflash` 和 bootloader 都依赖
  `.flash.appdesc` 开头的 `0xABCD5432`。
- **GPIO 输出忘了 `OEN_SEL=1`**：`OUT_SEL=256` 只选了输出信号，输出使能
  还要由 `GPIO_ENABLE` 控制时必须设 `OEN_SEL=1`。
- **GPIO32+ 用错寄存器**：用 `OUT1_W1TS/OUT1_W1TC/ENABLE1_*`。
- **W1C 寄存器写反方向**：该 `write_bits(mask)` 却用读改写。
- **除零 / 位移错误**：`1u32 << 32` 是 UB。
- **内联汇编写 `1:` 标签**：`1b` 会被当成二进制字面量，`binary_asm_labels`
  是 deny 级 lint；数字标签从 `2` 开始。`asm_sub_register` 是 IDE 对 RISC-V
  的误报（它只针对 x86 子寄存器），`cargo clippy --target riscv32imafc` 不会报。
- **硬件事实要核实**：GPIO function = 1；IO_MUX 0–53 才有寄存器；
  LP pad hold 在 `LP_IO_MUX.LP_PAD_HOLD`，不在 HP_SYS。
- **不要依赖 `ESP32-P4` 的 cache 配置**：换 bootloader 时同步改
  `memory.x`。

---

## 13. 新增一个外设模块的清单

1. `cargo` 确认 PAC 已有对应外设（`esp32p4-pac/src/generated/`）。
2. 新建 `src/<name>.rs`，照第 2 节骨架写；确认时钟/复位位。
3. 在 `lib.rs` 加 `pub mod <name>;` 和 `pub use <name>::<Type>;`。
4. 跑第 11 节的验证命令，全绿。
5. 需要跨模块复用的类型从源头 `pub` 并 `use crate::...` 引用。
