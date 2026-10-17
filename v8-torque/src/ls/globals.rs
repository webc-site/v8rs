// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/globals.h 生成; 修订就地改, 复跑覆盖
// LS 全局: Logger (上下文单例 → 结构体)

// 原版 Logger 上下文单例 (VSCode 下 stdout 被协议占用, 诊断写文件) → 结构体;
// TorqueFileList 上下文 → 编译驱动字段 (登记偏差)。
/// 原版 Logger (ContextualClass → 结构体)
#[derive(Debug, Default)]
pub struct Logger {
  enabled: bool,
  /// 原版 std::ofstream → 缓冲 + 路径 (惰性打开)
  logfile_path: Option<String>,
  buffer: String,
}

impl Logger {
  /// 原版 Enable
  pub fn enable(&mut self, path: String) {
    self.enabled = true;
    self.logfile_path = Some(path);
  }

  #[must_use]
  pub const fn enabled(&self) -> bool {
    self.enabled
  }

  /// 原版 Log → 文件缓冲
  pub fn log(&mut self, line: &str) {
    if !self.enabled {
      return;
    }
    // TODO(填体): 接 log crate (dev 依赖, 运行时日志走文件缓冲)
    self.buffer.push_str(line);
    self.buffer.push('\n');
  }

  /// 原版 Flush
  pub fn flush(&mut self) {
    // TODO(填体): 落盘 logfile_path
  }
}

/// 原版 TorqueFileList 上下文 → 显式传递
pub type TorqueFileList = Vec<String>;
