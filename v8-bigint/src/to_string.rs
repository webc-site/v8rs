// 由 tool/src/bigint/gen_consts.js 从 v8/src/bigint/tostring.cc 生成; 修订就地改, 复跑覆盖
// 常量表基线; ToStringFormatter / RecursionLevel 为语义体修订

//! BigInt → 字符串: 经典算法 + 快速分治算法 (对应原版 tostring.cc)

use std::mem::swap;

use crate::{
  bigint::{
    Digit, K_DIGIT_BITS, K_HALF_DIGIT_BITS, K_HALF_DIGIT_MASK, K_TO_STRING_FAST_THRESHOLD,
    Platform, Status, compare, count_leading_zeros, div_ceil, divide_single, msd, norm,
  },
  bigint_internal::Processor,
  div_helpers::{
    ShiftedDigits, divide_barrett_scratch_space, invert_scratch_space, left_shift, right_shift,
  },
  util::{count_trailing_zeros, div_ceil_u64, is_power_of_two},
  vector_arithmetic::bit_length_digits,
};

// 每字符最大位数表 (值放大 32 倍以提升估算精度)
pub(crate) const K_MAX_BITS_PER_CHAR: [u8; 37] = [
  0, 0, 32, 51, 64, 75, 83, 90, 96, 102, 107, 111, 115, 119, 122, 126, 128, 131, 134, 136, 139,
  141, 143, 145, 147, 149, 151, 153, 154, 156, 158, 159, 160, 162, 163, 165, 166,
];

// 位数表放大倍数
/// v8: src/bigint/tostring.cc:31
const K_BITS_PER_CHAR_TABLE_MULTIPLIER: u64 = 32;

// 进制字符表
/// v8: src/bigint/tostring.cc:34
const K_CONVERSION_CHARS: &[u8; 36] = b"0123456789abcdefghijklmnopqrstuvwxyz";

/// base^exponent (不查溢出; 调用方保证不溢出)
/// v8: src/bigint/tostring.cc:37
const fn digit_pow(mut base: Digit, mut exponent: u32) -> Digit {
  let mut result: Digit = 1;
  while exponent > 0 {
    if exponent & 1 == 1 {
      result = result.wrapping_mul(base);
    }
    exponent >>= 1;
    base = base.wrapping_mul(base);
  }
  result
}

/// 结果长度预算 (分配字符缓冲用)
/// v8: src/bigint/tostring.cc:597
pub fn to_string_result_length(x: &[Digit], radix: u32, sign: bool) -> u32 {
  let bit_len = bit_length_digits(x);
  let mut result = if is_power_of_two(radix) {
    let bits_per_char = count_trailing_zeros(radix);
    div_ceil(bit_len, bits_per_char)
  } else {
    // 每字符最大位数; 预算须悲观, 用"每字符最小位数"(= 最大位数 - 1)
    let max_bits_per_char = K_MAX_BITS_PER_CHAR[radix as usize] as u64;
    let min_bits_per_char = max_bits_per_char - 1;
    div_ceil_u64(
      bit_len as u64 * K_BITS_PER_CHAR_TABLE_MULTIPLIER,
      min_bits_per_char,
    ) as u32
  };
  result += sign as u32;
  result
}

/// 递归层级: 除数平方链 + 各自逆元
/// v8: src/bigint/tostring.cc:321
struct Level {
  leading_zero_shift: u32,
  /// 本层每半产生的字符数
  char_count: u32,
  is_toplevel: bool,
  /// bit 规范化后的除数
  divisor: Vec<Digit>,
  /// TrimOne 后的逆元; 顶层延迟计算
  inverse: Vec<Digit>,
}

impl Level {
  /// 除数左移至 bit 规范化 (记录移位量供还原)
  /// v8: src/bigint/tostring.cc:347
  fn left_shift_divisor(&mut self) {
    self.leading_zero_shift = count_leading_zeros(msd(&self.divisor));
    // 就地左移经临时副本保语义
    let tmp = self.divisor.clone();
    left_shift(&mut self.divisor, &tmp, self.leading_zero_shift);
  }

  /// 求除数逆元; dividend_length 非零时按被除数长度裁剪 (顶层优化)
  /// v8: src/bigint/tostring.cc:402
  fn compute_inverse<P: Platform>(&mut self, proc: &mut Processor<P>, dividend_length: u32) {
    let mut inverse_len = self.divisor.len() as u32;
    if dividend_length != 0 {
      inverse_len = dividend_length - self.divisor.len() as u32;
      debug_assert!(inverse_len <= self.divisor.len() as u32);
    }
    let scratch_len = invert_scratch_space(inverse_len) as usize;
    let mut scratch = vec![0; scratch_len];
    let mut inverse = vec![0; inverse_len as usize + 1];
    let input = &self.divisor[self.divisor.len() - inverse_len as usize..];
    proc.invert_pub(&mut inverse, input, &mut scratch);
    inverse.truncate(inverse_len as usize); // TrimOne: 摘掉隐式顶位 0
    self.inverse = inverse;
  }

