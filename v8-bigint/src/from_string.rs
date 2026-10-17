// 由 tool/src/bigint/gen_consts.js 从 v8/src/bigint/bigint-inl.h 生成; 修订就地改, 复跑覆盖
// 常量表基线; FromStringAccumulator / Parse / AddPart 为语义体修订

//! 字符串 → BigInt 解析: 累加器 + 各进制算法 (对应原版 fromstring.cc + bigint-inl.h 的解析部分)

use std::mem::{replace, take};

use crate::{
  bigint::{
    Digit, K_DIGIT_BITS, K_FROM_STRING_LARGE_THRESHOLD, Platform, Status, bit_length, compare,
    digit_add2, digit_add3, digit_mul,
  },
  bigint_internal::Processor,
  mul_helpers::multiply_single_prefix,
  util::is_power_of_two,
  vector_arithmetic::add_and_return_overflow,
};

// 前 127 个 ASCII 字符的数值, 255 表示"非法"
pub(crate) const K_CHAR_VALUE: [u8; 128] = [
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
  255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255, 255, 255,
  255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
  29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255, 255, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
  20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 255, 255, 255, 255, 255,
];

// {2,4,8,16,32} → {1,2,3,4,5} 的紧凑映射 (下标 = radix >> 2)
pub(crate) const K_CHAR_BITS: [u8; 9] = [1, 2, 3, 0, 4, 0, 0, 0, 5];

/// 解析结果状态
/// v8: src/bigint/bigint-inl.h:138
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ParseResult {
  Ok,
  MaxSizeExceeded,
}

/// 栈上 part 数量上限
/// v8: src/bigint/bigint-inl.h:173
const K_STACK_PARTS: usize = 8;

/// 短串内联阈值 (字符数): 8 part × 64 bit ÷ 5.17 bit/字符 ≈ 99
/// v8: src/bigint/bigint-inl.h:272
const K_INLINE_THRESHOLD: u32 = K_STACK_PARTS as u32 * K_DIGIT_BITS * 100 / 517;

/// 字符 → 数值 (非法返回 255); 要求 c < 128
#[inline]
const fn char_value(c: u8) -> Digit {
  K_CHAR_VALUE[c as usize] as Digit
}

/// 字符串解析累加器: 分块读入字符, 供 Processor 组装成 BigInt
/// v8: src/bigint/bigint-inl.h:136
pub struct FromStringAccumulator {
  /// 栈上 parts (前 8 个)
  stack_parts: [Digit; K_STACK_PARTS],
  /// 堆上 parts (栈满后启用, 含栈内容副本)
  heap_parts: Vec<Digit>,
  max_multiplier: Digit,
  /// 末段乘子; 2 幂进制下复用为"末 part 未用 bit 数"
  last_multiplier: Digit,
  max_digits: u32,
  result: ParseResult,
  stack_parts_used: u32,
  inline_everything: bool,
  radix: u8,
}

/// 读下一字符数值; 耗尽/非法/超进制 → None (parse 双算法共用)
#[inline]
fn next_digit(chars: &[u8], pos: usize, radix: Digit) -> Option<Digit> {
  let &c = chars.get(pos)?;
  if (c as usize) >= K_CHAR_VALUE.len() {
    return None;
  }
  let d = char_value(c);
  (d < radix).then_some(d)
}

impl FromStringAccumulator {
  /// {max_digits} 只用于拒绝超出上限, 不预分配
  /// v8: src/bigint/bigint-inl.h:149
  pub fn new(max_digits: u32) -> Self {
    Self {
      stack_parts: [0; K_STACK_PARTS],
      heap_parts: Vec::new(),
      max_multiplier: 0,
      last_multiplier: 0,
      max_digits: max_digits.max(K_STACK_PARTS as u32),
      result: ParseResult::Ok,
      stack_parts_used: 0,
      inline_everything: false,
      radix: 0,
    }
  }

  /// 读入全部字符, 返回停止位置 (遇非法字符或读完)
  /// v8: src/bigint/bigint-inl.h:263
  pub fn parse(&mut self, chars: &[u8], radix: Digit) -> usize {
    debug_assert!((2..=36).contains(&radix));
    self.inline_everything = chars.len() as u32 <= K_INLINE_THRESHOLD;
    if !self.inline_everything && radix & (radix - 1) == 0 {
      return self.parse_power_two(chars, radix);
    }
    let mut pos = 0usize;
    let mut done = false;
    while !done {
      let mut multiplier: Digit = 1;
      let mut part: Digit = 0;
      loop {
        // 原版此处解引用 end (UB), Rust 版防御: 空串/耗尽即终止
        let d = match next_digit(chars, pos, radix) {
          Some(d) => d,
          None => {
            done = true;
            break;
          }
        };
        // 乘子溢出即当前 part 收尾
        let Some(m) = multiplier.checked_mul(radix) else {
          break;
        };
        multiplier = m;
        part = part * radix + d;
        pos += 1;
        if pos == chars.len() {
          done = true;
          break;
        }
      }
      if !self.add_part_mul(multiplier, part, done) {
        return pos;
      }
    }
    pos
  }

