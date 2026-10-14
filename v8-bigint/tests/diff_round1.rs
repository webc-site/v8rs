// round1 对抗审查差分抽查 (常驻回归): 固定 seed RNG + 独立 oracle 交叉验证
//! oracle 1: u128/i128 原生算术 (≤128 位操作数)
//! oracle 2: 手工 32 位 limb schoolbook 乘法 (与 crate 的 64 位 digit 实现不同代码路径)
//! 覆盖: 乘法全算法族 / 除法全算法族 / 移位 / asIntN·asUintN / 位运算 9 变体 / 字符串往返

mod common;

use aok::{OK, Void};
use common::{Rng, format_hex, generate_random, processor};
use v8_bigint::{
  bigint::{
    Digit, RightShiftState, Status, add, as_int_n, as_uint_n_neg, as_uint_n_pos,
    as_uint_n_pos_result_length, bitwise_and_neg_neg, bitwise_and_pos_neg, bitwise_and_pos_pos,
    bitwise_or_neg_neg, bitwise_or_pos_neg, bitwise_or_pos_pos, bitwise_xor_neg_neg,
    bitwise_xor_pos_neg, bitwise_xor_pos_pos, compare, divide_result_length, divide_single,
    divide_small, left_shift, mod_single, modulo_small, multiply_result_length, multiply_small,
    norm, right_shift, right_shift_result_length, subtract,
  },
  from_string::FromStringAccumulator,
  to_string::to_string_result_length,
};

// ---------------------------------------------------------------------------
// oracle: u128 原生算术
// ---------------------------------------------------------------------------

fn to_u128(x: &[Digit]) -> u128 {
  let mut v = 0u128;
  for (i, &d) in x.iter().enumerate().take(2) {
    v |= (d as u128) << (64 * i);
  }
  v
}

fn from_u128(v: u128) -> Vec<Digit> {
  vec![v as u64, (v >> 64) as u64]
}

// ---------------------------------------------------------------------------
// oracle: 32 位 limb 暴力乘法 + 大数辅助
// ---------------------------------------------------------------------------

fn to_u32_limbs(x: &[Digit]) -> Vec<u32> {
  let mut v = Vec::with_capacity(x.len() * 2);
  for &d in x {
    v.push(d as u32);
    v.push((d >> 32) as u32);
  }
  while v.last() == Some(&0) {
    v.pop();
  }
  v
}

fn from_u32_limbs(v: &[u32]) -> Vec<Digit> {
  let mut out = vec![0u64; v.len().div_ceil(2)];
  for (i, &l) in v.iter().enumerate() {
    out[i / 2] |= (l as u64) << (32 * (i % 2));
  }
  out
}

/// 暴力 schoolbook 乘法 (O(n²), u128 累加, 32 位 limb)
fn oracle_mul(a: &[Digit], b: &[Digit]) -> Vec<Digit> {
  let a32 = to_u32_limbs(a);
  let b32 = to_u32_limbs(b);
  if a32.is_empty() || b32.is_empty() {
    return vec![0];
  }
  let mut r32 = vec![0u32; a32.len() + b32.len()];
  for i in 0..a32.len() {
    let mut carry = 0u128;
    for j in 0..b32.len() {
      let cur = r32[i + j] as u128 + (a32[i] as u128) * (b32[j] as u128) + carry;
      r32[i + j] = cur as u32;
      carry = cur >> 32;
    }
    let mut k = i + b32.len();
    while carry > 0 {
      let cur = r32[k] as u128 + carry;
      r32[k] = cur as u32;
      carry = cur >> 32;
      k += 1;
    }
  }
  from_u32_limbs(&r32)
}

fn big_add(a: &[Digit], b: &[Digit]) -> Vec<Digit> {
  let mut out = vec![0u64; a.len().max(b.len()) + 1];
  add(&mut out, a, b);
  out
}

fn big_sub(a: &[Digit], b: &[Digit]) -> Vec<Digit> {
  // 前置: a >= b
  let mut out = vec![0u64; a.len().max(b.len())];
  subtract(&mut out, a, b);
  out
}