  /// 取本层逆元 (按被除数长度取高位段)
  /// v8: src/bigint/tostring.cc:421
  fn get_inverse(&self, dividend_length: usize) -> &[Digit] {
    debug_assert!(!self.inverse.is_empty());
    let inverse_len = dividend_length - self.divisor.len();
    debug_assert!(inverse_len <= self.inverse.len());
    &self.inverse[self.inverse.len() - inverse_len..]
  }
}

/// 自底向上构建递归层级: 下一层除数 = 当前², 直至超过输入位长
/// 返回 None 表示被中断
/// v8: src/bigint/tostring.cc:363
fn create_levels<P: Platform>(
  proc: &mut Processor<P>,
  base_divisor: Digit,
  base_char_count: u32,
  target_bit_length: u32,
) -> Option<Vec<Level>> {
  let mut levels = vec![Level {
    leading_zero_shift: 0,
    char_count: base_char_count,
    is_toplevel: true,
    divisor: vec![base_divisor],
    inverse: Vec::new(),
  }];
  // 位长是不完美预测: 平方通常丢 1 位, 故用 2×-1 保守条件保证严格变大
  while bit_length_digits(&levels[levels.len() - 1].divisor) * 2 - 1 <= target_bit_length {
    let prev = levels.last_mut()?;
    let mut new_divisor = vec![0; prev.divisor.len() * 2];
    proc.multiply(&mut new_divisor, &prev.divisor, &prev.divisor);
    if proc.should_terminate() {
      return None;
    }
    let new_divisor = norm(&new_divisor).to_vec();
    let new_char_count = prev.char_count * 2;
    debug_assert!(new_char_count < u32::MAX / 2);
    // 除数用完再左移; 非顶层随即求逆
    prev.left_shift_divisor();
    prev.compute_inverse(proc, 0);
    if proc.should_terminate() {
      return None;
    }
    prev.is_toplevel = false;
    levels.push(Level {
      leading_zero_shift: 0,
      char_count: new_char_count,
      is_toplevel: true,
      divisor: new_divisor,
      inverse: Vec::new(),
    });
  }
  // 顶层逆元延迟, 由调用方按输入长度计算
  if let Some(top) = levels.last_mut() {
    top.left_shift_divisor();
  }
  Some(levels)
}

/// 格式化器: 从缓冲尾部向前写字符
/// v8: src/bigint/tostring.cc:119
struct Formatter<'a, 'p, P: Platform> {
  /// 规格化后的输入
  digits: &'a [Digit],
  radix: u32,
  chunk_chars: u32,
  sign: bool,
  buf: &'a mut [u8],
  /// 写游标 (相对 buf 起点, 向左递减; 越下界仅计数不落盘)
  out: isize,
  chunk_divisor: Digit,
  proc: &'p mut Processor<P>,
}

impl<'a, 'p, P: Platform> Formatter<'a, 'p, P> {
  fn new(
    proc: &'p mut Processor<P>,
    x: &'a [Digit],
    radix: u32,
    sign: bool,
    buf: &'a mut [u8],
  ) -> Self {
    debug_assert!((2..=36).contains(&radix));
    let out = buf.len() as isize;
    Self {
      digits: norm(x),
      radix,
      chunk_chars: 0,
      sign,
      buf,
      out,
      chunk_divisor: 0,
      proc,
    }
  }

  /// 写一个字符 (游标越下界时仅计数)
  #[inline]
  fn put(&mut self, c: u8) {
    self.out -= 1;
    let i = self.out;
    if i >= 0 {
      self.buf[i as usize] = c;
    }
  }

  /// 是否已被中断
  fn terminated(&self) -> bool {
    self.proc.should_terminate()
  }

  /// Classic 预备: 计算每轮除数与字符块大小
  /// v8: src/bigint/tostring.cc:221
  fn start(&mut self) {
    let max_bits_per_char = K_MAX_BITS_PER_CHAR[self.radix as usize] as u32;
    self.chunk_chars = K_DIGIT_BITS * K_BITS_PER_CHAR_TABLE_MULTIPLIER as u32 / max_bits_per_char;
    self.chunk_divisor = digit_pow(self.radix as Digit, self.chunk_chars);
    // chunk_chars 的构造保证不溢出
    debug_assert!(self.chunk_divisor != 0);
  }

