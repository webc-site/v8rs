// 由手工从 v8/test/bigint/bigint-shell.cc 的 9 个 Test* 转写; 断言强度与原版守恒
//! 原版断言方式: 双算法对拍 AssertEquals + error 标志; 此处直接 assert 双算法输出一致
//! 原 ThresholdRunner (阈值基准测量) 不含正确性断言, 未迁移 (见 README)

mod common;

use aok::{OK, Void};
use common::{Rng, format_hex, generate_random, generate_random_string, processor};
use log::info;
use v8_bigint::{
  bigint::{
    DefaultPlatform, Digit, K_BARRETT_THRESHOLD, K_BURNIKEL_THRESHOLD, K_FFT_THRESHOLD,
    K_FROM_STRING_LARGE_THRESHOLD, K_KARATSUBA_THRESHOLD, K_TO_STRING_FAST_THRESHOLD,
    K_TOOM_THRESHOLD, Status, compare, divide_result_length, modulo_small, multiply_result_length,
  },
  bigint_internal::Processor,
  from_string::FromStringAccumulator,
  mul_helpers::multiply_schoolbook,
  to_string::to_string_result_length,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 双 BigInt 对拍 (原版 AssertEquals)
fn assert_equals(a: &[Digit], b: &[Digit], expected: &[Digit], actual: &[Digit]) {
  assert!(
    compare(expected, actual) == 0,
    "input1: {}\ninput2: {}\nexpected: {}\nactual:   {}",
    format_hex(a),
    format_hex(b),
    format_hex(expected),
    format_hex(actual)
  );
}

#[test]
fn karatsuba() -> Void {
  // 直接调 multiply_karatsuba 要求 left >= right 且 right >= 阈值
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let kmin = K_KARATSUBA_THRESHOLD;
  let kmax = 3 * K_KARATSUBA_THRESHOLD;
  let mut count = 0u64;
  for right_size in kmin..=kmax {
    for left_size in right_size..=kmax {
      let a = generate_random(&mut rng, left_size as usize);
      let b = generate_random(&mut rng, right_size as usize);
      let rl = multiply_result_length(&a, &b) as usize;
      let mut result = vec![0; rl];
      let mut reference = vec![0; rl];
      proc.multiply_karatsuba(&mut result, &a, &b);
      multiply_schoolbook(&mut reference, &a, &b);
      assert_equals(&a, &b, &reference, &result);
      count += 1;
    }
  }
  info!("> karatsuba: {count} cases");
  OK
}

#[test]
fn toom() -> Void {
  // multiply_toom_cook 在阈值之下也能正确工作, 从小处起测省时
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let kmin = K_TOOM_THRESHOLD - 60;
  let kmax = K_TOOM_THRESHOLD + 10;
  let mut count = 0u64;
  for right_size in kmin..=kmax {
    for left_size in right_size..=kmax {
      let a = generate_random(&mut rng, left_size as usize);
      let b = generate_random(&mut rng, right_size as usize);
      let rl = multiply_result_length(&a, &b) as usize;
      let mut result = vec![0; rl];
      let mut reference = vec![0; rl];
      proc.multiply_toom_cook(&mut result, &a, &b);
      // 以 Karatsuba 为参照
      proc.multiply_karatsuba(&mut reference, &a, &b);
      assert_equals(&a, &b, &reference, &result);
      count += 1;
    }
  }
  info!("> toom: {count} cases");
  OK
}

#[test]
fn fft() -> Void {
  // 大乘法慢, 随机采样若干组 (固定种子可复现)
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let mut random_bits = rng.next_u64();
  let min = K_FFT_THRESHOLD - (random_bits & 511) as u32;
  random_bits >>= 10;
  let max = K_FFT_THRESHOLD + (random_bits & 1023) as u32;
  random_bits >>= 10;
  // delta 太小会过慢, 为零会死循环
  let delta = 10 + (random_bits & 127) as u32;
  info!("> fft: min {min} max {max} delta {delta}");
  let mut count = 0u64;
  let mut right_size = min;
  while right_size <= max {
    let mut left_size = right_size;
    while left_size <= max {
      let a = generate_random(&mut rng, left_size as usize);
      let b = generate_random(&mut rng, right_size as usize);
      let rl = multiply_result_length(&a, &b) as usize;
      let mut result = vec![0; rl];
      let mut reference = vec![0; rl];
      proc.multiply_fft(&mut result, &a, &b);
      // 以 Toom-Cook 为参照
      proc.multiply_toom_cook(&mut reference, &a, &b);
      assert_equals(&a, &b, &reference, &result);
      count += 1;
      left_size += delta;
    }
    right_size += delta;
  }
  info!("> fft: {count} cases");
  OK
}

#[test]
fn burnikel() -> Void {
  // 从小处起测省时
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let kmin = K_BURNIKEL_THRESHOLD / 2;
  let kmax = 2 * K_BURNIKEL_THRESHOLD;
  let mut count = 0u64;
  for right_size in kmin..=kmax {
    for left_size in right_size..=kmax {
      let a = generate_random(&mut rng, left_size as usize);
      let b = generate_random(&mut rng, right_size as usize);
      let ql = divide_result_length(&a, &b) as usize;
      let rl = right_size as usize;
      let mut quotient = vec![0; ql];
      let mut quotient_ref = vec![0; ql];
      let mut remainder = vec![0; rl];
      let mut remainder_ref = vec![0; rl];
      proc.divide_burnikel_ziegler(&mut quotient, &mut remainder, &a, &b);
      proc.divide_schoolbook(&mut quotient_ref, &mut remainder_ref, &a, &b);
      assert_equals(&a, &b, &quotient_ref, &quotient);
      assert_equals(&a, &b, &remainder_ref, &remainder);
      count += 1;
    }
  }
  info!("> burnikel: {count} cases");
  OK
}

#[test]
fn barrett() -> Void {
  // 围绕 kBurnikelThreshold 测 (而非 kBarrettThreshold) 以省时
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let kmin = K_BURNIKEL_THRESHOLD / 2;
  let kmax = 2 * K_BURNIKEL_THRESHOLD;
  let mut count = 0u64;
  fn test_barrett_internal(
    proc: &mut Processor<DefaultPlatform>,
    rng: &mut Rng,
    left_size: u32,
    right_size: u32,
  ) {
    let a = generate_random(rng, left_size as usize);
    let b = generate_random(rng, right_size as usize);
    let mut ql = divide_result_length(&a, &b) as usize;
    // divide_result_length 未按 kBarrettThreshold 以下调用设计, 手动补 1
    if (b.len() as u32) < K_BARRETT_THRESHOLD {
      ql += 1;
    }
    let rl = right_size as usize;
    let mut quotient = vec![0; ql];
    let mut quotient_ref = vec![0; ql];
    let mut remainder = vec![0; rl];
    let mut remainder_ref = vec![0; rl];
    proc.divide_barrett(&mut quotient, &mut remainder, &a, &b);
    proc.divide_burnikel_ziegler(&mut quotient_ref, &mut remainder_ref, &a, &b);
    assert_equals(&a, &b, &quotient_ref, &quotient);
    assert_equals(&a, &b, &remainder_ref, &remainder);
  }
  // divide_barrett 要求 A.len > B.len
  for right_size in kmin..=kmax {
    for left_size in right_size + 1..=kmax {
      test_barrett_internal(&mut proc, &mut rng, left_size, right_size);
      count += 1;
    }
  }
  // 另测一个随机大 case
  let mut random_bits = rng.next_u64();
  let right_size = K_BARRETT_THRESHOLD + (random_bits & 0x3FF) as u32;
  random_bits >>= 10;
  let left_size = right_size + 1 + (random_bits & 0x3FFF) as u32;
  test_barrett_internal(&mut proc, &mut rng, left_size, right_size);
  count += 1;
  info!("> barrett: {count} cases");
  OK
}

#[test]
fn cachedmod() -> Void {
  // b_len == 1 实践中不相关, 实现也不支持, 从 2 起
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let mut count = 0u64;
  for b_len in 2..=20u32 {
    let b = generate_random(&mut rng, b_len as usize);
    let mut r = vec![0; b_len as usize];
    let mut r_exp = vec![0; b_len as usize];
    proc.cached_mod_make_inverse(&b);
    // 缓存逆元的长度决定可处理的 a_len 上限
    for a_len in b_len..=2 * b_len {
      for _ in 0..200 {
        let a = generate_random(&mut rng, a_len as usize);
        proc.cached_mod(&mut r, &a);
        let (done, _) = modulo_small(&mut r_exp, &a, &b);
        if !done {
          proc.modulo_large(&mut r_exp, &a, &b);
        }
        assert_equals(&a, &b, &r_exp, &r);
        count += 1;
      }
    }
  }
  info!("> cachedmod: {count} cases");
  OK
}

#[test]
fn tostring() -> Void {
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let kmin = K_TO_STRING_FAST_THRESHOLD / 2;
  let kmax = 2 * K_TO_STRING_FAST_THRESHOLD;
  let mut count = 0u64;
  for size in kmin..kmax {
    let x = generate_random(&mut rng, size as usize);
    for radix in 2..=36u32 {
      let chars_required = to_string_result_length(&x, radix, false) as usize;
      let mut result = vec![0u8; chars_required];
      let mut reference = vec![0u8; chars_required];
      let n_fast = proc.write_impl(&mut result, &x, radix, false, true);
      let n_slow = proc.write_impl(&mut reference, &x, radix, false, false);
      assert_eq!(
        n_fast,
        n_slow,
        "radix {radix} size {size} input {}",
        format_hex(&x)
      );
      assert_eq!(
        &result[..n_fast as usize],
        &reference[..n_slow as usize],
        "radix {radix} size {size} input {}",
        format_hex(&x)
      );
      count += 1;
    }
  }
  info!("> tostring: {count} cases");
  OK
}

#[test]
fn fromstring() -> Void {
  const K_MAX_DIGITS: u32 = 1 << 20; // 足够大即可
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let kmin = K_FROM_STRING_LARGE_THRESHOLD / 2;
  let kmax = 2 * K_FROM_STRING_LARGE_THRESHOLD;
  // 2 幂进制走专用数据格式, Large/Classic 不适用, 单独在 fromstring_base2 测
  // 尾部以 10 填充到 32 个, 让十进制有额外覆盖, 均匀映射随机数
  const RADIXES: [u8; 32] = [
    3, 5, 6, 7, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
    31, 33, 34, 35, 36, 10, 10,
  ];
  let mut count = 0u64;
  for size in kmin..kmax {
    let radix_index = (rng.next_u64() & 31) as usize;
    let radix = RADIXES[radix_index] as u64;
    let num_chars = (size as f64 * 64.0 / (radix as f64).log2()).round() as usize;
    let chars = generate_random_string(&mut rng, num_chars, radix as u32);
    let mut accumulator = FromStringAccumulator::new(K_MAX_DIGITS);
    let mut ref_accumulator = FromStringAccumulator::new(K_MAX_DIGITS);
    accumulator.parse(&chars, radix);
    ref_accumulator.parse(&chars, radix);
    let mut result = vec![0; accumulator.result_length() as usize];
    let mut reference = vec![0; ref_accumulator.result_length() as usize];
    proc.parse_large(&mut result, &mut accumulator);
    proc.parse_classic(&mut reference, &mut ref_accumulator);
    assert!(
      compare(&result, &reference) == 0,
      "input: {} radix: {radix}\nlarge:    {}\nclassic:  {}",
      String::from_utf8_lossy(&chars),
      format_hex(&result),
      format_hex(&reference)
    );
    count += 1;
  }
  info!("> fromstring: {count} cases");
  OK
}

#[test]
fn fromstring_base2() -> Void {
  const K_MAX_DIGITS: u32 = 1 << 20; // 足够大即可
  let mut rng = Rng::new(314159265359);
  let mut proc = processor();
  let mut count = 0u64;
  for size in 1..100u32 {
    let x = generate_random(&mut rng, size as usize);
    for bits in 1..=5u32 {
      let radix = 1 << bits;
      let chars_required = to_string_result_length(&x, radix, false) as usize;
      let mut chars = vec![0u8; chars_required];
      let n = proc.write_impl(&mut chars, &x, radix, false, true);
      // 剩余分配空间填垃圾一并测到
      for c in chars[n as usize..].iter_mut() {
        *c = b'?';
      }
      let mut accumulator = FromStringAccumulator::new(K_MAX_DIGITS);
      accumulator.parse(&chars, radix as u64);
      let mut result = vec![0; accumulator.result_length() as usize];
      let status = proc.parse_string(&mut result, &mut accumulator);
      assert_eq!(status, Status::Ok);
      assert!(
        compare(&x, &result) == 0,
        "input: {} radix: {radix}\nexpected: {}\nactual:   {}",
        String::from_utf8_lossy(&chars[..n as usize]),
        format_hex(&x),
        format_hex(&result)
      );
      count += 1;
    }
  }
  info!("> fromstring_base2: {count} cases");
  OK
}