/// 独立 oracle: ceil(a / 2^s) = (a + 2^s - 1) >> s, 纯 limb 运算
fn oracle_ceil_shift(a: &[Digit], s: u64) -> Vec<Digit> {
  let digit_shift = (s / 64) as usize;
  let bits = (s % 64) as u32;
  let mut t = a.to_vec();
  t.push(0);
  // t += 2^s - 1
  let mut carry = 0u128;
  for (i, d) in t.iter_mut().enumerate() {
    let addend: u64 = if i < digit_shift {
      u64::MAX
    } else if i == digit_shift && bits > 0 {
      (1u64 << bits) - 1
    } else {
      0
    };
    let sum = (*d as u128) + addend as u128 + (carry & 1);
    *d = sum as u64;
    carry = sum >> 64;
  }
  // t >>= s
  let out_len = t.len().saturating_sub(digit_shift).max(1);
  let mut out = vec![0u64; out_len];
  for (i, o) in out.iter_mut().enumerate() {
    let lo = t.get(i + digit_shift).copied().unwrap_or(0);
    let hi = t.get(i + digit_shift + 1).copied().unwrap_or(0);
    *o = if bits == 0 {
      lo
    } else {
      (lo >> bits) | (hi << (64 - bits))
    };
  }
  out
}

fn assert_big_eq(a: &[Digit], b: &[Digit], ctx: &str) {
  assert_eq!(
    compare(norm(a), norm(b)),
    0,
    "{ctx}\na: {}\nb: {}",
    format_hex(a),
    format_hex(b)
  );
}

/// 2^n 的 digit 向量
fn power_of_two(n: u64) -> Vec<Digit> {
  let mut v = vec![0u64; (n / 64) as usize + 1];
  v[(n / 64) as usize] = 1u64 << (n % 64);
  v
}

// ---------------------------------------------------------------------------
// 边界值清单
// ---------------------------------------------------------------------------

fn edge_values() -> Vec<Vec<Digit>> {
  vec![
    vec![0],
    vec![1],
    vec![2],
    vec![3],
    vec![Digit::MAX],
    vec![0, 1],
    vec![Digit::MAX, Digit::MAX],
    vec![Digit::MAX, 0],
    vec![1, 1],
    vec![0, 0, 1],
    vec![Digit::MAX, Digit::MAX, Digit::MAX],
  ]
}

