// 由手工从 v8/src/bigint/mul-toom.cc 转写; 对应原版逐行
//! Toom-Cook 乘法 (Toom-3)
//! 原版用指针别名做就地运算, 此处以段视图 + Z==X/Z==Y 变体函数保持语义

use crate::{
  bigint::{
    Digit, K_DIGIT_BITS, K_HALF_DIGIT_BITS, K_HALF_DIGIT_MASK, Platform, add, add_signed,
    add_signed_zx, norm, subtract_signed_zx, subtract_signed_zy,
  },
  bigint_internal::Processor,
  util::div_ceil_u64,
  vector_arithmetic::add_and_return_overflow,
};

/// X *= 2 就地
fn times_two(x: &mut [Digit]) {
  let mut carry = 0;
  for d in x.iter_mut() {
    let old = *d;
    *d = (old << 1) | carry;
    carry = old >> (K_DIGIT_BITS - 1);
  }
}

/// X /= 2 就地
fn divide_by_two(x: &mut [Digit]) {
  let mut carry = 0;
  for d in x.iter_mut().rev() {
    let old = *d;
    *d = (old >> 1) | carry;
    carry = old << (K_DIGIT_BITS - 1);
  }
}

/// X /= 3 就地 (半 digit 分解, 避免宽除法)
fn divide_by_three(x: &mut [Digit]) {
  let mut remainder = 0;
  for d in x.iter_mut().rev() {
    let old = *d;
    let upper = (remainder << K_HALF_DIGIT_BITS) | (old >> K_HALF_DIGIT_BITS);
    let u_result = upper / 3;
    remainder = upper - 3 * u_result;
    let lower = (remainder << K_HALF_DIGIT_BITS) | (old & K_HALF_DIGIT_MASK);
    let l_result = lower / 3;
    remainder = lower - 3 * l_result;
    *d = (u_result << K_HALF_DIGIT_BITS) | l_result;
  }
}

