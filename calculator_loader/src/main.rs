//! 计算器加载器
//!
//! 用法：
//!   cargo run -p calculator_loader -- <路径> <操作> <a> <b>
//!
//! 支持两种加载方式：
//!   1. 动态库（.dll / .so / .dylib）—— 使用 libloading
//!   2. 可执行文件（.exe 或 Linux/macOS 无扩展名可执行文件）—— 使用子进程
//!
//! 示例：
//!   # 加载动态库 (Linux)
//!   cargo run -p calculator_loader -- ./target/release/libcalculator_plugin.so add 10 5
//!   # 加载 exe (Windows)
//!   cargo run -p calculator_loader -- ./target/release/calculator_exe.exe add 10 5
//!   # 加载可执行文件 (Linux/macOS)
//!   cargo run -p calculator_loader -- ./target/release/calculator_exe add 10 5
//!
//! 支持的操作：add | subtract | multiply | divide

mod loader;
mod exe_loader;

use std::env;
use std::path::Path;
use loader::CalculatorLoader;
use exe_loader::ExeCalculator;

/// 统一封装两种计算器加载方式
enum AnyCalculator {
    Library(CalculatorLoader),
    Exe(ExeCalculator),
}

impl AnyCalculator {
    fn name(&self) -> &str {
        match self {
            AnyCalculator::Library(c) => c.name(),
            AnyCalculator::Exe(c)     => c.name(),
        }
    }

    fn calculate(&self, operation: &str, a: f64, b: f64) -> f64 {
        match operation {
            "add"      => self.add(a, b),
            "subtract" => self.subtract(a, b),
            "multiply" => self.multiply(a, b),
            "divide"   => self.divide(a, b),
            other => {
                eprintln!("未知操作: '{}'，支持: add | subtract | multiply | divide", other);
                std::process::exit(1);
            }
        }
    }

    fn add(&self, a: f64, b: f64) -> f64 {
        match self {
            AnyCalculator::Library(c) => c.add(a, b),
            AnyCalculator::Exe(c)     => c.add(a, b),
        }
    }

    fn subtract(&self, a: f64, b: f64) -> f64 {
        match self {
            AnyCalculator::Library(c) => c.subtract(a, b),
            AnyCalculator::Exe(c)     => c.subtract(a, b),
        }
    }

    fn multiply(&self, a: f64, b: f64) -> f64 {
        match self {
            AnyCalculator::Library(c) => c.multiply(a, b),
            AnyCalculator::Exe(c)     => c.multiply(a, b),
        }
    }

    fn divide(&self, a: f64, b: f64) -> f64 {
        match self {
            AnyCalculator::Library(c) => c.divide(a, b),
            AnyCalculator::Exe(c)     => c.divide(a, b),
        }
    }
}

/// 根据文件路径判断应使用 exe 加载方式还是动态库加载方式
fn is_exe_path(path: &str) -> bool {
    let p = Path::new(path);
    match p.extension().and_then(|e| e.to_str()) {
        // 明确的动态库扩展名 → 动态库模式
        Some("dll") | Some("so") | Some("dylib") => false,
        // .exe 扩展名 → exe 模式
        Some("exe") => true,
        // 无扩展名：在 Unix 系统上检查可执行权限，否则默认动态库模式（让后续加载报错）
        _ => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(meta) = std::fs::metadata(p) {
                    return meta.permissions().mode() & 0o111 != 0;
                }
            }
            false
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("用法: {} <路径> <操作> <数字A> <数字B>", args[0]);
        eprintln!("操作: add | subtract | multiply | divide");
        eprintln!();
        eprintln!("示例 (动态库, Linux):");
        eprintln!(
            "  {} ./target/release/libcalculator_plugin.so add 10 5",
            args[0]
        );
        eprintln!("示例 (exe, Windows):");
        eprintln!(
            "  {} ./target/release/calculator_exe.exe add 10 5",
            args[0]
        );
        std::process::exit(1);
    }

    let path      = &args[1];
    let operation = &args[2];
    let a: f64    = args[3].parse().expect("数字A 解析失败");
    let b: f64    = args[4].parse().expect("数字B 解析失败");

    // 根据路径类型选择加载方式
    let calc: AnyCalculator = if is_exe_path(path) {
        AnyCalculator::Exe(ExeCalculator::new(path))
    } else {
        let lib = CalculatorLoader::new(path)
            .expect("加载动态库失败，请检查路径和文件是否存在");
        AnyCalculator::Library(lib)
    };

    println!("已加载: {}", calc.name());
    let result = calc.calculate(operation, a, b);

    if result.is_nan() {
        println!("{} {} {} = 错误: 除数不能为零", a, operation, b);
    } else {
        println!("{} {} {} = {}", a, operation, b, result);
    }
}
