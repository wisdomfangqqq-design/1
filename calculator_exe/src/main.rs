//! 计算器独立可执行程序
//!
//! 用法：calculator_exe <操作> <数字A> <数字B>
//!
//! 示例：
//!   calculator_exe add 10 5      # 输出: 15
//!   calculator_exe divide 9 0    # 输出: NaN
//!
//! 支持的操作：add | subtract | multiply | divide

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        eprintln!("用法: {} <操作> <数字A> <数字B>", args[0]);
        eprintln!("操作: add | subtract | multiply | divide");
        std::process::exit(1);
    }

    let operation = &args[1];
    let a: f64 = args[2].parse().unwrap_or_else(|_| {
        eprintln!("数字A 解析失败: '{}'", args[2]);
        std::process::exit(1);
    });
    let b: f64 = args[3].parse().unwrap_or_else(|_| {
        eprintln!("数字B 解析失败: '{}'", args[3]);
        std::process::exit(1);
    });

    let result = match operation.as_str() {
        "add"      => a + b,
        "subtract" => a - b,
        "multiply" => a * b,
        "divide"   => {
            if b == 0.0 {
                f64::NAN
            } else {
                a / b
            }
        }
        other => {
            eprintln!("未知操作: '{}'，支持: add | subtract | multiply | divide", other);
            std::process::exit(1);
        }
    };

    if result.is_nan() {
        println!("NaN");
    } else {
        println!("{}", result);
    }
}
