//! 原版 v8/src/base/numbers/bignum.h/cc — 28-bit bigit 大数 (dtoa 慢速路径)

use std::cmp::{max, min};

use crate::{immediate_crash::debug_unreachable, strings::hex_char_of_value};

/// Chunk = uint32_t
type Chunk = u32;
/// DoubleChunk = uint64_t
type DoubleChunk = u64;

/// kMaxSignificantBits = 128 * 28, 可精确编码 2^3584 > 10^1000
pub const MAX_SIGNIFICANT_BITS: i32 = 3584;
const CHUNK_SIZE: i32 = Chunk::BITS as i32;
// DoubleChunk 位宽 64: 仅用于 DCHECK (64 > 28+32 恒真), 常量本体不引入
const BIGIT_SIZE: i32 = 28;
const BIGIT_MASK: Chunk = (1 << BIGIT_SIZE) - 1;
const BIGIT_CAPACITY: usize = (MAX_SIGNIFICANT_BITS / BIGIT_SIZE) as usize;

/// Bignum — 栈上定容量 (不可增长, 与原版一致)
#[derive(Clone)]
pub struct Bignum {
  bigits_buffer_: [Chunk; BIGIT_CAPACITY],
  used_digits_: i32,
  /// 值 = bigits_ × 2^(exponent_ × kBigitSize)
  exponent_: i32,
}

impl Default for Bignum {
  fn default() -> Self {
    Self::new()
  }
}

impl Bignum {
  #[must_use]
  pub fn new() -> Self {
    Self {
      bigits_buffer_: [0; BIGIT_CAPACITY],
      used_digits_: 0,
      exponent_: 0,
    }
  }

  fn ensure_capacity(&self, size: i32) {
    assert!(
      usize::try_from(size).unwrap_or(usize::MAX) <= BIGIT_CAPACITY,
      "容量不足"
    );
  }

  fn zero(&mut self) {
    for i in 0..self.used_digits_ {
      self.bigits_buffer_[i as usize] = 0;
    }
    self.used_digits_ = 0;
    self.exponent_ = 0;
  }

  #[must_use]
  pub fn is_zero(&self) -> bool {
    self.used_digits_ == 0
  }

  pub fn assign_uint16(&mut self, value: u16) {
    // BIGIT_SIZE(28) >= 16: 编译期常量性质, 注释保留原版 DCHECK 语义
    self.zero();
    if value == 0 {
      return;
    }
    self.ensure_capacity(1);
    self.bigits_buffer_[0] = u32::from(value);
    self.used_digits_ = 1;
  }

  pub fn assign_uint64(&mut self, value: u64) {
    const K_UINT64_SIZE: i32 = 64;
    self.zero();
    if value == 0 {
      return;
    }
    let needed_bigits = K_UINT64_SIZE / BIGIT_SIZE + 1;
    self.ensure_capacity(needed_bigits);
    let mut value = value;
    for i in 0..needed_bigits {
      self.bigits_buffer_[i as usize] = (value & u64::from(BIGIT_MASK)) as Chunk;
      value >>= BIGIT_SIZE as u32;
    }
    self.used_digits_ = needed_bigits;
    self.clamp();
  }

  pub fn assign_bignum(&mut self, other: &Bignum) {
    self.exponent_ = other.exponent_;
    for i in 0..other.used_digits_ {
      let v = other.bigits_buffer_[i as usize];
      self.bigits_buffer_[i as usize] = v;
    }
    // 清除多余位
    for i in other.used_digits_..self.used_digits_ {
      self.bigits_buffer_[i as usize] = 0;
    }
    self.used_digits_ = other.used_digits_;
  }

  /// AssignDecimalString — 每 19 位十进制一组 (2^64 > 10^19)
  pub fn assign_decimal_string(&mut self, value: &[u8]) {
    const MAX_UINT64_DECIMAL_DIGITS: i32 = 19;
    self.zero();
    let mut length = value.len() as i32;
    let mut pos = 0;
    // 每位按 4 bit 估算
    while length >= MAX_UINT64_DECIMAL_DIGITS {
      let digits = read_uint64(value, pos, MAX_UINT64_DECIMAL_DIGITS);
      pos += MAX_UINT64_DECIMAL_DIGITS;
      length -= MAX_UINT64_DECIMAL_DIGITS;
      self.multiply_by_power_of_ten(MAX_UINT64_DECIMAL_DIGITS);
      self.add_uint64(digits);
    }
    let digits = read_uint64(value, pos, length);
    self.multiply_by_power_of_ten(length);
    self.add_uint64(digits);
    self.clamp();
  }

