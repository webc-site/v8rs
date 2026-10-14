// 由 tool/src/numbers/ieee754.h.js 从 v8/src/numbers/ieee754.{h,cc} 生成; 修订就地改, 复跑覆盖
// math::pow 分派层与 IEEE 754 浮点运算辅助

/// math::pow 决策分支登记 (原版 ieee754.cc, 按求值序)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowBranch {
  /// 指数 NaN → quiet NaN
  ExponentNaN,
  /// |base|==1 且指数 ±∞ → NaN (ECMAScript 历史; IEEE 754-2008 规定为 1)
  BaseOneInfiniteExponent,
  /// 底数 NaN → 归一 quiet NaN (libm 区分 signaling, JS 不区分)
  BaseNaN,
  /// y==2 → x*x
  Square,
  /// y==0.5 → sqrt(x+0); x 为 -∞ 时 → +∞
  Sqrt,
  /// use_std_math_pow: 走 std::pow
  StdPow,
  /// 兜底: base::ieee754::legacy::pow
  LegacyPow,
}

/// pow 分派与 IEEE 754 运算辅助
pub mod pow_skel {
  use super::PowBranch;
  use crate::ieee754;

  /// 求值序分支数对数守卫
  pub const BRANCH_COUNT: usize = 6;

  /// correctness fuzzer 跨架构抑制分支: y==0→1, x NaN→x, 其余哑值 42
  pub const FUZZER_DUMMY: f64 = 42.0;

  /// 分支判断分类器 (原版 ieee754.cc 决策序)
  #[inline]
  #[must_use]
  pub fn classify_pow(x: f64, y: f64) -> PowBranch {
    if y.is_nan() {
      PowBranch::ExponentNaN
    } else if y.is_infinite() && (x == 1.0 || x == -1.0) {
      PowBranch::BaseOneInfiniteExponent
    } else if x.is_nan() {
      PowBranch::BaseNaN
    } else if y == 2.0 {
      PowBranch::Square
    } else if y == 0.5 {
      PowBranch::Sqrt
    } else {
      PowBranch::LegacyPow
    }
  }

  /// 原版: double math::pow(double x, double y)
  /// 差异要点: (±1, ±∞) = NaN; y==2 → x*x; y==0.5 → sqrt(x+0) (+0 保证 -0**0.5 = +0)
  #[inline]
  #[must_use]
  pub fn pow(x: f64, y: f64) -> f64 {
    ieee754::pow(x, y)
  }

  /// 带分支分类记录的 pow 求值 (单次分支遍历, 同步完成特化计算与分支登记)
  #[inline]
  #[must_use]
  pub fn pow_with_branch(x: f64, y: f64) -> (f64, PowBranch) {
    if y.is_nan() {
      (f64::NAN, PowBranch::ExponentNaN)
    } else if y.is_infinite() && (x == 1.0 || x == -1.0) {
      (f64::NAN, PowBranch::BaseOneInfiniteExponent)
    } else if x.is_nan() {
      (f64::NAN, PowBranch::BaseNaN)
    } else if y == 2.0 {
      (x * x, PowBranch::Square)
    } else if y == 0.5 {
      let res = if x.is_infinite() {
        f64::INFINITY
      } else {
        (x + 0.0).sqrt()
      };
      (res, PowBranch::Sqrt)
    } else {
      (ieee754::pow(x, y), PowBranch::LegacyPow)
    }
  }

  /// 负零判断
  #[inline]
  #[must_use]
  pub const fn is_minus_zero(x: f64) -> bool {
    x.to_bits() == (-0.0f64).to_bits()
  }

  /// quiet NaN 发生器
  #[inline]
  #[must_use]
  pub const fn quiet_nan() -> f64 {
    f64::NAN
  }
}