#[test]
fn diff_u128_arithmetic() -> Void {
  let mut rng = Rng::new(20261011);
  let mut proc = processor();
  for _ in 0..30000 {
    let len_a = 1 + (rng.next_u64() % 2) as usize;
    let len_b = 1 + (rng.next_u64() % 2) as usize;
    let mut a = generate_random(&mut rng, len_a);
    let mut b = generate_random(&mut rng, len_b);
    if len_a == 2 && rng.next_u64() & 1 == 1 {
      a[0] &= u32::MAX as u64;
    }
    if len_b == 2 && rng.next_u64() & 1 == 1 {
      b[0] &= u32::MAX as u64;
    }
    let (va, vb) = (to_u128(&a), to_u128(&b));
    // multiply_large 契约要求 |x| >= |y| (原版 DCHECK 同)
    let (big, small) = if norm(&a).len() >= norm(&b).len() {
      (&a, &b)
    } else {
      (&b, &a)
    };

    // 加减 (oracle: u64 半字独立求和, 不经 u128 截断)
    let mut sum = vec![0u64; 5];
    add(&mut sum, &a, &b);
    let (lo, c1) = {
      let r = (va as u64).overflowing_add(vb as u64);
      (r.0, r.1 as u128)
    };
    let hi = (va >> 64) + (vb >> 64) + c1;
    let expect_sum = vec![lo, hi as u64, (hi >> 64) as u64];
    assert_big_eq(&sum, &expect_sum, "add");
    if va >= vb {
      let mut diff = vec![0u64; 4];
      subtract(&mut diff, &a, &b);
      assert_big_eq(&diff, &from_u128(va - vb), "sub");
    }

    // 乘法 (统一调度入口: 小输入 multiply_small, 大输入 multiply_large)
    let mut prod = vec![0u64; multiply_result_length(&a, &b) as usize];
    let (done, _) = multiply_small(&mut prod, &a, &b);
    if !done {
      proc.multiply_large(&mut prod, big, small);
    }
    assert_big_eq(&prod, &oracle_mul(&a, &b), "mul");

    // 除法/取模
    if let Some(vq) = va.checked_div(vb) {
      let vr = va.checked_rem(vb).unwrap_or(0);
      let mut q = vec![0u64; 4];
      let (done, _) = divide_small(&mut q, &a, &b);
      if done {
        assert_big_eq(&q, &from_u128(vq), "div_small");
      }
      let mut r = vec![0u64; 4];
      let (done, _) = modulo_small(&mut r, &a, &b);
      if done {
        assert_big_eq(&r, &from_u128(vr), "mod_small");
      }
      if b.len() == 1 && b[0] != 0 {
        let mut q1 = vec![0u64; 4];
        let (_, rem) = divide_single(&mut q1, &a, b[0]);
        assert_big_eq(&q1, &from_u128(vq), "div_single");
        assert_eq!(rem, vr as u64, "rem_single");
        assert_eq!(mod_single(&a, b[0]), vr as u64, "mod_single");
      }
    }

    // 移位 (正数): 左移 oracle 用乘法 (u128 位移在 128 位外会截断)
    for shift in [0u64, 1, 31, 63, 64, 65, 100, 127, 128] {
      let mut ls = vec![0u64; 6];
      left_shift(&mut ls, &a, shift);
      assert_big_eq(&ls, &oracle_mul(&a, &power_of_two(shift)), "lshift");

      if shift < 128 && va >> shift != 0 || shift == 0 {
        let mut state = RightShiftState::default();
        let rl = right_shift_result_length(&a, false, shift, Some(&mut state));
        if rl > 0 {
          let mut rs = vec![0u64; rl as usize];
          right_shift(&mut rs, &a, shift, &state);
          assert_big_eq(&rs, &from_u128(va >> shift), "rshift");
          assert!(!state.must_round_down);
        }
      }
    }

    // asIntN / asUintN
    for n in [1u32, 2, 31, 63, 64, 65, 127, 128] {
      let wrapped = ((va as i128) << (128 - n)) >> (128 - n);
      let needed = n.div_ceil(64) as usize;
      if a.len() >= needed {
        let mut z = vec![0u64; needed + 1];
        let neg = as_int_n(&mut z, &a, false, n);
        assert_eq!(neg, wrapped < 0, "as_int_n sign n={n} v={va}");
        assert_big_eq(&z, &from_u128(wrapped.unsigned_abs()), "as_int_n mag");
      }
      let low = if n >= 128 {
        va
      } else {
        va & ((1u128 << n) - 1)
      };
      if as_uint_n_pos_result_length(&a, n) > 0 {
        let mut z = vec![0u64; needed];
        as_uint_n_pos(&mut z, &a, n);
        assert_big_eq(&z, &from_u128(low), "as_uint_n_pos");
      }
      // 负输入: 2^n - low (mod 2^n)
      let mut z = vec![0u64; needed + 1];
      as_uint_n_neg(&mut z, &a, n);
      let m = if n >= 128 {
        u128::MAX
      } else {
        (1u128 << n) - 1
      };
      let expect = m.wrapping_sub(low).wrapping_add(1) & m;
      assert_big_eq(&z, &from_u128(expect), "as_uint_n_neg");
    }
  }
  OK
}

