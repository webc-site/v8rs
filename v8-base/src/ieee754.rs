//! 原版 v8/src/base/ieee754.cc/h
//!
//! acosh/asinh/atanh/cosh/sinh 为 .cc 内自含算法, 逐位 1:1 转写。
//! acos/asin/atan/atan2/cos/sin/tan/exp/log/log1p/log2/log10/expm1/cbrt/legacy::pow
//! 原版委托 third_party/llvm-libc `shared::` 实现 (该第三方源码未随本仓库分发,
//! 推迟项: llvm-libc shared math 内核 1:1 转写; 当前以平台 libm 承接, 边界用例
//! 与 unittests 断言逐条核对)。

use std::{f64::consts::LN_2, hint::black_box};

use crate::memory::bit_cast;

/// 提取 double 高低 32 位 (EXTRACT_WORDS)
#[inline]
fn extract_words(d: f64) -> (i32, u32) {
  let bits: u64 = bit_cast(&d);
  ((bits >> 32) as i32, (bits & 0xFFFF_FFFF) as u32)
}

/// GET_HIGH_WORD
#[inline]
fn get_high_word(d: f64) -> i32 {
  let bits: u64 = bit_cast(&d);
  (bits >> 32) as i32
}

/// SET_HIGH_WORD
#[inline]
fn set_high_word(d: &mut f64, v: i32) {
  let mut bits: u64 = bit_cast(d);
  bits &= 0x0000_0000_FFFF_FFFF;
  bits |= (v as u32 as u64) << 32;
  *d = bit_cast(&bits);
}

#[inline]
fn fabs(x: f64) -> f64 {
  x.abs()
}

const SIGNALED_NAN: f64 = f64::NAN; // Rust 无法构造带 signaling NaN 差异的值

#[must_use]
pub fn acos(x: f64) -> f64 {
  x.acos()
}

/* acosh(x) — 1:1 自 .cc
 * acosh(x) is NaN with signal if x<1; acosh(NaN) is NaN without signal. */
#[must_use]
pub fn acosh(x: f64) -> f64 {
  const ONE: f64 = 1.0;
  const LN2: f64 = LN_2; // 0x3FE62E42, 0xFEFA39EF
  let (hx, lx) = extract_words(x);
  if hx < 0x3FF0_0000 {
    // x < 1
    return SIGNALED_NAN;
  } else if hx >= 0x41B0_0000 {
    // x > 2**28
    if hx >= 0x7FF0_0000 {
      // x 是 inf 或 NaN
      return x + x;
    }
    return x.ln() + LN2; // acosh(huge)=log(2x)
  } else if (((hx - 0x3FF0_0000) as u32) | lx) == 0 {
    return 0.0; // acosh(1) = 0
  } else if hx > 0x4000_0000 {
    // 2**28 > x > 2
    let t = x * x;
    return (2.0 * x - ONE / (x + (t - ONE).sqrt())).ln();
  }
  // 1<x<2
  let t = x - ONE;
  log1p(t + (2.0 * t + t * t).sqrt())
}

#[must_use]
pub fn asin(x: f64) -> f64 {
  x.asin()
}

/* asinh(x) — 1:1 自 .cc */
#[must_use]
pub fn asinh(x: f64) -> f64 {
  const ONE: f64 = 1.0;
  const LN2: f64 = LN_2;
  const HUGE: f64 = 1e300;

  let hx = get_high_word(x);
  let ix = hx & 0x7FFF_FFFF;
  if ix >= 0x7FF0_0000 {
    return x + x; // x 是 inf 或 NaN
  }
  if ix < 0x3E30_0000 && HUGE + x > ONE {
    // |x|<2**-28: 除 0 外返回 x 且置 inexact
    return x;
  }
  let w = if ix > 0x41B0_0000 {
    // |x| > 2**28
    fabs(x).ln() + LN2
  } else if ix > 0x4000_0000 {
    // 2**28 > |x| > 2.0
    let t = fabs(x);
    (2.0 * t + ONE / ((x * x + ONE).sqrt() + t)).ln()
  } else {
    // 2.0 > |x| > 2**-28
    let t = x * x;
    log1p(fabs(x) + t / (ONE + (ONE + t).sqrt()))
  };
  if hx > 0 { w } else { -w }
}

#[must_use]
pub fn atan(x: f64) -> f64 {
  x.atan()
}

#[must_use]
pub fn atan2(y: f64, x: f64) -> f64 {
  y.atan2(x)
}

#[must_use]
pub fn cos(x: f64) -> f64 {
  x.cos()
}

#[must_use]
pub fn sin(x: f64) -> f64 {
  x.sin()
}

#[must_use]
pub fn exp(x: f64) -> f64 {
  x.exp()
}

/* atanh(x) — 1:1 自 .cc
 * atanh(x) is NaN if |x| > 1 with signal; atanh(+-1) is +-INF with signal. */
