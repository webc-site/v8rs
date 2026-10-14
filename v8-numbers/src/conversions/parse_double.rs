//! 原版 conversions.cc 842-991 — StringToDouble / parseFloat 整族
//!
//! 原版十进制字面量解析内核为 third_party/fast_float (general|no_infnan|
//! allow_leading_plus); 本 crate 以 自写扫描器 + v8-base strtod (正确舍入,
//! bignum 兜底) 等价复刻: 两者对任意十进制输入产生同一 round-to-nearest-even
//! 结果, 且不在解析层吞入 inf/nan 字面量 (no_infnan 语义), 与
//! InternalStringToIntDouble 的 2^n 前缀路径与 ±Infinity 分支组合出
//! ECMA-262 9.3.1 (ToNumber 应用于 String) 的完整语义。

use v8_base::numbers::strtod::strtod;

use super::{
  parse_int::{
    ConversionFlag, NumChar, advance_to_nonspace, internal_string_to_int_double, sub_string_equals,
  },
  simple_builder::junk_string_value,
};

/// 指数字面量饱和阈值 — 超过即离开任何 double 的表示域, 结果必为 inf/0
const EXPONENT_SATURATION: i64 = 100_000;
/// strtod 交给内核前对十分指数的夹取 (double 域 ±324, 余量充分)
const DECIMAL_EXPONENT_CLAMP: i32 = 500;

/// 十进制字面量扫描结果: (值, 消费位数); 消费 0 表示未解析出数字
struct DecimalParse(f64, usize);

