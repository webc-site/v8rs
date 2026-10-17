// 由手工从 v8/src/bigint/bigint-internal.h + bigint-internal.cc 转写; 对应原版逐行
//! 处理器: 慢操作入口 (乘/除/模), 中断预算, CachedMod 逆元缓存
//! 原版 Processor/ProcessorImpl 二分结构合并为单一 Processor<P>, 各算法在自身模块中分散 impl

use crate::{
  bigint::{
    Digit, K_BARRETT_THRESHOLD, K_BURNIKEL_THRESHOLD, K_DIGIT_BITS, K_FFT_THRESHOLD,
    K_KARATSUBA_THRESHOLD, K_TOOM_THRESHOLD, Platform, Status, add_digit, digit_add2, digit_mul,
    divide_result_length, greater_or_equal, inplace_add_and_return_carry,
    inplace_sub_and_return_borrow, is_digit_normalized, msd, norm, subtract_and_return_borrow,
  },
  mul_helpers::{
    multiply_schoolbook, multiply_single, multiply_special_high, multiply_special_low,
  },
};

/// 防止 scratch 空间与位数计算溢出的上限
pub const K_MAX_NUM_DIGITS: u32 = u32::MAX / K_DIGIT_BITS;

/// 每单位近似对应一条 CPU 乘法指令; 到阈值就查询一次中断
pub const K_WORK_ESTIMATE_THRESHOLD: u64 = 5_000_000;

/// CachedMod 折叠商上限 (校正循环次数约束)
const K_MAX_FOLD_QUOTIENT: Digit = 4;

pub struct Processor<P: Platform> {
  platform: P,
  status: Status,
  work_estimate: u64,
  /// 缓存除数 (canonical)
  cached_divisor: Vec<Digit>,
  /// 缓存逆元 (可能为折叠模式下的陈旧占位)
  cached_inverse: Vec<Digit>,
  cached_mod_fold_factor: Digit,
  divisor_count: i32,
}

impl<P: Platform> Processor<P> {
  pub fn new(platform: P) -> Self {
    Self {
      platform,
      status: Status::Ok,
      work_estimate: 0,
      cached_divisor: Vec::new(),
      cached_inverse: Vec::new(),
      cached_mod_fold_factor: 0,
      divisor_count: 0,
    }
  }

  pub fn platform(&mut self) -> &mut P {
    &mut self.platform
  }

  pub(crate) fn get_and_clear_status(&mut self) -> Status {
    let r = self.status;
    self.status = Status::Ok;
    r
  }

  pub(crate) fn should_terminate(&self) -> bool {
    self.status == Status::Interrupted
  }

  pub(crate) fn add_work_estimate(&mut self, estimate: u64) {
    self.work_estimate = self.work_estimate.wrapping_add(estimate);
    if self.work_estimate >= K_WORK_ESTIMATE_THRESHOLD {
      self.work_estimate = 0;
      if self.platform.interrupt_requested() {
        self.status = Status::Interrupted;
      }
    }
  }

  /// 重置中断预算, 避免下一次小操作恰好触发中断查询
  pub fn reset_interrupt_check_budget(&mut self) {
    self.work_estimate = 0;
  }

  /// 除数计数 (帮助调用方判断是否值得换 CachedMod 缓存)
  pub fn inc_divisor_count(&mut self) -> i32 {
    self.divisor_count += 1;
    self.divisor_count
  }

  pub fn reset_divisor_count(&mut self) {
    self.divisor_count = 1;
  }

  // -------------------------------------------------------------------
  // 乘法调度 (原版 ProcessorImpl::Multiply)
  // -------------------------------------------------------------------

