//! v8-base/src/string_format.rs 集成测试 — 1:1 对应 string-format-unittest.cc
//!
//! 原版 `FormattedString{}` 编译期拼接 + `PrintToArray()` 的 Rust 承接者是 `format_args!`
//! + `StackWriter<N>`, 因此逐条转写为对 `StackWriter` / `JSONEscaped` 的断言:
//! - `FormattedStringTest.Empty` — 空消息 → 输出首字节为 NUL, 长度 1
//! - `FormattedStringTest.SingleString` — 字符串 "foo" 输出为 'f','o','o','\0'
//! - `FormattedStringTest.Int` / MaxInt / MinInt — 42 / i32::MAX / i32::MIN 到缓冲
//! - `FormattedStringTest.SizeT` / MaxSizeT — usize 42 / usize::MAX
//! - `FormattedStringTest.Combination` — "Expected 11 got 42!"
//! - `FormattedStringTest.Uint32AndUint64` — "1 != 2"

use std::ffi::CStr;

use aok::{OK, Void};
use v8_base::string_format::{JSONEscaped, formatted_to_array};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn formatted_string_test_empty() -> Void {
  // 原版空 FormattedString 输出首字节 '\0', kMaxLen = 1;
  // Rust 侧: 用 N=1 的栈写入, 空 args 只落终止符.
  let arr = formatted_to_array::<1>(format_args!("")).expect("fits");
  assert_eq!(arr[0], b'\0');
  OK
}

#[test]
fn formatted_string_test_single_string() -> Void {
  let arr = formatted_to_array::<4>(format_args!("{}", "foo")).expect("fits");
  assert_eq!(&arr, b"foo\0");
  OK
}

#[test]
fn formatted_string_test_int() -> Void {
  // kMaxPrintedIntLen = 11 (int32 位数) + 1 = 12
  let arr = formatted_to_array::<12>(format_args!("{}", 42i32)).expect("fits");
  let cstr = CStr::from_bytes_until_nul(&arr).unwrap();
  assert_eq!(cstr.to_str().unwrap(), "42");
  OK
}

#[test]
fn formatted_string_test_max_int() -> Void {
  let arr = formatted_to_array::<12>(format_args!("{}", i32::MAX)).expect("fits");
  let cstr = CStr::from_bytes_until_nul(&arr).unwrap();
  assert_eq!(cstr.to_str().unwrap(), "2147483647");
  OK
}

#[test]
fn formatted_string_test_min_int() -> Void {
  // kMaxPrintedIntLen = 11 覆盖 "-2147483648" (11 字符)
  let arr = formatted_to_array::<12>(format_args!("{}", i32::MIN)).expect("fits");
  let cstr = CStr::from_bytes_until_nul(&arr).unwrap();
  assert_eq!(cstr.to_str().unwrap(), "-2147483648");
  OK
}

#[test]
fn formatted_string_test_size_t() -> Void {
  let arr = formatted_to_array::<21>(format_args!("{}", 42usize)).expect("fits");
  let cstr = CStr::from_bytes_until_nul(&arr).unwrap();
  assert_eq!(cstr.to_str().unwrap(), "42");
  OK
}

#[test]
fn formatted_string_test_max_size_t() -> Void {
  let expected = usize::MAX.to_string();
  // 64 位下最大 20 字符 + 1 NUL = 21
  let arr = formatted_to_array::<21>(format_args!("{}", usize::MAX)).expect("fits");
  let cstr = CStr::from_bytes_until_nul(&arr).unwrap();
  assert_eq!(cstr.to_str().unwrap(), expected.as_str());
  OK
}

#[test]
fn formatted_string_test_combination() -> Void {
  // 原版: "Expected " << 11 << " got " << size_t{42} << "!" → "Expected 11 got 42!"
  // 静态 kMaxLen = strlen("Expected  got !") + kMaxPrintedIntLen + kMaxPrintedSizetLen + 1
  //                  = 15 + 11 + 20 + 1 = 47
  let arr =
    formatted_to_array::<47>(format_args!("Expected {} got {}!", 11i32, 42usize)).expect("fits");
  let cstr = CStr::from_bytes_until_nul(&arr).unwrap();
  assert_eq!(cstr.to_str().unwrap(), "Expected 11 got 42!");
  OK
}

#[test]
fn formatted_string_test_uint32_and_uint64() -> Void {
  // "1" << " != " << "2" → "1 != 2"; kMaxLen = 10 + 4 + 20 + 1 = 35
  let arr = formatted_to_array::<35>(format_args!("{} != {}", 1u32, 2u64)).expect("fits");
  let cstr = CStr::from_bytes_until_nul(&arr).unwrap();
  assert_eq!(cstr.to_str().unwrap(), "1 != 2");
  OK
}

/// 截断路径: 缓冲不够 → `formatted_to_array` 返回 None, 与原版 CHECK 语义一致.
#[test]
fn formatted_string_test_overflow_returns_none() -> Void {
  // 只有 4 字节 (含 NUL), 装不下 "hello" (5 + 1 = 6)
  assert!(formatted_to_array::<4>(format_args!("hello")).is_none());
  OK
}

/// 原版 `JSONEscaped`: 6 类转义 ( `" \ \b \f \n \r \t` ), 其他字符原样.
#[test]
fn string_format_json_escaped() -> Void {
  let raw = "\"a\\b\u{08}\u{0c}\n\r\t!\"".to_string();
  let escaped = JSONEscaped::new(raw).to_string();
  assert_eq!(&escaped, "\\\"a\\\\b\\b\\f\\n\\r\\t!\\\"");
  OK
}

/// `JSONEscaped::from_display` — 对应原版模板 `explicit JSONEscaped(const T&)`.
#[test]
fn string_format_json_escaped_from_display() -> Void {
  let escaped = JSONEscaped::from_display(&42i32).to_string();
  assert_eq!(&escaped, "42");
  OK
}
