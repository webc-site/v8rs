//! v8-base/src/bits.rs 集成测试 — 1:1 对应 bits-unittest.cc (26 用例)
//! TRACED_FORRANGE 展开为等价 for 循环; CHECK/EXPECT 统一 assert!/assert_eq!

use aok::{OK, Void};
use v8_base::bits::{
  BitOps, BitOpsUnsigned, count_leading_zeros32, count_leading_zeros64, count_trailing_zeros32,
  count_trailing_zeros64, rotate_right32, rotate_right64, signed_add_overflow32, signed_div32,
  signed_mod32, signed_mul_high_and_add32, signed_mul_high32, signed_sub_overflow32,
  unsigned_add_overflow32, unsigned_div32, unsigned_mod32, which_power_of_two,
};

/// BitOps::is_power_of_two 泛型入口 (原版模板函数; 避免落入 std 内建同名方法)
fn is_power_of_two<T: BitOps>(v: T) -> bool {
  v.is_power_of_two()
}

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn bits_count_population8() -> Void {
  assert_eq!(0, 0u8.count_population());
  assert_eq!(1, 1u8.count_population());
  assert_eq!(2, 0x11u8.count_population());
  assert_eq!(4, 0x0Fu8.count_population());
  assert_eq!(6, 0x3Fu8.count_population());
  assert_eq!(8, 0xFFu8.count_population());
  OK
}

#[test]
fn bits_count_population16() -> Void {
  assert_eq!(0, 0u16.count_population());
  assert_eq!(1, 1u16.count_population());
  assert_eq!(4, 0x1111u16.count_population());
  assert_eq!(8, 0xF0F0u16.count_population());
  assert_eq!(12, 0xF0FFu16.count_population());
  assert_eq!(16, 0xFFFFu16.count_population());
  OK
}

#[test]
fn bits_count_population32() -> Void {
  assert_eq!(0, 0u32.count_population());
  assert_eq!(1, 1u32.count_population());
  assert_eq!(8, 0x1111_1111u32.count_population());
  assert_eq!(16, 0xF0F0_F0F0u32.count_population());
  assert_eq!(24, 0xFFF0_F0FFu32.count_population());
  assert_eq!(32, 0xFFFF_FFFFu32.count_population());
  OK
}

#[test]
fn bits_count_population64() -> Void {
  assert_eq!(0, 0u64.count_population());
  assert_eq!(1, 1u64.count_population());
  assert_eq!(2, 0x8000_0000_0000_0001u64.count_population());
  assert_eq!(8, 0x1111_1111u64.count_population());
  assert_eq!(16, 0xF0F0_F0F0u64.count_population());
  assert_eq!(24, 0xFFF0_F0FFu64.count_population());
  assert_eq!(32, 0xFFFF_FFFFu64.count_population());
  assert_eq!(16, 0x1111_1111_1111_1111u64.count_population());
  assert_eq!(32, 0xF0F0_F0F0_F0F0_F0F0u64.count_population());
  assert_eq!(48, 0xFFF0_F0FF_FFF0_F0FFu64.count_population());
  assert_eq!(64, 0xFFFF_FFFF_FFFF_FFFFu64.count_population());
  OK
}

#[test]
fn bits_count_leading_zeros16() -> Void {
  assert_eq!(16, 0u16.count_leading_zeros());
  assert_eq!(15, 1u16.count_leading_zeros());
  for shift in 0..16u32 {
    assert_eq!(15 - shift, (1u16 << shift).count_leading_zeros());
  }
  assert_eq!(4, 0x0F0Fu16.count_leading_zeros());
  OK
}

#[test]
fn bits_count_leading_zeros32() -> Void {
  assert_eq!(32, 0u32.count_leading_zeros());
  assert_eq!(31, 1u32.count_leading_zeros());
  for shift in 0..32u32 {
    assert_eq!(31 - shift, (1u32 << shift).count_leading_zeros());
  }
  assert_eq!(4, 0x0F0F_0F0Fu32.count_leading_zeros());
  OK
}

#[test]
fn bits_count_leading_zeros64() -> Void {
  assert_eq!(64, 0u64.count_leading_zeros());
  assert_eq!(63, 1u64.count_leading_zeros());
  for shift in 0..64u32 {
    assert_eq!(63 - shift, (1u64 << shift).count_leading_zeros());
  }
  assert_eq!(36, 0x0F0F_0F0Fu64.count_leading_zeros());
  assert_eq!(4, 0x0F0F_0F0F_0000_0000u64.count_leading_zeros());
  // 固定宽度入口 (CountLeadingZeros32/64)
  assert_eq!(32, count_leading_zeros32(0));
  assert_eq!(31, count_leading_zeros32(1));
  assert_eq!(64, count_leading_zeros64(0));
  assert_eq!(63, count_leading_zeros64(1));
  OK
}