  /// 2 幂进制解析: 每个 part 装满 digit 的整数个字符位
  /// v8: src/bigint/bigint-inl.h:227
  fn parse_power_two(&mut self, chars: &[u8], radix: Digit) -> usize {
    self.radix = radix as u8;
    let char_bits = K_CHAR_BITS[(radix >> 2) as usize] as u32;
    let mut pos = 0usize;
    let mut done = false;
    let mut bits_left;
    loop {
      let mut part: Digit = 0;
      bits_left = K_DIGIT_BITS as i32;
      loop {
        let d = match next_digit(chars, pos, radix) {
          Some(d) => d,
          None => {
            done = true;
            break;
          }
        };
        if bits_left < char_bits as i32 {
          break;
        }
        bits_left -= char_bits as i32;
        part = (part << char_bits) | d;
        pos += 1;
        if pos == chars.len() {
          done = true;
          break;
        }
      }
      if !self.add_part(part) {
        return pos;
      }
      if done {
        break;
      }
    }
    // 复用 last_multiplier 传达末 part 的未用 bit 数
    self.last_multiplier = bits_left as Digit;
    pos
  }

  /// 带乘子的 part 累加: accumulator = accumulator × multiplier + part
  /// v8: src/bigint/bigint-inl.h:313
  fn add_part_mul(&mut self, multiplier: Digit, part: Digit, is_last: bool) -> bool {
    if self.inline_everything {
      // MultiplySingle 就地版: 现有 parts 乘 multiplier 再加 part
      let mut carry = part;
      let mut high: Digit = 0;
      for i in 0..self.stack_parts_used as usize {
        let (low, new_high) = digit_mul(self.stack_parts[i], multiplier);
        let (v, c) = digit_add3(low, high, carry);
        self.stack_parts[i] = v;
        carry = c;
        high = new_high;
      }
      high = digit_add2(high, carry).0;
      if high != 0 {
        self.stack_parts[self.stack_parts_used as usize] = high;
        self.stack_parts_used += 1;
      }
      debug_assert!(self.stack_parts_used as usize <= K_STACK_PARTS);
      return true;
    }
    if is_last {
      self.last_multiplier = multiplier;
    } else {
      debug_assert!(self.max_multiplier == 0 || self.max_multiplier == multiplier);
      self.max_multiplier = multiplier;
    }
    self.add_part(part)
  }

  /// 追加一个 part: 栈未满入栈, 满 8 后转入堆
  /// v8: src/bigint/bigint-inl.h:346
  fn add_part(&mut self, part: Digit) -> bool {
    if (self.stack_parts_used as usize) < K_STACK_PARTS {
      self.stack_parts[self.stack_parts_used as usize] = part;
      self.stack_parts_used += 1;
      return true;
    }
    if self.heap_parts.is_empty() {
      // 初始化堆存储, 拷贝栈内容便于后续统一处理
      self.heap_parts = self.stack_parts.to_vec();
    }
    if self.heap_parts.len() as u32 >= self.max_digits {
      self.result = ParseResult::MaxSizeExceeded;
      return false;
    }
    self.heap_parts.push(part);
    true
  }

  /// 结果长度 (分配 Z 用)
  /// v8: src/bigint/bigint-inl.h:164
  pub fn result_length(&self) -> u32 {
    self.stack_parts_used.max(self.heap_parts.len() as u32)
  }

  /// 解析结果状态
  /// v8: src/bigint/bigint-inl.h:163
  pub fn result(&self) -> ParseResult {
    self.result
  }
}

impl<P: Platform> Processor<P> {
  /// 经典算法: 每个 part 乘合适乘子后累加, 整体 O(n²)
  /// v8: src/bigint/fromstring.cc:14
  pub fn parse_classic(&mut self, z: &mut [Digit], acc: &mut FromStringAccumulator) {
    debug_assert!(acc.stack_parts_used > 0);
    let num_stack = acc.stack_parts_used as usize;
    z[0] = acc.stack_parts[0];
    z[1..].fill(0);
    if num_stack == 1 {
      return;
    }
    // part 来源: 堆存储启用则全部 part 已在堆 (含栈副本)
    let (parts, n) = if acc.heap_parts.is_empty() {
      (&acc.stack_parts[..num_stack], num_stack)
    } else {
      (&acc.heap_parts[..], acc.heap_parts.len())
    };
    debug_assert!(z.len() >= n);
    let mut already_set = 1usize;
    for &part in &parts[1..n - 1] {
      multiply_single_prefix(z, already_set, acc.max_multiplier);
      add_and_return_overflow(z, &[part]);
      already_set += 1;
    }
    multiply_single_prefix(z, already_set, acc.last_multiplier);
    add_and_return_overflow(z, &[parts[n - 1]]);
  }

