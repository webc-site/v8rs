//! 原版 conversions.cc 1147-1656 — Number→String 整族 (ECMAScript 格式层)
//!
//! 语义红线 (todo 10.3): Number.prototype.toString 的阈值/最短表示/指数切换
//! 格式层全部自写; 最短十进制内核用 zmij (Schubfach, 与原版 dragonbox 同为
//! shortest correctly-rounded), 定点/精度模式用 v8-base DoubleToAscii
//! (原版 base::DoubleToAscii 1:1), 整数内核用 itoa。

use std::str::from_utf8_unchecked;

use itoa::Buffer as ItoaBuffer;
use v8_base::{
  fpu::Fpu,
  numbers::{
    double::Double,
    dtoa::{DtoaMode, double_to_ascii},
  },
};
use zmij::Buffer as ZmijBuffer;

pub use super::tables::DOUBLE_TO_STRING_MIN_BUFFER_SIZE;
use super::{
  fast_d2i, is_int32_double,
  simple_builder::SimpleStringBuilder,
  tables::{DOUBLE_TO_RADIX_MAX_CHARS, MAX_FRACTION_DIGITS},
};

/// SAFETY: 本模块写入缓冲的字节恒为 ASCII 数字/符号/字母, 100% 合法 UTF-8
#[inline]
fn ascii(buf: &[u8]) -> &str {
  debug_assert!(buf.is_ascii());
  unsafe { from_utf8_unchecked(buf) }
}

/// 安全解析 ASCII 有符号整数 (科学计数法指数段)
#[inline]
fn parse_ascii_i32(bytes: &[u8]) -> i32 {
  let (neg, digits) = match bytes.first() {
    Some(b'-') => (true, &bytes[1..]),
    Some(b'+') => (false, &bytes[1..]),
    _ => (false, bytes),
  };
  let mut val: i32 = 0;
  for &b in digits {
    if b.is_ascii_digit() {
      val = val.wrapping_mul(10).wrapping_add((b - b'0') as i32);
    }
  }
  if neg { -val } else { val }
}

// ---------------------------------------------------------------------------
// zmij 内核 → (digits, dec_exp) 最短表示解析
// ---------------------------------------------------------------------------

/// zmij 最短表示 → 规范十进制对 (digits, dec_exp): v = digits × 10^dec_exp
///
/// 输出形态: [-]d[.ddd][e±XX]; 定点区间内为展开式 (尾零 + ".0" 需回收),
/// 解析后统一还原为最短无尾零数字串 — 与 dragonbox (significand, exponent)
/// 等价。
pub fn shortest_decimal(v: f64, digits: &mut [u8; 24]) -> (usize, i32, bool) {
  let mut zb = ZmijBuffer::new();
  let s = zb.format_finite(v).as_bytes();

  let negative = s[0] == b'-';
  let s = if negative { &s[1..] } else { s };

  // 科学计数: 尾数在 'e' 前
  let (mantissa, exp_part) = match s.iter().position(|&c| c == b'e') {
    Some(pos) => (&s[..pos], parse_ascii_i32(&s[pos + 1..])),
    None => (s, 0),
  };

  let dot = mantissa.iter().position(|&c| c == b'.');
  let int_len = dot.unwrap_or(mantissa.len()) as i32;
  let mut raw_buf = [0u8; 32];
  let mut raw_len = 0usize;
  for &c in mantissa {
    if c != b'.' {
      raw_buf[raw_len] = c;
      raw_len += 1;
    }
  }
  let digits_raw = &raw_buf[..raw_len];

  // 去前导零 ("0.001234" → "1234") 与尾零 (定点展开回收)
  let z = digits_raw
    .iter()
    .position(|&c| c != b'0')
    .unwrap_or(digits_raw.len());
  let trimmed = &digits_raw[z..];
  let t = trimmed.iter().rev().take_while(|&&c| c == b'0').count();
  let len = trimmed.len() - t;
  let out = &mut digits[..len];
  out.copy_from_slice(&trimmed[..len]);
  // dec_exp: 点前有效位数 - z - len_D + 指数部分
  // (zmij 的定点/科学切换仅影响输出形态, 解析后统一为最短无尾零对)
  let dec_exp = int_len - z as i32 - len as i32 + exp_part;
  (len, dec_exp, negative)
}