  /// AssignHexString
  pub fn assign_hex_string(&mut self, value: &[u8]) {
    self.zero();
    let length = value.len() as i32;
    let needed_bigits = length * 4 / BIGIT_SIZE + 1;
    self.ensure_capacity(needed_bigits);
    let mut string_index = length - 1;
    for i in 0..needed_bigits - 1 {
      // 这些 bigit 一定是满的
      let mut current_bigit: Chunk = 0;
      for j in 0..BIGIT_SIZE / 4 {
        current_bigit += (hex_char_value(value[string_index as usize]) as Chunk) << (j * 4);
        string_index -= 1;
      }
      self.bigits_buffer_[i as usize] = current_bigit;
    }
    self.used_digits_ = needed_bigits - 1;

    let mut most_significant_bigit: Chunk = 0;
    for j in 0..=string_index {
      most_significant_bigit <<= 4;
      most_significant_bigit += hex_char_value(value[j as usize]) as Chunk;
    }
    if most_significant_bigit != 0 {
      self.bigits_buffer_[self.used_digits_ as usize] = most_significant_bigit;
      self.used_digits_ += 1;
    }
    self.clamp();
  }

  pub fn add_uint16(&mut self, operand: u16) {
    self.add_uint64(u64::from(operand));
  }

  pub fn add_uint64(&mut self, operand: u64) {
    if operand == 0 {
      return;
    }
    let mut other = Bignum::new();
    other.assign_uint64(operand);
    self.add_bignum(&other);
  }

  pub fn add_bignum(&mut self, other: &Bignum) {
    debug_assert!(self.is_clamped());
    debug_assert!(other.is_clamped());

    // 若 this 指数更大, 先补 0-bigit; 之后 exponent_ <= other.exponent_
    self.align(other);

    let self_len = self.bigit_length();
    let other_len = other.bigit_length();
    let cap = 1 + max(self_len, other_len) - self.exponent_;
    self.ensure_capacity(cap);
    let mut carry: Chunk = 0;
    let mut bigit_pos = other.exponent_ - self.exponent_;
    debug_assert!(bigit_pos >= 0);
    for i in 0..other.used_digits_ {
      let sum = u64::from(self.bigits_buffer_[bigit_pos as usize])
        + u64::from(other.bigits_buffer_[i as usize])
        + u64::from(carry);
      self.bigits_buffer_[bigit_pos as usize] = (sum & u64::from(BIGIT_MASK)) as Chunk;
      carry = (sum >> BIGIT_SIZE as u32) as Chunk;
      bigit_pos += 1;
    }

    while carry != 0 {
      let sum = u64::from(self.bigits_buffer_[bigit_pos as usize]) + u64::from(carry);
      self.bigits_buffer_[bigit_pos as usize] = (sum & u64::from(BIGIT_MASK)) as Chunk;
      carry = (sum >> BIGIT_SIZE as u32) as Chunk;
      bigit_pos += 1;
    }
    self.used_digits_ = max(bigit_pos, self.used_digits_);
    debug_assert!(self.is_clamped());
  }

  /// 前提: self >= other
  pub fn subtract_bignum(&mut self, other: &Bignum) {
    debug_assert!(self.is_clamped());
    debug_assert!(other.is_clamped());
    debug_assert!(Self::less_equal(other, self));

    self.align(other);

    let offset = other.exponent_ - self.exponent_;
    let mut borrow: Chunk = 0;
    let mut i: i32 = 0;
    while i < other.used_digits_ {
      debug_assert!(borrow == 0 || borrow == 1);
      let idx = (i + offset) as usize;
      // 与原版一致: uint32 回绕减法, 借位取差值最高位
      let difference = self.bigits_buffer_[idx]
        .wrapping_sub(other.bigits_buffer_[i as usize])
        .wrapping_sub(borrow);
      self.bigits_buffer_[idx] = difference & BIGIT_MASK;
      borrow = difference >> (CHUNK_SIZE - 1);
      i += 1;
    }
    while borrow != 0 {
      let idx = (i + offset) as usize;
      let difference = self.bigits_buffer_[idx].wrapping_sub(borrow);
      self.bigits_buffer_[idx] = difference & BIGIT_MASK;
      borrow = difference >> (CHUNK_SIZE - 1);
      i += 1;
    }
    self.clamp();
  }