  /// 末位 (最高有效) digit: 不写前导零
  /// v8: src/bigint/tostring.cc:184
  fn basecase_last(&mut self, mut digit: Digit) {
    let radix = self.radix as Digit;
    loop {
      self.put(K_CONVERSION_CHARS[(digit % radix) as usize]);
      digit /= radix;
      if digit == 0 {
        return;
      }
    }
  }

  /// 中间 digit: 固定写满 chunk_chars 个字符
  /// v8: src/bigint/tostring.cc:196
  fn basecase_middle(&mut self, mut digit: Digit) {
    let radix = self.radix as Digit;
    for _ in 0..self.chunk_chars {
      self.put(K_CONVERSION_CHARS[(digit % radix) as usize]);
      digit /= radix;
    }
    debug_assert!(digit == 0);
  }

  /// radix 10 特化: 半 digit 魔数除法 (编译期常量除数 → 模乘逆元指令序列)
  /// v8: src/bigint/tostring.cc:79 DivideByMagic<10>
  fn divide_by_magic_10(&mut self, rest: &mut [Digit], input: &[Digit]) {
    const RADIX: Digit = 10;
    const MAX_BITS_PER_CHAR: u32 = K_MAX_BITS_PER_CHAR[10] as u32;
    const CHUNK_CHARS: u32 = K_HALF_DIGIT_BITS * 32 / MAX_BITS_PER_CHAR;
    const CHUNK_DIVISOR: Digit = digit_pow(RADIX, CHUNK_CHARS);
    let mut remainder: Digit = 0;
    for i in (0..input.len()).rev() {
      let d = input[i];
      let upper = (remainder << K_HALF_DIGIT_BITS) | (d >> K_HALF_DIGIT_BITS);
      let u_result = upper / CHUNK_DIVISOR;
      remainder = upper % CHUNK_DIVISOR;
      let lower = (remainder << K_HALF_DIGIT_BITS) | (d & K_HALF_DIGIT_MASK);
      let l_result = lower / CHUNK_DIVISOR;
      remainder = lower % CHUNK_DIVISOR;
      rest[i] = (u_result << K_HALF_DIGIT_BITS) | l_result;
    }
    // remainder 即当前块, 从低向高写 CHUNK_CHARS 个字符
    for _ in 0..CHUNK_CHARS {
      self.put(b'0' + (remainder % RADIX) as u8);
      remainder /= RADIX;
    }
    debug_assert!(remainder == 0);
  }

  /// 经典算法: 逐轮除以 chunk_divisor, 从低位向高位成块写出
  /// v8: src/bigint/tostring.cc:138
  fn classic(&mut self) {
    if self.digits.is_empty() {
      self.put(b'0');
      return;
    }
    if self.digits.len() == 1 {
      self.basecase_last(self.digits[0]);
      return;
    }
    // 原版就地除 (dividend = rest); Rust 版双缓冲轮换等价
    let len = self.digits.len();
    let mut a = self.digits.to_vec();
    let mut b = vec![0; len];
    let mut dividend_len = len;
    loop {
      let dividend = &a[..dividend_len];
      let rest = &mut b[..dividend_len];
      if self.radix == 10 {
        // 最常见进制单独优化
        self.divide_by_magic_10(rest, dividend);
      } else {
        let (_, chunk) = divide_single(rest, dividend, self.chunk_divisor);
        self.basecase_middle(chunk);
      }
      dividend_len = norm(rest).len();
      swap(&mut a, &mut b);
      if dividend_len <= 1 {
        break;
      }
    }
    let last = if dividend_len == 1 { a[0] } else { 0 };
    self.basecase_last(last);
  }