  /// 按尺寸选择算法的统一乘法入口
  pub(crate) fn multiply(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit]) {
    let (x, y) = (norm(x), norm(y));
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
    self.multiply_large_impl(z, x, y)
  }

  fn multiply_large_impl(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit]) {
    debug_assert!(is_digit_normalized(x));
    debug_assert!(is_digit_normalized(y));
    debug_assert!(x.len() >= y.len());
    assert!(x.len() as u32 <= K_MAX_NUM_DIGITS);
    if (y.len() as u32) < K_TOOM_THRESHOLD {
      self.multiply_karatsuba(z, x, y);
    } else if (y.len() as u32) < K_FFT_THRESHOLD {
      self.multiply_toom_cook(z, x, y);
    } else {
      self.multiply_fft(z, x, y);
    }
  }

  /// 大输入乘法 (公开 API)
  pub fn multiply_large(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit]) -> Status {
    self.multiply_large_impl(z, x, y);
    self.get_and_clear_status()
  }

  /// 大输入除法: Q := A / B
  pub fn divide_large(&mut self, q: &mut [Digit], a: &[Digit], b: &[Digit]) -> Status {
    assert!(!norm(b).is_empty(), "division by zero");
    debug_assert!(is_digit_normalized(a));
    debug_assert!(is_digit_normalized(b));
    debug_assert!(b.len() > 1);
    let r0: &mut [Digit] = &mut [];
    if (b.len() as u32) < K_BURNIKEL_THRESHOLD {
      self.divide_schoolbook(q, r0, a, b);
      return self.get_and_clear_status();
    }
    assert!(a.len() as u32 <= K_MAX_NUM_DIGITS);
    if (b.len() as u32) < K_BARRETT_THRESHOLD || a.len() == b.len() {
      self.divide_burnikel_ziegler(q, r0, a, b);
    } else {
      let mut r = vec![0; b.len()];
      self.divide_barrett(q, &mut r, a, b);
    }
    self.get_and_clear_status()
  }

  /// 大输入取模: R := A % B
  pub fn modulo_large(&mut self, r: &mut [Digit], a: &[Digit], b: &[Digit]) -> Status {
    assert!(!norm(b).is_empty(), "division by zero");
    debug_assert!(is_digit_normalized(a));
    debug_assert!(is_digit_normalized(b));
    debug_assert!(b.len() > 1);
    if (b.len() as u32) < K_BURNIKEL_THRESHOLD {
      let q: &mut [Digit] = &mut [];
      self.divide_schoolbook(q, r, a, b);
      return self.get_and_clear_status();
    }
    assert!((a.len() as u32) < K_MAX_NUM_DIGITS);
    let q_len = divide_result_length(a, b) as usize;
    let mut q = vec![0; q_len];
    if (b.len() as u32) < K_BARRETT_THRESHOLD || a.len() == b.len() {
      self.divide_burnikel_ziegler(&mut q, r, a, b);
    } else {
      self.divide_barrett(&mut q, r, a, b);
    }
    self.get_and_clear_status()
  }

  // -------------------------------------------------------------------
  // CachedMod (原版 Processor::CachedMod 族)
  // -------------------------------------------------------------------

  /// 预计算 B 的逆元 (或折叠因子); 之后可反复调用 cached_mod
  pub fn cached_mod_make_inverse(&mut self, b: &[Digit]) {
    let n = b.len();
    let cached_b: Vec<Digit> = b.to_vec();
    // B 必须已规范化且非零
    assert!(
      !cached_b.is_empty() && msd(&cached_b) != 0,
      "division by zero"
    );

    self.cached_mod_fold_factor = 0;

    // T := 1 << (n * kDigitBits): 检验除数是否贴近 digit 基的幂
    // 布局: [T: n+1][Q: 2][R: n] | [A: 2n] (前后两段时间上先后复用)
    let mut scratch = vec![0; n + 1 + 2 + n];
    {
      let (t, rest) = scratch.split_at_mut(n + 1);
      let (qq, rr) = rest.split_at_mut(2);
      t.fill(0);
      t[n] = 1;
      self.divide_schoolbook(qq, rr, t, &cached_b);

      let q = norm(qq);
      let r = norm(rr);
      // 商很小且余数为单 digit 时, 记折叠因子直接返回
      if q.len() == 1 && q[0] <= K_MAX_FOLD_QUOTIENT && r.len() == 1 {
        self.cached_divisor = cached_b;
        self.cached_inverse = vec![0; n + 1]; // 折叠模式下陈旧占位
        self.cached_mod_fold_factor = r[0];
        return;
      }
    }

    // A = 2^2n - (B << n) 的近似 (按位取反实现, 见原版注释)
    let a = &mut scratch[..2 * n];
    a[..n].fill(Digit::MAX);
    for (ai, bi) in a[n..].iter_mut().zip(cached_b.iter()) {
      *ai = !bi;
    }
    let mut inv_full = vec![0; n + 1];
    let r: &mut [Digit] = &mut [];
    self.divide_schoolbook(&mut inv_full, r, a, &cached_b);
    // 还原近似减法: Inv += 1 << n
    add_digit(&mut inv_full[n..], 1);
    // 再 +1 提高校正循环的命中率, 但有溢出风险时不做
    if inv_full[0] != Digit::MAX || inv_full[n] != Digit::MAX {
      add_digit(&mut inv_full, 1);
    }
    self.cached_divisor = cached_b;
    self.cached_inverse = inv_full;
  }

  /// R := A % B (复用缓存逆元); 要求先 cached_mod_make_inverse, A.len ∈ [n, 2n]
  pub fn cached_mod(&self, r: &mut [Digit], a: &[Digit]) -> Digit {
    let c = self.cached_mod_fold_factor;
    if c != 0 {
      // 折叠模式: cached_inverse 已分配但陈旧, 本路径不读它
      return cached_mod_fold(&self.cached_divisor, r, a, c);
    }
    let b = &self.cached_divisor[..];
    let inv = &self.cached_inverse[..];
    let n = b.len();
    debug_assert!(n >= 2);
    debug_assert!(n <= a.len() && a.len() <= 2 * n);
    debug_assert!(inv.len() == n + 1);
    debug_assert!(r.len() == n);

    let scratch_space = a.len() + inv.len();
    // 原版复用处理器上的小 scratch; 此处等价局部分配
    let mut scratch = vec![0; scratch_space];
    // 乘以逆元估计商: 只取列 [2n-2, ...), 跳过的两列进位使 Q 至多比真商小 1
    let start_position = 2 * n - 2;
    if a.len() >= inv.len() {
      multiply_special_high(&mut scratch, a, inv, start_position);
    } else {
      multiply_special_high(&mut scratch, inv, a, start_position);
    }
    // 校正: 只需低 n+1 位乘积 (高位与 A 的高位一致)
    // Q = scratch[2n..], 与低段同时借出
    {
      let (product_low, rest) = scratch.split_at_mut(n + 1);
      let q = &rest[n - 1..]; // 绝对偏移 2n = (n+1) + (n-1)
      multiply_special_low(product_low, b, q);
    }

    let borrow = subtract_and_return_borrow(r, a, &scratch[..n]);
    let an = if a.len() > n { a[n] } else { 0 };
    let mut r_high = an.wrapping_sub(scratch[n]).wrapping_sub(borrow);
    if r_high & (1 << (K_DIGIT_BITS - 1)) != 0 {
      loop {
        r_high = r_high.wrapping_add(inplace_add_and_return_carry(r, b));
        if r_high == 0 {
          break;
        }
      }
    } else {
      while r_high != 0 || greater_or_equal(r, b) {
        r_high = r_high.wrapping_sub(inplace_sub_and_return_borrow(r, b));
      }
    }
    r[n - 1]
  }

  /// 缓存的除数 (供调用方判断是否值得换缓存)
  pub fn get_cached_divisor(&self) -> &[Digit] {
    debug_assert!(!self.cached_divisor.is_empty());
    &self.cached_divisor
  }
}

