# Rust 计算器加载器

本项目演示如何用 Rust 实现一个**插件式计算器加载器**，支持两种加载方式：

1. **动态库加载**：在运行时加载 `.so` / `.dll` / `.dylib`，通过 `libloading` 调用其中的计算函数
2. **exe 子进程加载**：启动本地可执行文件（`.exe` / 无扩展名可执行文件），通过 stdout 读取计算结果

## 项目结构

```
.
├── Cargo.toml                  # Workspace 配置
├── calculator_plugin/          # 计算器插件（编译为动态库）
│   ├── Cargo.toml
│   └── src/lib.rs              # 导出 add / subtract / multiply / divide
├── calculator_exe/             # 计算器独立可执行程序
│   ├── Cargo.toml
│   └── src/main.rs             # 接受 <操作> <a> <b>，将结果打印到 stdout
└── calculator_loader/          # 加载器主程序
    ├── Cargo.toml
    └── src/
        ├── main.rs             # 入口：解析参数、自动选择加载方式
        ├── loader.rs           # CalculatorLoader：动态库加载实现
        └── exe_loader.rs       # ExeCalculator：子进程调用实现
```

## 快速开始

### 方式一：加载动态库

#### 1. 编译插件动态库

```bash
cargo build --release -p calculator_plugin
```

编译产物：

| 平台    | 文件路径 |
|---------|----------|
| Linux   | `target/release/libcalculator_plugin.so` |
| macOS   | `target/release/libcalculator_plugin.dylib` |
| Windows | `target/release/calculator_plugin.dll` |

#### 2. 运行加载器（动态库模式）

```bash
# Linux
cargo run -p calculator_loader -- ./target/release/libcalculator_plugin.so add 10 5

# macOS
cargo run -p calculator_loader -- ./target/release/libcalculator_plugin.dylib multiply 3 7

# Windows
cargo run -p calculator_loader -- .\target\release\calculator_plugin.dll divide 9 3
```

示例输出：
```
已加载插件: BasicCalculator
10 add 5 = 15
```

---

### 方式二：加载本地 exe 可执行文件

#### 1. 编译 calculator_exe

```bash
cargo build --release -p calculator_exe
```

#### 2. 运行加载器（exe 模式）

```bash
# Windows（.exe 扩展名自动识别）
cargo run -p calculator_loader -- ./target/release/calculator_exe.exe add 10 5

# Linux / macOS（无扩展名，自动检测可执行权限）
cargo run -p calculator_loader -- ./target/release/calculator_exe add 10 5
```

示例输出：
```
已加载可执行程序: calculator_exe
10 add 5 = 15
```

---

### 支持的操作

| 操作       | 说明       |
|------------|------------|
| `add`      | 加法 a + b |
| `subtract` | 减法 a - b |
| `multiply` | 乘法 a × b |
| `divide`   | 除法 a ÷ b |

除零时输出：`a divide b = 错误: 除数不能为零`

---

## 加载方式判断逻辑

加载器根据文件扩展名自动选择策略：

| 扩展名              | 加载方式       |
|---------------------|----------------|
| `.dll` / `.so` / `.dylib` | libloading 动态库 |
| `.exe`              | std::process::Command 子进程 |
| 无扩展名（Unix 可执行位已设置） | std::process::Command 子进程 |

## 依赖

- [`libloading`](https://crates.io/crates/libloading) 0.8 — 跨平台动态库加载（仅动态库模式使用）
