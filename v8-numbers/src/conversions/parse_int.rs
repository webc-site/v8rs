//! 原版 conversions.cc 341-991 — parseInt 整族
//!
//! 覆盖: ES6 18.2.5 parseInt (NumberParseIntHelper) 的 radix 探测、
//! 2/4/8/16/32 进制快速路径 (InternalStringToIntDouble, 溢出按舍入到偶
//! 收敛到 53 位) 与 3..36 通用进制累乘路径。
//! BigInt 解析子类 (StringToBigIntHelper) 依赖 v8-bigint, 推迟 (见 crate README)。

use v8_base::numbers::strtod::strtod;

use super::simple_builder::{junk_string_value, signed_zero};

/// 转换标志 — 前缀进制与尾随垃圾的许可 (conversions.h 149-153)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConversionFlag {
  /// 都不允许
  NoConversionFlag,
  /// 允许 0x/0o/0b 前缀
  AllowNonDecimalPrefix,
  /// 允许尾随垃圾
  AllowTrailingJunk,
}

/// 数值输入字符抽象 — 原版模板参数 Char (uint8_t / uc16)
pub trait NumChar: Copy + Eq {
  /// 提升为码点 (ASCII 比较与空白谓词共用)
  fn code(self) -> u32;
}

impl NumChar for u8 {
  #[inline]
  fn code(self) -> u32 {
    self as u32
  }
}

impl NumChar for u16 {
  #[inline]
  fn code(self) -> u32 {
    self as u32
  }
}

/// isDigit — radix 进制数字判定 (conversions.cc 341-345)
#[must_use]
#[inline]
pub fn is_digit(x: u32, radix: u32) -> bool {
  (x >= '0' as u32 && x <= '9' as u32 && x < '0' as u32 + radix)
    || (radix > 10 && x >= 'a' as u32 && x < 'a' as u32 + radix - 10)
    || (radix > 10 && x >= 'A' as u32 && x < 'A' as u32 + radix - 10)
}

/// isBinaryDigit — '0' / '1'
#[must_use]
#[inline]
pub fn is_binary_digit(x: u32) -> bool {
  x == '0' as u32 || x == '1' as u32
}

/// IsWhiteSpaceOrLineTerminator — ECMAScript 空白 + 行终止符全集 (16 位内)
/// 原版在 strings/char-predicates, 因依赖方向 (v8-strings 依赖本 crate) 就地内联
#[must_use]
#[inline]
pub fn is_white_space_or_line_terminator(c: u32) -> bool {
  matches!(
    c,
    0x0009 | 0x000A | 0x000B | 0x000C | 0x000D | 0x0020 | 0x00A0 | 0x1680 | 0x2000
      ..=0x200A | 0x2028 | 0x2029 | 0x202F | 0x205F | 0x3000 | 0xFEFF
  )
}

/// SubStringEquals — 从 **current 起匹配 substring (首字符已确认相等),
/// 成功则推进 current 越过整个子串
#[must_use]
pub fn sub_string_equals<C: NumChar>(current: &mut usize, end: usize, s: &[C], sub: &[u8]) -> bool {
  debug_assert!(s[*current].code() == sub[0] as u32);
  for &b in &sub[1..] {
    *current += 1;
    if *current == end || s[*current].code() != u32::from(b) {
      return false;
    }
  }
  *current += 1;
  true
}

/// AdvanceToNonspace — 跳过空白; 返回是否遇到非空白字符
#[must_use]
pub fn advance_to_nonspace<C: NumChar>(current: &mut usize, end: usize, s: &[C]) -> bool {
  while *current != end {
    if !is_white_space_or_line_terminator(s[*current].code()) {
      return true;
    }
    *current += 1;
  }
  false
}

// ---------------------------------------------------------------------------
// InternalStringToIntDouble — 2^n 进制解析 (radix_log_2 传常量)
// ---------------------------------------------------------------------------

/// kMaxExponent — std::numeric_limits<double>::max_exponent
const K_MAX_EXPONENT: i32 = 1024;

