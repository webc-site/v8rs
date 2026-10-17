// 由手工从 v8/src/bigint/bigint.h + bigint-inl.h 转写; 对应原版逐行
//! digit 层原语与全部 "快" 操作 (O(n) 加减/位运算/移位/截断), 以及结果长度预算函数
//! Digits/RWDigits → &[Digit]/&mut [Digit]; Normalize → norm() 返回截断视图

use crate::mul_helpers::{multiply_schoolbook, multiply_single};

/// digit 类型: 寄存器宽度的无符号整数 (64 位平台)
pub type Digit = u64;
/// 双倍宽中间量 (原版 twodigit_t)
pub type TwoDigit = u128;

/// 状态码: 慢操作可能被中断
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
  Ok,
  Interrupted,
}

/// 嵌入方平台钩子: 长操作每隔一段时间查询中断
/// (原版还承担临时缓冲分配, Rust 版由 Vec 承担)
pub trait Platform {
  fn interrupt_requested(&mut self) -> bool {
    false
  }
}

/// 默认平台: 永不中断
#[derive(Debug, Clone, Copy, Default)]
pub struct DefaultPlatform;

impl Platform for DefaultPlatform {}

pub const K_LOG2_DIGIT_BITS: u32 = 6;
pub const K_DIGIT_BITS: u32 = 1 << K_LOG2_DIGIT_BITS;
pub const K_HALF_DIGIT_BITS: u32 = K_DIGIT_BITS / 2;
pub const K_HALF_DIGIT_BASE: Digit = 1 << K_HALF_DIGIT_BITS;
pub const K_HALF_DIGIT_MASK: Digit = K_HALF_DIGIT_BASE - 1;

/// CachedMod 的除数缓存上限 (32 digits 够 2048 位除数)
pub const K_MAX_CACHED_MOD_DIVISOR_SIZE: u32 = 32;

/// 整数除法向上取整 (原版 DIV_CEIL 宏)
#[inline]
pub const fn div_ceil(x: u32, y: u32) -> u32 {
  if x == 0 { 0 } else { (x - 1) / y + 1 }
}

// ---------------------------------------------------------------------------
// 算法阈值 (原版 namespace config, 64 位平台取值)
// ---------------------------------------------------------------------------

pub const K_KARATSUBA_THRESHOLD: u32 = 34;
pub const K_BURNIKEL_THRESHOLD: u32 = 57;
pub const K_NEWTON_INVERSION_THRESHOLD: u32 = 25;
pub const K_TOOM_THRESHOLD: u32 = 210;
pub const K_FFT_THRESHOLD: u32 = 720;
pub const K_FFT_INNER_THRESHOLD: u32 = 200;
pub const K_BARRETT_THRESHOLD: u32 = 13000;
pub const K_TO_STRING_FAST_THRESHOLD: u32 = 23;
pub const K_FROM_STRING_LARGE_THRESHOLD: u32 = 25;

// ---------------------------------------------------------------------------
// digit 层原语 (原版 bigint-inl.h)
// ---------------------------------------------------------------------------

/// 前导零个数; 0 返回 64
#[inline]
pub const fn count_leading_zeros(value: Digit) -> u32 {
  if value == 0 {
    64
  } else {
    value.leading_zeros()
  }
}

/// 32 位整数位长 (原版 BitLength(int)); 0 → 0
#[inline]
pub const fn bit_length(n: u32) -> u32 {
  if n == 0 { 0 } else { 32 - n.leading_zeros() }
}

#[inline]
pub const fn digit_ismax(x: Digit) -> bool {
  x == Digit::MAX
}

/// a + b, 返回 (结果, 进位 0/1)
#[inline]
pub fn digit_add2(a: Digit, b: Digit) -> (Digit, Digit) {
  let r = a as TwoDigit + b as TwoDigit;
  (
    (r & Digit::MAX as TwoDigit) as Digit,
    (r >> K_DIGIT_BITS) as Digit,
  )
}

/// a + b + c, 返回 (结果, 进位 0/1)
#[inline]
pub fn digit_add3(a: Digit, b: Digit, c: Digit) -> (Digit, Digit) {
  let r = a as TwoDigit + b as TwoDigit + c as TwoDigit;
  (
    (r & Digit::MAX as TwoDigit) as Digit,
    (r >> K_DIGIT_BITS) as Digit,
  )
}

/// a - b, 返回 (结果, 借位 0/1)
#[inline]
pub fn digit_sub(a: Digit, b: Digit) -> (Digit, Digit) {
  let r = (a as TwoDigit).wrapping_sub(b as TwoDigit);
  (
    (r & Digit::MAX as TwoDigit) as Digit,
    ((r >> K_DIGIT_BITS) & 1) as Digit,
  )
}

/// a - b - borrow_in, 返回 (结果, 借位 0/1)
#[inline]
pub fn digit_sub2(a: Digit, b: Digit, borrow_in: Digit) -> (Digit, Digit) {
  let r = (a as TwoDigit)
    .wrapping_sub(b as TwoDigit)
    .wrapping_sub(borrow_in as TwoDigit);
  (
    (r & Digit::MAX as TwoDigit) as Digit,
    ((r >> K_DIGIT_BITS) & 1) as Digit,
  )
}

