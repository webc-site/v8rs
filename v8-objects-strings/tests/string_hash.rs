//! 哈希与索引解析规约 (string.cc:1898-2036 + cctest/test-strings.cc:1852/1900)。

pub mod common;

use aok::{OK, Void};
use common::{Arena, Maps};
use v8_objects_core::name::Name;
use v8_objects_strings::string::StringExt;
use v8_strings::{
  hash_field::contains_cached_array_index as name_contains_cached, hash_seed::HashSeedData,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// HashArrayIndexStrings (cctest:1852): 数字串哈希 → 数组索引回读。
#[test]
fn hash_array_index_strings() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let indices = [0u32, 1, 9, 10, 99, 100, 12345, 9999999];
  for &i in &indices {
    let s = arena.seq_one_byte(maps.seq_one, i.to_string().as_bytes());
    // SAFETY: 竞技场对象布局契约成立
    unsafe {
      let field = s
        .compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
        .ok()
        .unwrap();
      // ≤7 位数字串: 索引直接缓存在哈希字段
      assert!(name_contains_cached(field), "i={i} raw={field:#x}");
      assert_eq!(s.as_array_index(), Some(i), "i={i}");
      assert_eq!(s.as_integer_index(), Some(i as usize), "i={i}");
    }
    // 双字节同内容同结果
    let units: Vec<u16> = i.to_string().encode_utf16().collect();
    let s = arena.seq_two_byte(maps.seq_two, &units);
    // SAFETY: 同上
    unsafe {
      s.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
        .ok();
      assert_eq!(s.as_array_index(), Some(i), "two-byte i={i}");
    }
  }
  OK
}

/// ArrayIndexHashRoundTrip (cctest:1900): 慢路径 (8-10 位) 与上界。
#[test]
fn array_index_hash_round_trip() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  // 8-10 位: 缓存不下, 走流式 StringToIndex
  for i in [12345678u32, 999999999, 1073741824, 4294967294] {
    let s = arena.seq_one_byte(maps.seq_one, i.to_string().as_bytes());
    // SAFETY: 竞技场对象布局契约成立
    unsafe {
      s.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
        .ok();
      assert_eq!(s.as_array_index(), Some(i), "i={i}");
    }
  }
  // kMaxArrayIndex = 2^32 - 2 为合法上界; 2^32 - 1 不是 array index
  let over = arena.seq_one_byte(maps.seq_one, u32::MAX.to_string().as_bytes());
  // SAFETY: 同上
  unsafe {
    over
      .compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
      .ok();
    assert_eq!(over.as_array_index(), None);
    // 但 ≤ 2^53-1 仍是 integer index
    assert_eq!(over.as_integer_index(), Some(u32::MAX as usize));
  }
  // 非数字串
  for text in ["", "a", "12a", "-1", "1.5", " 1", "+1", "01"] {
    let s = arena.seq_one_byte(maps.seq_one, text.as_bytes());
    // SAFETY: 同上
    unsafe {
      s.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
        .ok();
      assert_eq!(s.as_array_index(), None, "text={text}");
    }
  }
  // 前导零仅 "0" 合法 ("01" 已测)
  OK
}

/// ComputeAndSetRawHash 语义 (string.cc:1954-2005): 类型位/确定性/Thin 复用。
#[test]
fn compute_and_set_raw_hash_semantics() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let seed = HashSeedData::DEFAULT;
  let s = arena.seq_one_byte(maps.seq_one, b"hello world");
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let f1 = s.compute_and_set_raw_hash(None, &seed).ok().unwrap();
    // 已计算 → Hash 型, 幂等
    assert!(Name::is_hash_field_computed(f1));
    let f2 = s.compute_and_set_raw_hash(None, &seed).ok().unwrap();
    assert_eq!(f1, f2);
    assert_eq!(s.raw_hash_field(), f1);
    // 哈希载荷非零 (kZeroHash 兜底)
    assert_ne!(s.try_get_hash(), Some(0));

    // 非展平 Cons: 经 WriteToFlat 缓冲后哈希, 与等值 Seq 一致
    let a = arena.seq_one_byte(maps.seq_one, b"hello ");
    let b = arena.seq_one_byte(maps.seq_one, b"world");
    let cons = arena.cons(maps.cons_one, 11, a, b);
    let fc = cons.compute_and_set_raw_hash(None, &seed).ok().unwrap();
    assert_eq!(fc, f1, "cons 与等值 seq 哈希一致");

    // Sliced: 从 parent 偏移起哈希
    let base = arena.seq_one_byte(maps.seq_one, b"xxhello worldxx");
    let sliced = arena.sliced(maps.sliced_one, 11, base, 2);
    let fs = sliced.compute_and_set_raw_hash(None, &seed).ok().unwrap();
    assert_eq!(fs, f1, "sliced 与等值 seq 哈希一致");

    // 等长 Thin: 复用 actual 哈希 (string.cc:1984)
    let actual = arena.seq_one_byte(maps.int_one, b"hello world");
    actual.compute_and_set_raw_hash(None, &seed).ok().unwrap();
    let thin = arena.thin(maps.thin_one, 11, actual);
    let ft = thin.compute_and_set_raw_hash(None, &seed).ok().unwrap();
    assert_eq!(ft, actual.raw_hash_field());
  }
  OK
}

/// 长串 trivial 哈希 (string.cc:1904: > kMaxHashCalcLength 仅按长度)。
#[test]
fn trivial_hash_for_long_strings() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let long_a: Vec<u8> = vec![b'a'; 16384];
  let long_b: Vec<u8> = vec![b'b'; 16384];
  let a = arena.seq_one_byte(maps.seq_one, &long_a);
  let b = arena.seq_one_byte(maps.seq_one, &long_b);
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    let fa = a
      .compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
      .ok()
      .unwrap();
    let fb = b
      .compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
      .ok()
      .unwrap();
    // 内容不同但同长 → 同 trivial 哈希 (仅长度)
    assert_eq!(fa, fb);
    assert_eq!((fa >> 2) & 0x3FFF_FFFF, 16384);
  }
  OK
}

/// AsIntegerIndex 慢路径: > 7 位且 ≤ 16 位的整数串 (string.cc:2022)。
#[test]
fn integer_index_long_digits() -> Void {
  let mut arena = Arena::new();
  let maps = Maps::new(&mut arena);
  let s = arena.seq_one_byte(maps.seq_one, b"9007199254740991"); // 2^53-1
  // SAFETY: 竞技场对象布局契约成立
  unsafe {
    s.compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
      .ok();
    assert_eq!(s.as_integer_index(), Some(9_007_199_254_740_991));
  }
  let over = arena.seq_one_byte(maps.seq_one, b"9007199254740992"); // 2^53
  // SAFETY: 同上
  unsafe {
    over
      .compute_and_set_raw_hash(None, &HashSeedData::DEFAULT)
      .ok();
    assert_eq!(over.as_integer_index(), None);
  }
  OK
}