// ---------------------------------------------------------------------------
// DoubleToStringView — ECMA-262 9.8.1 (conversions.cc 1147-1212)
// ---------------------------------------------------------------------------

/// Number→String (ToString(number)): NaN/Infinity/整数快速路径 + 最短表示
/// 四分支定点/指数格式层; 结果写入 buffer, 返回其前缀视图
#[must_use]
pub fn double_to_string_view(v: f64, buffer: &mut [u8]) -> &str {
  if v.is_nan() {
    return "NaN";
  }
  if v.is_infinite() {
    return if v < 0.0 { "-Infinity" } else { "Infinity" };
  }
  if v == 0.0 {
    return "0";
  }
  if is_int32_double(v) {
    // 含 -0 → "0" (ES 7.1.12.1: -0 字符串化为 "0")
    return int_to_string_view(fast_d2i(v), buffer);
  }

  debug_assert!(buffer.len() >= DOUBLE_TO_STRING_MIN_BUFFER_SIZE);
  let mut decimal_rep = [0u8; 24];
  let (length, dec_exp, negative) = shortest_decimal(v, &mut decimal_rep);
  let decimal_point = length as i32 + dec_exp;

  let mut builder = SimpleStringBuilder::new(buffer);
  if negative {
    builder.add_character(b'-');
  }

  let digits = &decimal_rep[..length];
  if length as i32 <= decimal_point && decimal_point <= 21 {
    // ECMA-262 9.8.1 步 6: 纯整数
    builder.add_substring(digits);
    builder.add_padding(b'0', decimal_point as isize - length as isize);
  } else if 0 < decimal_point && decimal_point <= 21 {
    // 步 7: 带小数
    builder.add_substring(&digits[..decimal_point as usize]);
    builder.add_character(b'.');
    builder.add_substring(&digits[decimal_point as usize..]);
  } else if decimal_point <= 0 && decimal_point > -6 {
    // 步 8: 0.000xxx
    builder.add_substring(b"0.");
    builder.add_padding(b'0', -(decimal_point as isize));
    builder.add_substring(digits);
  } else {
    // 步 9+10 合并: 指数形式 d.ddde±XX
    builder.add_character(digits[0]);
    if length != 1 {
      builder.add_character(b'.');
      builder.add_substring(&digits[1..]);
    }
    builder.add_character(b'e');
    builder.add_character(if decimal_point >= 0 { b'+' } else { b'-' });
    let exponent = (decimal_point - 1).unsigned_abs();
    builder.add_exponent(exponent);
  }
  let n = builder.finish();
  ascii(&buffer[..n])
}

/// IntToStringView — int → 十进制串 (itoa 内核)
#[must_use]
pub fn int_to_string_view(n: i32, buffer: &mut [u8]) -> &str {
  let mut ib = ItoaBuffer::new();
  let s = ib.format(n);
  buffer[..s.len()].copy_from_slice(s.as_bytes());
  ascii(&buffer[..s.len()])
}

// ---------------------------------------------------------------------------
// toFixed (conversions.cc 1231-1299)
// ---------------------------------------------------------------------------

