//! v8/src/base/hashing.h 的 1:1 转写
//! 红线 (todo 10.3): 引擎语义 hash 自研, 与 rapidhash-v8 vendored 实现逐位一致
//! hash64 = rapid_mix(key ^ S1, key ^ S2); rapid_mix = mul128 高64 ^ 低64
//! V8_BASE_HASH_VALUE_* 宏族机械展开为 HashValue trait 的具体实现

use std::{
  hash::{DefaultHasher, Hash, Hasher as StdHasher},
  mem::size_of,
};

/// RAPIDHASH_DEFAULT_SECRET[0..2] (third_party/rapidhash-v8/secret.h)
pub const RAPIDHASH_SECRET1: u64 = 0x2d35_8dcc_aa6c_78a5;
/// secret[1]
pub const RAPIDHASH_SECRET2: u64 = 0x8bb8_4b93_962e_acc9;
/// secret[2] (供 rapidhash 主函数, 本层保留常量)
pub const RAPIDHASH_SECRET3: u64 = 0x4b33_a62e_d433_d4a3;

/// V8_HASHES_COLLIDE (测试用碰撞 hash) — 默认构建未开启, 登记常量
pub const K_COLLIDING_HASH: usize = 1234;

/// rapid_mul128 + rapid_mix: (hi64, lo64) 互换前返回 hi ^ lo
#[inline]
pub fn rapid_mix(a: u64, b: u64) -> u64 {
  let product = (a as u128) * (b as u128);
  let hi = (product >> 64) as u64;
  let lo = product as u64;
  hi ^ lo
}

/// hash64: 64 位主机走 rapid_mix; 32 位主机走 Thomas Wang 回退
#[inline]
pub fn hash64(key: u64) -> usize {
  #[cfg(target_pointer_width = "64")]
  {
    rapid_mix(key ^ RAPIDHASH_SECRET1, key ^ RAPIDHASH_SECRET2) as usize
  }
  #[cfg(not(target_pointer_width = "64"))]
  {
    // Thomas Wang, "64 bit to 32 bit Hash Functions"
    let mut hash = key;
    hash = (!hash).wrapping_add(hash << 18); // hash = (hash << 18) - hash - 1
    hash ^= hash >> 31;
    hash = hash.wrapping_mul(21);
    hash ^= hash >> 11;
    hash = hash.wrapping_add(hash << 6);
    hash ^= hash >> 22;
    hash as usize
  }
}

/// hash32: 64 位主机 = hash64(key) 截断; 32 位主机 Thomas Wang 32bit mix
#[inline]
pub fn hash32(key: u32) -> usize {
  #[cfg(target_pointer_width = "64")]
  {
    (hash64(key as u64) as u32) as usize
  }
  #[cfg(not(target_pointer_width = "64"))]
  {
    // Thomas Wang, "32 bit Mix Functions"
    let mut hash = key;
    hash = (!hash).wrapping_add(hash << 15);
    hash ^= hash >> 12;
    hash = hash.wrapping_add(hash << 2);
    hash ^= hash >> 4;
    hash = hash.wrapping_mul(2057);
    hash ^= hash >> 16;
    hash as usize
  }
}

/// hash_combine (MurmurHash 风格; 32/64 位分支)
#[inline]
pub fn hash_combine(seed: usize, hash: usize) -> usize {
  #[cfg(target_pointer_width = "32")]
  {
    const C1: u32 = 0xCC9E_2D51;
    const C2: u32 = 0x1B87_3593;
    let mut hash = (hash as u32).wrapping_mul(C1);
    hash = hash.rotate_right(15);
    hash = hash.wrapping_mul(C2);

    let mut seed = (seed as u32) ^ hash;
    seed = seed.rotate_right(13);
    seed = seed.wrapping_mul(5).wrapping_add(0xE654_6B64);
    seed as usize
  }
  #[cfg(not(target_pointer_width = "32"))]
  {
    const M: u64 = 0xC6A4_A793_5BD1_E995;
    const R: u32 = 47;
    let mut hash = (hash as u64).wrapping_mul(M);
    hash ^= hash >> R;
    hash = hash.wrapping_mul(M);

    let mut seed = (seed as u64) ^ hash;
    seed = seed.wrapping_mul(M);
    seed as usize
  }
}

// ---------------------------------------------------------------------------
// HashValue: 原版自由函数 hash_value(T) 与 base::hash<T> 的 trait 化
// ---------------------------------------------------------------------------

