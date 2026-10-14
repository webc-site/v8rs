//! 原版 v8/src/base/numbers/double.h — Double 位级访问器

use crate::numbers::diy_fp::DiyFp;

/// double ↔ uint64 位级转换 (与原版同假设: 同端序)
#[must_use]
pub const fn double_to_uint64(d: f64) -> u64 {
  d.to_bits()
}

#[must_use]
pub const fn uint64_to_double(d64: u64) -> f64 {
  f64::from_bits(d64)
}

/// Double — 辅助函数集合, 与原版方法 1:1
#[derive(Debug, Clone, Copy, Default)]
pub struct Double {
  d64_: u64,
}

// Default (d64_ = 0) 由 struct 上的 derive 提供

impl Double {
  pub const SIGN_MASK: u64 = 0x8000_0000_0000_0000;
  pub const EXPONENT_MASK: u64 = 0x7FF0_0000_0000_0000;
  pub const SIGNIFICAND_MASK: u64 = 0x000F_FFFF_FFFF_FFFF;
  pub const HIDDEN_BIT: u64 = 0x0010_0000_0000_0000;
  /// kPhysicalSignificandSize (不含隐藏位)
  pub const PHYSICAL_SIGNIFICAND_SIZE: i32 = 52;
  pub const SIGNIFICAND_SIZE: i32 = 53;

  const EXPONENT_BIAS: i32 = 0x3FF + Self::PHYSICAL_SIGNIFICAND_SIZE;
  const DENORMAL_EXPONENT: i32 = -Self::EXPONENT_BIAS + 1;
  const MAX_EXPONENT: i32 = 0x7FF - Self::EXPONENT_BIAS;
  const INFINITY_BITS: u64 = 0x7FF0_0000_0000_0000;

  #[must_use]
  pub const fn from_double(d: f64) -> Self {
    Self {
      d64_: double_to_uint64(d),
    }
  }

  #[must_use]
  pub const fn from_u64(d64: u64) -> Self {
    Self { d64_: d64 }
  }

  #[must_use]
  pub fn from_diy_fp(diy_fp: DiyFp) -> Self {
    Self {
      d64_: Self::diy_fp_to_uint64(diy_fp),
    }
  }

  /// 值必须 >= +0.0 且非特殊值
  #[must_use]
  pub fn as_diy_fp(&self) -> DiyFp {
    debug_assert!(self.sign() > 0);
    debug_assert!(!self.is_special());
    DiyFp::new(self.significand(), self.exponent())
  }

  /// 值必须严格 > 0
  #[must_use]
  pub fn as_normalized_diy_fp(&self) -> DiyFp {
    debug_assert!(self.value() > 0.0);
    let mut f = self.significand();
    let mut e = self.exponent();

    // 当前 double 可能是 denormal
    while f & Self::HIDDEN_BIT == 0 {
      f <<= 1;
      e -= 1;
    }
    // 最终位移一次完成
    f <<= DiyFp::SIGNIFICAND_SIZE as u32 - Self::SIGNIFICAND_SIZE as u32;
    e -= DiyFp::SIGNIFICAND_SIZE - Self::SIGNIFICAND_SIZE;
    DiyFp::new(f, e)
  }

  #[must_use]
  pub const fn as_uint64(&self) -> u64 {
    self.d64_
  }

  /// 下一个更大的 double; +inf 输入返回 +inf
  #[must_use]
  pub const fn next_double(&self) -> f64 {
    if self.d64_ == Self::INFINITY_BITS {
      return Self {
        d64_: Self::INFINITY_BITS,
      }
      .value();
    }
    if self.sign() < 0 && self.significand_const() == 0 {
      // -0.0
      return 0.0;
    }
    if self.sign() < 0 {
      Self {
        d64_: self.d64_ - 1,
      }
      .value()
    } else {
      Self {
        d64_: self.d64_ + 1,
      }
      .value()
    }
  }

  #[must_use]
  pub const fn exponent(&self) -> i32 {
    if self.is_denormal() {
      return Self::DENORMAL_EXPONENT;
    }
    let d64 = self.d64_;
    let biased_e = ((d64 & Self::EXPONENT_MASK) >> Self::PHYSICAL_SIGNIFICAND_SIZE as u32) as i32;
    biased_e - Self::EXPONENT_BIAS
  }