/// StrDecimalLiteral 扫描器 — fast_float general 语法子集
/// [+-]? digits ('.' digits*)? ([eE] [+-]? digits+)?; 指数不完整时停在 e 前
fn parse_decimal_literal<C: NumChar>(s: &[C], from: usize) -> DecimalParse {
  let end = s.len();
  let mut i = from;
  if i >= end {
    return DecimalParse(f64::NAN, from);
  }

  // 符号
  let mut negative = false;
  let c0 = s[i].code();
  if c0 == u32::from(b'+') || c0 == u32::from(b'-') {
    negative = c0 == u32::from(b'-');
    i += 1;
    if i == end {
      return DecimalParse(f64::NAN, from);
    }
  }

  // 整数段
  let int_start = i;
  while i < end && (u32::from(b'0')..=u32::from(b'9')).contains(&s[i].code()) {
    i += 1;
  }
  let int_end = i;

  // 小数点与小数段
  let mut frac_start = i;
  let mut frac_end = i;
  if i < end && s[i].code() == u32::from(b'.') {
    i += 1;
    frac_start = i;
    while i < end && (u32::from(b'0')..=u32::from(b'9')).contains(&s[i].code()) {
      i += 1;
    }
    frac_end = i;
  }

  // 至少一位数字
  if int_start == int_end && frac_start == frac_end {
    return DecimalParse(f64::NAN, from);
  }

  // 指数段: 不完整 (e 后无有效数字) 则整体不消费
  let mut exp: i64 = 0;
  if i < end {
    let e = s[i].code();
    if e == u32::from(b'e') || e == u32::from(b'E') {
      let mut j = i + 1;
      let mut exp_negative = false;
      if j < end {
        let sign = s[j].code();
        if sign == u32::from(b'+') || sign == u32::from(b'-') {
          exp_negative = sign == u32::from(b'-');
          j += 1;
        }
      }
      let digits_start = j;
      let mut value: i64 = 0;
      let mut saturated = false;
      while j < end && (u32::from(b'0')..=u32::from(b'9')).contains(&s[j].code()) {
        if !saturated {
          value = value * 10 + i64::from(s[j].code() - u32::from(b'0'));
          if value > EXPONENT_SATURATION {
            saturated = true; // 保持饱和, 不再累计
          }
        }
        j += 1;
      }
      if j > digits_start {
        // 完整指数: 消费
        exp = if saturated {
          if exp_negative {
            -EXPONENT_SATURATION
          } else {
            EXPONENT_SATURATION
          }
        } else if exp_negative {
          -value
        } else {
          value
        };
        i = j;
      }
    }
  }

  // 组装规范数字串 (去点): digits × 10^(exp - 小数位数)
  let int_digits = &s[int_start..int_end];
  let frac_digits = &s[frac_start..frac_end];
  let signed = |v: f64| if negative { -v } else { v };

  // 全零 (含巨指数) 走 strtod 同样得 ±0, 但提前短路省一次拷贝
  if int_digits
    .iter()
    .chain(frac_digits.iter())
    .all(|&c| c.code() == u32::from(b'0'))
  {
    return DecimalParse(signed(0.0), i);
  }

  // 剥离前导零与尾零 (全零已在前文短路):
  // 尾数的真实有效位数不能计入前导零 (否则 "00...001" 会虚增位数推高数量级,
  // 在未进入 strtod 前即误判为 +inf)
  let total_digits = int_digits.len() + frac_digits.len();
  let leading_zeros = int_digits
    .iter()
    .chain(frac_digits.iter())
    .take_while(|&&c| c.code() == u32::from(b'0'))
    .count();
  let trailing_zeros = frac_digits
    .iter()
    .rev()
    .chain(int_digits.iter().rev())
    .take_while(|&&c| c.code() == u32::from(b'0'))
    .count();
  let sig_len = total_digits - leading_zeros - trailing_zeros;
  debug_assert!(sig_len > 0);

  // 去尾零后等价指数: exp - 小数位 + 削去的尾零数
  let dec_exp = exp - frac_digits.len() as i64 + trailing_zeros as i64;
  // 真实数量级 (首位权重): dec_exp + sig_len
  let magnitude = dec_exp + sig_len as i64;
  if magnitude > DECIMAL_EXPONENT_CLAMP as i64 {
    return DecimalParse(signed(f64::INFINITY), i);
  }
  if magnitude < -(DECIMAL_EXPONENT_CLAMP as i64) {
    return DecimalParse(signed(0.0), i);
  }

  // 常规长度走栈缓冲, 极端长字面量 (>800 位) 落堆;
  // 不截断有效尾数 — strtod 内部按 780 位上限裁剪并做 tie 修正, 截断反而丢舍入信息
  let mut stack_buf = [0u8; 800];
  let mut heap_buf: Vec<u8> = Vec::new();
  let digits: &mut [u8] = if sig_len <= stack_buf.len() {
    &mut stack_buf[..sig_len]
  } else {
    heap_buf.resize(sig_len, 0);
    &mut heap_buf[..]
  };
  for (out, &c) in digits.iter_mut().zip(
    int_digits
      .iter()
      .chain(frac_digits.iter())
      .skip(leading_zeros),
  ) {
    *out = c.code() as u8;
  }
  let safe_exp = i32::try_from(dec_exp).unwrap_or(if dec_exp < 0 { i32::MIN } else { i32::MAX });
  let v = strtod(digits, safe_exp);
  DecimalParse(signed(v), i)
}

