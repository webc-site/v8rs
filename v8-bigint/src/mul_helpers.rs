// 由手工从 v8/src/bigint/bigint-inl.h 的乘法部分抽出转写; 对应原版逐行
//! 单 digit 乘法 / Schoolbook 乘法 / CachedMod 专用乘法变体

use crate::bigint::{Digit, digit_add2, digit_add3, digit_mul, is_digit_normalized};

/// 列累加循环体 (原版 BODY 宏): Z[i] = Σ X[j]*Y[i-j] + 进位链
/// 用宏保持 4 个调用点与原版逐行对应
macro_rules! body {
  ($z:expr, $x:expr, $y:expr, $i:expr, $min:expr, $max:expr, $zi:expr, $carry:expr, $next:expr, $next_carry:expr) => {{
    let mut j = $min;
    while j <= $max {
      let (low, high) = digit_mul($x[j], $y[$i - j]);
      let (v, carrybit) = digit_add2($zi, low);
      $zi = v;
      $carry += carrybit;
      let (v2, carrybit2) = digit_add2($next, high);
      $next = v2;
      $next_carry += carrybit2;
      j += 1;
    }
    $z[$i] = $zi;
  }};
}

/// Z := X * y (单 digit), 返回 Z 最高位; 要求 Z 与 X 不重叠
pub fn multiply_single(z: &mut [Digit], x: &[Digit], y: Digit) -> Digit {
  debug_assert!(y != 0);
  let mut carry = 0;
  let mut high = 0;
  for i in 0..x.len() {
    let (low, new_high) = digit_mul(x[i], y);
    let (v, c) = digit_add3(low, high, carry);
    z[i] = v;
    carry = c;
    high = new_high;
  }
  let top = carry + high;
  z[x.len()] = top;
  let mut top = top;
  for d in z[x.len() + 1..].iter_mut() {
    *d = 0;
    top = 0;
  }
  top
}

/// Z := X * X[..used] * y 的就地前缀乘 (FromStringClassic 用: X 是 Z 的前缀)
pub fn multiply_single_prefix(z: &mut [Digit], used: usize, y: Digit) -> Digit {
  debug_assert!(y != 0);
  let mut carry = 0;
  let mut high = 0;
  for zi in z[..used].iter_mut() {
    let (low, new_high) = digit_mul(*zi, y);
    let (v, c) = digit_add3(low, high, carry);
    *zi = v;
    carry = c;
    high = new_high;
  }
  let top = carry + high;
  z[used] = top;
  let mut top = top;
  for d in z[used + 1..].iter_mut() {
    *d = 0;
    top = 0;
  }
  top
}

/// Z := X * Y (Schoolbook), 返回 Z 最高位; 要求 X.len >= Y.len
pub fn multiply_schoolbook(z: &mut [Digit], x: &[Digit], y: &[Digit]) -> Digit {
  debug_assert!(is_digit_normalized(x));
  debug_assert!(is_digit_normalized(y));
  debug_assert!(x.len() >= y.len());
  debug_assert!(z.len() >= x.len() + y.len());
  if x.is_empty() || y.is_empty() {
    z.fill(0);
    return 0;
  }
  let mut next;
  let mut next_carry = 0;
  let mut carry;
  // 展开首轮: 平凡
  let (low, high) = digit_mul(x[0], y[0]);
  z[0] = low;
  next = high;
  let mut i = 1usize;
  // 展开次轮: 少量准备
  if i < y.len() {
    let mut zi = next;
    next = 0;
    carry = 0;
    body!(z, x, y, i, 0, 1, zi, carry, next, next_carry);
    i += 1;
  } else {
    carry = 0;
  }
  // 主体: X.len >= Y.len > i, 无需边界检查
  while i < y.len() {
    let (v, c) = digit_add2(next, carry);
    let mut zi = v;
    carry = c;
    next = next_carry + carry;
    carry = 0;
    next_carry = 0;
    body!(z, x, y, i, 0, i, zi, carry, next, next_carry);
    i += 1;
  }
  // 尾段: i 超过 Y, 需要边界检查
  let loop_end = x.len() + y.len() - 2;
  while i <= loop_end {
    let max_x_index = i.min(x.len() - 1);
    let max_y_index = y.len() - 1;
    let min_x_index = i - max_y_index;
    let (v, c) = digit_add2(next, carry);
    let mut zi = v;
    carry = c;
    next = next_carry + carry;
    carry = 0;
    next_carry = 0;
    body!(
      z,
      x,
      y,
      i,
      min_x_index,
      max_x_index,
      zi,
      carry,
      next,
      next_carry
    );
    i += 1;
  }
  // 写最后一位, 清零 Z 多余空间
  let (top, c) = digit_add2(next, carry);
  carry = c;
  z[i] = top;
  let mut top = top;
  i += 1;
  debug_assert!(carry == 0);
  while i < z.len() {
    z[i] = 0;
    top = 0;
    i += 1;
  }
  top
}

