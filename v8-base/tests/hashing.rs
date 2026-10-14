//! v8-base/src/hashing.rs 集成测试 — 1:1 对应 hashing-unittest.cc
//!
//! 原版用例: HashingTest.HashBool / HashFloatZero / HashDoubleZero (非类型化 3 例),
//! TYPED_TEST(HashingTest, ...) 8 组用例 (EqualToImpliesSameHashCode /
//! HashEqualsHashValue / HashIsStateless / HashIsOkish / HashValueArrayUsesHashRange /
//! HashSpanUsesHashRange / BitEqualTo / BitEqualToImpliesSameBitHash),
//! 以及 HashUsesArgumentDependentLookup / BitEqualToFloat /
//! BitHashFloatDifferentForZeroAndMinusZero / BitEqualToDouble /
//! BitHashDoubleDifferentForZeroAndMinusZero。
//! 原版 HashingTypes 列出 20 个 C++ 类型; 本平台 (LP64) 与之等价的 Rust 宽度类型为
//! 下面 12 个 (signed char→i8, short→i16, int→i32, long/long long→i64 等重复项合并),
//! 每个原 TYPED_TEST 名对应本文件 1 个 #[test], 内部对 12 个类型逐一实例化。
//! 随机面: 原版用 RandomNumberGenerator; 本 crate dev-deps 无 RNG 且禁 cargo add,
//! 故以固定种子 xorshift64* 承接 (结果可复现)。
//! 末尾 4 例为本轮补充 (10.3 红线: 与参考算法位级一致, 防回归)。

use std::{
  any::type_name, array::from_fn, collections::HashSet, iter::empty, mem::size_of,
  slice::from_raw_parts,
};