  /// 快速算法: 平衡二叉树序合并相邻 part 对, 输入规模相近可走快速乘法
  /// 三缓冲轮换: parts / multipliers / temp; 大部分乘子相同, 相同则拷贝上一结果 (标 *)
  /// v8: src/bigint/fromstring.cc:90
  pub fn parse_large(&mut self, z: &mut [Digit], acc: &mut FromStringAccumulator) {
    let num_parts = acc.heap_parts.len();
    debug_assert!(num_parts >= 2);
    debug_assert!(z.len() >= num_parts);
    // 原版复用 heap 内存 + 2×num_parts scratch 作三段轮换视图;
    // Rust 版取走 heap 缓冲另配两块等长缓冲。轮内最大区间 = ceil(n/2^k)·2^k,
    // 奇数 part 数时超过 n, 故容量取 next_power_of_two (尾部补零与原版视图截断语义等价)
    let cap = num_parts.next_power_of_two();
    let mut parts = take(&mut acc.heap_parts);
    parts.resize(cap, 0);
    let mut multipliers = vec![0; cap];
    let mut temp = vec![0; cap];
    let mut num_parts = num_parts;

    // 首轮特化: part_len == 1, 乘子已知, 单 digit 硬展开
    // v8: src/bigint/fromstring.cc:102
    {
      let max_multiplier = acc.max_multiplier;
      let last_multiplier = acc.last_multiplier;
      let mut i = 0usize;
      while i + 1 < num_parts {
        let p_in = parts[i];
        let p_in2 = parts[i + 1];
        let m_in2 = if i == num_parts - 2 {
          last_multiplier
        } else {
          max_multiplier
        };
        // p[j] = p[i] * m[i+1] + p[i+1]
        let (p_low, p_high) = digit_mul(p_in, m_in2);
        let (v, carry) = digit_add2(p_low, p_in2);
        temp[i] = v;
        temp[i + 1] = digit_add2(p_high, carry).0;
        // m[j] = m[i] * m[i+1]
        if i > 0 {
          if i > 2 && m_in2 != last_multiplier {
            // 与前一个乘子相同, 直接拷贝
            parts[i] = parts[i - 2];
            parts[i + 1] = parts[i - 1];
          } else {
            let (lo, hi) = digit_mul(max_multiplier, m_in2);
            parts[i] = lo;
            parts[i + 1] = hi;
          }
        }
        i += 2;
      }
      // 奇数个 part 的尾部直通
      if i < num_parts {
        temp[i] = parts[i];
        parts[i] = last_multiplier;
        i += 2;
      }
      num_parts = i >> 1;
      // 轮换: parts ← temp(新 parts), multipliers ← parts(新乘子), temp ← multipliers(空)
      let t = take(&mut temp);
      let p_old = replace(&mut parts, t);
      temp = replace(&mut multipliers, p_old);
      self.add_work_estimate(num_parts as u64);
    }

    let mut part_len = 2usize;
    while num_parts > 1 {
      // v8: src/bigint/fromstring.cc:147
      // 原版末轮直接写 Z; Rust 版全程走轮换缓冲, 结束后统一拷贝
      let new_part_len = part_len * 2;
      let mut i = 0usize;
      while i + 1 < num_parts {
        let start = i * part_len;
        let (p_in, p_in2) = parts[start..start + new_part_len].split_at(part_len);
        let (m_in, m_in2) = multipliers[start..start + new_part_len].split_at(part_len);
        // p[j] = p[i] * m[i+1] + p[i+1]
        let p_out = &mut temp[start..start + new_part_len];
        self.multiply(p_out, p_in, m_in2);
        if self.should_terminate() {
          return;
        }
        let overflow = add_and_return_overflow(p_out, p_in2);
        debug_assert!(overflow == 0);
        // m[j] = m[i] * m[i+1]
        if i > 0 {
          let mut copied = false;
          if i > 2 {
            let prev_start = (i - 2) * part_len;
            let (m_in_prev, m_in2_prev) = multipliers[prev_start..start].split_at(part_len);
            if compare(m_in, m_in_prev) == 0 && compare(m_in2, m_in2_prev) == 0 {
              copied = true;
              let (lo, hi) = parts.split_at_mut(start);
              hi[..new_part_len].copy_from_slice(&lo[prev_start..]);
            }
          }
          if !copied {
            self.multiply(&mut parts[start..start + new_part_len], m_in, m_in2);
            if self.should_terminate() {
              return;
            }
          }
        }
        i += 2;
      }
      // 奇数个 part 的尾部: 拷贝补零
      if i < num_parts {
        let start = i * part_len;
        temp[start..start + part_len].copy_from_slice(&parts[start..start + part_len]);
        temp[start + part_len..start + new_part_len].fill(0);
        parts[start..start + part_len].copy_from_slice(&multipliers[start..start + part_len]);
        parts[start + part_len..start + new_part_len].fill(0);
        i += 2;
      }
      num_parts = i >> 1;
      part_len = new_part_len;
      // 轮换同首轮
      let t = take(&mut temp);
      let p_old = replace(&mut parts, t);
      temp = replace(&mut multipliers, p_old);
    }
    // 结果回填 Z (可能比 Z 长, 高位必为零)
    let n = z.len().min(part_len);
    z[..n].copy_from_slice(&parts[..n]);
    z[n..].fill(0);
  }