#[test]
fn diff_edge_values_sign_combos() -> Void {
  let vals = edge_values();
  // 1) 位运算 9 变体 vs i128 oracle
  for a in &vals {
    if a.len() > 2 {
      continue;
    }
    for b in &vals {
      if b.len() > 2 {
        continue;
      }
      let (va, vb) = (to_u128(a), to_u128(b));
      // i128 符号 oracle 只能表示 < 2^127 的幅度 (取负不得回绕)
      if va >= (1u128 << 127) || vb >= (1u128 << 127) {
        continue;
      }
      let (va, vb) = (va as i128, vb as i128);
      let max_len = a.len().max(b.len()) + 1;
      for &(na, nb) in &[(false, false), (true, true), (false, true), (true, false)] {
        // 负数幅度不得为 0 (非规范表示, 原版 DCHECK 同样会炸)
        if (na && norm(a).is_empty()) || (nb && norm(b).is_empty()) {
          continue;
        }
        let ea = if na { -va } else { va };
        let eb = if nb { -vb } else { vb };
        let cases: [(&str, i128, u8); 3] =
          [("and", ea & eb, 0), ("or", ea | eb, 1), ("xor", ea ^ eb, 2)];
        for (name, expect, op) in cases {
          let expect_neg = expect < 0;
          let expect_mag = expect.unsigned_abs();
          let mut z = vec![0u64; max_len + 1];
          // 返回值约定: 结果恒负 (除 xor 的 neg_neg), 0 结果按正零处理
          match (op, na, nb) {
            (0, false, false) | (1, false, false) | (2, false, false) => {
              match op {
                0 => bitwise_and_pos_pos(&mut z, a, b),
                1 => bitwise_or_pos_pos(&mut z, a, b),
                _ => bitwise_xor_pos_pos(&mut z, a, b),
              };
            }
            (0, true, true) => bitwise_and_neg_neg(&mut z, a, b),
            (1, true, true) => bitwise_or_neg_neg(&mut z, a, b),
            (2, true, true) => bitwise_xor_neg_neg(&mut z, a, b),
            (_, false, true) => match op {
              0 => bitwise_and_pos_neg(&mut z, a, b),
              1 => bitwise_or_pos_neg(&mut z, a, b),
              _ => bitwise_xor_pos_neg(&mut z, a, b),
            },
            (_, true, false) => match op {
              0 => bitwise_and_pos_neg(&mut z, b, a),
              1 => bitwise_or_pos_neg(&mut z, b, a),
              _ => bitwise_xor_pos_neg(&mut z, b, a),
            },
            _ => unreachable!(),
          }
          if expect_mag != 0 {
            // 非零结果符号按运算类型硬性判定 (and_pos_neg 符号不定, 跳过)
            let implied_neg = match (op, na, nb) {
              (_, false, false) => Some(false),
              (2, true, true) => Some(false),
              (0, false, true) | (0, true, false) => None,
              _ => Some(true),
            };
            if let Some(implied_neg) = implied_neg {
              assert_eq!(
                implied_neg,
                expect_neg,
                "{name} sign a={} b={}",
                format_hex(a),
                format_hex(b)
              );
            }
          }
          assert_big_eq(&z, &from_u128(expect_mag), name);
        }
      }
    }
  }

  // 2) 负数右移向下取整: floor(-a / 2^s) == -ceil(a / 2^s)
  for a in &vals {
    let a_nonzero = !norm(a).is_empty();
    for shift in [0u64, 1, 31, 63, 64, 65, 127, 128, 129] {
      let digit_shift = (shift / 64) as usize;
      let mut state = RightShiftState::default();
      let rl = right_shift_result_length(a, true, shift, Some(&mut state));
      // 期望 must_round_down: 有任何被移出的非零位
      let expect_round = a_nonzero && a.len() > digit_shift && {
        let bits = (shift % 64) as u32;
        let mask = if bits == 0 { 0 } else { (1u64 << bits) - 1 };
        (a[digit_shift] & mask) != 0 || a[..digit_shift].iter().any(|&d| d != 0)
      };
      if a.len() > digit_shift && rl > 0 {
        assert_eq!(
          state.must_round_down,
          expect_round,
          "round_down a={} s={shift}",
          format_hex(a)
        );
        let mut z = vec![0u64; rl as usize];
        right_shift(&mut z, a, shift, &state);
        // oracle: ceil(a / 2^s) = (a + 2^s - 1) >> s (limb 独立实现)
        let expect = oracle_ceil_shift(a, shift);
        assert_big_eq(
          &z,
          &expect,
          &format!("neg rshift a={} s={shift}", format_hex(a)),
        );
      }
    }
  }
  OK
}

