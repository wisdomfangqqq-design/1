# Rust 计算器加载器

本项目演示如何用 Rust 实现一个**插件式动态库加载器**，在运行时加载计算器插件（`.so` / `.dll` / `.dylib`）并调用其中的计算函数。

## 项目结构

```
.
├── Cargo.toml                  # Workspace 配置
├── calculator_plugin/          # 计算器插件（编译为动态库）
│   ├── Cargo.toml
│   └── src/lib.rs              # 导出 add / subtract / multiply / divide
└── calculator_loader/          # 加载器主程序
    ├── Cargo.toml
    └── src/
        ├── main.rs             # 入口：解析参数、调用加载器
        └── loader.rs           # CalculatorLoader 核心实现
```

## 快速开始

### 1. 编译插件动态库

```bash
cargo build --release -p calculator_plugin
```

编译产物：

| 平台    | 文件路径 |
|---------|----------|
| Linux   | `target/release/libcalculator_plugin.so` |
| macOS   | `target/release/libcalculator_plugin.dylib` |
| Windows | `target/release/calculator_plugin.dll` |

### 2. 运行加载器

```bash
# Linux
cargo run -p calculator_loader -- ./target/release/libcalculator_plugin.so add 10 5

# macOS
cargo run -p calculator_loader -- ./target/release/libcalculator_plugin.dylib multiply 3 7

# Windows
cargo run -p calculator_loader -- .\target\release\calculator_plugin.dll divide 9 3
```

### 支持的操作

| 操作       | 说明       |
|------------|------------|
| `add`      | 加法 a + b |
| `subtract` | 减法 a - b |
| `multiply` | 乘法 a × b |
| `divide`   | 除法 a ÷ b |

### 示例输出

```
已加载插件: BasicCalculator
10 add 5 = 15
```

## 扩展插件

只需新建一个 `cdylib` 类型的 crate，导出相同的 C ABI 函数，加载器无需修改即可加载新插件。

## 依赖

- [`libloading`](https://crates.io/crates/libloading) 0.8 — 跨平台动态库加载
