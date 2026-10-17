// 字符串哈希算法测试 (v8/src/strings/string-hasher.h 语义)

use aok::OK;
use v8_strings::{
  hash_field::contains_cached_array_index,
  hash_seed::HashSeedData,
  string_hasher::{
    RunningStringHasher, decode_array_index_from_hash_field, hash_sequential_one_byte,
    hash_sequential_two_byte, make_array_index_hash_raw,
  },
};

#[ctor::ctor(unsafe)]
fn init() {
  log_init::init();
}

#[test]
fn array_index_hash_roundtrip() -> aok::Void {
  for val in [0u32, 1, 42, 9999, 1234567, 9999999] {
    let len = if val == 0 {
      1
    } else {
      val.to_string().len() as u32
    };
    let hash = make_array_index_hash_raw(val, len);
    assert_eq!(decode_array_index_from_hash_field(hash), val);
    assert!(contains_cached_array_index(hash));
  }
  OK
}

#[test]
fn running_string_hasher() -> aok::Void {
  // 黄金值: SuperFastHash 算式独立重算 (h += c; h += h<<10; h ^= h>>6;
  // finalize: h += h<<3; h ^= h>>11; h += h<<15; ConvertRawHashToUsable)
  let mut hasher = RunningStringHasher::new(0);
  for &c in b"test_string" {
    hasher.add_character(u16::from(c));
  }
  let h1 = hasher.finalize();
  assert_eq!(h1, 0x1c5ef987, "SuperFastHash 黄金值锚定");

  // seed 参与: 非 0 种子产出不同可用哈希
  let mut seeded = RunningStringHasher::new(42);
  for &c in b"test_string" {
    seeded.add_character(u16::from(c));
  }
  assert_eq!(seeded.finalize(), 0x22a62e4a);

  // 双字节代理对串
  let mut surrogate = RunningStringHasher::new(0);
  for c in [0xD83D_u16, 0xDE00_u16] {
    surrogate.add_character(c);
  }
  assert_eq!(surrogate.finalize(), 0x1c28937a);

  // 确定性: 同输入两次一致且非零
  let mut hasher2 = RunningStringHasher::new(0);
  for &c in b"test_string" {
    hasher2.add_character(u16::from(c));
  }
  let h2 = hasher2.finalize();
  assert_eq!(h1, h2);
  assert_ne!(h1, 0);
  OK
}

#[test]
fn sequential_string_hash_index_detection() -> aok::Void {
  let seed = HashSeedData::DEFAULT;

  // "42" 是合法 array index
  let h_index = hash_sequential_one_byte(b"42", &seed, None);
  assert!(contains_cached_array_index(h_index));
  assert_eq!(decode_array_index_from_hash_field(h_index), 42);

  // "012" 有前导零, 不是 array index
  let h_non_index = hash_sequential_one_byte(b"012", &seed, None);
  assert!(!contains_cached_array_index(h_non_index));

  // "hello" 是普通字符串
  let h_str = hash_sequential_one_byte(b"hello", &seed, None);
  assert!(!contains_cached_array_index(h_str));

  // 双字节串
  let u16_chars = [b'4' as u16, b'2' as u16];
  let mut is_one_byte = false;
  let h_two = hash_sequential_two_byte(&u16_chars, &seed, Some(&mut is_one_byte));
  assert!(contains_cached_array_index(h_two));
  assert_eq!(decode_array_index_from_hash_field(h_two), 42);
  assert!(is_one_byte);
  OK
}