/// CachedMod 专用: 只算列 [start_position, X.len + Y.len) 的乘积
/// 低位列被跳过, 入站进位丢弃, 结果是近似的 (误差 <= 2 * B^(start_position+1))
pub fn multiply_special_high(z: &mut [Digit], x: &[Digit], y: &[Digit], start_position: usize) {
  debug_assert!(x.len() >= y.len());
  debug_assert!(!y.is_empty());
  // 收缩段公式要求 i >= Y.len - 1
  debug_assert!(start_position >= y.len() - 1);
  debug_assert!(z.len() >= x.len() + y.len());

  let mut next = 0;
  let mut next_carry = 0;
  let mut carry = 0;
  let loop_end = x.len() + y.len() - 2;
  let mut i = start_position;
  while i <= loop_end {
    let max_y_index = y.len() - 1;
    let min_x_index = i - max_y_index;
    let max_x_index = i.min(x.len() - 1);
    let (v, c) = digit_add2(next, carry);
    let mut zi = v;
    carry = c;
    next = next_carry + carry;
    carry = 0;
    next_carry = 0;
    body!(
      z,
      x,
      y,
      i,
      min_x_index,
      max_x_index,
      zi,
      carry,
      next,
      next_carry
    );
    i += 1;
  }
  let (top, c) = digit_add2(next, carry);
  z[i] = top;
  debug_assert!(c == 0);
}

/// CachedMod 专用: 只算 X*Y 的低 Z.len() 位
pub fn multiply_special_low(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  debug_assert!(!y.is_empty());
  debug_assert!(x.len() >= 2);
  debug_assert!(x.len() >= y.len() - 1);

  let mut next;
  let mut next_carry = 0;
  let mut carry;
  // 展开首轮
  let (low, high) = digit_mul(x[0], y[0]);
  z[0] = low;
  next = high;
  let mut i = 1usize;
  // 展开次轮
  if i < y.len() {
    let mut zi = next;
    next = 0;
    carry = 0;
    body!(z, x, y, i, 0, 1, zi, carry, next, next_carry);
    i += 1;
  } else {
    carry = 0;
  }
  // 主体: 循环内无边界检查
  let loop_end = z.len() - 1;
  let main_end = x.len().min(y.len()).min(loop_end);
  while i < main_end {
    let (v, c) = digit_add2(next, carry);
    let mut zi = v;
    carry = c;
    next = next_carry + carry;
    carry = 0;
    next_carry = 0;
    body!(z, x, y, i, 0, i, zi, carry, next, next_carry);
    i += 1;
  }
  // 尾段: 需要边界检查
  while i <= loop_end {
    let max_x_index = i.min(x.len() - 1);
    let max_y_index = i.min(y.len() - 1);
    let min_x_index = i - max_y_index;
    let (v, c) = digit_add2(next, carry);
    let mut zi = v;
    carry = c;
    next = next_carry + carry;
    carry = 0;
    next_carry = 0;
    body!(
      z,
      x,
      y,
      i,
      min_x_index,
      max_x_index,
      zi,
      carry,
      next,
      next_carry
    );
    i += 1;
  }
}