  #[must_use]
  pub const fn significand(&self) -> u64 {
    self.significand_const()
  }

  const fn significand_const(&self) -> u64 {
    let d64 = self.d64_;
    let significand = d64 & Self::SIGNIFICAND_MASK;
    if !self.is_denormal() {
      significand + Self::HIDDEN_BIT
    } else {
      significand
    }
  }

  #[must_use]
  pub const fn is_denormal(&self) -> bool {
    self.d64_ & Self::EXPONENT_MASK == 0
  }

  /// denormal 不算特殊值; 只有 inf/NaN 是
  #[must_use]
  pub const fn is_special(&self) -> bool {
    self.d64_ & Self::EXPONENT_MASK == Self::EXPONENT_MASK
  }

  #[must_use]
  pub const fn is_infinite(&self) -> bool {
    self.d64_ & Self::EXPONENT_MASK == Self::EXPONENT_MASK
      && self.d64_ & Self::SIGNIFICAND_MASK == 0
  }

  #[must_use]
  pub const fn sign(&self) -> i32 {
    if self.d64_ & Self::SIGN_MASK == 0 {
      1
    } else {
      -1
    }
  }

  /// 值必须 >= +0.0
  #[must_use]
  pub fn upper_boundary(&self) -> DiyFp {
    debug_assert!(self.sign() > 0);
    DiyFp::new(self.significand() * 2 + 1, self.exponent() - 1)
  }

  /// 返回 (m_minus, m_plus); m_plus 规格化, m_minus 与其同指数。值必须 > 0
  #[must_use]
  pub fn normalized_boundaries(&self) -> (DiyFp, DiyFp) {
    debug_assert!(self.value() > 0.0);
    let v = self.as_diy_fp();
    let m_plus = DiyFp::normalize_new(DiyFp::new((v.f() << 1) + 1, v.e() - 1));
    let mut m_minus =
      if self.as_uint64() & Self::SIGNIFICAND_MASK == 0 && v.e() != Self::DENORMAL_EXPONENT {
        // 边界更近: v = 1000e10 时 v- = 9999e9, 中点距离是 1e8 而非 1e9
        // 最小规格化数除外: 最大 denormal 与其后继等距
        DiyFp::new((v.f() << 2) - 1, v.e() - 2)
      } else {
        DiyFp::new((v.f() << 1) - 1, v.e() - 1)
      };
    m_minus.set_f(m_minus.f() << (m_minus.e() - m_plus.e()));
    m_minus.set_e(m_plus.e());
    (m_minus, m_plus)
  }

  #[must_use]
  pub const fn value(&self) -> f64 {
    uint64_to_double(self.d64_)
  }

  /// 给定数量级返回有效尾数位数 (denormal 更少)
  #[must_use]
  pub const fn significand_size_for_order_of_magnitude(order: i32) -> i32 {
    if order >= Self::DENORMAL_EXPONENT + Self::SIGNIFICAND_SIZE {
      return Self::SIGNIFICAND_SIZE;
    }
    if order <= Self::DENORMAL_EXPONENT {
      return 0;
    }
    order - Self::DENORMAL_EXPONENT
  }

  const fn diy_fp_to_uint64(diy_fp: DiyFp) -> u64 {
    let mut significand = diy_fp.f();
    let mut exponent = diy_fp.e();
    while significand > Self::HIDDEN_BIT + Self::SIGNIFICAND_MASK {
      significand >>= 1;
      exponent += 1;
    }
    if exponent >= Self::MAX_EXPONENT {
      return Self::INFINITY_BITS;
    }
    if exponent < Self::DENORMAL_EXPONENT {
      return 0;
    }
    while exponent > Self::DENORMAL_EXPONENT && significand & Self::HIDDEN_BIT == 0 {
      significand <<= 1;
      exponent -= 1;
    }
    let biased_exponent =
      if exponent == Self::DENORMAL_EXPONENT && significand & Self::HIDDEN_BIT == 0 {
        0
      } else {
        (exponent + Self::EXPONENT_BIAS) as u64
      };
    (significand & Self::SIGNIFICAND_MASK)
      | (biased_exponent << Self::PHYSICAL_SIGNIFICAND_SIZE as u32)
  }
}