#[must_use]
pub fn atanh(x: f64) -> f64 {
  const ONE: f64 = 1.0;
  const HUGE: f64 = 1e300;
  const ZERO: f64 = 0.0;

  let mut t: f64;
  let (hx, lx) = extract_words(x);
  let ix = hx & 0x7FFF_FFFF;
  // NegateWithWraparound<int32_t>(lx) — overflowing_math 语义的私有等价 (wrapping 取负)
  let neg_lx = (lx as i32).wrapping_neg() as u32;
  if (ix | (((lx | neg_lx) >> 31) as i32)) > 0x3FF0_0000 {
    // |x|>1
    return SIGNALED_NAN;
  }
  if ix == 0x3FF0_0000 {
    return if x > 0.0 {
      f64::INFINITY
    } else {
      f64::NEG_INFINITY
    };
  }
  if ix < 0x3E30_0000 && (HUGE + x) > ZERO {
    return x; // x<2**-28
  }
  let mut x = x;
  set_high_word(&mut x, ix);
  if ix < 0x3FE0_0000 {
    // x < 0.5
    t = x + x;
    t = 0.5 * log1p(t + t * x / (ONE - x));
  } else {
    t = 0.5 * log1p((x + x) / (ONE - x));
  }
  if hx >= 0 { t } else { -t }
}

#[must_use]
pub fn log(x: f64) -> f64 {
  x.ln()
}

#[must_use]
pub fn log1p(x: f64) -> f64 {
  x.ln_1p()
}

#[must_use]
pub fn log2(x: f64) -> f64 {
  x.log2()
}

#[must_use]
pub fn log10(x: f64) -> f64 {
  x.log10()
}

#[must_use]
pub fn expm1(x: f64) -> f64 {
  x.exp_m1()
}

#[must_use]
pub fn cbrt(x: f64) -> f64 {
  x.cbrt()
}

#[must_use]
pub fn tan(x: f64) -> f64 {
  x.tan()
}

/* cosh(x) — 1:1 自 .cc */
#[must_use]
pub fn cosh(x: f64) -> f64 {
  const KCOSH_OVERFLOW: f64 = 710.475_860_073_943_9;
  const ONE: f64 = 1.0;
  const HALF: f64 = 0.5;
  // 原版 volatile 防折叠; Rust 用 black_box 表达同义防止常量折叠
  let huge: f64 = 1.0e+300;

  let mut ix = get_high_word(x);
  ix &= 0x7FFF_FFFF;

  // |x| 在 [0, 0.5*log2]: 1+expm1(|x|)^2/(2*exp(|x|))
  if ix < 0x3FD6_2E43 {
    let t = expm1(fabs(x));
    let w = ONE + t;
    // |x| < 2^-55 时 cosh(x) = 1
    if ix < 0x3C80_0000 {
      return w;
    }
    return ONE + (t * t) / (w + w);
  }

  // |x| 在 [0.5*log2, 22]
  if ix < 0x4036_0000 {
    let t = exp(fabs(x));
    return HALF * t + HALF / t;
  }

  // |x| 在 [22, log(maxdouble)]
  if ix < 0x4086_2E42 {
    return HALF * exp(fabs(x));
  }

  // |x| 在 [log(maxdouble), overflowthreshold]
  if fabs(x) <= KCOSH_OVERFLOW {
    let w = exp(HALF * fabs(x));
    let t = HALF * w;
    return t * w;
  }

  // x 是 INF 或 NaN
  if ix >= 0x7FF0_0000 {
    return x * x;
  }

  // |x| 超溢出阈值 → huge*huge (上溢)
  black_box(huge) * black_box(huge)
}

pub mod legacy {
  /// legacy pow: 原版委托 llvm-libc shared::pow (ECMAScript 的 ±1/±Inf 特判在
  /// codegen 层 math::pow, 不在本层); 当前以平台 pow 承接, 见模块头推迟说明。
  #[must_use]
  pub fn pow(x: f64, y: f64) -> f64 {
    x.powf(y)
  }
}

/* sinh(x) — 1:1 自 .cc */
#[must_use]
pub fn sinh(x: f64) -> f64 {
  const KSINH_OVERFLOW: f64 = 710.475_860_073_943_9;
  const TWO_M28: f64 = 3.725_290_298_461_914e-9; // 2^-28
  const LOG_MAXD: f64 = 709.782_226_562_5; // 0x40862E42 00000000
  const SHUGE: f64 = 1.0e307;

  let h = if x < 0.0 { -0.5 } else { 0.5 };
  let ax = fabs(x);
  // |x| < 22 → sign(x)*0.5*(E+E/(E+1)), E=expm1(|x|)
  if ax < 22.0 {
    // |x| < 2^-28 → sinh(x) = x
    if ax < TWO_M28 {
      return x;
    }
    let t = expm1(ax);
    return if ax < 1.0 {
      h * (2.0 * t - t * t / (t + 1.0))
    } else {
      h * (t + t / (t + 1.0))
    };
  }
  // |x| 在 [22, log(maxdouble)]
  if ax < LOG_MAXD {
    return h * exp(ax);
  }
  // |x| 在 [log(maxdouble), overflowthreshold]
  if ax <= KSINH_OVERFLOW {
    let w = exp(0.5 * ax);
    let t = h * w;
    return t * w;
  }
  // |x| 超阈值或 NaN → 带符号无穷/NaN
  x * SHUGE
}

#[must_use]
pub fn tanh(x: f64) -> f64 {
  x.tanh()
}