use aok::{OK, Void};
use v8_base::hashing::{
  BitEqualTo, BitHash, HashValue, Hasher, K_COLLIDING_HASH, RAPIDHASH_SECRET1, RAPIDHASH_SECRET2,
  RAPIDHASH_SECRET3, hash, hash_combine, hash_range, hash32, hash64, rapid_mix, std_hash,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

// ---------------------------------------------------------------------------
// 固定种子随机字节面 (原版 RandomNumberGenerator::NextBytes 等价替身)
// ---------------------------------------------------------------------------

struct Rng {
  state: u64,
}

impl Rng {
  fn new(seed: u64) -> Self {
    Self {
      state: if seed == 0 {
        0x9E37_79B9_7F4A_7C15
      } else {
        seed
      },
    }
  }

  fn next_u64(&mut self) -> u64 {
    self.state ^= self.state >> 12;
    self.state ^= self.state << 25;
    self.state ^= self.state >> 27;
    self.state.wrapping_mul(0x2545_F491_4F6C_DD1D)
  }

  #[inline]
  fn next_value<T: RandBits>(&mut self) -> T {
    T::random(self)
  }

  #[inline]
  fn next_array<T: RandBits, const N: usize>(&mut self) -> [T; N] {
    from_fn(|_| self.next_value::<T>())
  }
}

/// 按位填充的随机取值
trait RandBits: Copy {
  fn random(rng: &mut Rng) -> Self;
}

macro_rules! impl_rand_bits_int {
  ($($t:ty => $u:ty),*) => {
    $(
      impl RandBits for $t {
        #[inline]
        fn random(rng: &mut Rng) -> Self {
          rng.next_u64() as $u as $t
        }
      }
    )*
  };
}

impl_rand_bits_int!(
  i8 => u8, u8 => u8, i16 => u16, u16 => u16, i32 => u32, u32 => u32,
  i64 => u64, u64 => u64, isize => u64, usize => u64
);

impl RandBits for f32 {
  #[inline]
  fn random(rng: &mut Rng) -> Self {
    f32::from_bits(rng.next_u64() as u32)
  }
}

impl RandBits for f64 {
  #[inline]
  fn random(rng: &mut Rng) -> Self {
    f64::from_bits(rng.next_u64())
  }
}

/// 原版 memcmp(&v1, &v2, sizeof(T)) == 0 的等价面 (标量类型无填充字节)
fn bytes_of<T>(v: &T) -> Vec<u8> {
  // SAFETY: 读取 v 自身对象表示的前 size_of::<T>() 字节, 借用期内 v 存活
  unsafe { from_raw_parts(v as *const T as *const u8, size_of::<T>()).to_vec() }
}

/// 12 个类型逐一实例化同一个泛型检查函数 (承接 TYPED_TEST 面)
macro_rules! for_hashing_types {
  ($f:ident) => {{
    // ident 元变量后可接 turbofish (path 片段则不允许)
    $f::<i8>(1001);
    $f::<u8>(1002);
    $f::<i16>(1003);
    $f::<u16>(1004);
    $f::<i32>(1005);
    $f::<u32>(1006);
    $f::<isize>(1007);
    $f::<usize>(1008);
    $f::<i64>(1009);
    $f::<u64>(1010);
    $f::<f32>(1011);
    $f::<f64>(1012);
  }};
}

// ---------------------------------------------------------------------------
// 泛型检查体 (每个对应原版一个 TYPED_TEST)
// ---------------------------------------------------------------------------

fn check_equal_to_implies_same_hash_code<T: HashValue + PartialEq + Copy + RandBits>(seed: u64) {
  let mut rng = Rng::new(seed);
  let values = rng.next_array::<T, 32>();
  for &v1 in values.iter() {
    for &v2 in values.iter() {
      if v1 == v2 {
        assert_eq!(hash(&v1), hash(&v2), "type={}", type_name::<T>());
      }
    }
  }
}

fn check_hash_equals_hash_value<T: HashValue + Copy + RandBits>(seed: u64) {
  let mut rng = Rng::new(seed);
  for _ in 0..128 {
    let v = rng.next_value::<T>();
    assert_eq!(hash(&v), v.hash_value(), "type={}", type_name::<T>());
  }
}

fn check_hash_is_stateless<T: HashValue + Copy + RandBits>(seed: u64) {
  let mut rng = Rng::new(seed);
  let h1 = |v: &T| hash(v);
  let h2 = |v: &T| hash(v);
  for _ in 0..128 {
    let v = rng.next_value::<T>();
    assert_eq!(h1(&v), h2(&v), "type={}", type_name::<T>());
  }
}

fn check_hash_is_okish<T: HashValue + PartialEq + Copy + RandBits>(seed: u64) {
  let mut rng = Rng::new(seed);
  let mut vs: Vec<T> = Vec::new();
  for _ in 0..128 {
    let v = rng.next_value::<T>();
    if !vs.contains(&v) {
      vs.push(v);
    }
  }
  let hs: HashSet<usize> = vs.iter().map(|v| hash(v)).collect();
  assert!(
    vs.len() / 4 <= hs.len(),
    "type={} vs={} hs={}",
    type_name::<T>(),
    vs.len(),
    hs.len()
  );
}

fn check_hash_value_array_uses_hash_range<T: HashValue + Copy + RandBits>(seed: u64) {
  let mut rng = Rng::new(seed);
  let values = rng.next_array::<T, 128>();
  // hash_range(first, last) == hash_value(array)
  assert_eq!(
    hash_range(values.iter()),
    hash(&values),
    "type={}",
    type_name::<T>()
  );
  assert_eq!(hash_range(values.iter()), hash(&values.as_slice()));
}

fn check_hash_span_uses_hash_range<T: HashValue + Copy + RandBits>(seed: u64) {
  let ty = type_name::<T>();
  let mut rng = Rng::new(seed);
  let values = rng.next_array::<T, 128>();
  let expected = hash_range(values.iter());

  let dynamic_span: &[T] = &values;
  let const_dynamic_span: &[T] = &values;
  let static_span: &[T; 128] = &values;
  let const_static_span: &[T; 128] = &values;

  assert_eq!(expected, hash(&dynamic_span), "type={ty}");
  assert_eq!(expected, hash(&const_dynamic_span), "type={ty}");
  assert_eq!(expected, hash(static_span), "type={ty}");
  assert_eq!(expected, hash(const_static_span), "type={ty}");

  assert_eq!(expected, dynamic_span.hash_value(), "type={ty}");
  assert_eq!(expected, const_dynamic_span.hash_value(), "type={ty}");
  assert_eq!(expected, static_span.hash_value(), "type={ty}");
  assert_eq!(expected, const_static_span.hash_value(), "type={ty}");

  // Hasher{}.AddHash(expected) == Hasher::Combine(span)
  let expected_combined = Hasher::new(0).add_hash(expected).hash();
  assert_eq!(
    expected_combined,
    Hasher::new(0).add(&dynamic_span).hash(),
    "type={ty}"
  );
  assert_eq!(
    expected_combined,
    Hasher::new(0).add(&const_dynamic_span).hash(),
    "type={ty}"
  );
  assert_eq!(
    expected_combined,
    Hasher::new(0).add(static_span).hash(),
    "type={ty}"
  );
  assert_eq!(
    expected_combined,
    Hasher::new(0).add(const_static_span).hash(),
    "type={ty}"
  );

  // 空区间
  let expected_empty = hash_range(empty::<&T>());
  assert_eq!(0, expected_empty, "type={ty}");
  let empty_dynamic_span: &[T] = &[];
  let empty_static_span: &[T; 0] = &[];
  assert_eq!(expected_empty, hash(&empty_dynamic_span), "type={ty}");
  assert_eq!(expected_empty, hash(&empty_static_span), "type={ty}");
  assert_eq!(expected_empty, empty_dynamic_span.hash_value(), "type={ty}");
  assert_eq!(expected_empty, empty_static_span.hash_value(), "type={ty}");
  let expected_empty_combined = Hasher::new(0).add_hash(expected_empty).hash();
  assert_eq!(
    expected_empty_combined,
    Hasher::new(0).add(&empty_dynamic_span).hash(),
    "type={ty}"
  );
  assert_eq!(
    expected_empty_combined,
    Hasher::new(0).add(&empty_static_span).hash(),
    "type={ty}"
  );

  // 逐元素折叠面 (原版 Combine(ts...) 的多参数路径)
  assert_eq!(expected, Hasher::combine(dynamic_span), "type={ty}");
}

fn check_bit_equal_to<T: BitEqualTo + Copy + RandBits>(seed: u64) {
  let ty = type_name::<T>();
  let mut rng = Rng::new(seed);
  for _ in 0..128 {
    let v1 = rng.next_value::<T>();
    let v2 = rng.next_value::<T>();
    assert!(v1.bit_equal_to(&v1), "type={ty}");
    assert!(v2.bit_equal_to(&v2), "type={ty}");
    assert_eq!(
      bytes_of(&v1) == bytes_of(&v2),
      v1.bit_equal_to(&v2),
      "type={ty}"
    );
  }
}

fn check_bit_equal_to_implies_same_bit_hash<T: BitEqualTo + BitHash + Copy + RandBits>(seed: u64) {
  let ty = type_name::<T>();
  let mut rng = Rng::new(seed);
  let values = rng.next_array::<T, 32>();
  for &v1 in values.iter() {
    for &v2 in values.iter() {
      if v1.bit_equal_to(&v2) {
        assert_eq!(v1.bit_hash(), v2.bit_hash(), "type={ty}");
      }
    }
  }
}

// ---------------------------------------------------------------------------
// 原版非类型化用例
// ---------------------------------------------------------------------------

/// 原版 TEST(HashingTest, HashBool)
#[test]
fn hashing_test_hash_bool() -> Void {
  let h1 = |v: &bool| hash(v);
  let h2 = |v: &bool| hash(v);
  assert_eq!(h1(&true), h2(&true));
  assert_eq!(h1(&false), h2(&false));
  assert_ne!(hash(&true), hash(&false));
  // V8_BASE_HASH_VALUE_TRIVIAL(bool): static_cast<size_t>
  assert_eq!(1, hash(&true));
  assert_eq!(0, hash(&false));
  OK
}

/// 原版 TEST(HashingTest, HashFloatZero)
#[test]
fn hashing_test_hash_float_zero() -> Void {
  assert_eq!(hash(&0.0f32), hash(&-0.0f32));
  assert_eq!(0, hash(&0.0f32));
  assert_eq!(0, hash(&-0.0f32));
  OK
}

/// 原版 TEST(HashingTest, HashDoubleZero)
#[test]
fn hashing_test_hash_double_zero() -> Void {
  assert_eq!(hash(&0.0f64), hash(&-0.0f64));
  assert_eq!(0, hash(&0.0f64));
  assert_eq!(0, hash(&-0.0f64));
  OK
}

/// 原版 TYPED_TEST(HashingTest, EqualToImpliesSameHashCode)
#[test]
fn hashing_test_equal_to_implies_same_hash_code() -> Void {
  for_hashing_types!(check_equal_to_implies_same_hash_code);
  OK
}

/// 原版 TYPED_TEST(HashingTest, HashEqualsHashValue)
#[test]
fn hashing_test_hash_equals_hash_value() -> Void {
  for_hashing_types!(check_hash_equals_hash_value);
  OK
}

/// 原版 TYPED_TEST(HashingTest, HashIsStateless)
#[test]
fn hashing_test_hash_is_stateless() -> Void {
  for_hashing_types!(check_hash_is_stateless);
  OK
}

/// 原版 TYPED_TEST(HashingTest, HashIsOkish)
#[test]
fn hashing_test_hash_is_okish() -> Void {
  for_hashing_types!(check_hash_is_okish);
  OK
}

/// 原版 TYPED_TEST(HashingTest, HashValueArrayUsesHashRange)
#[test]
fn hashing_test_hash_value_array_uses_hash_range() -> Void {
  for_hashing_types!(check_hash_value_array_uses_hash_range);
  OK
}

/// 原版 TYPED_TEST(HashingTest, HashSpanUsesHashRange)
#[test]
fn hashing_test_hash_span_uses_hash_range() -> Void {
  for_hashing_types!(check_hash_span_uses_hash_range);
  OK
}

/// 原版 TYPED_TEST(HashingTest, BitEqualTo)
#[test]
fn hashing_test_bit_equal_to() -> Void {
  for_hashing_types!(check_bit_equal_to);
  OK
}

/// 原版 TYPED_TEST(HashingTest, BitEqualToImpliesSameBitHash)
#[test]
fn hashing_test_bit_equal_to_implies_same_bit_hash() -> Void {
  for_hashing_types!(check_bit_equal_to_implies_same_bit_hash);
  OK
}

/// 原版匿名 struct Foo + hash_value(Foo) = hash_combine(x, y)
#[derive(Clone, Copy, Debug)]
struct Foo {
  x: i32,
  y: f64,
}

impl HashValue for Foo {
  #[inline]
  fn hash_value(&self) -> usize {
    hash_combine(self.x as usize, self.y as usize)
  }
}

/// 原版 TEST(HashingTest, HashUsesArgumentDependentLookup)
#[test]
fn hashing_test_hash_uses_argument_dependent_lookup() -> Void {
  const INT_VALUES: [i32; 6] = [i32::MIN, -1, 0, 1, 42, i32::MAX];
  const DOUBLE_VALUES: [f64; 6] = [f64::MIN, -1.0, -0.0, 0.0, 1.0, f64::MAX];
  for &x in INT_VALUES.iter() {
    for &y in DOUBLE_VALUES.iter() {
      let foo = Foo { x, y };
      assert_eq!(hash_combine(x as usize, y as usize), hash(&foo));
      let arr = [foo];
      assert_eq!(hash_range(arr.iter()), hash(&arr.as_slice()));
      assert_eq!(hash_range(arr.iter()), hash(&arr));
      assert_eq!(hash_range(arr.iter()), arr.as_slice().hash_value());
    }
  }
  OK
}

/// 原版 TEST(HashingTest, BitEqualToFloat)
/// Rust 无 signaling NaN 常量, 以位型 0x7F80_0001 承接 sNaN (断言面为位比较, 等价)
#[test]
fn hashing_test_bit_equal_to_float() -> Void {
  let pred = |a: &f32, b: &f32| a.bit_equal_to(b);
  assert!(!pred(&0.0f32, &-0.0f32));
  assert!(!pred(&-0.0f32, &0.0f32));
  let qnan = f32::NAN;
  let snan = f32::from_bits(0x7F80_0001);
  assert!(pred(&qnan, &qnan));
  assert!(pred(&snan, &snan));
  assert!(!pred(&qnan, &snan));
  OK
}

/// 原版 TEST(HashingTest, BitHashFloatDifferentForZeroAndMinusZero)
#[test]
fn hashing_test_bit_hash_float_different_for_zero_and_minus_zero() -> Void {
  let h = |v: &f32| v.bit_hash();
  assert_ne!(h(&0.0f32), h(&-0.0f32));
  // 对照: 引擎语义 hash 把 0/-0 归零 (原版 HashFloatZero)
  assert_eq!(hash(&0.0f32), hash(&-0.0f32));
  OK
}

/// 原版 TEST(HashingTest, BitEqualToDouble)
#[test]
fn hashing_test_bit_equal_to_double() -> Void {
  let pred = |a: &f64, b: &f64| a.bit_equal_to(b);
  assert!(!pred(&0.0f64, &-0.0f64));
  assert!(!pred(&-0.0f64, &0.0f64));
  let qnan = f64::NAN;
  let snan = f64::from_bits(0x7FF0_0000_0000_0001);
  assert!(pred(&qnan, &qnan));
  assert!(pred(&snan, &snan));
  assert!(!pred(&qnan, &snan));
  OK
}

/// 原版 TEST(HashingTest, BitHashDoubleDifferentForZeroAndMinusZero)
#[test]
fn hashing_test_bit_hash_double_different_for_zero_and_minus_zero() -> Void {
  let h = |v: &f64| v.bit_hash();
  assert_ne!(h(&0.0f64), h(&-0.0f64));
  assert_eq!(hash(&0.0f64), hash(&-0.0f64));
  OK
}

// ---------------------------------------------------------------------------
// 补充用例: 位级金标准 (参考 rapidhash-v8 secret + mul128 hi^lo 独立算出, 防算法回归)
// ---------------------------------------------------------------------------

/// 原版 TEST 面之外的 mix/hash64 金标准 (仅 64 位平台取值)
#[test]
fn hashing_test_hash64_golden() -> Void {
  assert_eq!(0x2d35_8dcc_aa6c_78a5_u64, RAPIDHASH_SECRET1);
  assert_eq!(0x8bb8_4b93_962e_acc9_u64, RAPIDHASH_SECRET2);
  assert_eq!(0x4b33_a62e_d433_d4a3_u64, RAPIDHASH_SECRET3);
  // rapid_mix = mul128 的高 64 位 ^ 低 64 位 (乘法交换律)
  assert_eq!(rapid_mix(3, 5), rapid_mix(5, 3));
  assert_eq!(0, rapid_mix(0, u64::MAX));
  #[cfg(target_pointer_width = "64")]
  {
    assert_eq!(14592010190165110953_usize, hash64(0));
    assert_eq!(113528333784847620_usize, hash64(1));
    assert_eq!(6675963248919020875_usize, hash64(2));
    assert_eq!(9181454181445443536_usize, hash64(42));
    assert_eq!(16470227512462765693_usize, hash64(u32::MAX as u64));
    assert_eq!(16810968453147192825_usize, hash64(1 << 31));
    assert_eq!(15322859967193777481_usize, hash64(u64::MAX));
    // hash64(0) = rapid_mix(S1, S2)
    assert_eq!(
      rapid_mix(RAPIDHASH_SECRET1, RAPIDHASH_SECRET2) as usize,
      hash64(0)
    );
    assert_eq!(hash64(7), hash(&7u64));
    assert_eq!(hash64(7), hash(&7usize));
    assert_eq!(hash64(7), hash(&7i64));
    assert_eq!(1950309895870198929_usize, hash(&1.5f64));
  }
  OK
}

/// hash32 金标准 + 与 hash64 的截断关系
#[test]
fn hashing_test_hash32_golden() -> Void {
  for k in [0u32, 1, 2, 42, 1 << 31, u32::MAX] {
    assert_eq!((hash64(k as u64) as u32) as usize, hash32(k));
  }
  #[cfg(target_pointer_width = "64")]
  {
    assert_eq!(3349934249_usize, hash32(0));
    assert_eq!(120591620_usize, hash32(1));
    assert_eq!(1096910155_usize, hash32(2));
    assert_eq!(3197411280_usize, hash32(42));
    assert_eq!(1680010745_usize, hash32(1 << 31));
    assert_eq!(3660934781_usize, hash32(u32::MAX));
    assert_eq!(hash32(7), hash(&7u32));
    assert_eq!(hash32(7), hash(&7i32));
    assert_eq!(3208741688_usize, hash(&1.5f32));
  }
  OK
}

/// hash_combine 金标准 (Murmur 风格 mix; 64 位分支 m=0xC6A4A7935BD1E995, r=47)
#[test]
fn hashing_test_hash_combine_golden() -> Void {
  assert_eq!(0, hash_combine(0, 0));
  #[cfg(target_pointer_width = "64")]
  {
    assert_eq!(9404205338720111761_usize, hash_combine(1, 1));
    assert_eq!(
      16812094153218439481_usize,
      hash_combine(123456789, 987654321)
    );
    assert_eq!(16498177628567252948_usize, hash_combine(0, 42));
  }
  // Hasher 折叠 = 逐次 hash_combine
  let mut hasher = Hasher::new(0);
  hasher.add_hash(hash(&1u32));
  hasher.add_hash(hash(&2u64));
  assert_eq!(hasher.hash(), Hasher::new(0).add(&1u32).add(&2u64).hash());
  assert_eq!(hasher.hash(), hash(&(1u32, 2u64)));
  assert_eq!(Hasher::combine2(&1u32, &2u64), hasher.hash());
  assert_eq!(usize::from(hasher), hasher.hash());
  // range 折叠与手工 combine 一致
  assert_eq!(
    hash_range([1u32, 2u64 as u32].iter()),
    Hasher::combine(&[1u32, 2])
  );
  OK
}

/// 宏族展开的平凡/位型映射面 + std_hash 桥接 (补充)
#[test]
fn hashing_test_trivial_and_signed_value_encodings() -> Void {
  // V8_BASE_HASH_VALUE_TRIVIAL: static_cast<size_t>
  assert_eq!(255, hash(&255u8));
  assert_eq!(0, hash(&0u8));
  assert_eq!(7, hash(&7u16));
  assert_eq!(65535, hash(&u16::MAX));
  // V8_BASE_HASH_VALUE_SIGNED: bit_cast 到无符号同路径
  assert_eq!(hash(&u8::MAX), hash(&(-1i8)));
  assert_eq!(hash(&u16::MAX), hash(&(-1i16)));
  assert_eq!(hash(&u32::MAX), hash(&(-1i32)));
  assert_eq!(hash(&u64::MAX), hash(&(-1i64)));
  assert_eq!(hash(&usize::MAX), hash(&(-1isize)));
  // hash_value(T* const&): reinterpret_cast<uintptr_t>
  let probe = 0usize;
  let p: *const usize = &probe;
  assert_eq!(hash(&p), hash(&(p as usize)));
  assert_eq!(hash(&p), hash(&(p as *mut usize)));
  // tuple(3) 走 Hasher 折叠
  assert_eq!(
    hash(&(1u32, 2u64, 3usize)),
    Hasher::new(0).add(&1u32).add(&2u64).add(&3usize).hash()
  );
  // V8_HASHES_COLLIDE 登记的碰撞 hash 常量
  assert_eq!(1234, K_COLLIDING_HASH);
  // std::hash 桥接确定性 (不参与引擎语义)
  assert_eq!(std_hash(&7u64), std_hash(&7u64));
  assert_ne!(std_hash(&7u64), hash(&7u64));
  OK
}