  pub fn shift_left(&mut self, shift_amount: i32) {
    if self.used_digits_ == 0 {
      return;
    }
    self.exponent_ += shift_amount / BIGIT_SIZE;
    let local_shift = shift_amount % BIGIT_SIZE;
    self.ensure_capacity(self.used_digits_ + 1);
    self.bigits_shift_left(local_shift);
  }

  pub fn multiply_by_uint32(&mut self, factor: u32) {
    if factor == 1 {
      return;
    }
    if factor == 0 {
      self.zero();
      return;
    }
    if self.used_digits_ == 0 {
      return;
    }

    // bigit×factor 为 kBigitSize+32 位, 加进位仍适合 DoubleChunk
    // (DOUBLE_CHUNK_SIZE(64) > BIGIT_SIZE+32(60): 常量性质, 注释保留原版 DCHECK)
    let mut carry: DoubleChunk = 0;
    for i in 0..self.used_digits_ {
      let product = u64::from(factor) * u64::from(self.bigits_buffer_[i as usize]) + carry;
      self.bigits_buffer_[i as usize] = (product & u64::from(BIGIT_MASK)) as Chunk;
      carry = product >> BIGIT_SIZE as u32;
    }
    while carry != 0 {
      self.ensure_capacity(self.used_digits_ + 1);
      self.bigits_buffer_[self.used_digits_ as usize] = (carry & u64::from(BIGIT_MASK)) as Chunk;
      self.used_digits_ += 1;
      carry >>= BIGIT_SIZE as u32;
    }
  }

  pub fn multiply_by_uint64(&mut self, factor: u64) {
    if factor == 1 {
      return;
    }
    if factor == 0 {
      self.zero();
      return;
    }
    // BIGIT_SIZE(28) < 32: 编译期常量性质, 注释保留原版 DCHECK 语义
    let mut carry: u64 = 0;
    let low = factor & 0xFFFF_FFFF;
    let high = factor >> 32;
    for i in 0..self.used_digits_ {
      let product_low = low * u64::from(self.bigits_buffer_[i as usize]);
      let product_high = high * u64::from(self.bigits_buffer_[i as usize]);
      let tmp = (carry & u64::from(BIGIT_MASK)) + product_low;
      self.bigits_buffer_[i as usize] = (tmp & u64::from(BIGIT_MASK)) as Chunk;
      carry = (carry >> BIGIT_SIZE as u32)
        + (tmp >> BIGIT_SIZE as u32)
        + (product_high << (32 - BIGIT_SIZE) as u32);
    }
    while carry != 0 {
      self.ensure_capacity(self.used_digits_ + 1);
      self.bigits_buffer_[self.used_digits_ as usize] = (carry & u64::from(BIGIT_MASK)) as Chunk;
      self.used_digits_ += 1;
      carry >>= BIGIT_SIZE as u32;
    }
  }

  pub fn multiply_by_power_of_ten(&mut self, exponent: i32) {
    const FIVE_27: u64 = 0x6765_C793_FA10_079D;
    const FIVE_13: u32 = 1_220_703_125; // 5^13
    #[rustfmt::skip]
    const FIVE_1_TO_12: [u32; 12] = [5, 25, 125, 625, 3125, 15625, 78125, 390625,
      1953125, 9765625, 48828125, 244140625];

    debug_assert!(exponent >= 0);
    if exponent == 0 {
      return;
    }
    if self.used_digits_ == 0 {
      return;
    }

    // 返回前统一做指数位移
    let mut remaining_exponent = exponent;
    while remaining_exponent >= 27 {
      self.multiply_by_uint64(FIVE_27);
      remaining_exponent -= 27;
    }
    while remaining_exponent >= 13 {
      self.multiply_by_uint32(FIVE_13);
      remaining_exponent -= 13;
    }
    if remaining_exponent > 0 {
      self.multiply_by_uint32(FIVE_1_TO_12[(remaining_exponent - 1) as usize]);
    }
    self.shift_left(exponent);
  }

  /// Times10
  pub fn times10(&mut self) {
    self.multiply_by_uint32(10);
  }