/// DoubleToFixedStringView — Number.prototype.toFixed(f) 的格式层
/// (缓冲容量 kDoubleToFixedMaxChars = 123)
#[must_use]
pub fn double_to_fixed_string_view(value: f64, f: i32, buffer: &mut [u8]) -> &str {
  const FIRST_NON_FIXED: f64 = 1e21;
  debug_assert!((0..=MAX_FRACTION_DIGITS).contains(&f));
  // 原版特殊值由 builtins 层拦截; raw API 收窄为 ECMAScript 字面输出
  if !value.is_finite() {
    return match (value.is_nan(), value.is_sign_negative()) {
      (true, _) => "NaN",
      (false, true) => "-Infinity",
      (false, false) => "Infinity",
    };
  }

  let mut negative = false;
  let mut abs_value = value;
  if value < 0.0 {
    abs_value = -value;
    negative = true;
  }

  // 整数部分超 21 位 → 回退到 toString
  if abs_value >= FIRST_NON_FIXED {
    return double_to_string_view(value, buffer);
  }

  // FIXED 模式取足精度的十进制表示
  const DECIMAL_REP_CAPACITY: usize = 123; // 21 + 100 + 1('\0' 位)
  let mut decimal_rep = [0u8; DECIMAL_REP_CAPACITY];
  let r = double_to_ascii(value, DtoaMode::Fixed, f, &mut decimal_rep);

  // 前后补零到 point + f 位
  let mut decimal_point = r.point;
  let mut zero_prefix_length: isize = 0;
  let mut zero_postfix_length: isize = 0;
  if decimal_point <= 0 {
    zero_prefix_length = (-(decimal_point as isize)) + 1;
    decimal_point = 1;
  }
  if zero_prefix_length + (r.length as isize) < decimal_point as isize + f as isize {
    zero_postfix_length =
      decimal_point as isize + f as isize - r.length as isize - zero_prefix_length;
  }

  let rep_length = (zero_prefix_length + r.length as isize + zero_postfix_length) as usize;
  debug_assert!(rep_length < DECIMAL_REP_CAPACITY);
  let mut rep_buffer = [0u8; DECIMAL_REP_CAPACITY];
  {
    let mut rep_builder = SimpleStringBuilder::new(&mut rep_buffer);
    rep_builder.add_padding(b'0', zero_prefix_length);
    rep_builder.add_substring(&decimal_rep[..r.length]);
    rep_builder.add_padding(b'0', zero_postfix_length);
    debug_assert_eq!(rep_builder.position(), rep_length);
  }

  // 拼装: [-] 整数部分 [. 小数部分]
  let mut builder = SimpleStringBuilder::new(buffer);
  if negative {
    builder.add_character(b'-');
  }
  builder.add_substring(&rep_buffer[..decimal_point as usize]);
  if f > 0 {
    builder.add_character(b'.');
    builder.add_substring(&rep_buffer[decimal_point as usize..decimal_point as usize + f as usize]);
  }
  let n = builder.finish();
  ascii(&buffer[..n])
}

// ---------------------------------------------------------------------------
// toExponential (conversions.cc 1301-1368)
// ---------------------------------------------------------------------------

/// CreateExponentialRepresentation — d[.ddd]e±XX 组装
/// (conversions.cc 1301-1326, 原版 static)
#[must_use]
pub fn create_exponential_representation<'a>(
  decimal_rep: &[u8],
  rep_length: usize,
  exponent: i32,
  negative: bool,
  significant_digits: usize,
  buffer: &'a mut [u8],
) -> &'a str {
  let negative_exponent = exponent < 0;
  let exponent = exponent.unsigned_abs();

  let mut builder = SimpleStringBuilder::new(buffer);

  if negative {
    builder.add_character(b'-');
  }
  builder.add_character(decimal_rep[0]);
  if significant_digits != 1 {
    builder.add_character(b'.');
    debug_assert!(significant_digits >= rep_length);
    builder.add_substring(&decimal_rep[1..rep_length]);
    builder.add_padding(b'0', significant_digits as isize - rep_length as isize);
  }
  builder.add_character(b'e');
  builder.add_character(if negative_exponent { b'-' } else { b'+' });
  builder.add_exponent(exponent);
  let n = builder.finish();
  ascii(&buffer[..n])
}