#[test]
fn diff_big_multiplication_all_algorithms() -> Void {
  let mut rng = Rng::new(31415926);
  let mut proc = processor();
  // 尺寸对: 覆盖 schoolbook/karatsuba/toom/fft 阈值两侧 + 非对称分块
  let sizes: &[(usize, usize)] = &[
    (1, 1),
    (2, 1),
    (33, 34),
    (34, 34),
    (100, 57),
    (150, 150),
    (209, 100),
    (210, 210),
    (500, 250),
    (719, 400),
    (720, 720),
    (730, 730),
    (900, 3),
    (1100, 700),
    (1500, 1500),
    (2100, 900),
  ];
  for &(la, lb) in sizes {
    let (la, lb) = if la >= lb { (la, lb) } else { (lb, la) };
    let a = generate_random(&mut rng, la);
    let b = generate_random(&mut rng, lb);
    let rl = multiply_result_length(&a, &b) as usize;
    let mut got = vec![0u64; rl];
    let (done, _) = multiply_small(&mut got, &a, &b);
    if !done {
      proc.multiply_large(&mut got, &a, &b);
    }
    let expect = oracle_mul(&a, &b);
    assert_big_eq(&got, &expect, "big mul {la}x{lb}");
  }
  // 平方检测路径 (同一缓冲指针)
  for la in [730usize, 1500] {
    let a = generate_random(&mut rng, la);
    let rl = multiply_result_length(&a, &a) as usize;
    let mut got = vec![0u64; rl];
    proc.multiply_large(&mut got, &a, &a);
    let expect = oracle_mul(&a, &a);
    assert_big_eq(&got, &expect, "big square {la}");
  }
  OK
}

#[test]
fn diff_big_division_all_algorithms() -> Void {
  let mut rng = Rng::new(27182818);
  let mut proc = processor();
  // (a_len, b_len): 覆盖 divide_small / schoolbook / burnikel
  let sizes: &[(usize, usize)] = &[
    (2, 1),
    (5, 2),
    (57, 56),
    (58, 57),
    (100, 57),
    (114, 57),
    (200, 60),
    (300, 150),
    (600, 120),
    (1000, 3),
    (1300, 650),
  ];
  for &(la, lb) in sizes {
    for _ in 0..3 {
      let a = generate_random(&mut rng, la.max(lb + 1));
      let b = generate_random(&mut rng, lb);
      let ql = divide_result_length(&a, &b) as usize + 1;
      let mut q = vec![0u64; ql];
      let mut r = vec![0u64; lb];
      if norm(&b).len() == 1 {
        // divide_large 契约: B 多 digit (原版 DCHECK 同); 单 digit 走 small 路径
        let (done, _) = divide_small(&mut q, &a, &b);
        assert!(done);
        r[0] = mod_single(&a, b[0]);
      } else {
        proc.divide_schoolbook(&mut q, &mut r, &a, &b);
      }
      // 不变量: A == B*Q + R, R < B (oracle 乘法重建)
      let q_norm = norm(&q);
      let b_prod = oracle_mul(&b, q_norm);
      let recon = big_add(&b_prod, &r);
      assert_big_eq(&recon, &a, "div invariant {la}/{lb}");
      assert!(
        compare(norm(&r), norm(&b)) < 0,
        "remainder too big {la}/{lb}"
      );
      // divide_large 的商再对拍 (其契约只产 Q, 余数由调用方另取)
      if b.len() >= 2 {
        let mut q2 = vec![0u64; ql];
        proc.divide_large(&mut q2, &a, &b);
        assert_big_eq(&q2, &q, "div_large q {la}/{lb}");
      }
    }
  }
  // Barrett 核心直测 (to_string 快路径同款入口)
  for &(la, lb) in &[(130usize, 65), (260, 70), (600, 200)] {
    let a = generate_random(&mut rng, la);
    let b = generate_random(&mut rng, lb);
    let ql = divide_result_length(&a, &b) as usize + 1;
    let mut q = vec![0u64; ql];
    let mut r = vec![0u64; lb];
    proc.divide_barrett(&mut q, &mut r, &a, &b);
    let b_prod = oracle_mul(&b, norm(&q));
    let recon = big_add(&b_prod, &r);
    assert_big_eq(&recon, &a, "barrett invariant {la}/{lb}");
    assert!(compare(norm(&r), norm(&b)) < 0);
  }
  OK
}