  pub fn square(&mut self) {
    debug_assert!(self.is_clamped());
    let product_length = 2 * self.used_digits_;
    self.ensure_capacity(product_length);

    // Comba 乘法: 逐列累加
    if (1 << (2 * (CHUNK_SIZE - BIGIT_SIZE))) <= self.used_digits_ {
      debug_assert!(false, "Square: used_digits 过大 (原版 UNIMPLEMENTED)");
    }
    let mut accumulator: DoubleChunk = 0;
    // 先平移数字避免覆写
    let copy_offset = self.used_digits_;
    for i in 0..self.used_digits_ {
      let v = self.bigits_buffer_[i as usize];
      self.bigits_buffer_[(copy_offset + i) as usize] = v;
    }
    for i in 0..self.used_digits_ {
      // 临时位 i 与幂 i 相乘: 两个下标之和恒为 i
      let mut bigit_index1 = i;
      let mut bigit_index2 = 0;
      while bigit_index1 >= 0 {
        let chunk1 = self.bigits_buffer_[(copy_offset + bigit_index1) as usize] as DoubleChunk;
        let chunk2 = self.bigits_buffer_[(copy_offset + bigit_index2) as usize] as DoubleChunk;
        accumulator += chunk1 * chunk2;
        bigit_index1 -= 1;
        bigit_index2 += 1;
      }
      self.bigits_buffer_[i as usize] = (accumulator as Chunk) & BIGIT_MASK;
      accumulator >>= BIGIT_SIZE as u32;
    }
    for i in self.used_digits_..product_length {
      let mut bigit_index1 = self.used_digits_ - 1;
      let mut bigit_index2 = i - bigit_index1;
      while bigit_index2 < self.used_digits_ {
        let chunk1 = self.bigits_buffer_[(copy_offset + bigit_index1) as usize] as DoubleChunk;
        let chunk2 = self.bigits_buffer_[(copy_offset + bigit_index2) as usize] as DoubleChunk;
        accumulator += chunk1 * chunk2;
        bigit_index1 -= 1;
        bigit_index2 += 1;
      }
      self.bigits_buffer_[i as usize] = (accumulator as Chunk) & BIGIT_MASK;
      accumulator >>= BIGIT_SIZE as u32;
    }
    debug_assert_eq!(accumulator, 0);

    self.used_digits_ = product_length;
    self.exponent_ *= 2;
    self.clamp();
  }

  /// base 通常 2..32 (常见 10); 左到右幂运算
  pub fn assign_power_uint16(&mut self, base: u16, power_exponent: i32) {
    debug_assert_ne!(base, 0);
    debug_assert!(power_exponent >= 0);
    if power_exponent == 0 {
      self.assign_uint16(1);
      return;
    }
    self.zero();
    let mut base = base;
    let mut shifts = 0;
    while base & 1 == 0 {
      base >>= 1;
      shifts += 1;
    }
    let mut bit_size = 0;
    let mut tmp_base = base;
    while tmp_base != 0 {
      tmp_base >>= 1;
      bit_size += 1;
    }
    let final_size = bit_size * power_exponent;
    // 位移 1 个额外 bigit, 取整再 1 个
    self.ensure_capacity(final_size / BIGIT_SIZE + 2);

    let mut mask = 1;
    while power_exponent >= mask {
      mask <<= 1;
    }
    mask >>= 2;
    let mut this_value = u64::from(base);

    let mut delayed_multiplication = false;
    const MAX_32_BITS: u64 = 0xFFFF_FFFF;
    while mask != 0 && this_value <= MAX_32_BITS {
      this_value = this_value * this_value;
      // 检查 this_value 剩余空间足够乘 base: 高 bit_size 位必须为 0
      if power_exponent & mask != 0 {
        let base_bits_mask = !((1u64 << (64 - bit_size)) - 1);
        let high_bits_zero = this_value & base_bits_mask == 0;
        if high_bits_zero {
          this_value *= u64::from(base);
        } else {
          delayed_multiplication = true;
        }
      }
      mask >>= 1;
    }
    self.assign_uint64(this_value);
    if delayed_multiplication {
      self.multiply_by_uint32(u32::from(base));
    }

    // 以 Bignum 继续同样的幂运算
    while mask != 0 {
      self.square();
      if power_exponent & mask != 0 {
        self.multiply_by_uint32(u32::from(base));
      }
      mask >>= 1;
    }

    // 补回保存的移位
    self.shift_left(shifts * power_exponent);
  }