#[test]
fn bits_count_trailing_zeros16() -> Void {
  assert_eq!(16, 0u16.count_trailing_zeros());
  assert_eq!(15, 0x8000u16.count_trailing_zeros());
  for shift in 0..16u32 {
    assert_eq!(shift, (1u16 << shift).count_trailing_zeros());
  }
  assert_eq!(4, 0xF0F0u16.count_trailing_zeros());
  OK
}

#[test]
fn bits_count_trailing_zerosu32() -> Void {
  assert_eq!(32, 0u32.count_trailing_zeros());
  assert_eq!(31, 0x8000_0000u32.count_trailing_zeros());
  for shift in 0..32u32 {
    assert_eq!(shift, (1u32 << shift).count_trailing_zeros());
  }
  assert_eq!(4, 0xF0F0_F0F0u32.count_trailing_zeros());
  assert_eq!(32, count_trailing_zeros32(0));
  assert_eq!(4, count_trailing_zeros32(0xF0F0_F0F0));
  OK
}

#[test]
fn bits_count_trailing_zerosi32() -> Void {
  assert_eq!(32, 0i32.count_trailing_zeros());
  for shift in 0..32u32 {
    assert_eq!(shift, (1i32 << shift).count_trailing_zeros());
  }
  assert_eq!(4, (0x70F0_F0F0u32 as i32).count_trailing_zeros());
  assert_eq!(2, (-4i32).count_trailing_zeros());
  assert_eq!(0, (-1i32).count_trailing_zeros());
  OK
}

#[test]
fn bits_count_trailing_zeros64() -> Void {
  assert_eq!(64, 0u64.count_trailing_zeros());
  assert_eq!(63, 0x8000_0000_0000_0000u64.count_trailing_zeros());
  for shift in 0..64u32 {
    assert_eq!(shift, (1u64 << shift).count_trailing_zeros());
  }
  assert_eq!(4, 0xF0F0_F0F0u64.count_trailing_zeros());
  assert_eq!(36, 0xF0F0_F0F0_0000_0000u64.count_trailing_zeros());
  assert_eq!(64, count_trailing_zeros64(0));
  OK
}

#[test]
fn bits_is_power_of_two32() -> Void {
  assert!(!is_power_of_two(0u32));
  for shift in 0..32u32 {
    assert!(is_power_of_two(1u32 << shift));
    assert!(!is_power_of_two((1u32 << shift) + 5));
    assert!(!is_power_of_two(!(1u32 << shift)));
  }
  for shift in 2..32u32 {
    assert!(!is_power_of_two((1u32 << shift) - 1));
  }
  assert!(!is_power_of_two(0xFFFF_FFFFu32));
  OK
}

#[test]
fn bits_is_power_of_two64() -> Void {
  assert!(!is_power_of_two(0u64));
  for shift in 0..64u32 {
    assert!(is_power_of_two(1u64 << shift));
    assert!(!is_power_of_two((1u64 << shift) + 5));
    assert!(!is_power_of_two(!(1u64 << shift)));
  }
  for shift in 2..64u32 {
    assert!(!is_power_of_two((1u64 << shift) - 1));
  }
  assert!(!is_power_of_two(0xFFFF_FFFF_FFFF_FFFFu64));
  OK
}

#[test]
fn bits_which_power_of_two32() -> Void {
  // TRACED_FORRANGE(int, shift, 0, 30) — 有符号
  for shift in 0..=30i32 {
    assert_eq!(shift as u32, which_power_of_two(1i32 << shift));
  }
  for shift in 0..=31u32 {
    assert_eq!(shift, which_power_of_two(1u32 << shift));
  }
  // BitOpsUnsigned trait 入口
  assert_eq!(0, 1u32.which_power_of_two());
  assert_eq!(31, (1u32 << 31).which_power_of_two());
  OK
}

#[test]
fn bits_which_power_of_two64() -> Void {
  for shift in 0..=62i32 {
    assert_eq!(shift as u32, which_power_of_two(1i64 << shift));
  }
  for shift in 0..=63u32 {
    assert_eq!(shift, which_power_of_two(1u64 << shift));
  }
  OK
}

