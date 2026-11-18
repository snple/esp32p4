# esp32p4-pac

ESP32-P4 的外设访问 crate，由仓库根目录的 `esp32p4.svd` 生成。

**不要手改 `src/generated/`。** 需要调整生成结果时改 `xtask/src/generator.rs`，
然后：

```bash
cargo xtask gen-pac
```

CI / 本地校验生成代码是否与 SVD 一致：

```bash
cargo xtask gen-pac --check
```

完整 PAC 约 39 MB / 130 万行（89 个外设、约 5600 个展开寄存器），
首次编译需要几十秒。

## API 形态

- 每个外设一个模块（`gpio`、`io_mux`、`systimer`、…），模块里是
  `#[repr(transparent)]` 的寄存器结构和 `#[repr(C)]` 的 `RegisterBlock`。
- 寄存器内部使用 `common::VolatileCell<u32>`：`read_bits` / `write_bits` /
  `modify_bits`。
- 每个字段生成：
  - `XXX_OFFSET` / `XXX_MASK` 常量；
  - 单比特字段额外生成 `XXX` 位常量；
  - 读方法（RO/RW）：`fn xxx()`；
  - 写方法（RW/WO 多比特）：`fn write_xxx(v)`；
  - 单比特 RW：`fn set_xxx()` / `fn clr_xxx()`（读改写，非原子）；
  - 单比特 WO：`fn trigger_xxx()`。
- 每个寄存器还生成一个 `XxxVal` 值类型，用于批量读改写。
- `Peripherals::take()` 一次性取得所有外设；`Peripherals::steal()` 是逃生口。
- `Interrupt` 枚举列出 SVD 中的中断源编号（中断矩阵源号，不是 CLIC 线号）。

## ESP32-P4 与 RP2350 PAC 的差异

- **没有** Cortex-M 风格的 `+0x1000/+0x2000/+0x3000` 原子别名。
  所有寄存器都是 `VolatileCell`，`set_xxx()` / `clr_xxx()` 是读改写。
  有 W1TS/W1TC 的外设（如 GPIO）请直接使用对应的寄存器。
- SVD 使用 `<dim>` + `%s` / `[%s]` 表示寄存器数组。生成器会把它们展开成
  `GPIO0`、`GPIO1`、`IV_MEM0` 等普通寄存器，并在类型名冲突时追加数字后缀。
- `derivedFrom` 外设（`UART1..4`、`I2C1`、`I2S1/2` 等）会复制基础外设的
  寄存器定义，生成独立的 `RegisterBlock`。
