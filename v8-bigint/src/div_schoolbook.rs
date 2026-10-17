// 由手工从 v8/src/bigint/div-schoolbook.cc 转写; 对应原版逐行
//! Knuth Algorithm D 除法 (Schoolbook), 含乘法逆元 digit 除法加速

use crate::{
  bigint::{
    Digit, K_DIGIT_BITS, Platform, TwoDigit, digit_add2, digit_add3, digit_mul,
    inplace_add_and_return_carry, inplace_sub_and_return_borrow, norm,
  },
  bigint_internal::Processor,
  div_helpers::{ShiftedDigits, left_shift, right_shift},
  mul_helpers::multiply_single,
};

/// (factor1 * factor2) > (high << kDigitBits) + low ?
fn product_greater_than(factor1: Digit, factor2: Digit, high: Digit, low: Digit) -> bool {
  let (result_low, result_high) = digit_mul(factor1, factor2);
  result_high > high || (result_high == high && result_low > low)
}

/// 基于模乘逆元的单 digit 除法 (要求除数最高位为 1)
/// 参考 https://gmplib.org/~tege/division-paper.pdf "Algorithm 4"
struct MultiplicativeDigitDiv {
  divisor: Digit,
  inverse: Digit,
}

impl MultiplicativeDigitDiv {
  fn new(divisor: Digit) -> Self {
    debug_assert!((divisor >> (K_DIGIT_BITS - 1)) == 1);
    Self {
      divisor,
      inverse: Self::compute_inverse(divisor),
    }
  }

  fn compute_inverse(divisor: Digit) -> Digit {
    // ((~divisor) << kDigitBits | ~0) / divisor
    let n = ((!divisor as TwoDigit) << K_DIGIT_BITS) | Digit::MAX as TwoDigit;
    (n / divisor as TwoDigit) as Digit
  }

  /// 返回 (商, 余数); 要求 high < divisor
  fn div(&self, high: Digit, low: Digit) -> (Digit, Digit) {
    // 论文记号: u1=high, u0=low, d=divisor, v=inverse; "mod β" 即截断到 digit 宽
    let (mut q0, mut q1) = digit_mul(high, self.inverse);
    // <q1,q0> += <u1,u0>
    let (v, carry) = digit_add2(q0, low);
    q0 = v;
    let (v2, _) = digit_add3(q1, high, carry);
    q1 = v2;
    // q1 = (q1 + 1) mod β
    q1 = q1.wrapping_add(1);
    // r = (u0 - q1*d) mod β
    let mut r = low.wrapping_sub(q1.wrapping_mul(self.divisor));
    if r > q0 {
      q1 = q1.wrapping_sub(1);
      r = r.wrapping_add(self.divisor);
    }
    if r >= self.divisor {
      q1 = q1.wrapping_add(1);
      r -= self.divisor;
    }
    (q1, r)
  }
}

impl<P: Platform> Processor<P> {
  /// Knuth Algorithm D: Q = A/B 的商, R = 余数; Q/R 任一可为空切片
  /// Q 非空时长度 >= A.len - B.len + 1; R 非空时长度 >= B.len
  pub fn divide_schoolbook(&mut self, q: &mut [Digit], r: &mut [Digit], a: &[Digit], b: &[Digit]) {
    assert!(!norm(b).is_empty(), "division by zero");
    debug_assert!(b.len() >= 2); // 单 digit 用 divide_single
    debug_assert!(a.len() >= b.len());
    debug_assert!(r.is_empty() || r.len() >= b.len());

    let n = b.len();
    let m = a.len() - n;

    // scratch 布局: [qhatv: n+1][U: A.len+1]
    let needed = n + 1 + a.len() + 1;
    let mut scratch = vec![0; needed];
    let (qhatv, u) = scratch.split_at_mut(n + 1);

    // D1: 左移使除数最高位为 1 (防 digit 除法溢出)
    let bn = ShiftedDigits::new(b, None);
    let bn_d = bn.digits();
    // U 持续保存被除数剩余部分, 最终成为余数
    left_shift(u, a, bn.shift);

    // D2: 逐位求商; vn1 是除数最高位
    let vn1 = bn_d[n - 1];
    let vn1_divisor = MultiplicativeDigitDiv::new(vn1);
    let mut j = m as i64;
    while j >= 0 {
      let ju = j as usize;
      // D3: 估计当前商 digit (可能偏大)
      let mut qhat = Digit::MAX;
      let ujn = u[ju + n];
      if ujn != vn1 {
        let (q_est, mut rhat) = vn1_divisor.div(ujn, u[ju + n - 1]);
        qhat = q_est;
        // 看低一位修正: qhat * v_{n-2} > (rhat << kDigitBits) + u_{ju+n-2} 则减
        let vn2 = bn_d[n - 2];
        let ujn2 = u[ju + n - 2];
        while product_greater_than(qhat, vn2, rhat, ujn2) {
          qhat -= 1;
          let prev_rhat = rhat;
          rhat = rhat.wrapping_add(vn1);
          // rhat 溢出说明已足够小
          if rhat < prev_rhat {
            break;
          }
        }
      }

      // D4: 乘减; 借位非零说明商估大 1, 回加一次
      if qhat != 0 {
        multiply_single(qhatv, bn_d, qhat);
        self.add_work_estimate(n as u64);
        let u_j = &mut u[ju..n + 1 + ju];
        let c = inplace_sub_and_return_borrow(u_j, qhatv);
        if c != 0 {
          let c2 = inplace_add_and_return_carry(u_j, bn_d);
          u_j[n] = u_j[n].wrapping_add(c2);
          qhat -= 1;
        }
      }

      if !q.is_empty() {
        if ju < q.len() {
          q[ju] = qhat;
        } else {
          debug_assert!(qhat == 0);
        }
      }
      j -= 1;
    }
    if !r.is_empty() {
      right_shift(r, u, bn.shift);
    }
    // Q 多余空间清零 (Q 可为空切片)
    if q.len() > m + 1 {
      for d in q[m + 1..].iter_mut() {
        *d = 0;
      }
    }
  }
}
