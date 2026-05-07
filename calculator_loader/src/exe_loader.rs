//! ExeCalculator —— 通过子进程调用本地可执行文件的计算器

use std::path::{Path, PathBuf};
use std::process::Command;

/// 通过启动本地可执行程序来执行计算的加载器
pub struct ExeCalculator {
    path: PathBuf,
}

impl ExeCalculator {
    /// 从指定路径创建 ExeCalculator
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
        }
    }

    /// 返回可执行文件名称（不含扩展名）
    pub fn name(&self) -> &str {
        self.path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("<unknown>")
    }

    /// 启动子进程执行计算，返回结果
    fn call(&self, op: &str, a: f64, b: f64) -> Result<f64, String> {
        let output = Command::new(&self.path)
            .arg(op)
            .arg(a.to_string())
            .arg(b.to_string())
            .output()
            .map_err(|e| format!("启动子进程失败: {}", e))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("子进程退出异常: {}", stderr.trim()));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let trimmed = stdout.trim();

        if trimmed == "NaN" {
            return Ok(f64::NAN);
        }

        trimmed
            .parse::<f64>()
            .map_err(|e| format!("输出解析失败: '{}' — {}", trimmed, e))
    }

    /// 加法：a + b
    pub fn add(&self, a: f64, b: f64) -> f64 {
        self.call("add", a, b).unwrap_or_else(|e| {
            eprintln!("计算失败: {}", e);
            f64::NAN
        })
    }

    /// 减法：a - b
    pub fn subtract(&self, a: f64, b: f64) -> f64 {
        self.call("subtract", a, b).unwrap_or_else(|e| {
            eprintln!("计算失败: {}", e);
            f64::NAN
        })
    }

    /// 乘法：a * b
    pub fn multiply(&self, a: f64, b: f64) -> f64 {
        self.call("multiply", a, b).unwrap_or_else(|e| {
            eprintln!("计算失败: {}", e);
            f64::NAN
        })
    }

    /// 除法：a / b（b=0 时子进程返回 NaN）
    pub fn divide(&self, a: f64, b: f64) -> f64 {
        self.call("divide", a, b).unwrap_or_else(|e| {
            eprintln!("计算失败: {}", e);
            f64::NAN
        })
    }
}
