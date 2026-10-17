//! v8-base/src/strings.rs 集成测试
//!
//! 原版对应: v8/test/unittests/base/strings-unittest.cc (本仓库不存在), 与
//! cctest/test-strings.cc 的 sprintf/HexValue 相关部分; 覆盖 HexValue/HexCharOfValue/
//! vsn_printf/SNPrintF 截断与缓冲区上限/StrNCpy 语义.
//!
//! 规约先行:
//! - HexValue('0'..'9') → 0..9, HexValue('a'..'f'/'A'..'F') → 10..15, 非法 → -1
//! - HexCharOfValue(0..15) → '0'..'9','A'..'F'
//! - vsn_printf 截断判定: 写入长度 >= 缓冲区容量 → -1 且末尾 NUL
//! - vsn_printf 正常路径: 返回写入字符数, str[total] == 0
//! - StrNCpy: strncpy 语义, src 提前 NUL 时剩余空间补 0

use aok::{OK, Void};
use v8_base::strings::{UC16_SIZE, hex_char_of_value, hex_value, sn_printf, str_ncpy, vsn_printf};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 `HexValue` 边界逐条: 数字/大小写字母/非法字符
#[test]
fn base_strings_hex_value() -> Void {
  for i in 0..=9u32 {
    assert_eq!(hex_value(b'0' as u32 + i), i as i32);
  }
  for i in 0..6u32 {
    assert_eq!(hex_value(b'a' as u32 + i), 10 + i as i32);
    assert_eq!(hex_value(b'A' as u32 + i), 10 + i as i32);
  }
  // 非法: 空格 / 冒号 / 反斜杠 / g
  assert!(hex_value(b' ' as u32) < 0);
  assert!(hex_value(b':' as u32) < 0);
  assert!(hex_value(b'g' as u32) < 0);
  assert!(hex_value(b'G' as u32) < 0);
  OK
}

/// 原版 `HexCharOfValue` 输出表
#[test]
fn base_strings_hex_char_of_value() -> Void {
  let expect = "0123456789ABCDEF";
  for (i, c) in expect.chars().enumerate() {
    assert_eq!(hex_char_of_value(i as i32), c);
  }
  OK
}

/// UC16_SIZE = 2 (kUC16Size)
#[test]
fn base_strings_uc16_size() -> Void {
  assert_eq!(UC16_SIZE, 2);
  OK
}

/// `VSNPrintF`: 原版正常路径, 返回写入字符数, 缓冲区以 NUL 结尾.
#[test]
fn base_strings_vsn_printf_normal() -> Void {
  let mut buf = [0u8; 32];
  let n = vsn_printf(&mut buf, format_args!("hi {}", 42));
  assert_eq!(n, 5);
  assert_eq!(&buf[..5], b"hi 42");
  assert_eq!(buf[5], 0);
  OK
}

/// `VSNPrintF`: 截断路径 — 应写长度 `>= str.len()` 时返回 -1, 缓冲区末位仍为 NUL.
#[test]
fn base_strings_vsn_printf_truncated() -> Void {
  let mut buf = [0u8; 4];
  // "hello" 长度 5 >= 4 → 截断
  let n = vsn_printf(&mut buf, format_args!("hello"));
  assert_eq!(n, -1);
  assert_eq!(buf[3], 0);
  // 前 min(total, len-1) = 3 字节应为 "hel"
  assert_eq!(&buf[..3], b"hel");
  OK
}

/// `VSNPrintF`: 恰好等于容量 (无终止符位置) 也算截断, 与 platform-posix.cc:1107 `n >= length` 一致.
#[test]
fn base_strings_vsn_printf_exact_fit_is_truncated() -> Void {
  let mut buf = [0u8; 3];
  // "abc" 长 3, 需要 4 字节容纳 NUL, 因此视为截断
  let n = vsn_printf(&mut buf, format_args!("abc"));
  assert_eq!(n, -1);
  assert_eq!(buf[2], 0);
  OK
}

/// `VSNPrintF`: 空缓冲区边界 — 无法写入, 但函数仍应返回 -1 (不越界).
#[test]
fn base_strings_vsn_printf_empty_buffer() -> Void {
  let mut buf: [u8; 0] = [];
  let n = vsn_printf(&mut buf, format_args!("x"));
  assert_eq!(n, -1);
  OK
}

/// `SNPrintF`: 与 `VSNPrintF` 完全同实现, 保留名字对齐原版调用点.
#[test]
fn base_strings_sn_printf_wraps_vsn() -> Void {
  let mut buf = [0u8; 16];
  let n = sn_printf(&mut buf, format_args!("{}+{}", 1, 2));
  assert_eq!(n, 3);
  assert_eq!(&buf[..3], b"1+2");
  assert_eq!(buf[3], 0);
  OK
}

/// 原版 `StrNCpy`: strncpy 语义 — src 短于 n 时补 0, src 长于 n 时截断
#[test]
fn base_strings_str_ncpy_semantics() -> Void {
  // src 提前终止 (NUL 在索引 3): 剩余补 0
  let mut dst = *b"XXXXXXX";
  str_ncpy(&mut dst, b"abc\0ZZ", 7);
  assert_eq!(&dst, b"abc\0\0\0\0");

  // 无终止 NUL: 完全按 n 拷贝, 不追加终止 (原版行为)
  let mut dst2 = *b"YYY";
  str_ncpy(&mut dst2, b"abcdef", 3);
  assert_eq!(&dst2, b"abc");

  // n = 0: 空操作
  let mut dst3 = *b"keep";
  str_ncpy(&mut dst3, b"src", 0);
  assert_eq!(&dst3, b"keep");
  OK
}