  /// 2 幂进制: 每 char_bits 位直出字符
  /// v8: src/bigint/tostring.cc:243
  fn base_power_of_two(&mut self) {
    if self.digits.is_empty() {
      // 原版此处依赖调用方保证非空 (空则下溢), 零值内聚处理
      self.put(b'0');
      return;
    }
    let bits_per_char = count_trailing_zeros(self.radix);
    let char_mask = self.radix as Digit - 1;
    let mut digit: Digit = 0;
    // digit 中尚未消费的位数
    let mut available_bits: i32 = 0;
    let last = self.digits.len() - 1;
    for i in 0..last {
      let new_digit = self.digits[i];
      // 并入上一轮剩余位
      let current = (digit | (new_digit << available_bits)) & char_mask;
      self.put(K_CONVERSION_CHARS[current as usize]);
      let consumed_bits = bits_per_char as i32 - available_bits;
      digit = new_digit >> consumed_bits;
      available_bits = K_DIGIT_BITS as i32 - consumed_bits;
      while available_bits >= bits_per_char as i32 {
        self.put(K_CONVERSION_CHARS[(digit & char_mask) as usize]);
        digit >>= bits_per_char;
        available_bits -= bits_per_char as i32;
      }
    }
    // 最高位 digit
    let msd_v = msd(self.digits);
    let current = (digit | (msd_v << available_bits)) & char_mask;
    self.put(K_CONVERSION_CHARS[current as usize]);
    digit = msd_v >> (bits_per_char - available_bits as u32);
    while digit != 0 {
      self.put(K_CONVERSION_CHARS[(digit & char_mask) as usize]);
      digit >>= bits_per_char;
    }
  }

  /// 快速分治: 递归对半切 (除法取余), 底层除数平方链预建
  /// v8: src/bigint/tostring.cc:428
  fn fast(&mut self) {
    let Some(mut levels) = create_levels(
      self.proc,
      self.chunk_divisor,
      self.chunk_chars,
      bit_length_digits(self.digits),
    ) else {
      return;
    };
    // 顶层逆元: 原版延迟到 ProcessLevel 首次到达除法步, 此处按同一 chunk
    // (左移后) 长度提前计算, 等价且免去跨层可变借用
    let top = levels.len() - 1;
    let divisor_len = levels[top].divisor.len();
    let clz = count_leading_zeros(msd(self.digits));
    let chunk_len = self.digits.len() + usize::from(levels[top].leading_zero_shift > clz);
    // 移位后严格大于除数长度才走 Barrett 分支 (等长走 schoolbook, 无需逆元)
    if chunk_len > divisor_len {
      levels[top].compute_inverse(self.proc, chunk_len as u32);
      if self.terminated() {
        return;
      }
    }
    self.process_level(&levels, Some(top), self.digits, true);
  }

  /// 右起补 '0' 直至本层应产字符数 (最左侧除外)
  /// v8: src/bigint/tostring.cc:438
  fn fill_with_zeros(
    &mut self,
    levels: &[Level],
    li: Option<usize>,
    right_boundary: isize,
    is_last_on_level: bool,
  ) {
    if is_last_on_level {
      return;
    }
    let chunk_chars = match li {
      None => self.chunk_chars as isize,
      Some(i) => (levels[i].char_count * 2) as isize,
    };
    let end = right_boundary - chunk_chars;
    debug_assert!(self.out >= end);
    while self.out > end {
      self.put(b'0');
    }
  }

  /// 递归处理一层: 除法切半, 右半先写 (更低位), 左半后写
  /// v8: src/bigint/tostring.cc:454
  fn process_level(
    &mut self,
    levels: &[Level],
    li: Option<usize>,
    chunk: &[Digit],
    is_last_on_level: bool,
  ) {
    // Step 0: 仅剩 ≤1 digit 走基线
    let normalized = norm(chunk);
    if normalized.len() <= 1 {
      let right_boundary = self.out;
      if let Some(&d) = normalized.first() {
        self.basecase_last(d);
      }
      return self.fill_with_zeros(levels, li, right_boundary, is_last_on_level);
    }
    // 底层除数为单 digit, len≥2 必然走除法分支, 此路径不可达
    let Some(li) = li else { return };
    // Step 1: 必然小于本层除数, 直落下一层
    if normalized.len() < levels[li].divisor.len() {
      let right_boundary = self.out;
      self.process_level(levels, li.checked_sub(1), chunk, is_last_on_level);
      return self.fill_with_zeros(levels, Some(li), right_boundary, is_last_on_level);
    }
    // Step 2: 左移 bit 规范化后精确比较
    let chunk_shifted = ShiftedDigits::new(chunk, Some(levels[li].leading_zero_shift));
    let shifted = norm(chunk_shifted.digits());
    let comparison = compare(shifted, &levels[li].divisor);
    if comparison <= 0 {
      let right_boundary = self.out;
      if comparison < 0 {
        // 严格小于: 原始 chunk 直接作下一层右半, 左半全 '0'
        self.process_level(levels, li.checked_sub(1), chunk, is_last_on_level);
      } else {
        // 相等: 右半全 '0', 左半 "...0001"
        self.fill_with_zeros(levels, li.checked_sub(1), right_boundary, false);
        self.put(b'1');
      }
      return self.fill_with_zeros(levels, Some(li), right_boundary, is_last_on_level);
    }
    // Step 3: 分配结果空间
    let divisor_len = levels[li].divisor.len();
    // 右半 +1 digit 供下层就地左移; 左半 +1 digit 供 DivideBarrett
    let mut right = vec![0; divisor_len + 1];
    let mut left = vec![0; shifted.len() - divisor_len + 1];
    // Step 4: 除法切半
    let inverse_len = shifted.len() - divisor_len;
    if inverse_len == 0 {
      self
        .proc
        .divide_schoolbook(&mut left, &mut right, shifted, &levels[li].divisor);
    } else if divisor_len == 1 {
      let (_, rem) = divide_single(&mut left, shifted, levels[li].divisor[0]);
      right[0] = rem;
      right[1..].fill(0);
    } else {
      let mut scratch = vec![0; divide_barrett_scratch_space(shifted.len() as u32) as usize];
      // 顶层逆元已由 fast() 按同长度预算; 其余层在 create_levels 预计算
      let inverse = levels[li].get_inverse(shifted.len());
      self.proc.divide_barrett_core(
        &mut left,
        &mut right,
        shifted,
        &levels[li].divisor,
        inverse,
        &mut scratch,
      );
      if self.terminated() {
        return;
      }
    }
    // 右移还原除数规范化引入的移位
    let shift = levels[li].leading_zero_shift;
    let tmp = right.clone();
    right_shift(&mut right, &tmp, shift);

    // Step 5: 递归; 右半恰写满本层半宽 (原版 DCHECK), 左半从其返回位置续写
    let boundary = self.out;
    self.process_level(levels, li.checked_sub(1), &right, false);
    if self.terminated() {
      return;
    }
    debug_assert_eq!(self.out, boundary - levels[li].char_count as isize);
    self.process_level(levels, li.checked_sub(1), &left, is_last_on_level);
  }