/// Crandall 折叠: 高半部分乘以 2^K/B 的余因子 c 后并回低半部分
/// 独立函数 (原版为 Processor 方法, 但只读缓存数据)
pub fn cached_mod_fold(b: &[Digit], r: &mut [Digit], a: &[Digit], c: Digit) -> Digit {
  let n = b.len();
  debug_assert!(n >= 2);
  debug_assert!(n <= a.len() && a.len() <= 2 * n);
  debug_assert!(r.len() == n);

  // 每列和 A[i] + A[n+i]*c + carry 至多 (D-1) + (D-1)^2 + (D-1) == D^2 - 1, 两 digit 装得下
  let mut carry = 0;
  for i in 0..n {
    let mut high = 0;
    let mut low = a[i];
    if n + i < a.len() {
      let (product_low, ph) = digit_mul(a[n + i], c);
      let (v, add_carry) = digit_add2(low, product_low);
      low = v;
      high = ph + add_carry;
    }
    let (v, sum_carry) = digit_add2(low, carry);
    r[i] = v;
    debug_assert!(high <= Digit::MAX - sum_carry);
    carry = high + sum_carry;
  }

  if carry != 0 {
    let (product_low, product_high) = digit_mul(carry, c);
    let (v, cr) = digit_add2(r[0], product_low);
    r[0] = v;
    carry = cr.wrapping_add(product_high);
    let mut i = 1;
    while i < n && carry != 0 {
      let (v2, c2) = digit_add2(r[i], carry);
      r[i] = v2;
      carry = c2;
      i += 1;
    }
  }

  while carry != 0 || greater_or_equal(r, b) {
    carry = carry.wrapping_sub(inplace_sub_and_return_borrow(r, b));
  }

  r[n - 1]
}