/// DoubleToExponentialStringView — Number.prototype.toExponential(f) 的格式层
/// f = -1 表示未指定 (走最短表示, f 退化为位数-1)
#[must_use]
pub fn double_to_exponential_string_view(mut value: f64, f: i32, buffer: &mut [u8]) -> &str {
  debug_assert!((-1..=MAX_FRACTION_DIGITS).contains(&f));
  // 原版特殊值由 builtins 层拦截; raw API 收窄为 ECMAScript 字面输出
  if !value.is_finite() {
    return match (value.is_nan(), value.is_sign_negative()) {
      (true, _) => "NaN",
      (false, true) => "-Infinity",
      (false, false) => "Infinity",
    };
  }

  let mut negative = false;
  if value < 0.0 {
    value = -value;
    negative = true;
  }

  // 小数点后 f 位 ⇒ 请求 f+1 位有效数字; +1 给终止符
  const V8_DTOA_BUFFER_CAPACITY: usize = 102; // kMaxFractionDigits + 1 + 1
  let mut decimal_rep = [0u8; V8_DTOA_BUFFER_CAPACITY];
  // (rep_length, point, f): f=-1 时最短表示退化为位数-1
  let (rep_length, point, f) = if f == -1 {
    let r = double_to_ascii(value, DtoaMode::Shortest, 0, &mut decimal_rep);
    (r.length, r.point, r.length - 1)
  } else {
    let r = double_to_ascii(value, DtoaMode::Precision, f + 1, &mut decimal_rep);
    debug_assert!(r.length > 0 && r.length <= f as usize + 1);
    (r.length, r.point, f as usize)
  };

  let exponent = point - 1;
  create_exponential_representation(
    &decimal_rep[..rep_length],
    rep_length,
    exponent,
    negative,
    f + 1,
    buffer,
  )
}

// ---------------------------------------------------------------------------
// toPrecision (conversions.cc 1370-1435)
// ---------------------------------------------------------------------------

/// DoubleToPrecisionStringView — Number.prototype.toPrecision(p) 的格式层
#[must_use]
pub fn double_to_precision_string_view(mut value: f64, p: i32, buffer: &mut [u8]) -> &str {
  debug_assert!((1..=MAX_FRACTION_DIGITS).contains(&p));
  // 原版特殊值由 builtins 层拦截; raw API 收窄为 ECMAScript 字面输出
  if !value.is_finite() {
    return match (value.is_nan(), value.is_sign_negative()) {
      (true, _) => "NaN",
      (false, true) => "-Infinity",
      (false, false) => "Infinity",
    };
  }

  let mut negative = false;
  if value < 0.0 {
    value = -value;
    negative = true;
  }

  // PRECISION 模式 p 位有效数字
  const V8_DTOA_BUFFER_CAPACITY: usize = 101; // kMaxFractionDigits + 1
  let mut decimal_rep = [0u8; V8_DTOA_BUFFER_CAPACITY];
  let r = double_to_ascii(value, DtoaMode::Precision, p, &mut decimal_rep);
  debug_assert!(r.length <= p as usize);

  let decimal_point = r.point;
  let exponent = decimal_point - 1;

  if exponent < -6 || exponent >= p {
    // 指数形式
    return create_exponential_representation(
      &decimal_rep[..r.length],
      r.length,
      exponent,
      negative,
      p as usize,
      buffer,
    );
  }

  // 定点形式
  let mut builder = SimpleStringBuilder::new(buffer);
  if negative {
    builder.add_character(b'-');
  }
  if decimal_point <= 0 {
    // 0.00ddd
    builder.add_substring(b"0.");
    builder.add_padding(b'0', -(decimal_point as isize));
    builder.add_substring(&decimal_rep[..r.length]);
    builder.add_padding(b'0', p as isize - r.length as isize);
  } else {
    // ddd.ddd / ddd000
    let m = r.length.min(decimal_point as usize);
    builder.add_substring(&decimal_rep[..m]);
    builder.add_padding(b'0', decimal_point as isize - r.length as isize);
    if decimal_point < p {
      builder.add_character(b'.');
      let extra: isize = if negative { 2 } else { 1 };
      if r.length > decimal_point as usize {
        let len = r.length - decimal_point as usize;
        let n =
          ((len as isize).min(p as isize - builder.position() as isize + extra)).max(0) as usize;
        builder.add_substring(&decimal_rep[decimal_point as usize..decimal_point as usize + n]);
      }
      builder.add_padding(b'0', extra + p as isize - builder.position() as isize);
    }
  }
  let n = builder.finish();
  ascii(&buffer[..n])
}