/// 2^radix_log_2 进制整数串 → double; 含 53 位溢出舍入 (ties-to-even)
/// (conversions.cc 373-472; current != end 由调用方保证)
pub(crate) fn internal_string_to_int_double<const RADIX_LOG_2: i32, C: NumChar>(
  s: &[C],
  start: usize,
  end: usize,
  negative: bool,
  allow_trailing_junk: bool,
) -> f64 {
  // RADIX_LOG_2 为常量泛型, let 即编译期折叠
  let radix: u32 = 1 << RADIX_LOG_2;
  let lim_0 = u32::from(b'0') + radix.min(10);
  let (lim_a, lim_upper_a) = if radix > 10 {
    (u32::from(b'a') + radix - 10, u32::from(b'A') + radix - 10)
  } else {
    (0, 0)
  };

  let mut current = start;
  if current >= end {
    return junk_string_value();
  }

  // 跳过前导 0
  while s[current].code() == u32::from(b'0') {
    current += 1;
    if current == end {
      return signed_zero(negative);
    }
  }

  let mut number: i64 = 0;
  let mut exponent: i32 = 0;

  loop {
    let c = s[current].code();
    let digit: i64 = if c >= u32::from(b'0') && c < lim_0 {
      (c - u32::from(b'0')) as i64
    } else if c >= u32::from(b'a') && c < lim_a {
      (c - u32::from(b'a')) as i64 + 10
    } else if c >= u32::from(b'A') && c < lim_upper_a {
      (c - u32::from(b'A')) as i64 + 10
    } else {
      // 未遇到任何数字 → 非法; 其余为尾随垃圾判定
      if current == start {
        return junk_string_value();
      }
      if allow_trailing_junk || !advance_to_nonspace(&mut current, end, s) {
        break;
      }
      return junk_string_value();
    };

    number = number * radix as i64 + digit;
    let mut overflow = (number >> 53) as i32;
    if overflow != 0 {
      // 溢出: 确定舍入方向
      let mut overflow_bits_count = 1;
      while overflow > 1 {
        overflow_bits_count += 1;
        overflow >>= 1;
      }

      let dropped_bits_mask = (1 << overflow_bits_count) - 1;
      let dropped_bits = number as i32 & dropped_bits_mask;
      number >>= overflow_bits_count;
      exponent = overflow_bits_count;

      // 消费剩余数字位; 指数封顶防 int 溢出
      let mut zero_tail = true;
      loop {
        current += 1;
        if current == end || !is_digit(s[current].code(), radix) {
          break;
        }
        zero_tail &= s[current].code() == u32::from(b'0');
        if exponent <= K_MAX_EXPONENT {
          exponent += RADIX_LOG_2;
        }
      }

      if !allow_trailing_junk && advance_to_nonspace(&mut current, end, s) {
        return junk_string_value();
      }

      let middle_value = 1 << (overflow_bits_count - 1);
      if dropped_bits > middle_value {
        number += 1; // 舍入上
      } else if dropped_bits == middle_value {
        // ties-to-even: 尾数奇或尾串非零则进位 (与十进制舍入一致)
        if (number & 1) != 0 || !zero_tail {
          number += 1;
        }
      }

      // 进位可能再溢出
      if number & (1i64 << 53) != 0 {
        exponent += 1;
        number >>= 1;
      }
      break;
    }
    current += 1;
    if current == end {
      break;
    }
  }

  debug_assert!(number < 1i64 << 53);
  debug_assert!(number as f64 as i64 == number);

  if exponent == 0 {
    if negative {
      if number == 0 {
        return -0.0;
      }
      number = -number;
    }
    return number as f64;
  }

  debug_assert_ne!(number, 0);
  // ldexp(±number, exponent): 2 的幂 powi 精确, 超域自然收敛到 inf
  let signed = if negative { -number } else { number };
  (signed as f64) * f64::powi(2.0, exponent)
}

// ---------------------------------------------------------------------------
// radix 探测 + NumberParseIntHelper
// ---------------------------------------------------------------------------

/// 解析状态机 (StringToIntHelper 内部)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
  Running,
  Error,
  Junk,
  Empty,
  Zero,
  Done,
}

/// 符号态
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sign {
  Negative,
  Positive,
  None,
}

/// radix 探测结果 (DetectRadixInternal): 探测后的实际 radix 仅 Running 态有效
struct RadixInfo {
  state: State,
  sign: Sign,
  cursor: usize,
  radix: i32,
}