#[test]
fn bits_rotate_right32() -> Void {
  for shift in 0..32u32 {
    assert_eq!(0, rotate_right32(0, shift));
  }
  assert_eq!(1, rotate_right32(1, 0));
  assert_eq!(1, rotate_right32(2, 1));
  assert_eq!(0x8000_0000, rotate_right32(1, 1));
  OK
}

#[test]
fn bits_rotate_right64() -> Void {
  for shift in 0..64u32 {
    assert_eq!(0, rotate_right64(0, shift));
  }
  assert_eq!(1, rotate_right64(1, 0));
  assert_eq!(1, rotate_right64(2, 1));
  assert_eq!(0x8000_0000_0000_0000, rotate_right64(1, 1));
  OK
}

#[test]
fn bits_signed_add_overflow32() -> Void {
  let (ov, val) = signed_add_overflow32(0, 0);
  assert!(!ov);
  assert_eq!(0, val);
  let (ov, val) = signed_add_overflow32(i32::MAX, 1);
  assert!(ov);
  assert_eq!(i32::MIN, val);
  let (ov, val) = signed_add_overflow32(i32::MIN, -1);
  assert!(ov);
  assert_eq!(i32::MAX, val);
  let (ov, val) = signed_add_overflow32(i32::MAX, i32::MAX);
  assert!(ov);
  assert_eq!(-2, val);
  for i in 1..=50i32 {
    for j in 1..=i {
      let (ov, val) = signed_add_overflow32(i, j);
      assert!(!ov);
      assert_eq!(i + j, val);
    }
  }
  OK
}

#[test]
fn bits_signed_sub_overflow32() -> Void {
  let (ov, val) = signed_sub_overflow32(0, 0);
  assert!(!ov);
  assert_eq!(0, val);
  let (ov, val) = signed_sub_overflow32(i32::MIN, 1);
  assert!(ov);
  assert_eq!(i32::MAX, val);
  let (ov, val) = signed_sub_overflow32(i32::MAX, -1);
  assert!(ov);
  assert_eq!(i32::MIN, val);
  for i in 1..=50i32 {
    for j in 1..=i {
      let (ov, val) = signed_sub_overflow32(i, j);
      assert!(!ov);
      assert_eq!(i - j, val);
    }
  }
  OK
}

#[test]
fn bits_signed_mul_high32() -> Void {
  assert_eq!(0, signed_mul_high32(0, 0));
  for i in 1..=50i32 {
    for j in 1..=i {
      assert_eq!(0, signed_mul_high32(i, j));
    }
  }
  assert_eq!(-1_073_741_824, signed_mul_high32(i32::MAX, i32::MIN));
  assert_eq!(-1_073_741_824, signed_mul_high32(i32::MIN, i32::MAX));
  assert_eq!(1, signed_mul_high32(1024 * 1024 * 1024, 4));
  assert_eq!(2, signed_mul_high32(8 * 1024, 1024 * 1024));
  OK
}

#[test]
fn bits_signed_mul_high_and_add32() -> Void {
  for i in 1..=50i32 {
    assert_eq!(i, signed_mul_high_and_add32(0, 0, i));
    for j in 1..=i {
      assert_eq!(i, signed_mul_high_and_add32(j, j, i));
    }
    assert_eq!(i + 1, signed_mul_high_and_add32(1024 * 1024 * 1024, 4, i));
  }
  OK
}

#[test]
fn bits_signed_div32() -> Void {
  assert_eq!(i32::MIN, signed_div32(i32::MIN, -1));
  assert_eq!(i32::MAX, signed_div32(i32::MAX, 1));
  for i in 0..=50i32 {
    assert_eq!(0, signed_div32(i, 0));
    for j in 1..=i {
      assert_eq!(1, signed_div32(j, j));
      assert_eq!(i / j, signed_div32(i, j));
      assert_eq!(-i / j, signed_div32(i, -j));
    }
  }
  OK
}

#[test]
fn bits_signed_mod32() -> Void {
  assert_eq!(0, signed_mod32(i32::MIN, -1));
  assert_eq!(0, signed_mod32(i32::MAX, 1));
  for i in 0..=50i32 {
    assert_eq!(0, signed_mod32(i, 0));
    for j in 1..=i {
      assert_eq!(0, signed_mod32(j, j));
      assert_eq!(i % j, signed_mod32(i, j));
      assert_eq!(i % j, signed_mod32(i, -j));
    }
  }
  OK
}