impl<P: Platform> Processor<P> {
  /// Toom-3 主算法; 变量名与 Wikipedia 文章对应
  fn toom3_main(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit]) {
    debug_assert!(z.len() >= x.len() + y.len());
    // 阶段 1: 切三段
    let i = div_ceil_u64(x.len().max(y.len()) as u64, 3) as usize;
    // 原版 Digits 视图: 偏移越界时长度截为 0
    fn seg(s: &[Digit], from: usize, to: usize) -> &[Digit] {
      &s[from.min(s.len())..to.min(s.len())]
    }
    let x0 = &x[..i.min(x.len())];
    let x1 = seg(x, i, 2 * i);
    let x2 = seg(x, 2 * i, 3 * i);
    let y0 = &y[..i.min(y.len())];
    let y1 = seg(y, i, 2 * i);
    let y2 = seg(y, 2 * i, 3 * i);

    // 临时空间: 前半 [po|qo] 后半 [p_1|q_1] 各 r_len, 再加 r_1, r_m1
    let p_len = i + 1;
    let r_len = 2 * p_len;
    let mut t = vec![0; 4 * r_len];
    // r_0 直接共享 Z 的低半部
    let r0_len = r_len.min(z.len());

    // 阶段 2a: 求值 0/1/-1 三个点
    let (t01, t23) = t.split_at_mut(2 * r_len);
    let (po_qo, p1_q1) = t01.split_at_mut(r_len);
    let (po, qo) = po_qo.split_at_mut(p_len);
    let (p1, q1) = p1_q1.split_at_mut(p_len);
    let (r1, rm1) = t23.split_at_mut(r_len);
    // po = X0 + X2; p_1 = po + X1; p_m1 = po - X1 (就地)
    add(po, x0, x2);
    add(p1, po, x1);
    let p_m1_sign = subtract_signed_zx(po, false, x1, false);
    // qo = Y0 + Y2; q_1 = qo + Y1; q_m1 = qo - Y1 (就地)
    add(qo, y0, y2);
    add(q1, qo, y1);
    let q_m1_sign = subtract_signed_zx(qo, false, y1, false);

    // 阶段 3a: 逐点乘 0/1/-1
    self.multiply(&mut z[..r0_len], x0, y0);
    self.multiply(r1, p1, q1);
    self.multiply(rm1, po, qo);
    let r_m1_sign = p_m1_sign != q_m1_sign;

    // 阶段 2b: 求值 -2 与 ∞; p_m2 复用 p_1 存储
    let p_m2_sign = add_signed(p1, po, p_m1_sign, x2, false);
    times_two(p1);
    let p_m2_sign = subtract_signed_zx(p1, p_m2_sign, x0, false);
    let q_m2_sign = add_signed(q1, qo, q_m1_sign, y2, false);
    times_two(q1);
    let q_m2_sign = subtract_signed_zx(q1, q_m2_sign, y0, false);

    // 阶段 3b: r_m2 = [po|qo] 整段, r_inf = [p_1|q_1] 整段 (覆盖旧值)
    self.multiply(po_qo, p1, q1);
    let r_m2_sign = p_m2_sign != q_m2_sign;
    self.multiply(p1_q1, x2, y2);

    // 阶段 4: 插值
    // R3 <- (r_m2 - r_1) / 3   (Z==X 就地)
    let mut r3_sign = subtract_signed_zx(po_qo, r_m2_sign, r1, false);
    divide_by_three(po_qo);
    // R1 <- (r_1 - r_m1) / 2
    let mut r1_sign = subtract_signed_zx(r1, false, rm1, r_m1_sign);
    divide_by_two(r1);
    // R2 <- r_m1 - r_0
    let mut r2_sign = subtract_signed_zx(rm1, r_m1_sign, &z[..r0_len], false);
    // R3 <- (R2 - R3) / 2 + 2 * r_inf   (Z==Y 就地)
    r3_sign = subtract_signed_zy(po_qo, rm1, r2_sign, r3_sign);
    divide_by_two(po_qo);
    r3_sign = add_signed_zx(po_qo, r3_sign, p1_q1, false);
    r3_sign = add_signed_zx(po_qo, r3_sign, p1_q1, false);
    // R2 <- R2 + R1 - R4
    r2_sign = add_signed_zx(rm1, r2_sign, r1, r1_sign);
    r2_sign = subtract_signed_zx(rm1, r2_sign, p1_q1, false);
    // R1 <- R1 - R3
    r1_sign = subtract_signed_zx(r1, r1_sign, po_qo, r3_sign);

    debug_assert!(!r1_sign || norm(r1).is_empty());
    debug_assert!(!r2_sign || norm(rm1).is_empty());
    debug_assert!(!r3_sign || norm(po_qo).is_empty());

    // 阶段 5: 重组 (R0 已就位), 不会溢出
    z[r0_len..].fill(0);
    add_and_return_overflow(&mut z[i..], r1);
    add_and_return_overflow(&mut z[2 * i..], rm1);
    add_and_return_overflow(&mut z[3 * i..], po_qo);
    add_and_return_overflow(&mut z[4 * i..], p1_q1);
  }

  /// Toom-Cook 入口, 大输入切成 Y 等长的块
  pub fn multiply_toom_cook(&mut self, z: &mut [Digit], x: &[Digit], y: &[Digit]) {
    debug_assert!(x.len() >= y.len());
    let k = y.len();
    let x0 = &x[..k];
    self.toom3_main(z, x0, y);
    if x.len() > y.len() {
      let mut t = vec![0; 2 * k];
      let mut i = k;
      while i < x.len() {
        let xi = &x[i..(i + k).min(x.len())];
        self.toom3_main(&mut t, xi, y);
        add_and_return_overflow(&mut z[i..], &t);
        i += k;
      }
    }
  }
}
