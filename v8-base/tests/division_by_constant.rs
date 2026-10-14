//! v8-base/src/division_by_constant.rs 集成测试 — 1:1 对应 division-by-constant-unittest.cc
//!
//! 《Hacker's Delight》表 10-1 全部魔数样例. 原版 4 用例:
//! - `DivisionByConstant.Signed32` / `Unsigned32` / `Signed64` / `Unsigned64`.
//!
//! 原版 `s32(d)` = `SignedDivisionByConstant<uint32_t>(static_cast<uint32_t>(d))`,
//! `u32(d)` = `UnsignedDivisionByConstant<uint32_t>(d)` (leading_zeros 默认 0).

use aok::{OK, Void};
use v8_base::division_by_constant::{
  MagicNumbersForDivision, signed_division_by_constant, unsigned_division_by_constant,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

type M32 = MagicNumbersForDivision<u32>;
type M64 = MagicNumbersForDivision<u64>;

fn s32(d: i32) -> M32 {
  signed_division_by_constant(d as u32)
}

fn s64(d: i64) -> M64 {
  signed_division_by_constant(d as u64)
}

fn u32(d: u32) -> M32 {
  unsigned_division_by_constant(d, 0)
}

fn u64(d: u64) -> M64 {
  unsigned_division_by_constant(d, 0)
}

#[test]
fn division_by_constant_signed32() -> Void {
  assert_eq!(M32::new(0x9999_9999, 1, false), s32(-5));
  assert_eq!(M32::new(0x5555_5555, 1, false), s32(-3));
  let mut d: i32 = -1;
  for k in 1..=31u32 {
    d = (d as u32).wrapping_mul(2) as i32;
    assert_eq!(M32::new(0x7FFF_FFFF, k - 1, false), s32(d));
  }
  for k in 1..=30u32 {
    assert_eq!(M32::new(0x8000_0001, k - 1, false), s32(1 << k));
  }
  assert_eq!(M32::new(0x5555_5556, 0, false), s32(3));
  assert_eq!(M32::new(0x6666_6667, 1, false), s32(5));
  assert_eq!(M32::new(0x2AAA_AAAB, 0, false), s32(6));
  assert_eq!(M32::new(0x9249_2493, 2, false), s32(7));
  assert_eq!(M32::new(0x38E3_8E39, 1, false), s32(9));
  assert_eq!(M32::new(0x6666_6667, 2, false), s32(10));
  assert_eq!(M32::new(0x2E8B_A2E9, 1, false), s32(11));
  assert_eq!(M32::new(0x2AAA_AAAB, 1, false), s32(12));
  assert_eq!(M32::new(0x51EB_851F, 3, false), s32(25));
  assert_eq!(M32::new(0x1062_4DD3, 3, false), s32(125));
  assert_eq!(M32::new(0x68DB_8BAD, 8, false), s32(625));
  OK
}

#[test]
fn division_by_constant_unsigned32() -> Void {
  assert_eq!(M32::new(0x0000_0000, 0, true), u32(1));
  for k in 1..=30u32 {
    assert_eq!(M32::new(1u32 << (32 - k), 0, false), u32(1u32 << k));
  }
  assert_eq!(M32::new(0xAAAA_AAAB, 1, false), u32(3));
  assert_eq!(M32::new(0xCCCC_CCCD, 2, false), u32(5));
  assert_eq!(M32::new(0xAAAA_AAAB, 2, false), u32(6));
  assert_eq!(M32::new(0x2492_4925, 3, true), u32(7));
  assert_eq!(M32::new(0x38E3_8E39, 1, false), u32(9));
  assert_eq!(M32::new(0xCCCC_CCCD, 3, false), u32(10));
  assert_eq!(M32::new(0xBA2E_8BA3, 3, false), u32(11));
  assert_eq!(M32::new(0xAAAA_AAAB, 3, false), u32(12));
  assert_eq!(M32::new(0x51EB_851F, 3, false), u32(25));
  assert_eq!(M32::new(0x1062_4DD3, 3, false), u32(125));
  assert_eq!(M32::new(0xD1B7_1759, 9, false), u32(625));
  OK
}

#[test]
fn division_by_constant_signed64() -> Void {
  assert_eq!(M64::new(0x9999_9999_9999_9999, 1, false), s64(-5));
  assert_eq!(M64::new(0x5555_5555_5555_5555, 1, false), s64(-3));
  let mut d: i64 = -1;
  for k in 1..=63u32 {
    d = (d as u64).wrapping_mul(2) as i64;
    assert_eq!(M64::new(0x7FFF_FFFF_FFFF_FFFF, k - 1, false), s64(d));
  }
  for k in 1..=62u32 {
    assert_eq!(
      M64::new(0x8000_0000_0000_0001, k - 1, false),
      s64(1i64 << k)
    );
  }
  assert_eq!(M64::new(0x5555_5555_5555_5556, 0, false), s64(3));
  assert_eq!(M64::new(0x6666_6666_6666_6667, 1, false), s64(5));
  assert_eq!(M64::new(0x2AAA_AAAA_AAAA_AAAB, 0, false), s64(6));
  assert_eq!(M64::new(0x4924_9249_2492_4925, 1, false), s64(7));
  assert_eq!(M64::new(0x1C71_C71C_71C7_1C72, 0, false), s64(9));
  assert_eq!(M64::new(0x6666_6666_6666_6667, 2, false), s64(10));
  assert_eq!(M64::new(0x2E8B_A2E8_BA2E_8BA3, 1, false), s64(11));
  assert_eq!(M64::new(0x2AAA_AAAA_AAAA_AAAB, 1, false), s64(12));
  assert_eq!(M64::new(0xA3D7_0A3D_70A3_D70B, 4, false), s64(25));
  assert_eq!(M64::new(0x20C4_9BA5_E353_F7CF, 4, false), s64(125));
  assert_eq!(M64::new(0x346D_C5D6_3886_594B, 7, false), s64(625));
  OK
}

#[test]
fn division_by_constant_unsigned64() -> Void {
  assert_eq!(M64::new(0x0000_0000_0000_0000, 0, true), u64(1));
  for k in 1..=62u32 {
    assert_eq!(M64::new(1u64 << (64 - k), 0, false), u64(1u64 << k));
  }
  assert_eq!(M64::new(0xAAAA_AAAA_AAAA_AAAB, 1, false), u64(3));
  assert_eq!(M64::new(0xCCCC_CCCC_CCCC_CCCD, 2, false), u64(5));
  assert_eq!(M64::new(0xAAAA_AAAA_AAAA_AAAB, 2, false), u64(6));
  assert_eq!(M64::new(0x2492_4924_9249_2493, 3, true), u64(7));
  assert_eq!(M64::new(0xE38E_38E3_8E38_E38F, 3, false), u64(9));
  assert_eq!(M64::new(0xCCCC_CCCC_CCCC_CCCD, 3, false), u64(10));
  assert_eq!(M64::new(0x2E8B_A2E8_BA2E_8BA3, 1, false), u64(11));
  assert_eq!(M64::new(0xAAAA_AAAA_AAAA_AAAB, 3, false), u64(12));
  assert_eq!(M64::new(0x47AE_147A_E147_AE15, 5, true), u64(25));
  assert_eq!(M64::new(0x0624_DD2F_1A9F_BE77, 7, true), u64(125));
  assert_eq!(M64::new(0x346D_C5D6_3886_594B, 7, false), u64(625));
  OK
}