#[test]
fn diff_big_shift_roundtrip() -> Void {
  let mut rng = Rng::new(16180339);
  let mut proc = processor();
  for la in [1usize, 3, 17, 80, 300] {
    let a = generate_random(&mut rng, la);
    for shift in [0u64, 1, 63, 64, 127, 130, 400, 64 * (la as u64 + 2)] {
      // 左移 == 乘 2^shift (oracle)
      let mut ls = vec![0u64; la + (shift / 64) as usize + 3];
      left_shift(&mut ls, &a, shift);
      let mut expect = vec![0u64; la + (shift / 64) as usize + 2];
      expect[(shift / 64) as usize..(shift / 64) as usize + la].copy_from_slice(&a);
      if shift % 64 != 0 {
        expect = oracle_mul(&expect, &[1u64 << (shift % 64)]);
      }
      assert_big_eq(&ls, &expect, "big lshift {la} {shift}");

      // 右移 (正) == floor 除 2^shift (schoolbook 交叉)
      if la as u64 * 64 > shift {
        let mut state = RightShiftState::default();
        let rl = right_shift_result_length(&a, false, shift, Some(&mut state));
        let mut rs = vec![0u64; rl as usize + 2];
        right_shift(&mut rs, &a, shift, &state);
        let divisor = power_of_two(shift);
        let ql = divide_result_length(&a, &divisor) as usize + 1;
        let mut q = vec![0u64; ql];
        let mut r = vec![0u64; divisor.len()];
        if divisor.len() == 1 {
          let (_, rem) = divide_single(&mut q, &a, divisor[0]);
          r[0] = rem;
        } else {
          proc.divide_schoolbook(&mut q, &mut r, &a, &divisor);
        }
        // a == divisor*q + r 且 r < divisor
        let recon = big_add(&oracle_mul(&divisor, norm(&q)), &r);
        assert_big_eq(&recon, &a, "shift-div invariant");
        assert!(compare(norm(&r), &divisor) < 0);
        assert_big_eq(&rs, norm(&q), "big rshift {la} {shift}");
      }
    }
  }
  OK
}

#[test]
fn diff_string_roundtrip_all_radixes() -> Void {
  const K_MAX_DIGITS: u32 = 1 << 22;
  let mut rng = Rng::new(606060);
  let mut proc = processor();
  for la in [1usize, 2, 10, 24, 26, 60, 200, 800] {
    let x = generate_random(&mut rng, la);
    for radix in 2..=36u32 {
      let chars_req = to_string_result_length(&x, radix, false) as usize;
      let mut fast_buf = vec![0u8; chars_req];
      let mut slow_buf = vec![0u8; chars_req];
      let n_fast = proc.write_impl(&mut fast_buf, &x, radix, false, true);
      let n_slow = proc.write_impl(&mut slow_buf, &x, radix, false, false);
      assert_eq!(n_fast, n_slow, "len mismatch la={la} radix={radix}");
      assert_eq!(
        &fast_buf[..n_fast as usize],
        &slow_buf[..n_slow as usize],
        "bytes mismatch la={la} radix={radix} x={}",
        format_hex(&x)
      );
      // 往返: parse 回来必须相等
      let mut acc = FromStringAccumulator::new(K_MAX_DIGITS);
      acc.parse(&fast_buf[..n_fast as usize], radix as u64);
      let mut back = vec![0u64; acc.result_length() as usize + 1];
      let status = proc.parse_string(&mut back, &mut acc);
      assert_eq!(status, Status::Ok);
      assert_big_eq(&back, &x, "roundtrip la={la} radix={radix}");
    }
  }
  OK
}