  /// 前提: this/other < 16bit。返回商 (this 变为余数)
  pub fn divide_modulo_int_bignum(&mut self, other: &Bignum) -> u16 {
    debug_assert!(self.is_clamped());
    debug_assert!(other.is_clamped());
    debug_assert!(other.used_digits_ > 0);

    // 位数不足 → 商 0 (同时覆盖 this == 0)
    if self.bigit_length() < other.bigit_length() {
      return 0;
    }

    // align 只改 this; other 只读借用需先释放: 用 clone 传入
    self.align(other);

    let mut result: u16 = 0;

    // 先消去比除数多出的高位
    while self.bigit_length() > other.bigit_length() {
      debug_assert!(
        other.bigits_buffer_[(other.used_digits_ - 1) as usize] >= (1 << BIGIT_SIZE) / 16
      );
      // 用首位消倍数, 例 this=23, other=9 → 消 2 倍
      let top = self.bigits_buffer_[(self.used_digits_ - 1) as usize];
      result += top as u16;
      self.subtract_times(other, top as i32);
    }

    debug_assert_eq!(self.bigit_length(), other.bigit_length());

    let this_bigit = self.bigits_buffer_[(self.used_digits_ - 1) as usize];
    let other_bigit = other.bigits_buffer_[(other.used_digits_ - 1) as usize];

    if other.used_digits_ == 1 {
      // 常见情形捷径
      let quotient = this_bigit / other_bigit;
      let v = self.bigits_buffer_.as_mut_slice();
      v[(self.used_digits_ - 1) as usize] = this_bigit - other_bigit * quotient;
      result += quotient as u16;
      self.clamp();
      return result;
    }

    let division_estimate = this_bigit / (other_bigit + 1);
    result += division_estimate as u16;
    self.subtract_times(other, division_estimate as i32);

    if other_bigit * (division_estimate + 1) > this_bigit {
      // 再减必过量
      return result;
    }

    while Self::less_equal(other, self) {
      self.subtract_bignum(other);
      result += 1;
    }
    result
  }

  /// ToHexString — 写入 buffer (含终止 \0); 空间不足返回 false
  #[must_use]
  pub fn to_hex_string(&self, buffer: &mut [u8]) -> bool {
    debug_assert!(self.is_clamped());
    // BIGIT_SIZE(28) %% 4 == 0: 编译期常量性质, 注释保留原版 DCHECK 语义
    const HEX_CHARS_PER_BIGIT: i32 = BIGIT_SIZE / 4;

    if self.used_digits_ == 0 {
      if buffer.len() < 2 {
        return false;
      }
      buffer[0] = b'0';
      buffer[1] = 0;
      return true;
    }
    // 终止符占 1
    let needed_chars = (self.bigit_length() - 1) * HEX_CHARS_PER_BIGIT
      + size_in_hex_chars(self.bigits_buffer_[(self.used_digits_ - 1) as usize])
      + 1;
    if needed_chars as usize > buffer.len() {
      return false;
    }
    // 有符号游标: 写完最高位后会落到 -1 (原版 int 语义), 不能用 usize
    let mut string_index: isize = (needed_chars - 1) as isize;
    buffer[string_index as usize] = 0;
    string_index -= 1;
    for _ in 0..self.exponent_ {
      for _ in 0..HEX_CHARS_PER_BIGIT {
        buffer[string_index as usize] = b'0';
        string_index -= 1;
      }
    }
    for i in 0..self.used_digits_ - 1 {
      let mut current_bigit = self.bigits_buffer_[i as usize];
      for _ in 0..HEX_CHARS_PER_BIGIT {
        buffer[string_index as usize] = hex_char_of_value((current_bigit & 0xF) as i32) as u8;
        string_index -= 1;
        current_bigit >>= 4;
      }
    }
    let mut most_significant_bigit = self.bigits_buffer_[(self.used_digits_ - 1) as usize];
    while most_significant_bigit != 0 {
      buffer[string_index as usize] =
        hex_char_of_value((most_significant_bigit & 0xF) as i32) as u8;
      string_index -= 1;
      most_significant_bigit >>= 4;
    }
    true
  }

  #[must_use]
  fn bigit_at(&self, index: i32) -> Chunk {
    if index >= self.bigit_length() {
      return 0;
    }
    if index < self.exponent_ {
      return 0;
    }
    self.bigits_buffer_[(index - self.exponent_) as usize]
  }

