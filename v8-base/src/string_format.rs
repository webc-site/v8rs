//! 原版 v8/src/base/string-format.h — 静态格式化容器 + JSON 转义
//!
//! C++ 端 `FormattedString<Ts...>` 的核心动机: 在**不动态分配**、**类型/数量编译期已知**
//! 的场合 (OOM/FATAL) 用 `SNPrintF` 写入栈上的 `std::array<char, kMaxLen>`.
//!
//! Rust 侧承接者是 `core::fmt` + `format_args!`, 无需重复实现格式化引擎:
//! - 编译期已知参数由 `format_args!` 天然生成 `Arguments<'_>`, 类型安全;
//! - 零分配写入由 [`StackWriter`] 提供 (栈 [u8; N], 达到容量即视为截断, 与原版 CHECK
//!   `characters >= 0 && characters < kMaxLen` 一致);
//! - `JSONEscaped` 逐字节对齐原版 `PipeCharacter`.

use core::fmt::{self, Arguments, Display, Formatter, Write as _};

/// 栈上无堆分配写入器; 对应原版 `std::array<char, kMaxLen> message` + `SNPrintF`.
///
/// `buf` 始终以 `\0` 结尾, `written` 为已写入字节数 (不含终止符).
pub struct StackWriter<const N: usize> {
  buf: [u8; N],
  written: usize,
  /// 溢出后置 true, 与 V8 SNPrintF 返回 -1 语义一致.
  truncated: bool,
}

impl<const N: usize> StackWriter<N> {
  #[must_use]
  pub const fn new() -> Self {
    Self {
      buf: [0; N],
      written: 0,
      truncated: false,
    }
  }

  /// 渲染 `args` 到栈缓冲, Ok(written) 或 Err(截断)
  pub fn write_args(&mut self, args: Arguments<'_>) -> Result<usize, Truncated> {
    self.truncated = false;
    if fmt::Write::write_fmt(self, args).is_err() {
      self.truncated = true;
      return Err(Truncated);
    }
    Ok(self.written)
  }

  /// 已写入内容 (不含终止符).
  #[must_use]
  pub fn as_bytes(&self) -> &[u8] {
    &self.buf[..self.written]
  }

  /// 触发截断标记 (供 `fmt::Write` 内部使用).
  #[must_use]
  pub const fn is_truncated(&self) -> bool {
    self.truncated
  }
}

impl<const N: usize> Default for StackWriter<N> {
  fn default() -> Self {
    Self::new()
  }
}

impl<const N: usize> fmt::Write for StackWriter<N> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    let bytes = s.as_bytes();
    if self.written + bytes.len() >= N {
      // 与原版 CHECK 语义一致: 一旦放不下即视为失败.
      return Err(fmt::Error);
    }
    self.buf[self.written..self.written + bytes.len()].copy_from_slice(bytes);
    self.written += bytes.len();
    self.buf[self.written] = 0;
    Ok(())
  }
}

/// 对应 `FormattedString{}` 编译期零分配写入 `PrintToArray()`.
///
/// 用法: `formatted_to_array::<N>(format_args!("... ", a, b)) -> Option<&[u8; N]>`
pub fn formatted_to_array<const N: usize>(args: Arguments<'_>) -> Option<[u8; N]> {
  let mut w = StackWriter::<N>::new();
  if w.write_args(args).is_err() {
    return None;
  }
  let mut out = [0u8; N];
  out[..w.written].copy_from_slice(&w.buf[..w.written]);
  Some(out)
}

/// 原版 `JSONEscaped` — 在输出流上转义 `" \ \b \f \n \r \t` 六类字符.
///
/// C++ 侧构造接受 `ostringstream` / 字符串; Rust 侧只保留 `String` 版本 (无 ostringstream
/// 概念, 上层用 `format!`/`format_args!` 生成即可).
pub struct JSONEscaped {
  s: String,
}

/// 栈缓冲渲染的截断错误 (原版无返回值, Rust 以 Result 承接)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Truncated;

impl JSONEscaped {
  #[must_use]
  pub fn new(s: String) -> Self {
    Self { s }
  }

  /// 从 `Display` 值构造 (对应原版模板 `explicit JSONEscaped(const T& value)` 的 ostringstream 分支).
  #[must_use]
  pub fn from_display<T: Display>(v: &T) -> Self {
    let mut buf = String::new();
    // 写入 String 不会失败
    let _ = write!(buf, "{v}");
    Self { s: buf }
  }

  #[must_use]
  pub fn as_str(&self) -> &str {
    &self.s
  }

  /// 原版 `PipeCharacter` — 逐字符转义.
  fn pipe(f: &mut Formatter<'_>, c: char) -> fmt::Result {
    match c {
      '"' => f.write_str("\\\""),
      '\\' => f.write_str("\\\\"),
      '\u{08}' => f.write_str("\\b"),
      '\u{0c}' => f.write_str("\\f"),
      '\n' => f.write_str("\\n"),
      '\r' => f.write_str("\\r"),
      '\t' => f.write_str("\\t"),
      other => f.write_str(other.encode_utf8(&mut [0u8; 4])),
    }
  }
}

impl Display for JSONEscaped {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    for c in self.s.chars() {
      Self::pipe(f, c)?;
    }
    Ok(())
  }
}
