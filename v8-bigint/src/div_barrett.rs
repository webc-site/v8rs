// 由手工从 v8/src/bigint/div-barrett.cc 转写; 对应原版逐行
//! Barrett 除法, Newton 迭代求逆
//! 参考 "Fast Division of Large Integers" (Karl Hasselström)
//! 原版 S/U 在同一 scratch 上时间复用且区域重叠 (写落后读), Rust 版 U 用独立缓冲保语义

use crate::{
  bigint::{
    Digit, K_BURNIKEL_THRESHOLD, K_DIGIT_BITS, K_NEWTON_INVERSION_THRESHOLD, Platform, add,
    add_digit, digit_div, digit_sub2, greater_or_equal, inplace_add_and_return_carry,
    inplace_sub_and_return_borrow, norm, subtract_and_return_borrow, subtract_digit,
  },
  bigint_internal::Processor,
  div_helpers::{
    K_INVERT_NEWTON_EXTRA_SPACE, ShiftedDigits, divide_barrett_scratch_space,
    invert_newton_scratch_space, invert_scratch_space, left_shift, put_at, right_shift,
  },
  util::div_ceil_u64,
  vector_arithmetic::is_bit_normalized,
};

impl<P: Platform> Processor<P> {
  /// Z := 1/V 的小数部分 (朴素除法求逆基例)
  fn invert_basecase(&mut self, z: &mut [Digit], v: &[Digit], scratch: &mut [Digit]) {
    debug_assert!(z.len() > v.len());
    debug_assert!(!v.is_empty());
    debug_assert!(scratch.len() >= 2 * v.len());
    let n = v.len();
    let x = &mut scratch[..2 * n];
    // X = 2^(2n*kDigitBits) - V*2^(n*kDigitBits) 的补码形式
    let mut borrow = 0;
    x[..n].fill(0);
    for (xi, vi) in x[n..2 * n].iter_mut().zip(v) {
      let (val, b) = digit_sub2(0, *vi, borrow);
      *xi = val;
      borrow = b;
    }
    debug_assert!(borrow == 1);
    let r: &mut [Digit] = &mut [];
    if (n as u32) < K_BURNIKEL_THRESHOLD {
      self.divide_schoolbook(z, r, x, v);
    } else {
      self.divide_burnikel_ziegler(z, r, x, v);
    }
  }

  /// 论文算法 4.2: Newton 迭代求逆, 精度 vn+1 digit, 结果带隐式顶位 1
  fn invert_newton(&mut self, z: &mut [Digit], v: &[Digit], scratch: &mut [Digit]) {
    let vn = v.len();
    debug_assert!(z.len() >= vn);
    debug_assert!(scratch.len() >= invert_newton_scratch_space(vn as u32) as usize);

    debug_assert!(v.len() >= 3);
    let basecase_precision =
      (K_NEWTON_INVERSION_THRESHOLD - 1).min(div_ceil_u64(vn as u64, 2) as u32);
    debug_assert!(v.len() as u32 > basecase_precision);
    debug_assert!(is_bit_normalized(v));

    // 步骤 1: 每步精度 (k 为当前迭代的分数位数)
    let mut k = (vn as u64) * K_DIGIT_BITS as u64;
    let mut target_fraction_bits = vec![0u64; 64];
    let mut iteration: i64 = -1;
    while k > basecase_precision as u64 * K_DIGIT_BITS as u64 {
      iteration += 1;
      target_fraction_bits[iteration as usize] = k;
      k = div_ceil_u64(k, 2);
    }
    let mut k = k as u32;

    // 步骤 2: 基例初始逼近
    let initial_digits = div_ceil_u64(k as u64 + 1, K_DIGIT_BITS as u64) as usize;
    let top_part_of_v = &v[vn - initial_digits..];
    self.invert_basecase(z, top_part_of_v, scratch);
    z[initial_digits] = z[initial_digits].wrapping_add(1); // 隐式顶位物化
    let mut z_len = initial_digits + 1;

    // 步骤 3: 精度倍增循环
    loop {
      // (3b): S = Z² (顶位弃用)
      {
        let s = &mut scratch[..2 * z_len];
        self.multiply(s, &z[..z_len], &z[..z_len]);
        if self.should_terminate() {
          return;
        }
      }
      let s_len = 2 * z_len - 1;
      // (3c): T = V 截到至少 2k+3 个分数位
      let fraction_digits = div_ceil_u64(2 * k as u64 + 3, K_DIGIT_BITS as u64) as usize;
      let t_len = v.len().min(fraction_digits);
      let t = &v[v.len() - t_len..];
      // (3d): U = T * S 截到至少 2k+1 个分数位 (U 有一个可能为零的整数位)
      let fraction_digits = div_ceil_u64(2 * k as u64 + 1, K_DIGIT_BITS as u64) as usize;
      let u_len = s_len + t_len;
      let mut u_buf = vec![0; u_len];
      self.multiply(&mut u_buf, &scratch[..s_len], t);
      if self.should_terminate() {
        return;
      }
      let u_len_effective = 1 + fraction_digits;
      let u = &u_buf[u_len - u_len_effective..];
      // (3e): W = 2 * Z, 低部补零对齐 U 的分数位数 (覆盖已死的 S 区域)
      let w_len = u.len();
      debug_assert!(w_len >= z_len);
      let padding_digits = w_len - z_len;
      {
        let w = &mut scratch[..w_len];
        w[..padding_digits].fill(0);
        left_shift(&mut w[padding_digits..], &z[..z_len], 1);
      }
      // (3f): Z = W - U
      if u.len() <= vn {
        // 常规减法 (非末轮)
        debug_assert!(iteration > 0);
        z_len = u.len();
        let borrow = subtract_and_return_borrow(z, &scratch[..w_len], u);
        debug_assert!(borrow == 0);
      } else {
        // 末轮: 截到 vn 位分数, 整数位单独算
        debug_assert!(iteration == 0);
        let w_part = &scratch[w_len - vn - 1..w_len - 1];
        let u_part = &u[u.len() - vn - 1..u.len() - 1];
        let borrow = subtract_and_return_borrow(z, w_part, u_part);
        let integer_part = scratch[w_len - 1]
          .wrapping_sub(u[u.len() - 1])
          .wrapping_sub(borrow);
        debug_assert!(integer_part == 1 || integer_part == 2);
        if integer_part == 2 {
          // 正确结果应是 2.0, 只能表示为 [1.]9999...
          z[..vn].fill(Digit::MAX);
        }
        break;
      }
      // (3g, 3h): 更新并循环
      k = target_fraction_bits[iteration as usize] as u32;
      iteration -= 1;
    }
    let _ = K_INVERT_NEWTON_EXTRA_SPACE; // scratch 布局常量由 invert_newton_scratch_space 统一核算
  }