  #[must_use]
  pub fn compare(a: &Bignum, b: &Bignum) -> i32 {
    debug_assert!(a.is_clamped());
    debug_assert!(b.is_clamped());
    let bigit_length_a = a.bigit_length();
    let bigit_length_b = b.bigit_length();
    if bigit_length_a < bigit_length_b {
      return -1;
    }
    if bigit_length_a > bigit_length_b {
      return 1;
    }
    let mut i = bigit_length_a - 1;
    let stop = min(a.exponent_, b.exponent_);
    while i >= stop {
      let bigit_a = a.bigit_at(i);
      let bigit_b = b.bigit_at(i);
      if bigit_a < bigit_b {
        return -1;
      }
      if bigit_a > bigit_b {
        return 1;
      }
      i -= 1;
    }
    0
  }

  #[must_use]
  pub fn equal(a: &Bignum, b: &Bignum) -> bool {
    Self::compare(a, b) == 0
  }

  #[must_use]
  pub fn less_equal(a: &Bignum, b: &Bignum) -> bool {
    Self::compare(a, b) <= 0
  }

  #[must_use]
  pub fn less(a: &Bignum, b: &Bignum) -> bool {
    Self::compare(a, b) < 0
  }

  /// Compare(a + b, c)
  #[must_use]
  pub fn plus_compare(a: &Bignum, b: &Bignum, c: &Bignum) -> i32 {
    debug_assert!(a.is_clamped());
    debug_assert!(b.is_clamped());
    debug_assert!(c.is_clamped());
    if a.bigit_length() < b.bigit_length() {
      return Self::plus_compare(b, a, c);
    }
    if a.bigit_length() + 1 < c.bigit_length() {
      return -1;
    }
    if a.bigit_length() > c.bigit_length() {
      return 1;
    }
    // 指数即隐含 0 位: 若 a 的隐含 0 多于 b 的位数, a+b 长度即 a 长度
    if a.exponent_ >= b.bigit_length() && a.bigit_length() < c.bigit_length() {
      return -1;
    }

    let mut borrow: Chunk = 0;
    // min_exponent 以下全为 0, 无需比较
    let min_exponent = min(a.exponent_, min(b.exponent_, c.exponent_));
    let mut i = c.bigit_length() - 1;
    while i >= min_exponent {
      let chunk_a = a.bigit_at(i);
      let chunk_b = b.bigit_at(i);
      let chunk_c = c.bigit_at(i);
      let sum = chunk_a + chunk_b;
      if sum > chunk_c + borrow {
        return 1;
      }
      borrow = chunk_c + borrow - sum;
      if borrow > 1 {
        return -1;
      }
      borrow <<= BIGIT_SIZE as u32;
      i -= 1;
    }
    if borrow == 0 {
      return 0;
    }
    -1
  }

  #[must_use]
  pub fn plus_equal(a: &Bignum, b: &Bignum, c: &Bignum) -> bool {
    Self::plus_compare(a, b, c) == 0
  }

  #[must_use]
  pub fn plus_less_equal(a: &Bignum, b: &Bignum, c: &Bignum) -> bool {
    Self::plus_compare(a, b, c) <= 0
  }

  #[must_use]
  pub fn plus_less(a: &Bignum, b: &Bignum, c: &Bignum) -> bool {
    Self::plus_compare(a, b, c) < 0
  }

  fn clamp(&mut self) {
    while self.used_digits_ > 0 && self.bigits_buffer_[(self.used_digits_ - 1) as usize] == 0 {
      self.used_digits_ -= 1;
    }
    if self.used_digits_ == 0 {
      self.exponent_ = 0;
    }
  }

  fn is_clamped(&self) -> bool {
    self.used_digits_ == 0 || self.bigits_buffer_[(self.used_digits_ - 1) as usize] != 0
  }

  fn align(&mut self, other: &Bignum) {
    if self.exponent_ > other.exponent_ {
      // 将 this 的部分隐含位替换为显式 0 位
      let zero_digits = self.exponent_ - other.exponent_;
      self.ensure_capacity(self.used_digits_ + zero_digits);
      let used = self.used_digits_;
      for i in (0..used).rev() {
        let v = self.bigits_buffer_[i as usize];
        self.bigits_buffer_[(i + zero_digits) as usize] = v;
      }
      for i in 0..zero_digits {
        self.bigits_buffer_[i as usize] = 0;
      }
      self.used_digits_ += zero_digits;
      self.exponent_ -= zero_digits;
      debug_assert!(self.used_digits_ >= 0);
      debug_assert!(self.exponent_ >= 0);
    }
  }

