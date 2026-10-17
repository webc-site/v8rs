//! 原版 v8/src/base/strings.h/cc — Latin1/UTF-16 常量、十六进制字符、安全格式化打印
//!
//! 原版 `VSNPrintF/SNPrintF` 直接调用 `OS::VSNPrintF` → libc `vsnprintf`;
//! Rust 侧依 rust_review 规则 **不重复实现格式化引擎**: 承接者是 `core::fmt`,
//! 调用点用 `format_args!` 构造运行时 [`Arguments`], 由 `write_fmt` 输出到字节缓冲.
//!
//! 严格保留原版调用点语义 (v8/src/base/platform/platform-posix.cc:1102-1116):
//! - 缓冲区始终以 `\0` 结尾;
//! - `vsnprintf` 的返回 `n` (应写长度) `>= length` 视为截断, 返回 `-1` 并把缓冲区最后一字节置 `\0`;
//! - `n < 0` 同样返回 `-1`;
//! - 否则返回实际写入字符数 `n`.

use core::fmt::Arguments;
use std::io::Write as _;

/// Latin1/UTF-16 常量: UTF-16 码元为 16 位 (原版 `using uc16 = uint16_t`)
pub type Uc16 = u16;
/// Unicode 4.0 码点 21 位, 以 32 位承载 (原版 `using uc32 = uint32_t`)
pub type Uc32 = u32;
/// `sizeof(uc16)` (原版 `kUC16Size`)
pub const UC16_SIZE: usize = size_of::<Uc16>();

/// 十六进制字符的值 (0..15); 非法字符返回负值.
///
/// 与原版逐条对齐: 无符号回绕比较, 大小写均可.
#[must_use]
pub fn hex_value(c: Uc32) -> i32 {
  // c -= '0'; 若回绕后 <= 9 则为数字 0..9
  let wrapped = c.wrapping_sub(u32::from(b'0'));
  if wrapped <= 9 {
    return wrapped as i32;
  }
  // c = (c | 0x20) - ('a' - '0'); 检测 0x11..0x16 与 0x31..0x36 (a..f/A..F)
  let c2 = (wrapped | 0x20).wrapping_sub(u32::from(b'a' - b'0'));
  if c2 <= 5 {
    return c2 as i32 + 10;
  }
  -1
}

/// 值 (0..=16) → 大写十六进制字符 (原版 `HexCharOfValue`)
#[must_use]
pub fn hex_char_of_value(value: i32) -> char {
  debug_assert!((0..=16).contains(&value));
  if value < 10 {
    char::from(u8::try_from(value + i32::from(b'0')).unwrap_or(b'?'))
  } else {
    char::from(u8::try_from(value - 10 + i32::from(b'A')).unwrap_or(b'?'))
  }
}

/// 原版 `OS::VSNPrintF` 的 Rust 等价.
///
/// `args` 由调用点用 `format_args!(...)` 构造; 返回写入字符数, 截断或错误返回 `-1`,
/// 行为完全对齐 `platform-posix.cc` 中的 V8 包装.
pub fn vsn_printf(str: &mut [u8], args: Arguments<'_>) -> i32 {
  // 让 core::fmt 完成一次完整渲染, 得到 `vsnprintf` 语义的 n (应写长度).
  let mut buf: Vec<u8> = Vec::new();
  if buf.write_fmt(args).is_err() {
    // fmt 写入失败 (仅底层 sink 错误才发生, Vec sink 不会);
    // 与原版 n<0 同路径: 置尾字节为 \0 (platform-posix.cc:1111-1113)
    if let Some(last) = str.last_mut() {
      *last = 0;
    }
    return -1;
  }
  let total = buf.len();
  // V8 判定截断: n >= length; 另外 n 溢出 i32 时也视为失败.
  if total >= str.len() || total > i32::MAX as usize {
    let n = total.min(str.len().saturating_sub(1));
    str[..n].copy_from_slice(&buf[..n]);
    if !str.is_empty() {
      str[str.len() - 1] = 0;
    }
    return -1;
  }
  str[..total].copy_from_slice(&buf[..total]);
  str[total] = 0;
  total as i32
}

/// 原版 `OS::SNPrintF` (VSNPrintF 的免 va_list 包装).
///
/// Rust 无 va_list 概念, 与 [`vsn_printf`] 同一实现, 保留双名以对齐调用点.
#[inline]
pub fn sn_printf(str: &mut [u8], args: Arguments<'_>) -> i32 {
  vsn_printf(str, args)
}

/// 便捷宏: 直接以 `format_args!` 风格写入 `Vector<char>` 缓冲区, 返回 `i32`.
#[macro_export]
macro_rules! v8_sn_printf {
  ($buf:expr, $($arg:tt)*) => {
    $crate::strings::sn_printf($buf, ::core::format_args!($($arg)*))
  };
}

/// 原版 `base::StrNCpy` → `OS::StrNCpy` → `strncpy(dest, src, n)`.
///
/// 语义 (与 libc strncpy 一致): 至多拷贝 n 字节, 遇源终止符提前结束, 剩余空间补 0.
/// 注意: 原版忽略 dest 容量, 调用方须保证 `dest.len() >= n`.
pub fn str_ncpy(dest: &mut [u8], src: &[u8], n: usize) {
  debug_assert!(dest.len() >= n);
  // strncpy 只扫描 src 至终止符; Rust 切片无终止, 故按 src 中的首个 0 或 n 界定.
  let copy = src.iter().take_while(|&&c| c != 0).take(n).count();
  dest[..copy].copy_from_slice(&src[..copy]);
  dest[copy..n].fill(0);
}