/// a * b, 返回 (低位, 高位)
#[inline]
pub fn digit_mul(a: Digit, b: Digit) -> (Digit, Digit) {
  let r = (a as TwoDigit) * b as TwoDigit;
  (
    (r & Digit::MAX as TwoDigit) as Digit,
    (r >> K_DIGIT_BITS) as Digit,
  )
}

/// (high:low) / divisor, 返回 (商, 余数); 要求 high < divisor 且 divisor != 0
#[inline]
pub fn digit_div(high: Digit, low: Digit, divisor: Digit) -> (Digit, Digit) {
  assert!(divisor != 0, "division by zero");
  debug_assert!(high < divisor);
  let n = ((high as TwoDigit) << K_DIGIT_BITS) | low as TwoDigit;
  (
    (n / divisor as TwoDigit) as Digit,
    (n % divisor as TwoDigit) as Digit,
  )
}

// ---------------------------------------------------------------------------
// 视图工具
// ---------------------------------------------------------------------------

/// 去掉高位 (切片尾部) 的零 digit
#[inline]
pub fn norm(x: &[Digit]) -> &[Digit] {
  let mut len = x.len();
  while len > 0 && x[len - 1] == 0 {
    len -= 1;
  }
  &x[..len]
}

/// 只读切片的 msd (空切片返回 0)
#[inline]
pub fn msd(x: &[Digit]) -> Digit {
  x.last().copied().unwrap_or(0)
}

/// 是否 digit 规范化 (最高位非零或长度为 0)
#[inline]
pub fn is_digit_normalized(x: &[Digit]) -> bool {
  x.is_empty() || msd(x) != 0
}

// ---------------------------------------------------------------------------
// 比较
// ---------------------------------------------------------------------------

/// 不做规范化的比较: <0 / 0 / >0
pub fn compare_no_normalize(a: &[Digit], b: &[Digit]) -> i32 {
  let diff = a.len() as i64 - b.len() as i64;
  if diff != 0 {
    return diff as i32;
  }
  let mut i = a.len() as i64 - 1;
  while i >= 0 && a[i as usize] == b[i as usize] {
    i -= 1;
  }
  if i < 0 {
    return 0;
  }
  if a[i as usize] > b[i as usize] { 1 } else { -1 }
}

/// 规范化后的比较: <0 / 0 / >0
pub fn compare(a: &[Digit], b: &[Digit]) -> i32 {
  compare_no_normalize(norm(a), norm(b))
}

#[inline]
pub fn greater_or_equal(a: &[Digit], b: &[Digit]) -> bool {
  compare(a, b) >= 0
}

// ---------------------------------------------------------------------------
// 加减法 (原版 bigint-inl.h)
// ---------------------------------------------------------------------------

/// Z := X + Y, 返回 Z 最高位
pub fn add(z: &mut [Digit], x: &[Digit], y: &[Digit]) -> Digit {
  // 保证 x.len() >= y.len()
  let (x, y) = if x.len() < y.len() { (y, x) } else { (x, y) };
  debug_assert!(z.len() >= x.len());
  let mut i = 0;
  let mut carry = 0;
  let mut top = 0;
  while i < y.len() {
    let (v, c) = digit_add3(x[i], y[i], carry);
    z[i] = v;
    top = v;
    carry = c;
    i += 1;
  }
  while i < x.len() {
    let (v, c) = digit_add2(x[i], carry);
    z[i] = v;
    top = v;
    carry = c;
    i += 1;
  }
  while i < z.len() {
    z[i] = carry;
    top = carry;
    carry = 0;
    i += 1;
  }
  top
}

/// Z := X - Y (要求 X >= Y), 返回 Z 最高位
pub fn subtract(z: &mut [Digit], x: &[Digit], y: &[Digit]) -> Digit {
  debug_assert!(is_digit_normalized(x));
  debug_assert!(is_digit_normalized(y));
  debug_assert!(z.len() >= x.len() && x.len() >= y.len());
  let mut i = 0;
  let mut borrow = 0;
  let mut top = 0;
  while i < y.len() {
    let (v, b) = digit_sub2(x[i], y[i], borrow);
    z[i] = v;
    top = v;
    borrow = b;
    i += 1;
  }
  while i < x.len() {
    let (v, b) = digit_sub(x[i], borrow);
    z[i] = v;
    top = v;
    borrow = b;
    i += 1;
  }
  debug_assert!(borrow == 0);
  while i < z.len() {
    z[i] = 0;
    top = 0;
    i += 1;
  }
  top
}

/// 规范化后做减法
pub fn subtract_with_normalization(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  subtract(z, norm(x), norm(y));
}

