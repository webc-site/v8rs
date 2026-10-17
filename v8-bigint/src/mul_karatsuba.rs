// 由手工从 v8/src/bigint/mul-karatsuba.cc 转写; 对应原版逐行
//! Karatsuba 乘法 (含长度取整启发式与分块入口)

use std::mem::swap;

use crate::{
  bigint::{
    Digit, K_KARATSUBA_THRESHOLD, Platform, bit_length, compare, digit_sub, digit_sub2, norm,
  },
  bigint_internal::Processor,
  mul_helpers::{multiply_schoolbook, multiply_single},
  util::round_up,
  vector_arithmetic::{add_and_return_overflow, sub_and_return_borrow},
};

/// 长度向上取整启发式 (实验确定)
fn round_up_len(len: u32) -> u32 {
  if len <= 36 {
    return round_up(len, 2);
  }
  // 保留最高 4-5 个非零位
  let mut shift = bit_length(len) - 5;
  if (len >> shift) >= 0x18 {
    shift += 1;
  }
  // 刚过阈值时不取整, 抹平耗时阶梯
  let additive = (1 << shift) - 1;
  if shift >= 2 && (len & additive) < (1 << (shift - 2)) {
    return len;
  }
  ((len + additive) >> shift) << shift
}

/// Karatsuba 输入长度决策
fn karatsuba_length(mut n: u32) -> u32 {
  n = round_up_len(n);
  let mut i = 0;
  while n > K_KARATSUBA_THRESHOLD {
    n >>= 1;
    i += 1;
  }
  n << i
}

/// KaratsubaMain 需要的带符号减法 (较小的从较大的中减, 翻符号)
fn karatsuba_subtraction_helper(result: &mut [Digit], x: &[Digit], y: &[Digit], sign: &mut i32) {
  let mut x = norm(x);
  let mut y = norm(y);
  let mut borrow = 0;
  let mut i = 0;
  if compare(x, y) < 0 {
    *sign = -*sign;
    swap(&mut x, &mut y);
  }
  while i < y.len() {
    let (v, b) = digit_sub2(x[i], y[i], borrow);
    result[i] = v;
    borrow = b;
    i += 1;
  }
  while i < x.len() {
    let (v, b) = digit_sub(x[i], borrow);
    result[i] = v;
    borrow = b;
    i += 1;
  }
  debug_assert!(borrow == 0);
  while i < result.len() {
    result[i] = 0;
    i += 1;
  }
}

