use v8_bigint::{bigint::Status, div_helpers::left_shift};
mod common;

use std::panic::catch_unwind;

use aok::{OK, Void};
use common::{Rng, format_hex, generate_random, processor};
use v8_bigint::{
  bigint::{
    Digit, K_BARRETT_THRESHOLD, K_BURNIKEL_THRESHOLD, K_FROM_STRING_LARGE_THRESHOLD,
    K_KARATSUBA_THRESHOLD, K_TO_STRING_FAST_THRESHOLD, K_TOOM_THRESHOLD, as_int_n,
    as_int_n_result_length, as_uint_n_neg, as_uint_n_neg_result_length,
    as_uint_n_pos_result_length, bit_length, compare, digit_div, divide_result_length,
    divide_single, divide_small, mod_single, modulo_small, msd, multiply_result_length,
    multiply_small, norm, truncate_and_sub_from_power_of_two, truncate_to_n_bits,
  },
  div_helpers::ShiftedDigits,
  from_string::FromStringAccumulator,
  mul_helpers::multiply_schoolbook,
  to_string::to_string_result_length,
  util::div_ceil_u64,
  vector_arithmetic::add_and_return_overflow,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

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
fn test_zero_and_empty_handling() -> Void {
  // 1. div_ceil / div_ceil_u64
  assert_eq!(div_ceil_u64(0, 1), 0);
  assert_eq!(div_ceil_u64(0, 64), 0);
  assert_eq!(div_ceil_u64(1, 64), 1);
  assert_eq!(div_ceil_u64(64, 64), 1);
  assert_eq!(div_ceil_u64(65, 64), 2);

  // 2. msd & compare on empty and zero-padded slices
  // BigInt 在内存中按 little-endian 存放, msd 对应末尾元素
  assert_eq!(msd(&[]), 0);
  assert_eq!(msd(&[0]), 0);
  assert_eq!(msd(&[0, 0, 0]), 0);
  assert_eq!(msd(&[0, 0, 42]), 42);
  assert_eq!(msd(&[42]), 42);
  assert_eq!(msd(norm(&[42, 0, 0])), 42);

  assert_eq!(compare(&[], &[]), 0);
  assert_eq!(compare(&[], &[0, 0]), 0);
  assert_eq!(compare(&[0], &[]), 0);
  assert_eq!(compare(&[0, 0], &[0]), 0);
  assert_eq!(compare(&[1], &[]), 1);
  assert_eq!(compare(&[], &[1]), -1);
  assert_eq!(compare(&[1, 0], &[1]), 0);

  // 3. bit_length
  assert_eq!(bit_length(0), 0);
  assert_eq!(bit_length(1), 1);
  assert_eq!(bit_length(u32::MAX), 32);

  // 4. ShiftedDigits & left_shift trailing zero fill
  let empty_shifted = ShiftedDigits::new(&[], None);
  assert!(empty_shifted.digits().is_empty());
  let empty_shifted_explicit = ShiftedDigits::new(&[], Some(10));
  assert!(empty_shifted_explicit.digits().is_empty());

  let mut left_shift_buf = [999u64; 4];
  left_shift(&mut left_shift_buf, &[1], 1);
  assert_eq!(left_shift_buf[0], 2);
  assert_eq!(left_shift_buf[1], 0);
  assert_eq!(left_shift_buf[2], 0);
  assert_eq!(left_shift_buf[3], 0);

  // 5. FromStringAccumulator on empty / zeros
  for radix in 2..=36u64 {
    let mut acc = FromStringAccumulator::new(100);
    acc.parse(b"", radix);
    assert_eq!(acc.result_length(), 0);

    let mut acc_zero = FromStringAccumulator::new(100);
    acc_zero.parse(b"0000000000", radix);
    assert_eq!(acc_zero.result_length(), 0);
  }

  // 6. ToString write_impl with 0 input and empty out buffer
  let mut proc = processor();
  let mut empty_buf: [u8; 0] = [];
  assert_eq!(proc.write_impl(&mut empty_buf, &[], 10, false, false), 1);
  assert_eq!(proc.write_impl(&mut empty_buf, &[123], 10, false, false), 0);

  for radix in 2..=36u32 {
    let mut buf = vec![0u8; 16];
    let len = proc.write_impl(&mut buf, &[], radix, false, false);
    assert_eq!(&buf[..len as usize], b"0");

    let len_fast = proc.write_impl(&mut buf, &[], radix, false, true);
    assert_eq!(&buf[..len_fast as usize], b"0");

    let len_zero = proc.write_impl(&mut buf, &[0, 0, 0], radix, false, true);
    assert_eq!(&buf[..len_zero as usize], b"0");

    // 负零不应带负号
    let len_neg_zero = proc.write_impl(&mut buf, &[0], radix, true, true);
    assert_eq!(&buf[..len_neg_zero as usize], b"0");
  }

  // 7. Multiplication by 0
  let mut res = vec![0; 4];
  let a = [12345, 67890];
  multiply_schoolbook(&mut res, &a, &[]);
  assert!(norm(&res).is_empty());
  let (done1, _) = multiply_small(&mut res, &a, &[]);
  assert!(done1 && norm(&res).is_empty());
  let (done2, _) = multiply_small(&mut res, &[], &a);
  assert!(done2 && norm(&res).is_empty());

  // 8. Truncation and bit operations on 0 bits / 0 length
  assert_eq!(as_uint_n_neg_result_length(0), 0);
  assert_eq!(as_uint_n_neg_result_length(64), 1);
  assert_eq!(as_int_n_result_length(&[], false, 0), -1);
  assert_eq!(as_int_n_result_length(&[42], false, 0), 0);
  assert_eq!(as_uint_n_pos_result_length(&[], 0), -1);
  assert_eq!(as_uint_n_pos_result_length(&[42], 0), 0);

  let mut trunc_buf = [999u64; 2];
  truncate_to_n_bits(&mut trunc_buf, &[42], 0);
  assert_eq!(trunc_buf[0], 999);
  truncate_and_sub_from_power_of_two(&mut trunc_buf, &[42], 0);
  assert_eq!(trunc_buf[0], 999);

  // 9. Division length when a.len() < b.len()
  assert_eq!(divide_result_length(&[], &[1, 2]), 1);
  assert_eq!(divide_result_length(&[1], &[1, 2, 3]), 1);

  OK
}

#[test]
fn test_division_by_zero_defenses() -> Void {
  // digit_div by 0
  let r = catch_unwind(|| {
    digit_div(10, 0, 0);
  });
  assert!(r.is_err());

  // divide_single by 0
  let mut q = [0; 2];
  let r = catch_unwind(move || {
    divide_single(&mut q, &[1, 2], 0);
  });
  assert!(r.is_err());

  // mod_single by 0
  let r = catch_unwind(|| {
    mod_single(&[1, 2], 0);
  });
  assert!(r.is_err());

  // divide_small by 0
  let mut q = [0; 2];
  let r = catch_unwind(move || {
    divide_small(&mut q, &[1, 2], &[]);
  });
  assert!(r.is_err());

  let mut q = [0; 2];
  let r = catch_unwind(move || {
    divide_small(&mut q, &[1, 2], &[0, 0]);
  });
  assert!(r.is_err());

  // modulo_small by 0
  let mut rem = [0; 2];
  let r = catch_unwind(move || {
    modulo_small(&mut rem, &[1, 2], &[]);
  });
  assert!(r.is_err());

  let mut rem = [0; 2];
  let r = catch_unwind(move || {
    modulo_small(&mut rem, &[1, 2], &[0]);
  });
  assert!(r.is_err());

  // divide_large by 0
  let mut q = [0; 4];
  let r = catch_unwind(move || {
    let mut p = processor();
    p.divide_large(&mut q, &[1, 2, 3, 4], &[0]);
  });
  assert!(r.is_err());

  // modulo_large by 0
  let mut rem = [0; 4];
  let r = catch_unwind(move || {
    let mut p = processor();
    p.modulo_large(&mut rem, &[1, 2, 3, 4], &[0]);
  });
  assert!(r.is_err());

  // cached_mod_make_inverse by 0
  let r = catch_unwind(move || {
    let mut p = processor();
    p.cached_mod_make_inverse(&[0, 0]);
  });
  assert!(r.is_err());

  // divide_schoolbook by 0
  let mut q = [0; 4];
  let mut rem = [0; 4];
  let r = catch_unwind(move || {
    let mut p = processor();
    p.divide_schoolbook(&mut q, &mut rem, &[1, 2, 3, 4], &[]);
  });
  assert!(r.is_err());

  // divide_burnikel_ziegler by 0
  let mut q = [0; 4];
  let mut rem = [0; 4];
  let r = catch_unwind(move || {
    let mut p = processor();
    p.divide_burnikel_ziegler(&mut q, &mut rem, &[1, 2, 3, 4], &[0]);
  });
  assert!(r.is_err());

  // divide_barrett by 0
  let mut q = [0; 4];
  let mut rem = [0; 4];
  let r = catch_unwind(move || {
    let mut p = processor();
    p.divide_barrett(&mut q, &mut rem, &[1, 2, 3, 4], &[]);
  });
  assert!(r.is_err());

  OK
}

#[test]
fn test_truncate_sub_power_of_two_borrow_regression() -> Void {
  // round1 审查回归: TruncateAndSubFromPowerOfTwo 的 bits==0 分支借位出 == 1 属正常
  // (低 n 位非零时, 虚拟 2^n 顶位吸收借位); 原版同分支无断言, 转写版误加的
  // debug_assert!(b == 0) 已删 (原版 v8/src/bigint/bigint-inl.h:1387 分支无 DCHECK)
  // 1. asUintN_Neg: 2^128 - 5, 低 128 位非零 → bits==0 且末位借位 1
  let mut z = vec![0u64; 3];
  as_uint_n_neg(&mut z, &[0, 5], 128);
  assert_eq!(z[0], 0);
  assert_eq!(z[1], Digit::MAX - 4);
  // 2. asIntN: 2^192 - 1, bit 191 置位 → 2^n 减法路径, bits == 0
  let x = vec![Digit::MAX, Digit::MAX, Digit::MAX];
  let mut z2 = vec![0u64; 3];
  let neg = as_int_n(&mut z2, &x, false, 192);
  assert!(neg);
  assert_eq!(compare(&z2, &[1]), 0); // |2^192 - 1 截断补码| == 1
  // 3. asIntN 负输入: asIntN(192, -(2^192-1)) == +1 (补码回绕为正)
  let mut z3 = vec![0u64; 3];
  let neg3 = as_int_n(&mut z3, &x, true, 192);
  assert!(!neg3);
  assert_eq!(compare(&z3, &[1]), 0);
  OK
}

#[test]
fn test_multiplication_threshold_boundaries() -> Void {
  let mut rng = Rng::new(123456789);
  let mut proc = processor();

  // Karatsuba threshold 边界: 32, 33, 34, 35, 36
  for right_size in (K_KARATSUBA_THRESHOLD - 2)..=(K_KARATSUBA_THRESHOLD + 2) {
    for left_size in right_size..=(right_size + 2) {
      let a = generate_random(&mut rng, left_size as usize);
      let b = generate_random(&mut rng, right_size as usize);
      let rl = multiply_result_length(&a, &b) as usize;
      let mut result = vec![0; rl];
      let mut reference = vec![0; rl];
      multiply_schoolbook(&mut reference, &a, &b);

      if right_size >= K_KARATSUBA_THRESHOLD {
        proc.multiply_karatsuba(&mut result, &a, &b);
      } else {
        multiply_schoolbook(&mut result, &a, &b);
      }
      assert_equals(&a, &b, &reference, &result);
    }
  }

  // Toom threshold 边界: 208, 209, 210, 211, 212
  for right_size in (K_TOOM_THRESHOLD - 2)..=(K_TOOM_THRESHOLD + 2) {
    let left_size = right_size + 1;
    let a = generate_random(&mut rng, left_size as usize);
    let b = generate_random(&mut rng, right_size as usize);
    let rl = multiply_result_length(&a, &b) as usize;
    let mut result = vec![0; rl];
    let mut reference = vec![0; rl];
    proc.multiply_toom_cook(&mut result, &a, &b);
    proc.multiply_karatsuba(&mut reference, &a, &b);
    assert_equals(&a, &b, &reference, &result);
  }

  OK
}

#[test]
fn test_division_threshold_boundaries() -> Void {
  let mut rng = Rng::new(987654321);
  let mut proc = processor();

  // Burnikel-Ziegler 边界: 55, 56, 57, 58, 59
  for right_size in (K_BURNIKEL_THRESHOLD - 2)..=(K_BURNIKEL_THRESHOLD + 2) {
    for left_size in (right_size + 1)..=(right_size + 3) {
      let a = generate_random(&mut rng, left_size as usize);
      let b = generate_random(&mut rng, right_size as usize);
      let ql = divide_result_length(&a, &b) as usize;
      let rl = right_size as usize;
      let mut q_bz = vec![0; ql];
      let mut r_bz = vec![0; rl];
      let mut q_ref = vec![0; ql];
      let mut r_ref = vec![0; rl];

      proc.divide_burnikel_ziegler(&mut q_bz, &mut r_bz, &a, &b);
      proc.divide_schoolbook(&mut q_ref, &mut r_ref, &a, &b);

      assert_equals(&a, &b, &q_ref, &q_bz);
      assert_equals(&a, &b, &r_ref, &r_bz);

      // 数学不变量: A == B * Q + R 且 R < B
      let q_norm = norm(&q_bz);
      let b_norm = norm(&b);
      let mut reconstructed = vec![0; a.len() + 2];
      if !q_norm.is_empty() {
        let ql = multiply_result_length(b_norm, q_norm) as usize;
        let (big, small) = if b_norm.len() >= q_norm.len() {
          (b_norm, q_norm)
        } else {
          (q_norm, b_norm)
        };
        let mut prod = vec![0; ql];
        multiply_schoolbook(&mut prod, big, small);
        reconstructed[..prod.len()].copy_from_slice(&prod);
      }
      let r_norm = norm(&r_bz);
      let carry = add_and_return_overflow(&mut reconstructed, r_norm);
      assert_eq!(carry, 0);
      assert_eq!(compare(norm(&a), norm(&reconstructed)), 0);
      assert!(compare(r_norm, b_norm) < 0);
    }
  }

  // Barrett 除法代数不变量: A == B * Q + R 且 R < B
  for right_size in [2usize, 5, 10, 20] {
    for left_size in (right_size + 1)..=(right_size + 4) {
      let a = generate_random(&mut rng, left_size);
      let b = generate_random(&mut rng, right_size);
      let mut ql = divide_result_length(&a, &b) as usize;
      if (b.len() as u32) < K_BARRETT_THRESHOLD {
        ql += 1;
      }
      let rl = right_size;
      let mut q_bar = vec![0; ql];
      let mut r_bar = vec![0; rl];
      proc.divide_barrett(&mut q_bar, &mut r_bar, &a, &b);

      let q_norm = norm(&q_bar);
      let b_norm = norm(&b);
      let mut reconstructed = vec![0; a.len() + 2];
      if !q_norm.is_empty() {
        let ql = multiply_result_length(b_norm, q_norm) as usize;
        let (big, small) = if b_norm.len() >= q_norm.len() {
          (b_norm, q_norm)
        } else {
          (q_norm, b_norm)
        };
        let mut prod = vec![0; ql];
        multiply_schoolbook(&mut prod, big, small);
        reconstructed[..prod.len()].copy_from_slice(&prod);
      }
      let r_norm = norm(&r_bar);
      let carry = add_and_return_overflow(&mut reconstructed, r_norm);
      assert_eq!(carry, 0);
      assert_eq!(compare(norm(&a), norm(&reconstructed)), 0);
      assert!(compare(r_norm, b_norm) < 0);
    }
  }

  OK
}

#[test]
fn test_string_conversion_threshold_boundaries() -> Void {
  let mut rng = Rng::new(555555555);
  let mut proc = processor();

  // ToString fast 边界: 21, 22, 23, 24, 25
  for size in (K_TO_STRING_FAST_THRESHOLD - 2)..=(K_TO_STRING_FAST_THRESHOLD + 2) {
    let x = generate_random(&mut rng, size as usize);
    for radix in [2, 8, 10, 16, 36] {
      let chars_required = to_string_result_length(&x, radix, false) as usize;
      let mut res_fast = vec![0u8; chars_required];
      let mut res_slow = vec![0u8; chars_required];
      let n_fast = proc.write_impl(&mut res_fast, &x, radix, false, true);
      let n_slow = proc.write_impl(&mut res_slow, &x, radix, false, false);
      assert_eq!(n_fast, n_slow);
      assert_eq!(&res_fast[..n_fast as usize], &res_slow[..n_slow as usize]);
    }
  }

  // FromString large 边界 (非 2 幂进制): 23, 24, 25, 26, 27
  const K_MAX_DIGITS: u32 = 1 << 20;
  for size in (K_FROM_STRING_LARGE_THRESHOLD - 2)..=(K_FROM_STRING_LARGE_THRESHOLD + 2) {
    for radix in [10u64, 36u64] {
      let num_chars = (size as f64 * 64.0 / (radix as f64).log2()).round() as usize;
      let chars = common::generate_random_string(&mut rng, num_chars, radix as u32);
      let mut acc1 = FromStringAccumulator::new(K_MAX_DIGITS);
      let mut acc2 = FromStringAccumulator::new(K_MAX_DIGITS);
      acc1.parse(&chars, radix);
      acc2.parse(&chars, radix);

      let mut res_large = vec![0; acc1.result_length() as usize];
      let mut res_classic = vec![0; acc2.result_length() as usize];
      proc.parse_large(&mut res_large, &mut acc1);
      proc.parse_classic(&mut res_classic, &mut acc2);
      assert_equals(&res_large, &res_classic, &res_classic, &res_large);
    }
  }

  OK
}

#[test]
fn test_extreme_asymmetric_division_and_patterns() -> Void {
  let mut proc = processor();

  // 1. 全 1 (Digit::MAX)
  let a_max = vec![Digit::MAX; 10];
  let b_single = vec![Digit::MAX; 1];
  let mut q_single = vec![0; 10];
  let (_, r_single) = divide_single(&mut q_single, &a_max, b_single[0]);
  let mut reconstructed = vec![0; 12];
  let q_single_norm = norm(&q_single);
  let ql = multiply_result_length(&b_single, q_single_norm) as usize;
  let mut prod = vec![0; ql];
  multiply_schoolbook(&mut prod, q_single_norm, &b_single);
  reconstructed[..prod.len()].copy_from_slice(&prod);
  add_and_return_overflow(&mut reconstructed, &[r_single]);
  assert_eq!(compare(norm(&a_max), norm(&reconstructed)), 0);

  // 2. 高度不对称操作数: 100 digits 除以 1 / 2 digits
  let mut rng = Rng::new(777888999);
  let a_huge = generate_random(&mut rng, 100);
  for b_size in [1usize, 2, 3] {
    let b = generate_random(&mut rng, b_size);
    let ql = divide_result_length(&a_huge, &b) as usize;
    let mut q = vec![0; ql];
    let mut r = vec![0; b_size];
    let (done, _) = divide_small(&mut q, &a_huge, &b);
    if !done {
      proc.divide_schoolbook(&mut q, &mut r, &a_huge, &b);
    } else if b.len() == 1 {
      r[0] = mod_single(&a_huge, b[0]);
    } else {
      modulo_small(&mut r, &a_huge, &b);
    }

    let q_norm = norm(&q);
    let b_norm = norm(&b);
    let mut reconstructed = vec![0; a_huge.len() + 2];
    if !q_norm.is_empty() {
      let (big, small) = if b_norm.len() >= q_norm.len() {
        (b_norm, q_norm)
      } else {
        (q_norm, b_norm)
      };
      let prod_len = multiply_result_length(big, small) as usize;
      let mut prod = vec![0; prod_len];
      multiply_schoolbook(&mut prod, big, small);
      reconstructed[..prod.len()].copy_from_slice(&prod);
    }
    let r_norm = norm(&r);
    add_and_return_overflow(&mut reconstructed, r_norm);
    assert_eq!(compare(norm(&a_huge), norm(&reconstructed)), 0);
    assert!(compare(r_norm, b_norm) < 0);
  }

  // 3. 交替 bit 模式 0x5555 / 0xAAAA (b_size >= 2)
  let a_alt = vec![0x5555_5555_5555_5555u64; 20];
  let b_alt = vec![0xAAAA_AAAA_AAAA_AAAAu64; 10];
  let ql = divide_result_length(&a_alt, &b_alt) as usize;
  let mut q = vec![0; ql];
  let mut r = vec![0; 10];
  proc.divide_schoolbook(&mut q, &mut r, &a_alt, &b_alt);

  let q_norm = norm(&q);
  let b_norm = norm(&b_alt);
  let mut reconstructed = vec![0; a_alt.len() + 2];
  if !q_norm.is_empty() {
    let (big, small) = if b_norm.len() >= q_norm.len() {
      (b_norm, q_norm)
    } else {
      (q_norm, b_norm)
    };
    let prod_len = multiply_result_length(big, small) as usize;
    let mut prod = vec![0; prod_len];
    multiply_schoolbook(&mut prod, big, small);
    reconstructed[..prod.len()].copy_from_slice(&prod);
  }
  let r_norm = norm(&r);
  add_and_return_overflow(&mut reconstructed, r_norm);
  assert_eq!(compare(norm(&a_alt), norm(&reconstructed)), 0);
  assert!(compare(r_norm, b_norm) < 0);

  OK
}

#[test]
fn test_roundtrip_all_bases() -> Void {
  let mut rng = Rng::new(42424242);
  let mut proc = processor();
  const K_MAX_DIGITS: u32 = 1 << 20;

  for &size in &[1usize, 2, 10, 24] {
    let x = generate_random(&mut rng, size);
    for radix in 2..=36u32 {
      let chars_req = to_string_result_length(&x, radix, false) as usize;
      let mut str_buf = vec![0u8; chars_req];
      let str_len = proc.write_impl(&mut str_buf, &x, radix, false, true);
      let slice = &str_buf[..str_len as usize];

      let mut acc = FromStringAccumulator::new(K_MAX_DIGITS);
      acc.parse(slice, radix as u64);
      let mut parsed = vec![0; acc.result_length() as usize];
      let status = proc.parse_string(&mut parsed, &mut acc);
      assert_eq!(status, Status::Ok);
      assert_eq!(
        compare(norm(&x), norm(&parsed)),
        0,
        "roundtrip failure for size {size} radix {radix} hex: {}",
        format_hex(&x)
      );
    }
  }

  OK
}