/// 带符号加法, 返回结果是否为负
pub fn add_signed(
  z: &mut [Digit],
  x: &[Digit],
  x_negative: bool,
  y: &[Digit],
  y_negative: bool,
) -> bool {
  if x_negative == y_negative {
    add(z, x, y);
    return x_negative;
  }
  let (x, y) = (norm(x), norm(y));
  if compare_no_normalize(x, y) >= 0 {
    subtract(z, x, y);
    x_negative
  } else {
    subtract(z, y, x);
    !x_negative
  }
}

/// 带符号减法, 返回结果是否为负
pub fn subtract_signed(
  z: &mut [Digit],
  x: &[Digit],
  x_negative: bool,
  y: &[Digit],
  y_negative: bool,
) -> bool {
  if x_negative != y_negative {
    add(z, x, y);
    return x_negative;
  }
  let (x, y) = (norm(x), norm(y));
  if compare_no_normalize(x, y) >= 0 {
    subtract(z, x, y);
    x_negative
  } else {
    subtract(z, y, x);
    !x_negative
  }
}

// --- Z 与 X 同一缓冲的就地变体 (原版通过指针别名实现) ---

/// X += Y (X 既为被加数又为结果), 返回结果符号对应的进位语义同 add
fn add_zx(zx: &mut [Digit], y: &[Digit]) -> Digit {
  debug_assert!(zx.len() >= y.len());
  let mut carry = 0;
  let mut top = 0;
  for i in 0..y.len() {
    let (v, c) = digit_add3(zx[i], y[i], carry);
    zx[i] = v;
    top = v;
    carry = c;
  }
  let mut i = y.len();
  while i < zx.len() && carry != 0 {
    let (v, c) = digit_add2(zx[i], carry);
    zx[i] = v;
    top = v;
    carry = c;
    i += 1;
  }
  top
}

/// X -= Y 就地 (要求 X >= Y)
fn subtract_zx(zx: &mut [Digit], y: &[Digit]) -> Digit {
  debug_assert!(zx.len() >= y.len());
  let mut borrow = 0;
  let mut top = 0;
  let mut i = 0;
  while i < y.len() {
    let (v, b) = digit_sub2(zx[i], y[i], borrow);
    zx[i] = v;
    top = v;
    borrow = b;
    i += 1;
  }
  // 剩余位继续传播借位
  while i < zx.len() {
    let (v, b) = digit_sub(zx[i], borrow);
    zx[i] = v;
    top = v;
    borrow = b;
    i += 1;
  }
  debug_assert!(borrow == 0);
  top
}

/// Z==X 的带符号加法 (Toom 插值用), 返回结果是否为负
pub fn add_signed_zx(zx: &mut [Digit], x_negative: bool, y: &[Digit], y_negative: bool) -> bool {
  if x_negative == y_negative {
    add_zx(zx, y);
    return x_negative;
  }
  let y = norm(y);
  // 比较 zx 与 y: 先规范 zx 视图
  let zx_norm = norm(zx);
  if compare_no_normalize(zx_norm, y) >= 0 {
    subtract_zx(zx, y);
    x_negative
  } else {
    // 结果 = y - zx, 就地翻转: 逐位 y[i] - zx[i]
    let mut borrow = 0;
    for i in 0..y.len() {
      let (v, b) = digit_sub2(y[i], zx[i], borrow);
      zx[i] = v;
      borrow = b;
    }
    debug_assert!(borrow == 0);
    let mut i = y.len();
    while i < zx.len() {
      zx[i] = 0;
      i += 1;
    }
    !x_negative
  }
}

/// Z==X 的带符号减法, 返回结果是否为负
pub fn subtract_signed_zx(
  zx: &mut [Digit],
  x_negative: bool,
  y: &[Digit],
  y_negative: bool,
) -> bool {
  if x_negative != y_negative {
    add_zx(zx, y);
    return x_negative;
  }
  let y = norm(y);
  let zx_norm = norm(zx);
  if compare_no_normalize(zx_norm, y) >= 0 {
    subtract_zx(zx, y);
    x_negative
  } else {
    let mut borrow = 0;
    for i in 0..y.len() {
      let (v, b) = digit_sub2(y[i], zx[i], borrow);
      zx[i] = v;
      borrow = b;
    }
    debug_assert!(borrow == 0);
    let mut i = y.len();
    while i < zx.len() {
      zx[i] = 0;
      i += 1;
    }
    !x_negative
  }
}

/// Z==Y 的带符号减法 (Toom 插值用), 返回结果是否为负
pub fn subtract_signed_zy(
  zy: &mut [Digit],
  x: &[Digit],
  x_negative: bool,
  y_negative: bool,
) -> bool {
  if x_negative != y_negative {
    // Add(Z, X, Y) 就地: z += x
    add_zx(zy, x);
    return x_negative;
  }
  let x = norm(x);
  let zy_norm = norm(zy);
  if compare_no_normalize(x, zy_norm) >= 0 {
    // Subtract(Z, X, Y): z = x - z (旧值)
    let mut borrow = 0;
    for j in 0..x.len() {
      let (v, b) = digit_sub2(x[j], zy[j], borrow);
      zy[j] = v;
      borrow = b;
    }
    debug_assert!(borrow == 0);
    let mut j = x.len();
    while j < zy.len() {
      zy[j] = 0;
      j += 1;
    }
    x_negative
  } else {
    // Subtract(Z, Y, X): z -= x 就地
    subtract_zx(zy, x);
    !y_negative
  }
}

