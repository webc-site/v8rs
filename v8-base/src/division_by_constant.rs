//! 原版 v8/src/base/division-by-constant.h/.cc — 常数除法魔数
//! (Warren《Hacker's Delight》第 10 章; uint32/uint64 显式实例化)。
//!
//! 无符号算术全部走 wrapping 语义, 与 C++ 无符号溢出回绕逐位一致;
//! 除零路径 (原版 DCHECK 排除的非法输入) 以 0 兜底以避免 panic。

/// 常数除法魔数: multiplier / shift / add (原版 MagicNumbersForDivision<T>)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MagicNumbersForDivision<T> {
  pub multiplier: T,
  pub shift: u32,
  pub add: bool,
}

impl<T> MagicNumbersForDivision<T> {
  #[must_use]
  pub const fn new(multiplier: T, shift: u32, add: bool) -> Self {
    Self {
      multiplier,
      shift,
      add,
    }
  }
}

/// 无符号常数除法所需整型能力 (u32/u64): 全部 wrapping 位级运算
pub trait UnsignedDiv: Copy + Ord {
  const BITS: u32;
  fn lit(v: u32) -> Self;
  fn all_ones() -> Self;
  fn wadd(self, r: Self) -> Self;
  fn wsub(self, r: Self) -> Self;
  fn wmul(self, r: Self) -> Self;
  fn band(self, r: Self) -> Self;
  fn wshl(self, n: u32) -> Self;
  fn wshr(self, n: u32) -> Self;
  /// 无符号整除 (除零返 0)
  fn wdiv(self, r: Self) -> Self;
  /// 无符号取模 (除零返 0)
  fn wrem(self, r: Self) -> Self;
}

macro_rules! impl_unsigned_div {
  ($t:ty) => {
    impl UnsignedDiv for $t {
      const BITS: u32 = <$t>::BITS;
      fn lit(v: u32) -> Self {
        v as $t
      }
      fn all_ones() -> Self {
        <$t>::MAX
      }
      fn wadd(self, r: Self) -> Self {
        self.wrapping_add(r)
      }
      fn wsub(self, r: Self) -> Self {
        self.wrapping_sub(r)
      }
      fn wmul(self, r: Self) -> Self {
        self.wrapping_mul(r)
      }
      fn band(self, r: Self) -> Self {
        self & r
      }
      fn wshl(self, n: u32) -> Self {
        self.wrapping_shl(n)
      }
      fn wshr(self, n: u32) -> Self {
        self.wrapping_shr(n)
      }
      fn wdiv(self, r: Self) -> Self {
        if r == Self::lit(0) {
          Self::lit(0)
        } else {
          self / r
        }
      }
      fn wrem(self, r: Self) -> Self {
        if r == Self::lit(0) {
          Self::lit(0)
        } else {
          self % r
        }
      }
    }
  };
}
impl_unsigned_div!(u32);
impl_unsigned_div!(u64);

/// 有符号常数除法对应的无符号类型桥 (原版 make_unsigned_t + static_cast 位保持)
pub trait SignedDiv: Copy {
  type Unsigned: UnsignedDiv;
  fn to_unsigned(self) -> Self::Unsigned;
  fn from_unsigned(v: Self::Unsigned) -> Self;
}

impl SignedDiv for i32 {
  type Unsigned = u32;
  fn to_unsigned(self) -> u32 {
    self as u32
  }
  fn from_unsigned(v: u32) -> i32 {
    v as i32
  }
}

impl SignedDiv for i64 {
  type Unsigned = u64;
  fn to_unsigned(self) -> u64 {
    self as u64
  }
  fn from_unsigned(v: u64) -> i64 {
    v as i64
  }
}