  fn bigits_shift_left(&mut self, shift_amount: i32) {
    debug_assert!(shift_amount < BIGIT_SIZE);
    debug_assert!(shift_amount >= 0);
    let mut carry: Chunk = 0;
    for i in 0..self.used_digits_ {
      let new_carry = self.bigits_buffer_[i as usize] >> (BIGIT_SIZE - shift_amount) as u32;
      self.bigits_buffer_[i as usize] =
        ((self.bigits_buffer_[i as usize] << shift_amount as u32) + carry) & BIGIT_MASK;
      carry = new_carry;
    }
    if carry != 0 {
      self.bigits_buffer_[self.used_digits_ as usize] = carry;
      self.used_digits_ += 1;
    }
  }

  fn subtract_times(&mut self, other: &Bignum, factor: i32) {
    debug_assert!(self.exponent_ <= other.exponent_);
    // 原版 #ifdef DEBUG oracle (bignum.cc:681-687,713): 快路径结果须与慢路径一致
    #[cfg(debug_assertions)]
    let oracle = {
      let mut a = self.clone();
      let mut b = other.clone();
      b.multiply_by_uint32(factor as u32);
      a.subtract_bignum(&b);
      a
    };
    if factor < 3 {
      for _ in 0..factor {
        self.subtract_bignum(other);
      }
      return;
    }
    let mut borrow: Chunk = 0;
    let exponent_diff = other.exponent_ - self.exponent_;
    for i in 0..other.used_digits_ {
      let product: DoubleChunk =
        u64::from(factor as Chunk) * u64::from(other.bigits_buffer_[i as usize]);
      let remove = u64::from(borrow) + product;
      let idx = (i + exponent_diff) as usize;
      // 借位 = 差值 (uint32 回绕) 的最高位 + remove 超出 bigit 的高位
      let difference =
        self.bigits_buffer_[idx].wrapping_sub((remove & u64::from(BIGIT_MASK)) as Chunk);
      self.bigits_buffer_[idx] = difference & BIGIT_MASK;
      borrow =
        (difference >> (CHUNK_SIZE - 1)) + ((remove >> BIGIT_SIZE as u32) as Chunk & BIGIT_MASK);
    }
    // 逐个高位 bigit 消解借位; 借位清零即返回 (与原版一致, 不做 clamp)
    let mut i = other.used_digits_ + exponent_diff;
    while i < self.used_digits_ {
      if borrow == 0 {
        return;
      }
      let idx = i as usize;
      let difference = self.bigits_buffer_[idx].wrapping_sub(borrow);
      self.bigits_buffer_[idx] = difference & BIGIT_MASK;
      borrow = difference >> (CHUNK_SIZE - 1);
      i += 1;
    }
    self.clamp();
    // 原版 bignum.cc:713 DCHECK(Equal(a, *this)): 快慢路径守恒自检
    #[cfg(debug_assertions)]
    debug_assert!(
      Self::equal(&oracle, self),
      "SubtractTimes 快慢路径结果不一致"
    );
  }

  #[must_use]
  fn bigit_length(&self) -> i32 {
    self.used_digits_ + self.exponent_
  }

  #[must_use]
  pub fn bigits(&self) -> &[Chunk] {
    &self.bigits_buffer_[..self.used_digits_ as usize]
  }

  #[must_use]
  pub fn exponent(&self) -> i32 {
    self.exponent_
  }
}

fn size_in_hex_chars(mut number: Chunk) -> i32 {
  debug_assert!(number > 0);
  let mut result = 0;
  while number != 0 {
    number >>= 4;
    result += 1;
  }
  result
}

fn read_uint64(buffer: &[u8], from: i32, digits_to_read: i32) -> u64 {
  let mut result = 0u64;
  let to = (from + digits_to_read) as usize;
  for &b in &buffer[from as usize..to] {
    let digit = b - b'0';
    debug_assert!(digit <= 9);
    result = result * 10 + u64::from(digit);
  }
  result
}

fn hex_char_value(c: u8) -> i32 {
  match c {
    b'0'..=b'9' => (c - b'0') as i32,
    b'a'..=b'f' => 10 + (c - b'a') as i32,
    b'A'..=b'F' => 10 + (c - b'A') as i32,
    _ => unsafe { debug_unreachable() },
  }
}