// ---------------------------------------------------------------------------
// toString(radix) (conversions.cc 1437-1528)
// ---------------------------------------------------------------------------

/// DoubleToRadixStringView — Number.prototype.toString(radix) 的格式层
/// 缓冲容量 kDoubleToRadixMaxChars = 2200 (整数/小数各半, 从中间向两侧写)
#[must_use]
pub fn double_to_radix_string_view(mut value: f64, radix: i32, buffer: &mut [u8]) -> &str {
  // 原版 CHECK (conversions.cc:1444): 非法 radix 会写越界缓冲, 硬检查
  // (非法 radix 会致 CHARS 越界 / 死循环, 不可用 debug 断言降级)
  assert!((2..=36).contains(&radix));
  // 原版约定有限非零输入 (caller 处理特殊值); raw API 收窄为 ECMAScript
  // 字面输出, 亦免去 -0 下 delta=0 的边界分支
  if !value.is_finite() {
    return match (value.is_nan(), value.is_sign_negative()) {
      (true, _) => "NaN",
      (false, true) => "-Infinity",
      (false, false) => "Infinity",
    };
  }
  if value == 0.0 {
    return "0";
  }

  debug_assert!(buffer.len() >= DOUBLE_TO_RADIX_MAX_CHARS);

  /// 36 进制字符表
  const CHARS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

  let radix = radix as usize;
  let middle = buffer.len() / 2;
  let mut integer_cursor = middle;
  let mut fraction_cursor = middle;

  let mut negative = false;
  if value < 0.0 {
    negative = true;
    value = -value;
  }

  // 拆整数/小数两部分; 小数位只算到输入 double 的精度
  let mut integer = value.floor();
  let mut fraction = value - integer;
  // 0.5 * (value 的下一个可表示 double - value)
  let mut delta = 0.5 * (Double::from_double(value).next_double() - value);
  let mut delta_is_positive = true;
  if delta <= 0.0 {
    // 舍到零: 遇 FTZ 时跳过小数循环; 否则用最小 (denormal) 步长
    if Fpu::get_flush_denormals() {
      delta_is_positive = false;
    } else {
      delta = Double::from_double(0.0).next_double();
    }
  }
  if delta_is_positive && fraction >= delta {
    // 小数部分
    buffer[fraction_cursor] = b'.';
    fraction_cursor += 1;
    loop {
      // 上移一位
      fraction *= radix as f64;
      delta *= radix as f64;
      let digit = fraction as usize;
      buffer[fraction_cursor] = CHARS[digit];
      fraction_cursor += 1;
      fraction -= digit as f64;
      // 舍入到偶
      if (fraction > 0.5 || (fraction == 0.5 && (digit & 1) != 0)) && fraction + delta > 1.0 {
        // 进位回溯已写数字
        loop {
          fraction_cursor -= 1;
          if fraction_cursor == middle {
            debug_assert_eq!(buffer[fraction_cursor], b'.');
            // 进位并入整数部分
            integer += 1.0;
            break;
          }
          let c = buffer[fraction_cursor];
          let digit = if c > b'9' {
            (c - b'a' + 10) as usize
          } else {
            (c - b'0') as usize
          };
          if digit + 1 < radix {
            buffer[fraction_cursor] = CHARS[digit + 1];
            fraction_cursor += 1;
            break;
          }
        }
        break;
      }
      if fraction < delta {
        break;
      }
    }
  }

  // 整数部分: 不足的低位补零
  while Double::from_double(integer / radix as f64).exponent() > 0 {
    integer /= radix as f64;
    integer_cursor -= 1;
    buffer[integer_cursor] = b'0';
  }
  loop {
    let remainder = integer % radix as f64; // fmod 语义
    integer_cursor -= 1;
    buffer[integer_cursor] = CHARS[remainder as usize];
    integer = (integer - remainder) / radix as f64;
    if integer <= 0.0 {
      break;
    }
  }

  // 符号
  if negative {
    integer_cursor -= 1;
    buffer[integer_cursor] = b'-';
  }
  debug_assert!(fraction_cursor > integer_cursor);
  ascii(&buffer[integer_cursor..fraction_cursor])
}