  /// 求 1/V (左移 2*vn 个 digit), 精度 vn+1, 结果带隐式顶位 1
  fn invert(&mut self, z: &mut [Digit], v: &[Digit], scratch: &mut [Digit]) {
    self.invert_pub(z, v, scratch)
  }

  /// invert 的跨模块入口 (to_string 的 RecursionLevel 用)
  pub(crate) fn invert_pub(&mut self, z: &mut [Digit], v: &[Digit], scratch: &mut [Digit]) {
    debug_assert!(z.len() > v.len());
    debug_assert!(!v.is_empty());
    debug_assert!(is_bit_normalized(v));
    debug_assert!(scratch.len() >= invert_scratch_space(v.len() as u32) as usize);

    let vn = v.len();
    if vn as u32 >= K_NEWTON_INVERSION_THRESHOLD {
      return self.invert_newton(z, v, scratch);
    }
    if vn == 1 {
      let d = v[0];
      z[0] = digit_div(!d, Digit::MAX, d).0;
      z[1] = 0;
    } else {
      self.invert_basecase(z, v, scratch);
      if z[vn] == 1 {
        z[..vn].fill(Digit::MAX);
        z[vn] = 0;
      }
    }
  }

  /// 论文算法 3.5 核心: 用预计算逆元 I 做 A/B
  pub(crate) fn divide_barrett_core(
    &mut self,
    q: &mut [Digit],
    r: &mut [Digit],
    a: &[Digit],
    b: &[Digit],
    i_: &[Digit],
    scratch: &mut [Digit],
  ) {
    debug_assert!(q.len() > a.len() - b.len());
    debug_assert!(r.len() >= b.len());
    debug_assert!(a.len() > b.len()); // 注意是严格大于
    debug_assert!(a.len() <= 2 * b.len());
    debug_assert!(!b.is_empty());
    debug_assert!(is_bit_normalized(b));
    debug_assert!(i_.len() == a.len() - b.len());
    debug_assert!(scratch.len() >= divide_barrett_scratch_space(a.len() as u32) as usize);

    let orig_q_len = q.len();

    // (1): A1 = A 去掉低 B.len 位
    let a1 = &a[b.len()..];
    debug_assert!(a1.len() == i_.len());

    // (2): Q = A1*I 的低 I.len+1 位; I 有隐式顶位 1, 故高部再加 A1
    let k_len = 2 * i_.len();
    let k = &mut scratch[..k_len];
    self.multiply(k, a1, i_);
    if self.should_terminate() {
      return;
    }
    let q_len = i_.len() + 1;
    add(&mut q[..q_len], &k[i_.len()..], a1);

    // (3): R = A - B*Q (近似余数)
    let p = &mut scratch[..a.len() + 1];
    self.multiply(p, b, &q[..q_len]);
    if self.should_terminate() {
      return;
    }
    let borrow = subtract_and_return_borrow(r, a, &p[..b.len()]);
    for d in r[b.len()..].iter_mut() {
      *d = 0;
    }
    let mut r_high = a[b.len()].wrapping_sub(p[b.len()]).wrapping_sub(borrow);

    // 校正 R/Q (迭代次数有上界)
    if r_high >> (K_DIGIT_BITS - 1) == 1 {
      // (5b): R < 0, R += B, Q 减
      let mut q_sub = 0;
      loop {
        r_high = r_high.wrapping_add(inplace_add_and_return_carry(r, b));
        q_sub += 1;
        debug_assert!(q_sub <= 5);
        if r_high == 0 {
          break;
        }
      }
      subtract_digit(&mut q[..q_len], q_sub);
    } else {
      let mut q_add = 0;
      while r_high != 0 || greater_or_equal(r, b) {
        // (5c): R >= B, R -= B, Q 加
        r_high = r_high.wrapping_sub(inplace_sub_and_return_borrow(r, b));
        q_add += 1;
        debug_assert!(q_add <= 5);
      }
      add_digit(&mut q[..q_len], q_add);
    }
    // (5a): 恢复 Q 原长, 高部清零
    for d in q[q_len..orig_q_len].iter_mut() {
      *d = 0;
    }
  }