/// 可哈希类型 (原版 concept Hashable)
pub trait HashValue {
  fn hash_value(&self) -> usize;
}

/// 按引用转发
impl<T: HashValue> HashValue for &T {
  #[inline]
  fn hash_value(&self) -> usize {
    (**self).hash_value()
  }
}

/// hash_value(std::span<T, N>): Hasher{}.AddRange 折叠 (切片承接 span 语义)
impl<T: HashValue> HashValue for &[T] {
  #[inline]
  fn hash_value(&self) -> usize {
    Hasher::new(0).add_range(self.iter()).hash()
  }
}

/// 可变切片同路径 (与原版 const/non-const span 两重载等价)
impl<T: HashValue> HashValue for &mut [T] {
  #[inline]
  fn hash_value(&self) -> usize {
    Hasher::new(0).add_range(self.iter()).hash()
  }
}

/// V8_BASE_HASH_VALUE_TRIVIAL(bool/unsigned char/unsigned short): static_cast<size_t>
impl HashValue for bool {
  #[inline]
  fn hash_value(&self) -> usize {
    *self as usize
  }
}
impl HashValue for u8 {
  #[inline]
  fn hash_value(&self) -> usize {
    *self as usize
  }
}
impl HashValue for u16 {
  #[inline]
  fn hash_value(&self) -> usize {
    *self as usize
  }
}

/// hash_value(unsigned int): hash32
impl HashValue for u32 {
  #[inline]
  fn hash_value(&self) -> usize {
    hash32(*self)
  }
}

/// hash_value(unsigned long long): hash64
impl HashValue for u64 {
  #[inline]
  fn hash_value(&self) -> usize {
    hash64(*self)
  }
}

/// hash_value(unsigned long): 按宽度选 hash32/hash64
impl HashValue for usize {
  #[inline]
  fn hash_value(&self) -> usize {
    if size_of::<usize>() == 4 {
      hash32(*self as u32)
    } else {
      hash64(*self as u64)
    }
  }
}

/// V8_BASE_HASH_VALUE_SIGNED: bit_cast 到无符号后同路径
macro_rules! impl_hash_value_signed {
  ($($t:ty => $u:ty),*) => {
    $(
      impl HashValue for $t {
        #[inline]
        fn hash_value(&self) -> usize {
          (*self as $u).hash_value()
        }
      }
    )*
  };
}

impl_hash_value_signed!(i8 => u8, i16 => u16, i32 => u32, i64 => u64, isize => usize);

/// hash_value(float/double): 0 与 -0 同为 0 (原版注释语义)
impl HashValue for f32 {
  #[inline]
  fn hash_value(&self) -> usize {
    if *self != 0.0 {
      self.to_bits().hash_value()
    } else {
      0
    }
  }
}
impl HashValue for f64 {
  #[inline]
  fn hash_value(&self) -> usize {
    if *self != 0.0 {
      self.to_bits().hash_value()
    } else {
      0
    }
  }
}

/// hash_value(T* const&): reinterpret_cast<uintptr_t>
impl<T> HashValue for *const T {
  #[inline]
  fn hash_value(&self) -> usize {
    (*self as usize).hash_value()
  }
}
impl<T> HashValue for *mut T {
  #[inline]
  fn hash_value(&self) -> usize {
    (*self as usize).hash_value()
  }
}

/// hash_value(std::pair): Hasher::Combine(first, second)
impl<A: HashValue, B: HashValue> HashValue for (A, B) {
  #[inline]
  fn hash_value(&self) -> usize {
    Hasher::new(0).add(&self.0).add(&self.1).hash()
  }
}

/// hash_value(std::tuple(2/3 常用宽度)): Combine 全部元素
impl<A: HashValue, B: HashValue, C: HashValue> HashValue for (A, B, C) {
  #[inline]
  fn hash_value(&self) -> usize {
    Hasher::new(0).add(&self.0).add(&self.1).add(&self.2).hash()
  }
}

/// 数组/span: hash_value 经 Hasher{}.AddRange 折叠
impl<T: HashValue, const N: usize> HashValue for [T; N] {
  #[inline]
  fn hash_value(&self) -> usize {
    Hasher::new(0).add_range(self.iter()).hash()
  }
}

/// base::hash<T>{}(v) 入口
#[inline]
pub fn hash<T: HashValue + ?Sized>(v: &T) -> usize {
  v.hash_value()
}

// ---------------------------------------------------------------------------
// Hasher (原版 class Hasher)
// ---------------------------------------------------------------------------

