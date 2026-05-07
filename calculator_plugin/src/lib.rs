//! 计算器插件动态库
//! 编译后生成 .so (Linux) / .dll (Windows) / .dylib (macOS)
//! 由 calculator_loader 在运行时动态加载

use std::ffi::c_double;

/// 加法
#[no_mangle]
pub extern "C" fn add(a: c_double, b: c_double) -> c_double {
    a + b
}

/// 减法
#[no_mangle]
pub extern "C" fn subtract(a: c_double, b: c_double) -> c_double {
    a - b
}

/// 乘法
#[no_mangle]
pub extern "C" fn multiply(a: c_double, b: c_double) -> c_double {
    a * b
}

/// ��法（除数为 0 时返回 NaN）
#[no_mangle]
pub extern "C" fn divide(a: c_double, b: c_double) -> c_double {
    if b == 0.0 {
        f64::NAN
    } else {
        a / b
    }
}

/// 返回插件名称（静态字符串指针，以 null 结尾）
#[no_mangle]
pub extern "C" fn plugin_name() -> *const std::ffi::c_char {
    b"BasicCalculator\0".as_ptr() as *const std::ffi::c_char
}
