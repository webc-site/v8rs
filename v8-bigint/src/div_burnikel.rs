// 由手工从 v8/src/bigint/div-burnikel.cc 转写; 对应原版逐行
//! Burnikel-Ziegler 快速递归除法
//! 参考 "Fast Recursive Division" (Burnikel & Ziegler, 1998)

use crate::{
  bigint::{
    Digit, K_BURNIKEL_THRESHOLD, Platform, add_and_return_carry, bit_length, compare,
    count_leading_zeros, divide_single, inplace_add_and_return_carry,
    inplace_sub_and_return_borrow, norm, subtract_digit, subtract_with_normalization,
  },
  bigint_internal::Processor,
  div_helpers::{left_shift, put_at, right_shift},
  util::div_ceil_u64,
};

/// 比较 [a_high, A] 与 B: <0 / 0 / >0
fn special_compare(a_high: Digit, a: &[Digit], b: &[Digit]) -> i32 {
  let b = norm(b);
  let a_len = if a_high == 0 {
    norm(a).len()
  } else {
    a.len() + 1
  };
  let diff = a_len as i64 - b.len() as i64;
  if diff != 0 {
    return diff as i32;
  }
  let mut i = a_len as i64 - 1;
  if a_high != 0 {
    let bi = b[i as usize];
    if a_high > bi {
      return 1;
    }
    if a_high < bi {
      return -1;
    }
    i -= 1;
  }
  while i >= 0 && a[i as usize] == b[i as usize] {
    i -= 1;
  }
  if i < 0 {
    return 0;
  }
  if a[i as usize] > b[i as usize] { 1 } else { -1 }
}

/// X 全部置 1
fn set_ones(x: &mut [Digit]) {
  x.fill(Digit::MAX);
}

/// Burnikel-Ziegler 递归容器 (不变数据打包)
struct Bz<'a, P: Platform> {
  proc: &'a mut Processor<P>,
  /// D3n2n 的 D 工作区, 大小为最外层块长 (本层 D 视图取前 2n 位)
  scratch: Vec<Digit>,
}

impl<P: Platform> Bz<'_, P> {
  /// 基例: 交给 schoolbook 或单 digit 除法
  fn divide_basecase(&mut self, q: &mut [Digit], r: &mut [Digit], a: &[Digit], b: &[Digit]) {
    let a = norm(a);
    let b = norm(b);
    debug_assert!(!b.is_empty());
    let cmp = compare(a, b);
    if cmp <= 0 {
      q.fill(0);
      if cmp == 0 {
        // A == B: Q=1, R=0
        r.fill(0);
        q[0] = 1;
      } else {
        // A < B: Q=0, R=A
        put_at(r, a, r.len());
      }
      return;
    }
    if b.len() == 1 {
      divide_single(q, a, b[0]);
      self.proc.add_work_estimate(a.len() as u64);
      return;
    }
    self.proc.divide_schoolbook(q, r, a, b)
  }

  /// 算法 2 (论文 D3n2n): A = [A1,A2,A3], B = [B1,B2] 均半长 n, 求 A/B
  fn d3n2n(&mut self, q: &mut [Digit], r: &mut [Digit], a1a2: &[Digit], a3: &[Digit], b: &[Digit]) {
    debug_assert!((b.len() & 1) == 0);
    let n = b.len() / 2;
    debug_assert!(a1a2.len() == 2 * n);
    debug_assert!(compare(&a1a2[n..], b) < 0); // 实际条件更强: A < B * 2^(n*kDigitBits)
    debug_assert!(a3.len() == n);
    debug_assert!(q.len() == n);
    debug_assert!(r.len() == 2 * n);
    let a1 = &a1a2[n..];
    let b1 = &b[n..];
    let qhat = q;
    let mut r1_high = 0;
    if compare(a1, b1) < 0 {
      // 3a. Qhat = [A1,A2]/B1 余 R1 (R1 为 R 的高半)
      let r1 = &mut r[n..];
      self.d2n1n(qhat, r1, a1a2, b1);
      if self.proc.should_terminate() {
        return;
      }
    } else {
      // 3b. Qhat = 2^(kDigitBits*n) - 1; R1 = [A1,A2] - [B1,0] + [0,B1]
      set_ones(qhat);
      // 第一步: A1 - B1 (分支保证不溢出, 结果至多单 digit)
      let r1 = &mut r[n..];
      subtract_with_normalization(r1, a1, b1);
      let temp = norm(r1);
      debug_assert!(temp.len() <= 1);
      if !temp.is_empty() {
        r1_high = temp[0];
      }
      // 第二步: A2 + B1
      let a2 = &a1a2[..n];
      r1_high += add_and_return_carry(r1, a2, b1);
    }
    // 4. D = Qhat * B2
    let b2 = &b[..n];
    let d = &mut self.scratch[..2 * n];
    self.proc.multiply(d, qhat, b2);
    if self.proc.should_terminate() {
      return;
    }
    // 5/6. Rhat = [R1, A3] - D; Rhat < 0 时回加 B 并把商减 1
    put_at(r, a3, n);
    while special_compare(r1_high, r, d) < 0 {
      r1_high += inplace_add_and_return_carry(r, b);
      subtract_digit(qhat, 1);
    }
    // 7. Rhat -= D, 得最终余数
    let borrow = inplace_sub_and_return_borrow(r, d);
    debug_assert!(borrow == r1_high);
    debug_assert!(compare(r, b) < 0);
  }

  /// 算法 1 (论文 D2n1n): A 长至多 2n, B 长 n, 求 A/B
  fn d2n1n(&mut self, q: &mut [Digit], r: &mut [Digit], a: &[Digit], b: &[Digit]) {
    let n = b.len();
    debug_assert!(a.len() <= 2 * n);
    debug_assert!(compare(&a[n.min(a.len())..], b) < 0); // A 高半 < B
    debug_assert!(q.len() == n);
    debug_assert!(r.len() == n);
    // 1. n 为奇数或过小: 基例
    if !n.is_multiple_of(2) || (n as u32) < K_BURNIKEL_THRESHOLD {
      return self.divide_basecase(q, r, a, b);
    }
    // 2. A 切四段, B 切两段
    let a1a2 = &a[n..];
    let a3 = &a[n / 2..n];
    let a4 = &a[..n / 2];
    // 3. 高位商 Q1 = [A1,A2,A3]/[B1,B2] 余 R1
    let (q2, q1) = q.split_at_mut(n / 2);
    let mut r1 = vec![0; n];
    self.d3n2n(q1, &mut r1, a1a2, a3, b);
    if self.proc.should_terminate() {
      return;
    }
    // 4. 低位商 Q2 = [R1,A4]/[B1,B2] 余 R
    self.d3n2n(q2, r, &r1, a4, b);
  }
}