/// DetectRadixInternal — 跳空白/符号, radix==0 时探测 0x/0o/0b 前缀与隐式八进制,
/// 跳前导零并校验首数字
/// (conversions.cc 590-669; allow_trailing_junk 对应原版 allow_trailing_junk_,
/// 前导零尾随垃圾是否判 Zero 受其门控)
fn detect_radix<C: NumChar>(
  s: &[C],
  radix: i32,
  allow_binary_and_octal_prefixes: bool,
  allow_trailing_junk: bool,
) -> RadixInfo {
  let length = s.len();
  let mut current = 0usize;
  let mut radix = radix;
  let mut sign = Sign::None;
  let mut leading_zero = false;

  if radix != 0 && !(2..=36).contains(&radix) {
    return RadixInfo {
      state: State::Junk,
      sign,
      cursor: 0,
      radix: 0,
    };
  }

  if !advance_to_nonspace(&mut current, length, s) {
    return RadixInfo {
      state: State::Empty,
      sign,
      cursor: 0,
      radix: 0,
    };
  }

  let first = s[current].code();
  if first == u32::from(b'+') {
    // 跳过前导符号, 其后可跟空白 (原版语义: 仅继续解析)
    current += 1;
    if current == length {
      return RadixInfo {
        state: State::Junk,
        sign,
        cursor: 0,
        radix: 0,
      };
    }
    sign = Sign::Positive;
  } else if first == u32::from(b'-') {
    current += 1;
    if current == length {
      return RadixInfo {
        state: State::Junk,
        sign,
        cursor: 0,
        radix: 0,
      };
    }
    sign = Sign::Negative;
  }

  if radix == 0 {
    // radix 探测
    radix = 10;
    if s[current].code() == u32::from(b'0') {
      current += 1;
      if current == length {
        return RadixInfo {
          state: State::Zero,
          sign,
          cursor: 0,
          radix: 0,
        };
      }
      let c = s[current].code();
      if c == u32::from(b'x') || c == u32::from(b'X') {
        radix = 16;
        current += 1;
        if current == length {
          return RadixInfo {
            state: State::Junk,
            sign,
            cursor: 0,
            radix: 0,
          };
        }
      } else if allow_binary_and_octal_prefixes && (c == u32::from(b'o') || c == u32::from(b'O')) {
        radix = 8;
        current += 1;
        if current == length {
          return RadixInfo {
            state: State::Junk,
            sign,
            cursor: 0,
            radix: 0,
          };
        }
      } else if allow_binary_and_octal_prefixes && (c == u32::from(b'b') || c == u32::from(b'B')) {
        radix = 2;
        current += 1;
        if current == length {
          return RadixInfo {
            state: State::Junk,
            sign,
            cursor: 0,
            radix: 0,
          };
        }
      } else {
        leading_zero = true;
      }
    }
  } else if radix == 16 && s[current].code() == u32::from(b'0') {
    // 允许 "0x" 前缀
    current += 1;
    if current == length {
      return RadixInfo {
        state: State::Zero,
        sign,
        cursor: 0,
        radix: 0,
      };
    }
    let c = s[current].code();
    if c == u32::from(b'x') || c == u32::from(b'X') {
      current += 1;
      if current == length {
        return RadixInfo {
          state: State::Junk,
          sign,
          cursor: 0,
          radix: 0,
        };
      }
    } else {
      leading_zero = true;
    }
  }
  // 跳过前导零
  while s[current].code() == u32::from(b'0') {
    leading_zero = true;
    current += 1;
    if current == length {
      return RadixInfo {
        state: State::Zero,
        sign,
        cursor: 0,
        radix: 0,
      };
    }
  }
  // 前导零后的尾随垃圾检查 (conversions.cc:659: 受 allow_trailing_junk 门控;
  // 禁尾随垃圾时保持 Running, 交由各解析路径的垃圾复核拒绝)
  if leading_zero && allow_trailing_junk && !is_digit(s[current].code(), radix as u32) {
    return RadixInfo {
      state: State::Zero,
      sign,
      cursor: 0,
      radix: 0,
    };
  }
  if !leading_zero && !is_digit(s[current].code(), radix as u32) {
    return RadixInfo {
      state: State::Junk,
      sign,
      cursor: 0,
      radix: 0,
    };
  }

  debug_assert!((2..=36).contains(&radix));
  RadixInfo {
    state: State::Running,
    sign,
    cursor: current,
    radix,
  }
}