#[test]
fn diff_fromstring_large_vs_classic_random() -> Void {
  const K_MAX_DIGITS: u32 = 1 << 22;
  let mut rng = Rng::new(777001);
  let mut proc = processor();
  // 非零 radix 数字串: classic / large 双路交叉 + 写回再解析二次一致
  for size in [30usize, 40, 60, 100, 200, 500] {
    for radix in [3u32, 7, 10, 17, 29, 36] {
      let num_chars = (size as f64 * 64.0 / (radix as f64).log2()) as usize + 3;
      let chars = common::generate_random_string(&mut rng, num_chars, radix);
      let mut acc_c = FromStringAccumulator::new(K_MAX_DIGITS);
      let mut acc_l = FromStringAccumulator::new(K_MAX_DIGITS);
      acc_c.parse(&chars, radix as u64);
      acc_l.parse(&chars, radix as u64);
      assert_eq!(acc_c.result_length(), acc_l.result_length());
      let mut via_classic = vec![0u64; acc_c.result_length() as usize + 1];
      let mut via_large = vec![0u64; acc_l.result_length() as usize + 1];
      proc.parse_classic(&mut via_classic, &mut acc_c);
      proc.parse_large(&mut via_large, &mut acc_l);
      assert_big_eq(&via_classic, &via_large, "parse {size} radix={radix}");
      // 写回再解析 (二次一致性)
      let chars_req = to_string_result_length(&via_classic, radix, false) as usize;
      let mut buf = vec![0u8; chars_req];
      let n = proc.write_impl(&mut buf, &via_classic, radix, false, true);
      let mut acc2 = FromStringAccumulator::new(K_MAX_DIGITS);
      acc2.parse(&buf[..n as usize], radix as u64);
      let mut back = vec![0u64; acc2.result_length() as usize + 1];
      proc.parse_string(&mut back, &mut acc2);
      assert_big_eq(&back, &via_classic, "reparse {size} radix={radix}");
    }
  }
  OK
}

#[test]
fn diff_asintn_wide() -> Void {
  // 大位宽 asIntN (i128 之外): oracle = a mod 2^n (schoolbook 除法) + 第 n-1 位判断
  let mut rng = Rng::new(998877);
  let mut proc = processor();
  for la in [3usize, 5, 9] {
    for n in [130u32, 191, 192, 256, 383, 576] {
      let needed = n.div_ceil(64) as usize;
      if la < needed {
        continue;
      }
      let a = generate_random(&mut rng, la);
      let divisor = power_of_two(n as u64);
      let ql = divide_result_length(&a, &divisor) as usize + 1;
      let mut q = vec![0u64; ql];
      let mut r = vec![0u64; divisor.len()];
      if compare(norm(&a), norm(&divisor)) >= 0 {
        proc.divide_schoolbook(&mut q, &mut r, &a, &divisor);
      } else {
        r[..a.len()].copy_from_slice(&a);
      }
      let low = norm(&r).to_vec();
      // 第 n-1 位
      let bit_idx = (n - 1) as usize;
      let bit_set = low.len() > bit_idx / 64 && (low[bit_idx / 64] >> (bit_idx % 64)) & 1 == 1;
      let mut z = vec![0u64; needed + 1];
      let neg = as_int_n(&mut z, &a, false, n);
      assert_eq!(neg, bit_set, "asintn wide sign n={n} a={}", format_hex(&a));
      if bit_set {
        // |v| = 2^n - low
        let expect = big_sub(&divisor, &low);
        assert_big_eq(&z, &expect, "asintn wide mag n={n}");
      } else {
        assert_big_eq(&z, &low, "asintn wide mag n={n}");
      }
    }
  }
  OK
}