impl<P: Platform> Processor<P> {
  /// 算法 3 (论文主入口): 任意 A/B; R 可为空, Q 不可
  pub fn divide_burnikel_ziegler(
    &mut self,
    q: &mut [Digit],
    r: &mut [Digit],
    a: &[Digit],
    b: &[Digit],
  ) {
    assert!(!norm(b).is_empty(), "division by zero");
    debug_assert!(a.len() >= b.len());
    debug_assert!(r.is_empty() || r.len() >= b.len());
    debug_assert!(q.len() > a.len() - b.len());

    let s = b.len();
    // 1-2. m 取最小 2^k 使 2^k * 阈值 > s; n = j*m
    let m = 1u32 << bit_length(s as u32 / K_BURNIKEL_THRESHOLD);
    let j = div_ceil_u64(s as u64, m as u64) as u32;
    let n = (j * m) as usize;
    // 3-4. B 左移 sigma 规范化, A 同移
    let sigma = count_leading_zeros(b[s - 1]);
    let digit_shift = n - s;
    // temp 布局: [B_shifted: n][Z: 2n][Ri: n][Qi: n] (BZ 自带 D 工作区)
    let mut temp = vec![0; n * 5];
    let (b_shifted, rest) = temp.split_at_mut(n);
    left_shift(&mut b_shifted[digit_shift..], b, sigma);
    for d in b_shifted[..digit_shift].iter_mut() {
      *d = 0;
    }
    // A 顶位若放不下 sigma+1 的左移量则补一 digit; 且 A 顶位须为 0 (第 5 步的 -1)
    let extra_digit = if count_leading_zeros(a[a.len() - 1]) < sigma + 1 {
      1
    } else {
      0
    };
    let r_len = a.len() + digit_shift + extra_digit;
    let mut a_shifted = vec![0; r_len];
    left_shift(&mut a_shifted[digit_shift..], a, sigma);
    for d in a_shifted[..digit_shift].iter_mut() {
      *d = 0;
    }
    // 5. t = min{t >= 2 | A < 2^(kDigitBits*t*n - 1)}
    let t = (div_ceil_u64(r_len as u64, n as u64) as usize).max(2);
    // 6-7. Z_(t-2) = [A_(t-1), A_(t-2)]
    let z_len = n * 2;
    let (z, rest) = rest.split_at_mut(z_len);
    let (ri, qi_area) = rest.split_at_mut(n);
    put_at(z, &a_shifted[n * (t - 2)..], z_len);

    let mut bz = Bz {
      proc: self,
      scratch: vec![
        0;
        if n as u32 >= K_BURNIKEL_THRESHOLD {
          n
        } else {
          0
        }
      ],
    };
    {
      // 首轮展开: Q 顶部可能不足 n 位, 先算到临时区
      let qi = &mut qi_area[..n];
      bz.d2n1n(qi, ri, z, b_shifted);
      if bz.proc.should_terminate() {
        return;
      }
      let qi = norm(qi);
      let target = &mut q[n * (t - 2)..];
      debug_assert!(qi.len() <= target.len());
      put_at(target, qi, target.len());
    }
    // 剩余迭代
    let mut i = t as i64 - 3;
    while i >= 0 {
      let iu = i as usize;
      // Z_(iu-1) = [Ri, A_i]
      put_at(&mut z[n..], ri, n);
      put_at(z, &a_shifted[n * iu..], n);
      let qi_end = (iu * n + n).min(q.len());
      let qi = &mut q[iu * n..qi_end];
      bz.d2n1n(qi, ri, z, b_shifted);
      if bz.proc.should_terminate() {
        return;
      }
      i -= 1;
    }
    // 9. R = R_0 * 2^(-sigma)
    debug_assert!((0..digit_shift).all(|idx| ri[idx] == 0));
    if !r.is_empty() {
      let ri_part = norm(&ri[digit_shift..]);
      debug_assert!(ri_part.len() <= r.len());
      right_shift(r, ri_part, sigma);
    }
  }
}