/// SignedDivisionByConstant (无符号入参版, 原版 .cc 显式实例化的核心算法)
/// d 不得解释为 -1/0/1 (原版 DCHECK); 非法输入不再计算魔数有效性。
#[must_use]
pub fn signed_division_by_constant<T: UnsignedDiv>(d: T) -> MagicNumbersForDivision<T> {
  let bits = T::BITS;
  let zero = T::lit(0);
  let one = T::lit(1);
  let two = T::lit(2);
  let min = one.wshl(bits - 1);
  let neg = min.band(d) != zero;
  let ad = if neg { zero.wsub(d) } else { d };
  let t = min.wadd(d.wshr(bits - 1));
  // Absolute value of nc
  let anc = t.wsub(one).wsub(t.wrem(ad));
  // Init. p = bits-1
  let mut p = bits - 1;
  // Init. q1 = 2**p/|nc|, r1 = rem(2**p, |nc|)
  let mut q1 = min.wdiv(anc);
  let mut r1 = min.wsub(q1.wmul(anc));
  // Init. q2 = 2**p/|d|, r2 = rem(2**p, |d|)
  let mut q2 = min.wdiv(ad);
  let mut r2 = min.wsub(q2.wmul(ad));
  loop {
    p = p.wrapping_add(1);
    q1 = two.wmul(q1);
    r1 = two.wmul(r1);
    if r1 >= anc {
      q1 = q1.wadd(one);
      r1 = r1.wsub(anc);
    }
    q2 = two.wmul(q2);
    r2 = two.wmul(r2);
    if r2 >= ad {
      q2 = q2.wadd(one);
      r2 = r2.wsub(ad);
    }
    let delta = ad.wsub(r2);
    if !(q1 < delta || (q1 == delta && r1 == zero)) {
      break;
    }
  }
  let mul = q2.wadd(one);
  let multiplier = if neg { zero.wsub(mul) } else { mul };
  MagicNumbersForDivision::new(multiplier, p.wrapping_sub(bits), false)
}

/// SignedDivisionByConstant (有符号入参版): 委托无符号实现并位保持转回
#[must_use]
pub fn signed_division_by_constant_signed<S: SignedDiv>(d: S) -> MagicNumbersForDivision<S> {
  let magic = signed_division_by_constant(d.to_unsigned());
  MagicNumbersForDivision::new(S::from_unsigned(magic.multiplier), magic.shift, magic.add)
}

/// UnsignedDivisionByConstant (原版 .cc). d 不得为 0;
/// leading_zeros 为被除数高位已知为 0 的位数, 用于加速求解。
#[must_use]
pub fn unsigned_division_by_constant<T: UnsignedDiv>(
  d: T,
  leading_zeros: u32,
) -> MagicNumbersForDivision<T> {
  let bits = T::BITS;
  let zero = T::lit(0);
  let one = T::lit(1);
  let two = T::lit(2);
  let ones = T::all_ones().wshr(leading_zeros);
  let min = one.wshl(bits - 1);
  let max = T::all_ones().wshr(1);
  let nc = ones.wsub(ones.wsub(d).wrem(d));
  let mut a = false;
  let mut p = bits - 1;
  let mut q1 = min.wdiv(nc);
  let mut r1 = min.wsub(q1.wmul(nc));
  let mut q2 = max.wdiv(d);
  let mut r2 = max.wsub(q2.wmul(d));
  loop {
    p = p.wrapping_add(1);
    if r1 >= nc.wsub(r1) {
      q1 = two.wmul(q1).wadd(one);
      r1 = two.wmul(r1).wsub(nc);
    } else {
      q1 = two.wmul(q1);
      r1 = two.wmul(r1);
    }
    if r2.wadd(one) >= d.wsub(r2) {
      if q2 >= max {
        a = true;
      }
      q2 = two.wmul(q2).wadd(one);
      r2 = two.wmul(r2).wadd(one).wsub(d);
    } else {
      if q2 >= min {
        a = true;
      }
      q2 = two.wmul(q2);
      r2 = two.wmul(r2).wadd(one);
    }
    let delta = d.wsub(one).wsub(r2);
    if !(p < bits * 2 && (q1 < delta || (q1 == delta && r1 == zero))) {
      break;
    }
  }
  MagicNumbersForDivision::new(q2.wadd(one), p.wrapping_sub(bits), a)
}

/// 应用魔数完成无符号除以常数 (原版 test 的参考用法; add 分支含回绕语义)
#[must_use]
pub fn unsigned_divide<T: UnsignedDiv>(number: T, magic: &MagicNumbersForDivision<T>) -> T {
  let mut q = magic.multiplier.wmul(number);
  if magic.add {
    let t = number.wsub(q).wshr(1).wadd(q);
    q = t.wshr(magic.shift);
  } else {
    q = q.wshr(magic.shift);
  }
  q
}