  /// Barrett 除法入口 (含规范化与任意长度分块)
  pub fn divide_barrett(&mut self, q: &mut [Digit], r: &mut [Digit], a: &[Digit], b: &[Digit]) {
    assert!(!norm(b).is_empty(), "division by zero");
    debug_assert!(q.len() > a.len() - b.len());
    debug_assert!(r.len() >= b.len());
    debug_assert!(a.len() > b.len()); // 注意是严格大于
    debug_assert!(!b.is_empty());

    // 规范化 B, A 同移 (Rust 版 ShiftedDigits 恒为独立副本)
    let b_normalized = ShiftedDigits::new(b, None);
    let a_normalized = ShiftedDigits::new(a, Some(b_normalized.shift));
    let b = b_normalized.digits();
    let a = a_normalized.digits();

    // 核心算法仅支持 A ≤ 2*B; 更长的按 B 尺寸分块 (同 Burnikel-Ziegler)
    let barrett_dividend_length = a.len().min(2 * b.len());
    let i_len = barrett_dividend_length - b.len();
    // +1 供 Invert() 临时使用
    let mut i_buf = vec![0; i_len + 1];
    let scratch_len = invert_scratch_space(i_len as u32)
      .max(divide_barrett_scratch_space(barrett_dividend_length as u32))
      as usize;
    let mut scratch = vec![0; scratch_len];
    self.invert(&mut i_buf, &b[b.len() - i_len..], &mut scratch);
    if self.should_terminate() {
      return;
    }
    let i_ = &i_buf[..i_len]; // TrimOne: 摘掉隐式顶位
    debug_assert!(i_.len() == i_len);
    if a.len() > 2 * b.len() {
      let n = b.len();
      // (5): t 为 A 的 B 尺寸块数
      let t = div_ceil_u64(a.len() as u64, n as u64) as usize;
      debug_assert!(t >= 3);
      // (6)/(7): Z 取最高两块
      let z_len = n * 2;
      let mut z = vec![0; z_len];
      put_at(&mut z, &a[n * (t - 2)..], z_len);
      let qi_len = n + 1;
      let mut qi = vec![0; qi_len];
      let mut ri = vec![0; n];
      // 首轮展开
      {
        let i0 = t - 2;
        self.divide_barrett_core(&mut qi, &mut ri, &z, b, i_, &mut scratch);
        if self.should_terminate() {
          return;
        }
        let target = &mut q[n * i0..];
        let to_copy = qi_len.min(target.len());
        target[..to_copy].copy_from_slice(&qi[..to_copy]);
        for d in target[to_copy..].iter_mut() {
          *d = 0;
        }
        debug_assert!((to_copy..qi_len).all(|idx| qi[idx] == 0));
      }
      // 剩余迭代
      let mut i = t as i64 - 3;
      while i >= 0 {
        let iu = i as usize;
        // Z_(iu-1) = [Ri, A_i]
        put_at(&mut z[n..], &ri, n);
        put_at(&mut z, &a[n * iu..], n);
        self.divide_barrett_core(&mut qi, &mut ri, &z, b, i_, &mut scratch);
        debug_assert!(qi[qi_len - 1] == 0);
        if self.should_terminate() {
          return;
        }
        put_at(&mut q[n * iu..], &qi, n);
        i -= 1;
      }
      let ri = norm(&ri);
      debug_assert!(ri.len() <= r.len());
      // R = R_0 * 2^(-shift)
      right_shift(r, ri, b_normalized.shift);
    } else {
      self.divide_barrett_core(q, r, a, b, i_, &mut scratch);
      if self.should_terminate() {
        return;
      }
      // 就地右移 (经临时副本保语义)
      let tmp = r.to_vec();
      right_shift(r, &tmp, b_normalized.shift);
    }
  }
}