#[test]
fn bits_unsigned_add_overflow32() -> Void {
  let (ov, val) = unsigned_add_overflow32(0, 0);
  assert!(!ov);
  assert_eq!(0, val);
  let (ov, val) = unsigned_add_overflow32(u32::MAX, 1);
  assert!(ov);
  assert_eq!(u32::MIN, val);
  let (ov, val) = unsigned_add_overflow32(u32::MAX, u32::MAX);
  assert!(ov);
  assert_eq!(u32::MAX - 1, val); // 环绕值
  for i in 1..=50u32 {
    for j in 1..=i {
      let (ov, val) = unsigned_add_overflow32(i, j);
      assert!(!ov);
      assert_eq!(i + j, val);
    }
  }
  OK
}

#[test]
fn bits_unsigned_div32() -> Void {
  for i in 0..=50u32 {
    assert_eq!(0, unsigned_div32(i, 0));
    for j in (i + 1)..=100u32 {
      assert_eq!(1, unsigned_div32(j, j));
      assert_eq!(i / j, unsigned_div32(i, j));
    }
  }
  OK
}

#[test]
fn bits_unsigned_mod32() -> Void {
  for i in 0..=50u32 {
    assert_eq!(0, unsigned_mod32(i, 0));
    for j in (i + 1)..=100u32 {
      assert_eq!(0, unsigned_mod32(j, j));
      assert_eq!(i % j, unsigned_mod32(i, j));
    }
  }
  OK
}

#[test]
fn bits_power_of_two_rounding_and_boundaries() -> Void {
  use v8_base::bits::{floor_to_power_of_two, round_to_power_of_two, round_up_to_power_of_two};

  // 0 值行为
  assert_eq!(0u32, 0u32.round_up_to_power_of_two());
  assert_eq!(0u32, 0u32.round_to_power_of_two());
  assert_eq!(0u32, round_up_to_power_of_two(0u32));
  assert_eq!(0u32, round_to_power_of_two(0u32));
  assert_eq!(0u32, 0u32.floor_to_power_of_two());
  assert_eq!(0u32, floor_to_power_of_two(0u32));

  assert_eq!(0u64, 0u64.round_up_to_power_of_two());
  assert_eq!(0u64, 0u64.floor_to_power_of_two());

  // 1 值行为
  assert_eq!(1u32, 1u32.round_up_to_power_of_two());
  assert_eq!(1u32, 1u32.floor_to_power_of_two());

  // 常用非 2 的幂上舍与下舍
  assert_eq!(4u32, 3u32.round_up_to_power_of_two());
  assert_eq!(2u32, 3u32.floor_to_power_of_two());
  assert_eq!(8u32, 5u32.round_up_to_power_of_two());
  assert_eq!(4u32, 5u32.floor_to_power_of_two());
  assert_eq!(1024u32, 1000u32.round_up_to_power_of_two());
  assert_eq!(512u32, 1000u32.floor_to_power_of_two());

  // 已经是 2 的幂时保持不变
  for shift in 0..31u32 {
    let p = 1u32 << shift;
    assert_eq!(p, p.round_up_to_power_of_two());
    assert_eq!(p, p.floor_to_power_of_two());
  }

  // 边界极限: u32 最高位与超出位宽饱和行为
  assert_eq!(1u32 << 31, (1u32 << 31).round_up_to_power_of_two());
  assert_eq!(1u32 << 31, ((1u32 << 31) + 1).round_up_to_power_of_two());
  assert_eq!(1u32 << 31, u32::MAX.round_up_to_power_of_two());
  assert_eq!(1u32 << 31, u32::MAX.floor_to_power_of_two());

  // 边界极限: u64 最高位与饱和行为
  assert_eq!(1u64 << 63, (1u64 << 63).round_up_to_power_of_two());
  assert_eq!(1u64 << 63, ((1u64 << 63) + 1).round_up_to_power_of_two());
  assert_eq!(1u64 << 63, u64::MAX.round_up_to_power_of_two());
  assert_eq!(1u64 << 63, u64::MAX.floor_to_power_of_two());

  // rotate_right 0 移位与满位宽
  assert_eq!(0x1234_5678u32, 0x1234_5678u32.rotate_right(0));
  assert_eq!(0x1234_5678u32, 0x1234_5678u32.rotate_right(32));
  assert_eq!(0u32, 0u32.rotate_right(17));

  OK
}