/// InternalStringToDouble — ECMA-262 9.3.1 字符串→double 主入口
/// (conversions.cc 842-944; 原版 Handle/Tagged 入口的 raw 等价)
fn internal_string_to_double<C: NumChar>(
  s: &[C],
  flag: ConversionFlag,
  empty_string_val: f64,
) -> f64 {
  let end = s.len();
  let mut current = 0usize;

  if !advance_to_nonspace(&mut current, end, s) {
    return empty_string_val;
  }

  // 非十进制前缀必须紧跟空白之后
  if flag == ConversionFlag::AllowNonDecimalPrefix {
    let mut prefixed = current;
    if s[prefixed].code() == u32::from(b'0') {
      prefixed += 1;
      if prefixed == end {
        return 0.0;
      }
      let c = s[prefixed].code();
      if c == u32::from(b'x') || c == u32::from(b'X') {
        prefixed += 1;
        if prefixed == end {
          return junk_string_value(); // "0x"
        }
        return internal_string_to_int_double::<4, _>(s, prefixed, end, false, false);
      }
      if c == u32::from(b'o') || c == u32::from(b'O') {
        prefixed += 1;
        if prefixed == end {
          return junk_string_value(); // "0o"
        }
        return internal_string_to_int_double::<3, _>(s, prefixed, end, false, false);
      }
      if c == u32::from(b'b') || c == u32::from(b'B') {
        prefixed += 1;
        if prefixed == end {
          return junk_string_value(); // "0b"
        }
        return internal_string_to_int_double::<1, _>(s, prefixed, end, false, false);
      }
    }
  }

  // StrDecimalLiteral
  let allow_trailing_junk = flag == ConversionFlag::AllowTrailingJunk;

  let DecimalParse(value, consumed) = parse_decimal_literal(s, current);
  if consumed == end {
    return value;
  }
  if consumed > current {
    current = consumed;
    if !allow_trailing_junk && advance_to_nonspace(&mut current, end, s) {
      return junk_string_value();
    }
    return value;
  }

  // 未解析出数字 → 处理 ±Infinity
  debug_assert_eq!(consumed, current);
  debug_assert_ne!(current, end);
  const INFINITY_STRING: &[u8] = b"Infinity";
  match s[current].code() {
    c if c == u32::from(b'+') => {
      // 前导正号后必须是 Infinity
      current += 1;
      if current == end || s[current].code() != u32::from(INFINITY_STRING[0]) {
        return junk_string_value();
      }
      if !sub_string_equals(&mut current, end, s, INFINITY_STRING) {
        return junk_string_value();
      }
      if !allow_trailing_junk && advance_to_nonspace(&mut current, end, s) {
        return junk_string_value();
      }
      f64::INFINITY
    }
    c if c == u32::from(INFINITY_STRING[0]) => {
      if !sub_string_equals(&mut current, end, s, INFINITY_STRING) {
        return junk_string_value();
      }
      if !allow_trailing_junk && advance_to_nonspace(&mut current, end, s) {
        return junk_string_value();
      }
      f64::INFINITY
    }
    c if c == u32::from(b'-') => {
      current += 1;
      if current == end || s[current].code() != u32::from(INFINITY_STRING[0]) {
        return junk_string_value();
      }
      if !sub_string_equals(&mut current, end, s, INFINITY_STRING) {
        return junk_string_value();
      }
      if !allow_trailing_junk && advance_to_nonspace(&mut current, end, s) {
        return junk_string_value();
      }
      f64::NEG_INFINITY
    }
    _ => junk_string_value(),
  }
}

/// StringToDouble — 单/双字节串 → double (ECMA-262 9.3.1)
/// (原版三个重载的 raw 等价: Vector<const uint8_t>/Vector<const uc16>/const char*)
#[must_use]
pub fn string_to_double<C: NumChar>(s: &[C], flag: ConversionFlag, empty_string_val: f64) -> f64 {
  internal_string_to_double(s, flag, empty_string_val)
}

/// BinaryStringToDouble — "0b[01]*" → double (tc39 numericvalue)
#[must_use]
pub fn binary_string_to_double(s: &[u8]) -> f64 {
  if s.len() <= 2 {
    return junk_string_value();
  }
  debug_assert_eq!(s[0], b'0');
  debug_assert_eq!(s[1].to_ascii_lowercase(), b'b');
  internal_string_to_int_double::<1, _>(s, 2, s.len(), false, false)
}

/// OctalStringToDouble — "0o[0-7]*" → double
#[must_use]
pub fn octal_string_to_double(s: &[u8]) -> f64 {
  if s.len() <= 2 {
    return junk_string_value();
  }
  debug_assert_eq!(s[0], b'0');
  debug_assert_eq!(s[1].to_ascii_lowercase(), b'o');
  internal_string_to_int_double::<3, _>(s, 2, s.len(), false, false)
}

/// HexStringToDouble — "0x[0-9a-f]*" → double
#[must_use]
pub fn hex_string_to_double(s: &[u8]) -> f64 {
  if s.len() <= 2 {
    return junk_string_value();
  }
  debug_assert_eq!(s[0], b'0');
  debug_assert_eq!(s[1].to_ascii_lowercase(), b'x');
  internal_string_to_int_double::<4, _>(s, 2, s.len(), false, false)
}

/// ImplicitOctalStringToDouble — LegacyOctalIntegerLiteral "0[0-7]*" → double
#[must_use]
pub fn implicit_octal_string_to_double(s: &[u8]) -> f64 {
  if s.is_empty() {
    return junk_string_value();
  }
  internal_string_to_int_double::<3, _>(s, 0, s.len(), false, false)
}
