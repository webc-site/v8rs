//! Objects-Printer 可纯逻辑字符串面 (对应 objects-printer.cc 匿名命名空间 PrintDouble 与枚举 ToString)。
//!
//! 主体 HeapObject 打印面耦合 Tagged/Isolate, 推迟 (阻塞: L3 objects)。
//! 默认分支用 Rust Display 的最短往返表示, 与 C++ ostream 6 位有效数字 (%g 风格) 有意偏离, 见 readme。

use crate::objects_print_data::{
  ITER_HELPER_STATE_NAMES, ITER_ZIP_HELPER_MODE_NAMES, JsIteratorHelperState,
  JsIteratorZipHelperMode, MAX_SAFE_INTEGER_F64, MIN_SAFE_INTEGER_F64, MINUS_ZERO_STR, NAN_SUFFIX,
};

/// 对应 i::IsMinusZero: 值为 0 且符号位为负
pub fn is_minus_zero(v: f64) -> bool {
  v == 0.0 && v.is_sign_negative()
}

/// 对应 conversions-inl.h DoubleToInteger (逐分支 1:1)
pub fn double_to_integer(x: f64) -> f64 {
  if x.is_nan() || x == 0.0 {
    return 0.0;
  }
  if !x.is_finite() {
    return x;
  }
  (if x > 0.0 { x.floor() } else { x.ceil() }) + 0.0
}

/// 对应 PrintDouble: 负零 / 安全整数加 ".0" / NaN 附位模式 / 默认直出
#[must_use]
pub fn print_double(val: f64) -> String {
  if is_minus_zero(val) {
    return MINUS_ZERO_STR.to_string();
  }
  if val == double_to_integer(val) && (MIN_SAFE_INTEGER_F64..=MAX_SAFE_INTEGER_F64).contains(&val) {
    // 安全整数区间打印满精度: 9007199254740991.0 而非 9.0072e+15
    let i = val as i64;
    return format!("{i}.0");
  }
  if val.is_nan() {
    let sign = if val.is_sign_negative() { "-" } else { "" };
    return format!("{sign}nan{NAN_SUFFIX}{:x})", val.to_bits());
  }
  format!("{val}")
}

/// 对应 JSIteratorHelperStateToString (表驱动, 名字来自生成面)
pub fn iterator_helper_state_to_str(state: JsIteratorHelperState) -> &'static str {
  ITER_HELPER_STATE_NAMES[state as usize]
}

/// 对应 JSIteratorZipHelperModeToString
pub fn iterator_zip_helper_mode_to_str(mode: JsIteratorZipHelperMode) -> &'static str {
  ITER_ZIP_HELPER_MODE_NAMES[mode as usize]
}
