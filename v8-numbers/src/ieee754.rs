//! 原版 v8/src/numbers/ieee754.cc/h — math::pow (ECMAScript Math.pow 前置语义)

use v8_base::ieee754::legacy;

/// Math.pow 的 ECMA-262 语义包装 (ieee754.cc 14-58)
///
/// 与 libm pow 的差异: **(±1, ±∞) = NaN** (ECMAScript 历史行为, IEEE 754-2008
/// 为 1)。y==2 / y==0.5 的特化匹配优化编译器的强度削减;
/// `use_std_math_pow` flag 两条路径在当前实现下等价 (legacy 委托平台 pow),
/// flag 接线推迟 v8-flags (见 crate README)。
#[must_use]
pub fn pow(x: f64, y: f64) -> f64 {
  if y.is_nan() {
    // 指数为 NaN → NaN
    return f64::NAN;
  }
  if y.is_infinite() && (x == 1.0 || x == -1.0) {
    // |ℝ(base)| = 1 且指数 ±∞ → NaN (ECMAScript 历史行为)
    return f64::NAN;
  }
  let x = if x.is_nan() {
    // JS 不区分 quiet/signaling NaN
    f64::NAN
  } else {
    x
  };

  // 优化编译器行为对齐的特化路径
  if y == 2.0 {
    // x ** 2 → x * x
    return x * x;
  }
  if y == 0.5 {
    // x ** 0.5 → sqrt(x); -Inf 例外
    if x.is_infinite() {
      return f64::INFINITY;
    }
    // +0 保证 -0 ** 0.5 = +0
    return (x + 0.0).sqrt();
  }

  legacy::pow(x, y)
}
