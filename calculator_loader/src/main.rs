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

/// 根据文件路径判断应使用 exe 加载方式还是动态库加载方式
fn is_exe_path(path: &str) -> bool {
    let p = Path::new(path);
    match p.extension().and_then(|e| e.to_str()) {
        // 明确的动态库扩展名 → 动态库模式
        Some("dll") | Some("so") | Some("dylib") => false,
        // .exe 扩展名 → exe 模式
        Some("exe") => true,
        // 无扩展名：在 Unix 系统上检查可执行权限，否则默认 exe 模式
        _ => {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Ok(meta) = std::fs::metadata(p) {
                    return meta.permissions().mode() & 0o111 != 0;
                }
            }
            true
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
    let result = if is_exe_path(path) {
        let calc = ExeCalculator::new(path);
        println!("已加载可执行程序: {}", calc.name());
        match operation.as_str() {
            "add"      => calc.add(a, b),
            "subtract" => calc.subtract(a, b),
            "multiply" => calc.multiply(a, b),
            "divide"   => calc.divide(a, b),
            other => {
                eprintln!("未知操作: '{}'，支持: add | subtract | multiply | divide", other);
                std::process::exit(1);
            }
        }
    } else {
        let loader = CalculatorLoader::new(path)
            .expect("加载动态库失败，请检查路径和文件是否存在");
        println!("已加载插件: {}", loader.name());
        match operation.as_str() {
            "add"      => loader.add(a, b),
            "subtract" => loader.subtract(a, b),
            "multiply" => loader.multiply(a, b),
            "divide"   => loader.divide(a, b),
            other => {
                eprintln!("未知操作: '{}'，支持: add | subtract | multiply | divide", other);
                std::process::exit(1);
            }
        }
    };

    if result.is_nan() {
        println!("{} {} {} = 错误: 除数不能为零", a, operation, b);
    } else {
        println!("{} {} {} = {}", a, operation, b, result);
    }
}