  /// 2 幂进制组装: parts 即正确位序列, 逆序拼装 + 位对齐
  /// v8: src/bigint/fromstring.cc:239
  fn parse_base_power_of_two(&mut self, z: &mut [Digit], acc: &FromStringAccumulator) {
    let num_parts = acc.result_length() as usize;
    debug_assert!(num_parts >= 1);
    debug_assert!(z.len() >= num_parts);
    let parts: &[Digit] = if acc.heap_parts.is_empty() {
      &acc.stack_parts
    } else {
      &acc.heap_parts
    };
    let radix = acc.radix as u32;
    debug_assert!(matches!(radix, 2 | 4 | 8 | 16 | 32));
    let char_bits = bit_length(radix - 1);
    let unused_last_part_bits = acc.last_multiplier as i32;
    let unused_part_bits = (K_DIGIT_BITS % char_bits) as i32;
    let max_part_bits = K_DIGIT_BITS as i32 - unused_part_bits;
    let mut z_index = 0usize;
    let mut part_index = num_parts as i32 - 1;

    // 末 part 装满 ⇒ 全部 part 都装满, 逆序直拷
    if unused_last_part_bits == 0 {
      debug_assert!(K_DIGIT_BITS.is_multiple_of(char_bits));
      while part_index >= 0 {
        z[z_index] = parts[part_index as usize];
        z_index += 1;
        part_index -= 1;
      }
      z[z_index..].fill(0);
      return;
    }

    // 否则按位移拼装: digit 持有下一个待存 Z digit, digit_bits 是其低位已知位数
    let mut digit = parts[part_index as usize];
    part_index -= 1;
    let mut digit_bits = K_DIGIT_BITS as i32 - unused_last_part_bits;
    while part_index >= 0 {
      let mut part: Digit = 0;
      let mut part_bits = 0;
      while digit_bits < K_DIGIT_BITS as i32 {
        part = parts[part_index as usize];
        part_index -= 1;
        part_bits = max_part_bits;
        digit |= part << digit_bits;
        let part_shift = K_DIGIT_BITS as i32 - digit_bits;
        if part_shift > part_bits {
          digit_bits += part_bits;
          part = 0;
          part_bits = 0;
          if part_index < 0 {
            break;
          }
        } else {
          digit_bits = K_DIGIT_BITS as i32;
          part >>= part_shift;
          part_bits -= part_shift;
        }
      }
      z[z_index] = digit;
      z_index += 1;
      digit = part;
      digit_bits = part_bits;
    }
    if digit_bits > 0 {
      z[z_index] = digit;
      z_index += 1;
    }
    z[z_index..].fill(0);
  }

  /// 组装入口: 按累加器状态派发
  /// v8: src/bigint/fromstring.cc:304
  pub fn parse_string(&mut self, z: &mut [Digit], acc: &mut FromStringAccumulator) -> Status {
    if acc.inline_everything {
      let used = acc.stack_parts_used as usize;
      z[..used].copy_from_slice(&acc.stack_parts[..used]);
      z[used..].fill(0);
    } else if acc.stack_parts_used == 0 {
      z.fill(0);
    } else if is_power_of_two(acc.radix as u32) {
      self.parse_base_power_of_two(z, acc);
    } else if acc.result_length() < K_FROM_STRING_LARGE_THRESHOLD {
      self.parse_classic(z, acc);
    } else {
      self.parse_large(z, acc);
    }
    self.get_and_clear_status()
  }
}