/// Z := X + 1
pub fn add_one(z: &mut [Digit], x: &[Digit]) {
  let mut carry = 1;
  let mut i = 0;
  while carry > 0 && i < x.len() {
    let (v, c) = digit_add2(x[i], carry);
    z[i] = v;
    carry = c;
    i += 1;
  }
  if carry > 0 {
    z[i] = carry;
    i += 1;
  }
  while i < x.len() {
    z[i] = x[i];
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

/// Z := X - 1
pub fn subtract_one(z: &mut [Digit], x: &[Digit]) {
  let mut borrow = 1;
  let mut i = 0;
  while borrow > 0 {
    let (v, b) = digit_sub(x[i], borrow);
    z[i] = v;
    borrow = b;
    i += 1;
  }
  while i < x.len() {
    z[i] = x[i];
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

/// X += y (单 digit)
pub fn add_digit(x: &mut [Digit], y: Digit) {
  let mut carry = y;
  let mut i = 0;
  loop {
    let (v, c) = digit_add2(x[i], carry);
    x[i] = v;
    carry = c;
    i += 1;
    if carry == 0 {
      break;
    }
  }
}

/// X -= y (单 digit)
pub fn subtract_digit(x: &mut [Digit], y: Digit) {
  let mut borrow = y;
  let mut i = 0;
  loop {
    let (v, b) = digit_sub(x[i], borrow);
    x[i] = v;
    borrow = b;
    i += 1;
    if borrow == 0 {
      break;
    }
  }
}

/// Z += X, 返回进位 (0/1)
pub fn inplace_add_and_return_carry(z: &mut [Digit], x: &[Digit]) -> Digit {
  let mut carry = 0;
  for i in 0..x.len() {
    let (v, c) = digit_add3(z[i], x[i], carry);
    z[i] = v;
    carry = c;
  }
  carry
}

/// Z -= X, 返回借位 (0/1)
pub fn inplace_sub_and_return_borrow(z: &mut [Digit], x: &[Digit]) -> Digit {
  let mut borrow = 0;
  for i in 0..x.len() {
    let (v, b) = digit_sub2(z[i], x[i], borrow);
    z[i] = v;
    borrow = b;
  }
  borrow
}

/// Z[i] = X[i] + Y[i] (只算 Y.len 列), 返回进位
pub fn add_and_return_carry(z: &mut [Digit], x: &[Digit], y: &[Digit]) -> Digit {
  debug_assert!(z.len() >= y.len() && x.len() >= y.len());
  let mut carry = 0;
  for i in 0..y.len() {
    let (v, c) = digit_add3(x[i], y[i], carry);
    z[i] = v;
    carry = c;
  }
  carry
}

/// Z[i] = X[i] - Y[i] (只算 Y.len 列), 返回借位
pub fn subtract_and_return_borrow(z: &mut [Digit], x: &[Digit], y: &[Digit]) -> Digit {
  debug_assert!(z.len() >= y.len() && x.len() >= y.len());
  let mut borrow = 0;
  for i in 0..y.len() {
    let (v, b) = digit_sub2(x[i], y[i], borrow);
    z[i] = v;
    borrow = b;
  }
  borrow
}

// ---------------------------------------------------------------------------
// 按位运算 (输入为绝对值, 负数按补码语义)
// ---------------------------------------------------------------------------

pub fn bitwise_and_pos_pos(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  let pairs = x.len().min(y.len());
  debug_assert!(z.len() >= pairs);
  let mut i = 0;
  while i < pairs {
    z[i] = x[i] & y[i];
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

pub fn bitwise_and_neg_neg(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  // (-x) & (-y) == -(((x-1) | (y-1)) + 1)
  let pairs = x.len().min(y.len());
  let mut x_borrow = 1;
  let mut y_borrow = 1;
  let mut i = 0;
  while i < pairs {
    let (vx, bx) = digit_sub(x[i], x_borrow);
    let (vy, by) = digit_sub(y[i], y_borrow);
    z[i] = vx | vy;
    x_borrow = bx;
    y_borrow = by;
    i += 1;
  }
  // 两个尾巴循环至少一个零次
  while i < x.len() {
    let (v, b) = digit_sub(x[i], x_borrow);
    z[i] = v;
    x_borrow = b;
    i += 1;
  }
  while i < y.len() {
    let (v, b) = digit_sub(y[i], y_borrow);
    z[i] = v;
    y_borrow = b;
    i += 1;
  }
  debug_assert!(x_borrow == 0);
  debug_assert!(y_borrow == 0);
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
  add_digit(z, 1);
}

pub fn bitwise_and_pos_neg(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  // x & (-y) == x & ~(y-1)
  let pairs = x.len().min(y.len());
  let mut borrow = 1;
  let mut i = 0;
  while i < pairs {
    let (v, b) = digit_sub(y[i], borrow);
    z[i] = x[i] & !v;
    borrow = b;
    i += 1;
  }
  while i < x.len() {
    z[i] = x[i];
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

pub fn bitwise_or_pos_pos(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  let pairs = x.len().min(y.len());
  let mut i = 0;
  while i < pairs {
    z[i] = x[i] | y[i];
    i += 1;
  }
  while i < x.len() {
    z[i] = x[i];
    i += 1;
  }
  while i < y.len() {
    z[i] = y[i];
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

pub fn bitwise_or_neg_neg(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  // (-x) | (-y) == -(((x-1) & (y-1)) + 1)
  let pairs = x.len().min(y.len());
  let mut x_borrow = 1;
  let mut y_borrow = 1;
  let mut i = 0;
  while i < pairs {
    let (vx, bx) = digit_sub(x[i], x_borrow);
    let (vy, by) = digit_sub(y[i], y_borrow);
    z[i] = vx & vy;
    x_borrow = bx;
    y_borrow = by;
    i += 1;
  }
  // 残余借位无影响: '&' 会将其抹掉
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
  add_digit(z, 1);
}

pub fn bitwise_or_pos_neg(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  // x | (-y) == -(((y-1) &! x) + 1)
  let pairs = x.len().min(y.len());
  let mut borrow = 1;
  let mut i = 0;
  while i < pairs {
    let (v, b) = digit_sub(y[i], borrow);
    z[i] = v & !x[i];
    borrow = b;
    i += 1;
  }
  while i < y.len() {
    let (v, b) = digit_sub(y[i], borrow);
    z[i] = v;
    borrow = b;
    i += 1;
  }
  debug_assert!(borrow == 0);
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
  add_digit(z, 1);
}

pub fn bitwise_xor_pos_pos(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  // 保证 x 是较短的一方 (原版按需交换)
  let (x, y) = if y.len() < x.len() { (y, x) } else { (x, y) };
  debug_assert!(x.len() <= y.len());
  let mut i = 0;
  while i < x.len() {
    z[i] = x[i] ^ y[i];
    i += 1;
  }
  while i < y.len() {
    z[i] = y[i];
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

pub fn bitwise_xor_neg_neg(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  // (-x) ^ (-y) == (x-1) ^ (y-1)
  let pairs = x.len().min(y.len());
  let mut x_borrow = 1;
  let mut y_borrow = 1;
  let mut i = 0;
  while i < pairs {
    let (vx, bx) = digit_sub(x[i], x_borrow);
    let (vy, by) = digit_sub(y[i], y_borrow);
    z[i] = vx ^ vy;
    x_borrow = bx;
    y_borrow = by;
    i += 1;
  }
  while i < x.len() {
    let (v, b) = digit_sub(x[i], x_borrow);
    z[i] = v;
    x_borrow = b;
    i += 1;
  }
  while i < y.len() {
    let (v, b) = digit_sub(y[i], y_borrow);
    z[i] = v;
    y_borrow = b;
    i += 1;
  }
  debug_assert!(x_borrow == 0);
  debug_assert!(y_borrow == 0);
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
}

pub fn bitwise_xor_pos_neg(z: &mut [Digit], x: &[Digit], y: &[Digit]) {
  // x ^ (-y) == -((x ^ (y-1)) + 1)
  let pairs = x.len().min(y.len());
  let mut borrow = 1;
  let mut i = 0;
  while i < pairs {
    let (v, b) = digit_sub(y[i], borrow);
    z[i] = x[i] ^ v;
    borrow = b;
    i += 1;
  }
  while i < x.len() {
    z[i] = x[i];
    i += 1;
  }
  while i < y.len() {
    let (v, b) = digit_sub(y[i], borrow);
    z[i] = v;
    borrow = b;
    i += 1;
  }
  debug_assert!(borrow == 0);
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
  add_digit(z, 1);
}

// ---------------------------------------------------------------------------
// 移位
// ---------------------------------------------------------------------------

/// Z := X << shift (shift 为任意位宽)
pub fn left_shift(z: &mut [Digit], x: &[Digit], shift: Digit) {
  let digit_shift = (shift / K_DIGIT_BITS as u64) as usize;
  let bits_shift = (shift % K_DIGIT_BITS as u64) as u32;

  let mut i = 0;
  while i < digit_shift {
    z[i] = 0;
    i += 1;
  }
  if bits_shift == 0 {
    while i < x.len() + digit_shift {
      z[i] = x[i - digit_shift];
      i += 1;
    }
    while i < z.len() {
      z[i] = 0;
      i += 1;
    }
  } else {
    let mut carry = 0;
    while i < x.len() + digit_shift {
      let d = x[i - digit_shift];
      z[i] = (d << bits_shift) | carry;
      carry = d >> (K_DIGIT_BITS - bits_shift);
      i += 1;
    }
    if carry != 0 {
      z[i] = carry;
      i += 1;
    }
    while i < z.len() {
      z[i] = 0;
      i += 1;
    }
  }
}

/// RightShift 的预计算状态
#[derive(Debug, Clone, Copy, Default)]
pub struct RightShiftState {
  pub must_round_down: bool,
}

/// Z := X >> shift, state 由 right_shift_result_length 预填
pub fn right_shift(z: &mut [Digit], x: &[Digit], shift: Digit, state: &RightShiftState) {
  let digit_shift = (shift / K_DIGIT_BITS as u64) as usize;
  let bits_shift = (shift % K_DIGIT_BITS as u64) as u32;

  let mut i = 0;
  if bits_shift == 0 {
    while i < x.len() - digit_shift {
      z[i] = x[i + digit_shift];
      i += 1;
    }
  } else {
    let mut carry = x[digit_shift] >> bits_shift;
    while i < x.len() - digit_shift - 1 {
      let d = x[i + digit_shift + 1];
      z[i] = (d << (K_DIGIT_BITS - bits_shift)) | carry;
      carry = d >> bits_shift;
      i += 1;
    }
    z[i] = carry;
    i += 1;
  }
  while i < z.len() {
    z[i] = 0;
    i += 1;
  }
  if state.must_round_down {
    // 负数向下取整 = 绝对值加一, 不会溢出
    add_digit(z, 1);
  }
}

/// 右移结果长度预算; state 可为 None
pub fn right_shift_result_length(
  x: &[Digit],
  x_sign: bool,
  shift: Digit,
  state: Option<&mut RightShiftState>,
) -> u32 {
  let digit_shift = (shift / K_DIGIT_BITS as u64) as usize;
  let bits_shift = (shift % K_DIGIT_BITS as u64) as u32;
  if x.len() <= digit_shift {
    return 0;
  }
  let mut result_length = (x.len() - digit_shift) as u32;

  // 负数: 只要有位被移出就需向下取整 (如 -5n >> 1n == -3n)
  let mut must_round_down = false;
  if x_sign {
    let mask = (1u64 << bits_shift) - 1;
    if (x[digit_shift] & mask) != 0 {
      must_round_down = true;
    } else {
      for &d in &x[..digit_shift] {
        if d != 0 {
          must_round_down = true;
          break;
        }
      }
    }
  }
  // bits_shift 非 0 时会腾出位, 不会进位
  if must_round_down && bits_shift == 0 {
    // 最高位全 1 时才可能进位
    if digit_ismax(msd(x)) {
      result_length += 1;
    }
  }
  if let Some(s) = state {
    debug_assert!(!must_round_down || x_sign);
    s.must_round_down = must_round_down;
  }
  result_length
}

// ---------------------------------------------------------------------------
// 截断到 n 位 (asIntN / asUintN)
// ---------------------------------------------------------------------------

/// Z := X 的低 n 位
pub fn truncate_to_n_bits(z: &mut [Digit], x: &[Digit], n: u32) {
  if n == 0 {
    return;
  }
  let digits = div_ceil(n, K_DIGIT_BITS) as usize;
  let bits = n % K_DIGIT_BITS;
  let last = digits - 1;
  // 除最高位外直接复制
  z[..last].copy_from_slice(&x[..last]);
  // 最高位可能含多余位
  let mut m = x[last];
  if bits != 0 {
    let drop = K_DIGIT_BITS - bits;
    m = (m << drop) >> drop;
  }
  z[last] = m;
}

/// Z := 2^n - (X 的低 n 位)
pub fn truncate_and_sub_from_power_of_two(z: &mut [Digit], x: &[Digit], n: u32) {
  if n == 0 {
    return;
  }
  let digits = div_ceil(n, K_DIGIT_BITS) as usize;
  let bits = n % K_DIGIT_BITS;
  let last = digits - 1;
  let have_x = last.min(x.len());
  let mut borrow = 0;
  let mut i = 0;
  while i < have_x {
    let (v, b) = digit_sub2(0, x[i], borrow);
    z[i] = v;
    borrow = b;
    i += 1;
  }
  while i < last {
    let (v, b) = digit_sub(0, borrow);
    z[i] = v;
    borrow = b;
    i += 1;
  }
  // 最高位可能含多余位
  let mut m = if last < x.len() { x[last] } else { 0 };
  if bits == 0 {
    // 借位出 == 1 属正常 (低 n 位非零时, 虚拟 2^n 顶位吸收借位, 见原版同分支无断言)
    let (v, _b) = digit_sub2(0, m, borrow);
    z[last] = v;
  } else {
    let drop = K_DIGIT_BITS - bits;
    m = (m << drop) >> drop;
    let minuend_msd = 1u64 << bits;
    let (result_msd, b) = digit_sub2(minuend_msd, m, borrow);
    debug_assert!(b == 0); // 结果 < 2^n
    // 若被减各位全零, 需抹掉物化的 minuend_msd
    z[last] = result_msd & (minuend_msd - 1);
  }
}

/// Z := X 低 n 位按有符号解释, 返回结果是否为负
pub fn as_int_n(z: &mut [Digit], x: &[Digit], x_negative: bool, n: u32) -> bool {
  debug_assert!(!x.is_empty());
  debug_assert!(n > 0);
  let needed_digits = div_ceil(n, K_DIGIT_BITS) as usize;
  let top_digit = x[needed_digits - 1];
  let compare_digit = 1u64 << ((n - 1) % K_DIGIT_BITS);
  // (n-1) 位为 0: 截断绝对值保号; 为 1: 用 2^n 减并翻号 (输入为负且恰为最小 n 位整数时除外)
  let has_bit = (top_digit & compare_digit) == compare_digit;
  if !has_bit {
    truncate_to_n_bits(z, x, n);
    return x_negative;
  }
  truncate_and_sub_from_power_of_two(z, x, n);
  if !x_negative {
    return true;
  }
  // 特例扫描: 低于 (n-1) 位全零则结果为负 (如 asIntN(3, -12) => -4)
  if (top_digit & (compare_digit - 1)) != 0 {
    return false;
  }
  for &d in &x[..needed_digits - 1] {
    if d != 0 {
      return false;
    }
  }
  true
}

/// 有符号截断结果长度; 无变化返回 -1
pub fn as_int_n_result_length(x: &[Digit], x_negative: bool, n: u32) -> i32 {
  if n == 0 {
    return if x.is_empty() { -1 } else { 0 };
  }
  let needed_digits = div_ceil(n, K_DIGIT_BITS) as usize;
  if x.len() < needed_digits {
    return -1;
  }
  if x.len() > needed_digits {
    return needed_digits as i32;
  }
  let top_digit = x[needed_digits - 1];
  let compare_digit = 1u64 << ((n - 1) % K_DIGIT_BITS);
  if top_digit < compare_digit {
    return -1;
  }
  if top_digit > compare_digit {
    return needed_digits as i32;
  }
  // 特例: X == -2^(n-1) 时截断是空操作
  if !x_negative {
    return needed_digits as i32;
  }
  for &d in &x[..needed_digits - 1] {
    if d != 0 {
      return needed_digits as i32;
    }
  }
  -1
}

/// 无符号截断 (正输入) 结果长度; 无变化返回 -1
pub fn as_uint_n_pos_result_length(x: &[Digit], n: u32) -> i32 {
  let needed_digits = div_ceil(n, K_DIGIT_BITS) as usize;
  if x.len() < needed_digits {
    return -1;
  }
  if x.len() > needed_digits {
    return needed_digits as i32;
  }
  let bits_in_top_digit = n % K_DIGIT_BITS;
  if bits_in_top_digit == 0 {
    return -1;
  }
  let top_digit = x[needed_digits - 1];
  if (top_digit >> bits_in_top_digit) == 0 {
    return -1;
  }
  needed_digits as i32
}

/// Z := X 低 n 位 (正输入)
pub fn as_uint_n_pos(z: &mut [Digit], x: &[Digit], n: u32) {
  debug_assert!(as_uint_n_pos_result_length(x, n) > 0);
  truncate_to_n_bits(z, x, n);
}

/// Z := 2^n - (负输入绝对值低 n 位)
pub fn as_uint_n_neg(z: &mut [Digit], x: &[Digit], n: u32) {
  truncate_and_sub_from_power_of_two(z, x, n);
}

// ---------------------------------------------------------------------------
// "慢" 操作的小输入快路径
// ---------------------------------------------------------------------------

/// Z := X * Y 小输入路径; 返回 (是否已处理, Z 最高位)
pub fn multiply_small(z: &mut [Digit], x: &[Digit], y: &[Digit]) -> (bool, Digit) {
  debug_assert!(is_digit_normalized(x));
  debug_assert!(is_digit_normalized(y));
  if x.is_empty() || y.is_empty() {
    z.fill(0);
    return (true, 0);
  }
  let (x, y) = if x.len() < y.len() { (y, x) } else { (x, y) };
  if y.len() == 1 {
    let top = multiply_single(z, x, y[0]);
    return (true, top);
  }
  if y.len() as u32 >= K_KARATSUBA_THRESHOLD {
    return (false, 0);
  }
  let top = multiply_schoolbook(z, x, y);
  (true, top)
}

/// 单 digit 除法: Q = A / b, R = A % b; 返回 (商, 余数)
pub fn divide_single(q: &mut [Digit], a: &[Digit], b: Digit) -> (Digit, Digit) {
  assert!(b != 0, "division by zero");
  debug_assert!(!a.is_empty());
  debug_assert!(q.len() >= a.len());
  let mut rem = 0;
  let mut top;
  let length = a.len();
  if a[length - 1] >= b {
    debug_assert!(q.len() >= a.len());
    let mut i = length - 1;
    let (v, r) = digit_div(rem, a[i], b);
    q[i] = v;
    top = v;
    rem = r;
    while i > 0 {
      i -= 1;
      let (v, r) = digit_div(rem, a[i], b);
      q[i] = v;
      rem = r;
    }
    for qj in q[length..].iter_mut() {
      *qj = 0;
      top = 0;
    }
  } else {
    debug_assert!(q.len() + 1 >= a.len());
    rem = a[length - 1];
    top = 0;
    if length >= 2 {
      let mut i = length - 2;
      let (v, r) = digit_div(rem, a[i], b);
      q[i] = v;
      top = v;
      rem = r;
      while i > 0 {
        i -= 1;
        let (v, r) = digit_div(rem, a[i], b);
        q[i] = v;
        rem = r;
      }
    }
    for qj in q[length - 1..].iter_mut() {
      *qj = 0;
      top = 0;
    }
  }
  (top, rem)
}

/// A % b (单 digit)
pub fn mod_single(a: &[Digit], b: Digit) -> Digit {
  assert!(b != 0, "division by zero");
  debug_assert!(!a.is_empty());
  let mut rem = 0;
  let length = a.len();
  if a[length - 1] >= b {
    for &d in a[..].iter().rev() {
      let (_, r) = digit_div(rem, d, b);
      rem = r;
    }
  } else {
    rem = a[length - 1];
    for &d in a[..length - 1].iter().rev() {
      let (_, r) = digit_div(rem, d, b);
      rem = r;
    }
  }
  rem
}

/// Q := A / B 小输入路径; 返回 (是否已处理, Q 最高位)
pub fn divide_small(q: &mut [Digit], a: &[Digit], b: &[Digit]) -> (bool, Digit) {
  assert!(!norm(b).is_empty(), "division by zero");
  debug_assert!(is_digit_normalized(a));
  debug_assert!(is_digit_normalized(b));
  let cmp = compare_no_normalize(a, b);
  if cmp < 0 {
    q.fill(0);
    return (true, 0);
  }
  if cmp == 0 {
    q[0] = 1;
    let mut top = 1;
    for d in q[1..].iter_mut() {
      *d = 0;
      top = 0;
    }
    return (true, top);
  }
  if b.len() == 1 {
    let (top, _) = divide_single(q, a, b[0]);
    return (true, top);
  }
  (false, 0)
}

/// R := A % B 小输入路径; 返回 (是否已处理, R 最高位)
pub fn modulo_small(r: &mut [Digit], a: &[Digit], b: &[Digit]) -> (bool, Digit) {
  assert!(!norm(b).is_empty(), "division by zero");
  debug_assert!(is_digit_normalized(a));
  debug_assert!(is_digit_normalized(b));

  let cmp = compare_no_normalize(a, b);
  if cmp < 0 {
    let mut top = 0;
    for i in 0..a.len() {
      r[i] = a[i];
      top = a[i];
    }
    for ri in r[a.len()..].iter_mut() {
      *ri = 0;
      top = 0;
    }
    return (true, top);
  }
  if cmp == 0 {
    r.fill(0);
    return (true, 0);
  }
  if b.len() == 1 {
    let mut top = mod_single(a, b[0]);
    r[0] = top;
    // 原版循环体内 R[i] = top = 0: R 多余高位清零的同时把返回的 top 置 0
    for d in r[1..].iter_mut() {
      *d = 0;
      top = 0;
    }
    return (true, top);
  }
  (false, 0)
}

// ---------------------------------------------------------------------------
// 结果长度预算 (原版 bigint.h)
// ---------------------------------------------------------------------------

#[inline]
pub fn add_result_length(x_length: u32, y_length: u32) -> u32 {
  x_length.max(y_length) + 1
}

#[inline]
pub fn add_signed_result_length(x_length: u32, y_length: u32, same_sign: bool) -> u32 {
  if same_sign {
    add_result_length(x_length, y_length)
  } else {
    x_length.max(y_length)
  }
}

#[inline]
pub fn subtract_result_length(x_length: u32, _y_length: u32) -> u32 {
  x_length
}

#[inline]
pub fn subtract_signed_result_length(x_length: u32, y_length: u32, same_sign: bool) -> u32 {
  if same_sign {
    x_length.max(y_length)
  } else {
    add_result_length(x_length, y_length)
  }
}

#[inline]
pub fn multiply_result_length(x: &[Digit], y: &[Digit]) -> u32 {
  (x.len() + y.len()) as u32
}

/// Barrett 需要一格临时空间
#[inline]
pub fn divide_result_length(a: &[Digit], b: &[Digit]) -> u32 {
  if a.len() < b.len() {
    return 1;
  }
  let barrett_extra_scratch = if b.len() as u32 >= K_BARRETT_THRESHOLD {
    1
  } else {
    0
  };
  (a.len() - b.len()) as u32 + 1 + barrett_extra_scratch
}

#[inline]
pub fn modulo_result_length(b: &[Digit]) -> u32 {
  b.len() as u32
}

#[inline]
pub fn as_uint_n_neg_result_length(n: u32) -> u32 {
  div_ceil(n, K_DIGIT_BITS)
}
