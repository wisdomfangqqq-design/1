//! CalculatorLoader —— 动态库加载器核心模块

use libloading::Library;
use std::ffi::CStr;
use std::path::Path;

/// 函数类型别名
type CalcFn = unsafe extern "C" fn(f64, f64) -> f64;
type NameFn = unsafe extern "C" fn() -> *const std::ffi::c_char;

/// 计算器加载器
///
/// 持有已加载的动态库句柄，并提供对插件导出函数的安全封装。
pub struct CalculatorLoader {
    /// 保持库句柄存活（drop 时自动卸载）
    _lib: Library,
    add_fn:      CalcFn,
    subtract_fn: CalcFn,
    multiply_fn: CalcFn,
    divide_fn:   CalcFn,
    name_fn:     NameFn,
}

impl CalculatorLoader {
    /// 从指定路径加载计算器插件动态库
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, libloading::Error> {
        // SAFETY: 我们信任由本项目编译出的插件符合 ABI 约定
        unsafe {
            let lib = Library::new(path.as_ref())?;

            let add_fn:      CalcFn = *lib.get::<CalcFn>(b"add\0")?;
            let subtract_fn: CalcFn = *lib.get::<CalcFn>(b"subtract\0")?;
            let multiply_fn: CalcFn = *lib.get::<CalcFn>(b"multiply\0")?;
            let divide_fn:   CalcFn = *lib.get::<CalcFn>(b"divide\0")?;
            let name_fn:     NameFn = *lib.get::<NameFn>(b"plugin_name\0")?;

            Ok(Self {
                _lib: lib,
                add_fn,
                subtract_fn,
                multiply_fn,
                divide_fn,
                name_fn,
            })
        }
    }

    /// 返回插件名称
    pub fn name(&self) -> &str {
        unsafe {
            let ptr = (self.name_fn)();
            CStr::from_ptr(ptr).to_str().unwrap_or("<unknown>")
        }
    }

    /// 加法：a + b
    pub fn add(&self, a: f64, b: f64) -> f64 {
        unsafe { (self.add_fn)(a, b) }
    }

    /// 减法：a - b
    pub fn subtract(&self, a: f64, b: f64) -> f64 {
        unsafe { (self.subtract_fn)(a, b) }
    }

    /// 乘法：a * b
    pub fn multiply(&self, a: f64, b: f64) -> f64 {
        unsafe { (self.multiply_fn)(a, b) }
    }

    /// 除法：a / b（b=0 返回 NaN）
    pub fn divide(&self, a: f64, b: f64) -> f64 {
        unsafe { (self.divide_fn)(a, b) }
    }
}