impl<P: Platform> Processor<P> {
  /// Karatsuba 乘法入口; 要求 x.len >= y.len 且 y.len >= 阈值
  pub fn multiply_karatsuba(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit]) {
    debug_assert!(x.len() >= y.len());
    debug_assert!(y.len() as u32 >= K_KARATSUBA_THRESHOLD);
    debug_assert!(z.len() >= x.len() + y.len());
    let k = karatsuba_length(y.len() as u32) as usize;
    let mut scratch = vec![0; 4 * k];
    self.karatsuba_start(z, x, y, &mut scratch, k);
  }

  /// 不等长输入分块处理
  fn karatsuba_start(
    &mut self,
    z: &mut [Digit],
    x: &[Digit],
    y: &[Digit],
    scratch: &mut [Digit],
    k: usize,
  ) {
    self.karatsuba_main(z, x, y, scratch, k);
    if self.should_terminate() {
      return;
    }
    if z.len() > 2 * k {
      for d in z[2 * k..].iter_mut() {
        *d = 0;
      }
    }
    if k < y.len() || x.len() != y.len() {
      let mut t = vec![0; 2 * k];
      // 加 X0 * Y1 * b
      // 原版 Digits 视图: 长度自动截断到可用范围
      let x0 = &x[..k.min(x.len())];
      let y1 = &y[k.min(y.len())..];
      if !y1.is_empty() {
        self.karatsuba_chunk(&mut t, x0, y1, scratch);
        if self.should_terminate() {
          return;
        }
        add_and_return_overflow(&mut z[k..], &t); // 不会溢出
      }
      // 加 Xi * Y0 << i 与 Xi * Y1 * b << (i + k)
      let y0 = &y[..k.min(y.len())];
      let mut i = k;
      while i < x.len() {
        let xi = &x[i..(i + k).min(x.len())];
        self.karatsuba_chunk(&mut t, xi, y0, scratch);
        if self.should_terminate() {
          return;
        }
        add_and_return_overflow(&mut z[i..], &t);
        if !y1.is_empty() {
          self.karatsuba_chunk(&mut t, xi, y1, scratch);
          if self.should_terminate() {
            return;
          }
          add_and_return_overflow(&mut z[(i + k)..], &t);
        }
        i += k;
      }
    }
  }

  /// 分块乘法入口, 按尺寸选择算法
  fn karatsuba_chunk(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit], scratch: &mut [Digit]) {
    let x = norm(x);
    let y = norm(y);
    if x.is_empty() || y.is_empty() {
      z.fill(0);
      return;
    }
    let (x, y) = if x.len() < y.len() { (y, x) } else { (x, y) };
    if y.len() == 1 {
      multiply_single(z, x, y[0]);
      self.add_work_estimate(x.len() as u64);
      return;
    }
    if (y.len() as u32) < K_KARATSUBA_THRESHOLD {
      multiply_schoolbook(z, x, y);
      self.add_work_estimate((x.len() * y.len()) as u64);
      return;
    }
    let k = karatsuba_length(y.len() as u32) as usize;
    debug_assert!(scratch.len() >= 4 * k);
    self.karatsuba_start(z, x, y, scratch, k)
  }

  /// 递归主体
  fn karatsuba_main(
    &mut self,
    z: &mut [Digit],
    x: &[Digit],
    y: &[Digit],
    scratch: &mut [Digit],
    n: usize,
  ) {
    if (n as u32) < K_KARATSUBA_THRESHOLD {
      let x = norm(x);
      let y = norm(y);
      if x.len() >= y.len() {
        multiply_schoolbook(&mut z[..2 * n], x, y);
      } else {
        multiply_schoolbook(&mut z[..2 * n], y, x);
      }
      self.add_work_estimate((x.len() * y.len()) as u64);
      return;
    }
    debug_assert!(scratch.len() >= 4 * n);
    debug_assert!(n.is_multiple_of(2));
    let n2 = n >> 1;
    let (head, recurs) = scratch.split_at_mut(2 * n);
    // P0 = X0 * Y0 → Z 低半部
    self.karatsuba_main(&mut head[..n], &x[..n2], &y[..n2], recurs, n2);
    if self.should_terminate() {
      return;
    }
    z[..n].copy_from_slice(&head[..n]);
    // P2 = X1 * Y1 → Z 高半部
    // X1/Y1 = X[n2..n] 视图 (原版 Digits 构造自动截断, 输入可比 n 短)
    self.karatsuba_main(
      &mut head[n..],
      &x[n2..n.min(x.len())],
      &y[n2..n.min(y.len())],
      recurs,
      n2,
    );
    if self.should_terminate() {
      return;
    }
    let end = (z.len() - n).min(n);
    z[n..n + end].copy_from_slice(&head[n..n + end]);
    debug_assert!((end..n).all(|i| head[n + i] == 0));
    // 中间结果可能多一位, 随后的减法会修正
    let mut overflow = add_and_return_overflow(&mut z[n2..], &head[..n]);
    overflow += add_and_return_overflow(&mut z[n2..], &head[n..]);
    // X_diff = X1 - X0; Y_diff = Y0 - Y1 (同在 head 前 n 位, 时间上先后复用)
    let mut sign = 1;
    let x1 = &x[n2..n.min(x.len())];
    let y1 = &y[n2..n.min(y.len())];
    karatsuba_subtraction_helper(&mut head[..n2], x1, &x[..n2], &mut sign);
    karatsuba_subtraction_helper(&mut head[n2..n], &y[..n2], y1, &mut sign);
    // P1 = X_diff * Y_diff (输入与输出都在 head 内, 用 split 借出)
    let (front, p1) = head.split_at_mut(n);
    self.karatsuba_main(p1, &front[..n2], &front[n2..], recurs, n2);
    if sign > 0 {
      overflow += add_and_return_overflow(&mut z[n2..], p1);
    } else {
      overflow -= sub_and_return_borrow(&mut z[n2..], p1);
    }
    // 中间结果可能偏大, 最终结果必然收窄
    debug_assert!(overflow == 0);
  }
}