/// StringToInt — ES6 18.2.5 parseInt(string, radix) 的数值核心
/// (原版 StringToInt(Isolate, string, radix) 的 raw 等价; Handle 绑定推迟。
/// parseInt 路径 allow_trailing_junk 恒真 — NumberParseIntHelper 从不
/// set_disallow_trailing_junk; BigInt 子类接入时经参数传入)
#[must_use]
pub fn string_to_int<C: NumChar>(s: &[C], radix: i32) -> f64 {
  let info = detect_radix(s, radix, false, true);
  let negative = info.sign == Sign::Negative;
  match info.state {
    State::Junk | State::Empty => junk_string_value(),
    State::Zero => signed_zero(negative),
    State::Running => {
      let result = parse_internal(s, info.cursor, info.radix);
      // 三条路径 (strtod / 2^n / 通用) 均返回无符号值, 此处统一取负 (原版 GetResult)
      if negative { -result } else { result }
    }
    // DetectRadix 收敛后 Error/Done 不可达 (属 BigInt 子类状态)
    State::Error | State::Done => unreachable!(),
  }
}

/// ParseInternal — 按 radix 分派三条路径 (返回无符号值, 符号由调用方应用)
fn parse_internal<C: NumChar>(s: &[C], cursor: usize, radix: i32) -> f64 {
  let end = s.len();
  if radix == 10 {
    return handle_base_ten_case(&s[cursor..end]);
  }
  if (radix as u32).is_power_of_two() {
    // 符号位由调用方处理, 此处恒非负
    let log2 = radix.trailing_zeros() as i32;
    return match log2 {
      1 => internal_string_to_int_double::<1, _>(s, cursor, end, false, true),
      2 => internal_string_to_int_double::<2, _>(s, cursor, end, false, true),
      3 => internal_string_to_int_double::<3, _>(s, cursor, end, false, true),
      4 => internal_string_to_int_double::<4, _>(s, cursor, end, false, true),
      5 => internal_string_to_int_double::<5, _>(s, cursor, end, false, true),
      _ => unreachable!(),
    };
  }
  handle_generic_case(s, cursor, end, radix)
}

/// HandleBaseTenCase — 十进制走 strtod; 超 309 位按无穷解析
fn handle_base_ten_case<C: NumChar>(digits: &[C]) -> f64 {
  const MAX_SIGNIFICANT_DIGITS: usize = 309; // double < 1.8e308

  let mut buffer = [0u8; MAX_SIGNIFICANT_DIGITS + 1];
  let mut buffer_pos = 0usize;
  for &c in digits {
    let code = c.code();
    if !(u32::from(b'0')..=u32::from(b'9')).contains(&code) {
      break;
    }
    if buffer_pos <= MAX_SIGNIFICANT_DIGITS {
      buffer[buffer_pos] = code as u8;
      buffer_pos += 1;
    }
  }

  strtod(&buffer[..buffer_pos], 0)
}

/// HandleGenericCase — 3..36 通用进制: 32 位乘加环 + f64 累计
/// (大于 ~2^56 存在累计舍入误差, 规范明确允许)
fn handle_generic_case<C: NumChar>(s: &[C], mut current: usize, end: usize, radix: i32) -> f64 {
  let radix = radix as u32;
  let lim_0 = u32::from(b'0') + radix.min(10);
  let (lim_a, lim_upper_a) = if radix > 10 {
    (u32::from(b'a') + radix - 10, u32::from(b'A') + radix - 10)
  } else {
    (0, 0)
  };

  const MAXIMUM_MULTIPLIER: u32 = u32::MAX / 36;

  let mut result: f64 = 0.0;
  let mut done = false;
  while !done {
    // 解析最长 32 位安全的乘加段
    let mut part: u32 = 0;
    let mut multiplier: u32 = 1;
    loop {
      let c = s[current].code();
      let d: u32 = if c >= u32::from(b'0') && c < lim_0 {
        c - u32::from(b'0')
      } else if c >= u32::from(b'a') && c < lim_a {
        c - u32::from(b'a') + 10
      } else if c >= u32::from(b'A') && c < lim_upper_a {
        c - u32::from(b'A') + 10
      } else {
        done = true;
        break;
      };

      // 乘子将溢出 32 位时截段
      let m = multiplier.saturating_mul(radix);
      if m > MAXIMUM_MULTIPLIER {
        break;
      }
      part = part * radix + d;
      multiplier = m;
      debug_assert!(multiplier > part);

      current += 1;
      if current == end {
        done = true;
        break;
      }
    }
    result = result * f64::from(multiplier) + f64::from(part);
  }
  // (parseInt 路径 allow_trailing_junk 恒真, 无尾随垃圾复核)
  result
}