/// 字段折叠哈希器
#[derive(Debug, Clone, Copy, Default)]
pub struct Hasher {
  hash_: usize,
}

impl Hasher {
  /// constexpr Hasher() — hash_ = 0
  #[inline]
  pub const fn new(seed: usize) -> Self {
    Self { hash_: seed }
  }

  /// hash()
  #[inline]
  pub const fn hash(&self) -> usize {
    self.hash_
  }

  /// AddHash(other_hash)
  #[inline]
  pub fn add_hash(&mut self, other_hash: usize) -> &mut Self {
    self.hash_ = hash_combine(self.hash_, other_hash);
    self
  }

  /// Add(t)
  #[inline]
  pub fn add<T: HashValue + ?Sized>(&mut self, t: &T) -> &mut Self {
    self.add_hash(hash(t))
  }

  /// AddRange(first, last) — Rust 以 Iterator 承接
  #[inline]
  pub fn add_range<'a, I, T>(&mut self, iter: I) -> &mut Self
  where
    I: Iterator<Item = T>,
    T: HashValue + 'a,
  {
    for v in iter {
      self.add_hash(v.hash_value());
    }
    self
  }

  /// Hasher::Combine(ts...) — 以 slice 承接变参场景
  #[inline]
  pub fn combine<T: HashValue>(values: &[T]) -> usize {
    let mut hasher = Self::new(0);
    for v in values {
      hasher.add(v);
    }
    hasher.hash()
  }

  /// Combine 双值 (pair/tuple 常用路径)
  #[inline]
  pub fn combine2<A: HashValue, B: HashValue>(a: &A, b: &B) -> usize {
    Self::new(0).add(a).add(b).hash()
  }
}

/// 兼容 std::hash 的桥接 (供 bit_hash 特化等)
impl From<Hasher> for usize {
  #[inline]
  fn from(h: Hasher) -> usize {
    h.hash()
  }
}

// ---------------------------------------------------------------------------
// hash_combine / hash_range 旧接口 (TODO 注释保留: 建议迁移 Hasher::Combine)
// ---------------------------------------------------------------------------

/// hash_range(Iterator first, Iterator last)
#[inline]
pub fn hash_range<I, T>(mut iter: I) -> usize
where
  I: Iterator<Item = T>,
  T: HashValue,
{
  let mut hasher = Hasher::new(0);
  for v in iter.by_ref() {
    hasher.add_hash(v.hash_value());
  }
  hasher.hash()
}

// ---------------------------------------------------------------------------
// bit_equal_to / bit_hash (位比较/位哈希)
// ---------------------------------------------------------------------------

/// bit_equal_to<T>: 位表示相等比较 (整数与 std::equal_to 相同; 浮点按 bits)
pub trait BitEqualTo {
  fn bit_equal_to(&self, other: &Self) -> bool;
}

/// bit_hash<T>: 位表示哈希
pub trait BitHash {
  fn bit_hash(&self) -> usize;
}

macro_rules! impl_bit_trivial {
  ($($t:ty),*) => {
    $(
      impl BitEqualTo for $t {
        #[inline]
        fn bit_equal_to(&self, other: &Self) -> bool { self == other }
      }
      impl BitHash for $t {
        #[inline]
        fn bit_hash(&self) -> usize { hash(self) }
      }
    )*
  };
}

impl_bit_trivial!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

/// V8_BASE_BIT_SPECIALIZE_BIT_CAST(float, uint32_t) / (double, uint64_t)
impl BitEqualTo for f32 {
  #[inline]
  fn bit_equal_to(&self, other: &Self) -> bool {
    self.to_bits() == other.to_bits()
  }
}
impl BitHash for f32 {
  #[inline]
  fn bit_hash(&self) -> usize {
    hash(&self.to_bits())
  }
}
impl BitEqualTo for f64 {
  #[inline]
  fn bit_equal_to(&self, other: &Self) -> bool {
    self.to_bits() == other.to_bits()
  }
}
impl BitHash for f64 {
  #[inline]
  fn bit_hash(&self) -> usize {
    hash(&self.to_bits())
  }
}

// ---------------------------------------------------------------------------
// std::hash 桥接 (供小容器/断言用; 引擎语义仍走 hash_value)
// ---------------------------------------------------------------------------

/// 将 std Hash 接入 (单向辅助; 引擎语义仍走 hash_value)
#[inline]
pub fn std_hash<T: Hash>(v: &T) -> usize {
  let mut hasher = DefaultHasher::new();
  <T as Hash>::hash(v, &mut hasher);
  hasher.finish() as usize
}