  /// 收尾: 跳过前导零, 负号, 左移结果并清零尾部; 返回实际长度
  /// v8: src/bigint/tostring.cc:229
  fn finish(&mut self) -> u32 {
    // 非零值须写满预算内; 零值 + 空缓冲探测允许 out 到 -1 (仅计数不落盘)
    debug_assert!(self.digits.is_empty() || self.out >= 0);
    debug_assert!(self.digits.is_empty() || (self.out as usize) < self.buf.len());
    let end = self.buf.len();
    if self.digits.is_empty() {
      // 值 0: 单个 '0', 负零不带负号 (原版由调用方保证非零, 此处内聚)
      self.out = end as isize - 1;
      if self.out >= 0 {
        self.buf[self.out as usize] = b'0';
      }
    } else {
      while (self.out as usize) < end && self.buf[self.out as usize] == b'0' {
        self.out += 1;
      }
      if self.sign {
        self.out -= 1;
        if self.out >= 0 {
          self.buf[self.out as usize] = b'-';
        }
      }
    }
    let new_len = (end as isize - self.out) as u32;
    let start = self.out.max(0) as usize;
    if start > 0 {
      self.buf.copy_within(start..end, 0);
    }
    // 尾部 excess 清零 (原版 memset; 空缓冲探测时 new_len 可超容量, 取 min)
    self.buf[(new_len as usize).min(end)..].fill(0);
    new_len
  }
}

impl<P: Platform> Processor<P> {
  /// 顶层入口: 按尺寸自动选择快速/经典算法
  /// v8: src/bigint/tostring.cc:558
  pub fn write_string(
    &mut self,
    out: &mut [u8],
    x: &[Digit],
    radix: u32,
    sign: bool,
  ) -> (Status, u32) {
    let fast = norm(x).len() >= K_TO_STRING_FAST_THRESHOLD as usize;
    let n = self.write_impl(out, x, radix, sign, fast);
    (self.get_and_clear_status(), n)
  }

  /// 分治/经典可指定入口 (测试对拍用)
  /// v8: src/bigint/tostring.cc:565
  pub fn write_impl(
    &mut self,
    out: &mut [u8],
    x: &[Digit],
    radix: u32,
    sign: bool,
    fast: bool,
  ) -> u32 {
    // 原版 DCHECK 预算充足; Rust 版不足即返回 0 (不输出)
    if (out.len() as u32) < to_string_result_length(x, radix, sign) {
      return 0;
    }
    let mut f = Formatter::new(self, x, radix, sign, out);
    if is_power_of_two(radix) {
      f.base_power_of_two();
    } else if fast {
      f.start();
      f.fast();
      if f.terminated() {
        // 原版直接 return, 长度保持容量值
        return f.buf.len() as u32;
      }
    } else {
      f.start();
      f.classic();
    }
    f.finish()
  }
}
