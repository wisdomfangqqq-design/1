//! 计算器加载器
//!
//! 用法：
//!   cargo build --release -p calculator_plugin   # 先编译插件
//!   cargo run -p calculator_loader -- <插件路径> <操作> <a> <b>
//!
//! 示例（Linux）：
//!   cargo run -p calculator_loader -- ./target/release/libcalculator_plugin.so add 10 5
//!
//! 支持的操作：add | subtract | multiply | divide

mod loader;

use std::env;
use loader::CalculatorLoader;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        eprintln!("用法: {} <插件路径> <操作> <数字A> <数字B>", args[0]);
        eprintln!("操作: add | subtract | multiply | divide");
        eprintln!();
        eprintln!("示例 (Linux):");
        eprintln!(
            "  {} ./target/release/libcalculator_plugin.so add 10 5",
            args[0]
        );
        std::process::exit(1);
    }

    let plugin_path = &args[1];
    let operation   = &args[2];
    let a: f64      = args[3].parse().expect("数字A 解析失败");
    let b: f64      = args[4].parse().expect("数字B 解析失败");

    // 加载插件
    let loader = CalculatorLoader::new(plugin_path)
        .expect("加载插件失败，请检查路径和文件是否存在");

    println!("已加载插件: {}", loader.name());

    // 执行计算
    let result = match operation.as_str() {
        "add"      => loader.add(a, b),
        "subtract" => loader.subtract(a, b),
        "multiply" => loader.multiply(a, b),
        "divide"   => loader.divide(a, b),
        other => {
            eprintln!("未知操作: '{}'，支持: add | subtract | multiply | divide", other);
            std::process::exit(1);
        }
    };

    if result.is_nan() {
        println!("{} {} {} = 错误: 除数不能为零", a, operation, b);
    } else {
        println!("{} {} {} = {}", a, operation, b, result);
    }
}
