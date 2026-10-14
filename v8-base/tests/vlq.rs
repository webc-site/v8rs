//! v8-base/src/vlq.rs 集成测试 — 1:1 对应 vlq-unittest.cc (6 用例)
//! Random 用例: 固定 seed 的 RandomNumberGenerator 承接 (差分确定性约定)

use aok::{OK, Void};
use v8_base::{
  random_number_generator::RandomNumberGenerator,
  vlq::{vlq_decode, vlq_decode_unsigned_at, vlq_encode_to_vec, vlq_encode_unsigned_to_vec},
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 ExpectedBytesUsed
fn expected_bytes_used(value: i64, is_signed: bool) -> usize {
  let mut bits = value as u64;
  if is_signed {
    bits = value.unsigned_abs() << 1;
  }
  let mut num_bits = 0;
  while bits != 0 {
    num_bits += 1;
    bits >>= 1;
  }
  ((num_bits as f32) / 7.0).ceil().max(1.0) as usize
}

/// 原版 TestVLQUnsignedEquals
fn test_vlq_unsigned_equals(value: u32) {
  let mut buffer = Vec::new();
  vlq_encode_unsigned_to_vec(&mut buffer, value);
  let expected = expected_bytes_used(i64::from(value), false);
  assert_eq!(expected, buffer.len(), "value={value}");
  let mut index = 0;
  assert_eq!(value, vlq_decode_unsigned_at(&buffer, &mut index));
  assert_eq!(expected, index);
}

/// 原版 TestVLQEquals
fn test_vlq_equals(value: i32) {
  let mut buffer = Vec::new();
  vlq_encode_to_vec(&mut buffer, value);
  let expected = expected_bytes_used(i64::from(value), true);
  assert_eq!(expected, buffer.len(), "value={value}");
  let mut index = 0;
  assert_eq!(value, vlq_decode(&buffer, &mut index));
  assert_eq!(expected, index);
}

#[test]
fn vlq_unsigned() -> Void {
  for v in [0, 1, 63, 64, 127, 255, 256] {
    test_vlq_unsigned_equals(v);
  }
  OK
}

#[test]
fn vlq_positive() -> Void {
  for v in [0, 1, 63, 64, 127, 255, 256] {
    test_vlq_equals(v);
  }
  OK
}

#[test]
fn vlq_negative() -> Void {
  for v in [-1, -63, -64, -127, -255, -256] {
    test_vlq_equals(v);
  }
  OK
}

#[test]
fn vlq_limits_unsigned() -> Void {
  // 原版 LimitsUnsigned 混用了 TestVLQEquals (有符号入口, 值域 <= i8/u16 边界)
  test_vlq_equals(i64::from(u8::MAX) as i32);
  test_vlq_equals(i64::from(u8::MAX - 1) as i32);
  test_vlq_equals(i64::from(u8::MAX) as i32 + 1);
  test_vlq_equals(i64::from(u16::MAX) as i32);
  test_vlq_equals(i64::from(u16::MAX - 1) as i32);
  test_vlq_equals(i64::from(u16::MAX) as i32 + 1);
  test_vlq_equals(u32::MAX as i32);
  test_vlq_equals((u32::MAX - 1) as i32);
  OK
}

#[test]
fn vlq_limits_signed() -> Void {
  test_vlq_equals(i8::MAX as i32);
  test_vlq_equals(i64::from(i8::MAX) as i32 - 1);
  test_vlq_equals(i64::from(i8::MAX) as i32 + 1);
  test_vlq_equals(i16::MAX as i32);
  test_vlq_equals(i64::from(i16::MAX) as i32 - 1);
  test_vlq_equals(i64::from(i16::MAX) as i32 + 1);
  test_vlq_equals(i32::MAX);
  test_vlq_equals(i32::MAX - 1);
  // 原版注释: int32_t::min() 不支持 (vlq_convert_to_unsigned 的 DCHECK 排除)
  test_vlq_equals(i32::MIN + 1);
  OK
}

#[test]
fn vlq_random() -> Void {
  const RANDOM_RUNS: usize = 50;
  // 固定 seed (原版取 GTEST random_seed; 差分确定性约定禁随机)
  let mut rng = RandomNumberGenerator::with_seed(42);
  for _ in 0..RANDOM_RUNS {
    test_vlq_unsigned_equals(rng.next_bounded_int(i32::MAX) as u32);
  }
  for _ in 0..RANDOM_RUNS {
    test_vlq_equals(rng.next_int());
  }
  OK
}

/// 对抗补充: 全部 7 位边界 ±1 的编码↔解码往返 + 多值流游标串联
/// (round3 审查任务: vlq 编解码往返; 原版用例未覆盖 2^k 边界全域)
#[test]
fn vlq_roundtrip_boundaries() -> Void {
  let mut buffer = Vec::new();
  // u32 全部 2^k 边界及其 ±1 (k = 0..=32; 2^32 回绕为 0 与 k=0 重复, 无害)
  for k in 0..=32u32 {
    for v in [
      1u64 << k,
      (1u64 << k).wrapping_sub(1),
      (1u64 << k).wrapping_add(1),
    ] {
      let value = v as u32;
      test_vlq_unsigned_equals(value);
      // 有符号面: 同位型正负值 (排除 i32::MIN, 原版 DCHECK 排除)
      let signed = value as i32;
      if signed != i32::MIN {
        test_vlq_equals(signed);
        test_vlq_equals(-signed);
      }
    }
  }
  // 多值流: 连续编码后逐游标解码, index 必须首尾相接
  let corpus = [
    0u32,
    1,
    127,
    128,
    16383,
    16384,
    2_097_151,
    2_097_152,
    u32::MAX,
    u32::MAX - 1,
  ];
  buffer.clear();
  for &v in &corpus {
    vlq_encode_unsigned_to_vec(&mut buffer, v);
  }
  let mut index = 0;
  for &v in &corpus {
    assert_eq!(
      vlq_decode_unsigned_at(&buffer, &mut index),
      v,
      "stream v={v}"
    );
  }
  assert_eq!(index, buffer.len());
  // 有符号多值流: 负数 zigzag 后往返
  buffer.clear();
  for s in [i32::MIN + 1, -1, 0, 1, i32::MAX, -123456789, 123456789] {
    vlq_encode_to_vec(&mut buffer, s);
  }
  let mut index = 0;
  for s in [i32::MIN + 1, -1, 0, 1, i32::MAX, -123456789, 123456789] {
    assert_eq!(vlq_decode(&buffer, &mut index), s, "stream s={s}");
  }
  assert_eq!(index, buffer.len());
  OK
}
