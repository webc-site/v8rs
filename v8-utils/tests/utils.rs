// 迁移自 v8/test/unittests/utils/utils-unittest.cc + v8/test/cctest/test-utils.cc (Utils1 的移位断言)
// 断言规约: TYPED_TEST(UtilsTest, X) → 宏展开 8 整型 (上游 16 类型中 signed char/int8_t
// 等别名重复, Rust 8 类型覆盖同一类型域); PassesFilterTest 全断言照搬。
// 跳过: IsInBounds (base::IsInBounds 属 v8-base/base)。
use aok::{OK, Void};
use v8_utils::utils::{
  arithmetic_shift_right, js_max, js_min, passes_filter, saturate_add_i8, saturate_add_i16,
  saturate_add_i32, saturate_add_i64, saturate_add_u8, saturate_add_u16, saturate_add_u32,
  saturate_add_u64, saturate_sub_i8, saturate_sub_i16, saturate_sub_i32, saturate_sub_i64,
  saturate_sub_u8, saturate_sub_u16, saturate_sub_u32, saturate_sub_u64,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 上游 test_cases 常量表 (min/23, max/3, 63, min/6, max/55, min/2, max/2, 0..4, 42)
macro_rules! test_cases {
  ($t:ty) => {{
    let min = <$t>::MIN;
    let max = <$t>::MAX;
    [
      (min / 23) as $t,
      (max / 3) as $t,
      63 as $t,
      (min / 6) as $t,
      (max / 55) as $t,
      (min / 2) as $t,
      (max / 2) as $t,
      0,
      1,
      2,
      3,
      4,
      42,
    ]
  }};
}

// TYPED_TEST(UtilsTest, SaturateSub) — 有符号域
macro_rules! saturate_sub_signed {
  ($name:ident, $t:ty, $sub:expr) => {
    #[test]
    fn $name() -> Void {
      let min = <$t>::MIN;
      let max = <$t>::MAX;
      assert_eq!($sub(min, 0 as $t), min);
      assert_eq!($sub(max, 0 as $t), max);
      assert_eq!($sub(max, min), max);
      assert_eq!($sub(min, max), min);
      assert_eq!($sub(min, max / 3), min);
      assert_eq!($sub(min + 1, 2 as $t), min);
      assert_eq!($sub(min, min), 0 as $t);
      assert_eq!($sub(0 as $t, min), max);
      assert_eq!($sub(max / 3, min), max);
      assert_eq!($sub(max / 5, min), max);
      assert_eq!($sub(min / 3, max), min);
      assert_eq!($sub(min / 9, max), min);
      assert_eq!($sub(max, min / 3), max);
      assert_eq!($sub(min, max / 3), min);
      assert_eq!($sub(max / 3 * 2, min / 2), max);
      assert_eq!($sub(min / 3 * 2, max / 2), min);
      let test_cases = test_cases!($t);
      for &x in &test_cases {
        for &y in &test_cases {
          assert_eq!($sub(x, y), x.wrapping_sub(y), "SaturateSub({x}, {y})");
        }
      }
      OK
    }
  };
}

// TYPED_TEST(UtilsTest, SaturateSub) — 无符号域
macro_rules! saturate_sub_unsigned {
  ($name:ident, $t:ty, $sub:expr) => {
    #[test]
    fn $name() -> Void {
      let min = <$t>::MIN; // 0
      let max = <$t>::MAX;
      assert_eq!($sub(min, 0 as $t), min);
      assert_eq!($sub(max, 0 as $t), max);
      assert_eq!($sub(max, min), max);
      assert_eq!($sub(min, max), min);
      assert_eq!($sub(min, max / 3), min);
      assert_eq!($sub(min + 1, 2 as $t), min);
      assert_eq!($sub(min, min), min);
      assert_eq!($sub(0 as $t, min), min);
      assert_eq!($sub(0 as $t, max), min);
      assert_eq!($sub(max / 3, max), min);
      assert_eq!($sub(max - 3, max), min);
      let test_cases = test_cases!($t);
      for &x in &test_cases {
        for &y in &test_cases {
          let expected = if y > x { min } else { x.wrapping_sub(y) };
          assert_eq!($sub(x, y), expected, "SaturateSub({x}, {y})");
        }
      }
      OK
    }
  };
}

// TYPED_TEST(UtilsTest, SaturateAdd) — 全类型同构 (saturating_add 参照)
macro_rules! saturate_add_typed {
  ($name:ident, $t:ty, $add:expr) => {
    #[test]
    fn $name() -> Void {
      let min = <$t>::MIN;
      let max = <$t>::MAX;
      assert_eq!($add(min, min), min);
      assert_eq!($add(max, max), max);
      assert_eq!($add(min, min / 3), min);
      assert_eq!($add(max / 8 * 7, max / 3 * 2), max);
      assert_eq!($add(min / 3 * 2, min / 8 * 7), min);
      assert_eq!($add(max / 20 * 18, max / 25 * 18), max);
      assert_eq!($add(min / 3 * 2, min / 3 * 2), min);
      assert_eq!($add(max - 1, 2 as $t), max);
      assert_eq!($add(max - 100, 101 as $t), max);
      let test_cases = test_cases!($t);
      for &x in &test_cases {
        for &y in &test_cases {
          assert_eq!($add(x, y), x.saturating_add(y), "SaturateAdd({x}, {y})");
        }
      }
      OK
    }
  };
}

saturate_sub_signed!(saturate_sub_i8_t, i8, saturate_sub_i8);
saturate_sub_signed!(saturate_sub_i16_t, i16, saturate_sub_i16);
saturate_sub_signed!(saturate_sub_i32_t, i32, saturate_sub_i32);
saturate_sub_signed!(saturate_sub_i64_t, i64, saturate_sub_i64);
saturate_sub_unsigned!(saturate_sub_u8_t, u8, saturate_sub_u8);
saturate_sub_unsigned!(saturate_sub_u16_t, u16, saturate_sub_u16);
saturate_sub_unsigned!(saturate_sub_u32_t, u32, saturate_sub_u32);
saturate_sub_unsigned!(saturate_sub_u64_t, u64, saturate_sub_u64);

saturate_add_typed!(saturate_add_i8_t, i8, saturate_add_i8);
saturate_add_typed!(saturate_add_i16_t, i16, saturate_add_i16);
saturate_add_typed!(saturate_add_i32_t, i32, saturate_add_i32);
saturate_add_typed!(saturate_add_i64_t, i64, saturate_add_i64);
saturate_add_typed!(saturate_add_u8_t, u8, saturate_add_u8);
saturate_add_typed!(saturate_add_u16_t, u16, saturate_add_u16);
saturate_add_typed!(saturate_add_u32_t, u32, saturate_add_u32);
saturate_add_typed!(saturate_add_u64_t, u64, saturate_add_u64);

// PassesFilterTest
#[test]
fn passes_filter_test() -> Void {
  assert!(passes_filter("abcdefg", "abcdefg"));
  assert!(passes_filter("abcdefg", "abcdefg*"));
  assert!(passes_filter("abcdefg", "abc*"));
  assert!(passes_filter("abcdefg", "*"));
  assert!(passes_filter("abcdefg", "-~"));
  assert!(passes_filter("abcdefg", "-abcdefgh"));
  assert!(passes_filter("abdefg", "-"));
  assert!(!passes_filter("abcdefg", "-abcdefg"));
  assert!(!passes_filter("abcdefg", "-abcdefg*"));
  assert!(!passes_filter("abcdefg", "-abc*"));
  assert!(!passes_filter("abcdefg", "-*"));
  assert!(!passes_filter("abcdefg", "~"));
  assert!(!passes_filter("abcdefg", ""));
  assert!(!passes_filter("abcdefg", "abcdefgh"));

  assert!(passes_filter("", ""));
  assert!(passes_filter("", "*"));
  assert!(!passes_filter("", "-"));
  assert!(!passes_filter("", "-*"));
  assert!(!passes_filter("", "a"));

  // 上游尾部 UniqueCopy 防 OOB 的变体: 同内容非字面量输入
  let name = "abcdefgh".to_owned();
  let filter = "abcdefgh*".to_owned();
  assert!(passes_filter(&name, &filter));
  OK
}

// cctest TEST(Utils1) 的移位语义断言 (FastD2I 系列属 conversions, 跳过)
#[test]
fn arithmetic_shift_right_semantics() -> Void {
  assert_eq!(-2, -8i64 >> 2);
  assert_eq!(-2i8, (-8i8) >> 2);
  assert_eq!(arithmetic_shift_right(-8, 2), -2);
  assert_eq!(arithmetic_shift_right(-8, 2), -8i64 >> 2);
  OK
}

// 补充: JSMax/JSMin 的 -0 与 NaN 语义 (关键路径, 上游 utils 无单测)
#[test]
fn js_max_min_semantics() -> Void {
  assert_eq!(js_max(1.0, 2.0), 2.0);
  assert_eq!(js_min(1.0, 2.0), 1.0);
  // -0 严格小于 +0
  assert_eq!(js_max(-0.0, 0.0).to_bits(), 0.0f64.to_bits());
  assert_eq!(js_min(-0.0, 0.0).to_bits(), (-0.0f64).to_bits());
  // NaN 优先传播
  assert!(js_max(f64::NAN, 1.0).is_nan());
  assert!(js_min(1.0, f64::NAN).is_nan());
  OK
}

// 补充: 整数散列原语与 rapidhash-v8 位级公式对照
// (公式按 third_party/rapidhash-v8/secret.h + base/hashing.h 独立展开, 防实现漂移)
#[test]
fn hash_primitives_bit_exact() -> Void {
  use v8_utils::utils::{
    compute_address_hash, compute_seeded_hash, hash32, hash64, smi_hash32, smi_hash64,
  };

  // 独立展开: hash64(k) = mix(k^s1, k^s2), mix = u128 乘积高低半异或
  let mix = |a: u64, b: u64| -> u64 {
    let p = (a as u128) * (b as u128);
    (p as u64) ^ ((p >> 64) as u64)
  };
  let s1 = 0x2d358dccaa6c78a5u64; // RAPIDHASH_DEFAULT_SECRET[0]
  let s2 = 0x8bb84b93962eacc9u64; // RAPIDHASH_DEFAULT_SECRET[1]
  for k in [0u64, 1, 0x1234_5678_9abc_def0, u64::MAX] {
    assert_eq!(hash64(k), mix(k ^ s1, k ^ s2), "hash64({k})");
    // hash32 = hash64(同一 32 位键) 低 32 位 (64 位目标)
    let k32 = k as u32;
    assert_eq!(
      hash32(k32),
      (hash64(u64::from(k32)) as u32),
      "hash32({k32})"
    );
  }

  // Smi 掩码: 结果恒为非负 Smi 载荷 (<= (1<<30)-1)
  assert_eq!(smi_hash32(0xdead_beef), hash32(0xdead_beef) & 0x3fff_ffff);
  assert_eq!(
    smi_hash64(0xdead_beef_1234_5678),
    (hash64(0xdead_beef_1234_5678) as u32) & 0x3fff_ffff
  );

  // ComputeSeededHash (非 SIPHASH): SmiHash64(key ^ seed)
  assert_eq!(
    compute_seeded_hash(0x1234, 0xABCD),
    smi_hash64(0x1234 ^ 0xABCD)
  );
  // ComputeAddressHash: 低 32 位截断后 SmiHash32
  let addr = 0x1_2345_6789usize;
  assert_eq!(
    compute_address_hash(addr),
    smi_hash32((addr & 0xFFFF_FFFF) as u32)
  );
  OK
}

// 补充: 纯函数族抽查 (DoubleToBoolean / 位域提取 / is_intn / 页取整 / 字典序)
#[test]
fn misc_pure_helpers() -> Void {
  use std::cmp::Ordering;

  use v8_utils::utils::{
    compare_chars, compare_one_and_two_byte_chars, double_to_boolean, is_int8, is_int16, is_uint32,
    round_up_to_page_size, signed_bitextract_32, ten_to_the, unsigned_bitextract_32,
  };

  // DoubleToBoolean: NaN 与 ±0 为假, 其余为真 (utils.cc:203)
  assert!(!double_to_boolean(f64::NAN));
  assert!(!double_to_boolean(0.0));
  assert!(!double_to_boolean(-0.0));
  assert!(double_to_boolean(-0.1));
  assert!(double_to_boolean(f64::MIN_POSITIVE));
  assert!(double_to_boolean(f64::INFINITY));

  // 位域提取 (utils.h:657): [lsb, msb] 闭区间, 位 2..7
  assert_eq!(unsigned_bitextract_32(7, 2, 0b1010_1110), 0b10_1011);
  // 有符号提取: 位 4..7, 0xF0 → 1111b = -1, 0x10 → 0001b = 1
  assert_eq!(signed_bitextract_32(7, 4, 0xF0), -1);
  assert_eq!(signed_bitextract_32(7, 4, 0x10), 1);

  // 宽度谓词 (is_intn/is_uintn 展开)
  assert!(is_int8(127));
  assert!(!is_int8(128));
  assert!(is_int8(-128));
  assert!(!is_int8(-129));
  assert!(is_int16(32_767));
  assert!(is_uint32(0xFFFF_FFFF));
  assert!(!is_uint32(0x1_0000_0000));

  // TenToThe
  assert_eq!(ten_to_the(0), 1);
  assert_eq!(ten_to_the(19), 10_000_000_000_000_000_000);

  // RoundUpToPageSize: 常规/边界/越限
  assert_eq!(round_up_to_page_size(0, 4096, 1 << 20), Some(0));
  assert_eq!(round_up_to_page_size(1, 4096, 1 << 20), Some(1));
  assert_eq!(round_up_to_page_size(4096, 4096, 1 << 20), Some(1));
  assert_eq!(round_up_to_page_size(4097, 4096, 1 << 20), Some(2));
  assert_eq!(round_up_to_page_size(1 << 21, 4096, 1 << 20), None);

  // 字符序列比较: 同宽字典序 / 交叉逐码元
  assert_eq!(compare_chars(&[1u16, 2], &[1, 3]), Ordering::Less);
  assert_eq!(compare_chars(&[3u16], &[3]), Ordering::Equal);
  assert!(compare_one_and_two_byte_chars(&[0x61, 0x62], &[0x61, 0x62]));
  assert!(!compare_one_and_two_byte_chars(&[0x61], &[0x62]));
  OK
}

// 补充: StringToIndex 关键路径 (上游 utils-inl.h:59, 由字符串层用例覆盖;
// 前导 0 / 非数字 / 数组索引上限 4294967294 / 整数索引上限 2^53-1)
#[test]
fn string_to_index_paths() -> Void {
  use v8_utils::utils::{ToIndexMode, string_to_index};

  let parse = |s: &str, mode: ToIndexMode| -> Option<u64> {
    let mut it = s.chars().map(|c| c as u16);
    string_to_index(&mut it, mode)
  };

  let arr = ToIndexMode::ToArrayIndex;
  let int = ToIndexMode::ToIntegerIndex;

  // 常规数字
  assert_eq!(parse("0", arr), Some(0));
  assert_eq!(parse("7", arr), Some(7));
  assert_eq!(parse("123", arr), Some(123));
  // 前导 0: 仅 "0" 合法
  assert_eq!(parse("01", arr), None);
  assert_eq!(parse("00", arr), None);
  // 非数字 / 空
  assert_eq!(parse("", arr), None);
  assert_eq!(parse("12a", arr), None);
  assert_eq!(parse("a12", arr), None);
  // 数组索引上限 2^32-2 (4294967294 合法, 4294967295 非法)
  assert_eq!(parse("4294967294", arr), Some(4_294_967_294));
  assert_eq!(parse("4294967295", arr), None);
  assert_eq!(parse("4294967296", arr), None);
  // 整数索引上限 2^53-1
  assert_eq!(parse("9007199254740991", int), Some(9_007_199_254_740_991));
  assert_eq!(parse("9007199254740992", int), None);
  assert_eq!(parse("99999999999999999999", int), None);
  // 同一 42 位值: int 模式接受更长输入
  assert_eq!(parse("4294967294", int), Some(4_294_967_294));
  OK
}
