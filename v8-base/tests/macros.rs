//! v8-base/src/macros.rs 集成测试 — 1:1 对应 macros-unittest.cc
//!
//! 原版用例: AlignedAddressTest.AlignedAddress (5 断言);
//! ASSERT_TRIVIALLY_COPYABLE/NOT_... 为 C++ 编译期类型断言, Rust 以
//! `fn assert_copy<T: Copy>()` 编译期探测等价承接 (NonTrivial* 系列在
//! Rust 所有权模型下无对应概念, 登记)。
//! 另补 bounds.h/macros.h 运行期函数的规约测试 (原版无独立用例文件):
//! is_inbounds 边界常数按 f32/f64 精度回绕的黄金断言。

use aok::{OK, Void};
use v8_base::{bounds, macros};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// AlignedAddressTest.AlignedAddress
#[test]
fn aligned_address_test_aligned_address() -> Void {
  assert_eq!(0xFFFF0, macros::aligned_address(0xFFFF0, 16));
  assert_eq!(0xFFFF0, macros::aligned_address(0xFFFF2, 16));
  assert_eq!(0xFFFF0, macros::aligned_address(0xFFFF2, 16));
  assert_eq!(0xFFFF0, macros::aligned_address(0xFFFFF, 16));
  assert_eq!(0x0, macros::aligned_address(0xFFFFF, 0x100000));
  // RoundUpAddress 同源断言 (原版 RoundUpAddress 无独立用例, 补边界)
  assert_eq!(0x100000, macros::round_up_address(0xFFFFF, 16));
  assert_eq!(0xFFFF0, macros::round_up_address(0xFFFF0, 16));
  assert!(macros::is_aligned_address(0xFFFF0, 16));
  assert!(!macros::is_aligned_address(0xFFFF2, 16));
  OK
}

/// ASSERT_TRIVIALLY_COPYABLE 的 Rust 编译期等价: Copy 探测
fn assert_copy<T: Copy>() {}

/// TriviallyCopyable / StillTriviallyCopyable → i32 (Copy) 可探测
#[test]
fn trivially_copyable_probe() -> Void {
  assert_copy::<i32>();
  assert_copy::<u64>();
  OK
}

/// is_inbounds 黄金断言 (原版 macros.h:470-486 语义, f32/f64 双精度路径)
#[test]
fn is_inbounds_golden() -> Void {
  // i32 × f64: 下界 -2^31-1 (精确), 上界 2^31 (精确), 双侧开区间
  assert!(macros::is_inbounds::<i32, f64>(0.0));
  assert!(macros::is_inbounds::<i32, f64>(-2147483648.0));
  assert!(macros::is_inbounds::<i32, f64>(2147483647.0));
  assert!(!macros::is_inbounds::<i32, f64>(-2147483649.0));
  assert!(!macros::is_inbounds::<i32, f64>(2147483648.0));
  assert!(!macros::is_inbounds::<i32, f64>(f64::INFINITY));
  assert!(!macros::is_inbounds::<i32, f64>(f64::NEG_INFINITY));
  assert!(!macros::is_inbounds::<i32, f64>(f64::NAN));
  // i32 × f32: 界在 f32 精度下回绕 → 下界闭 (kLowerBoundIsMin=true), 上界开
  // kLowerBound = f32(-2^31) - 1 → 四舍五入回 -2^31 (与 MIN 相等)
  assert!(macros::is_inbounds::<i32, f32>(-2147483648.0_f32));
  assert!(!macros::is_inbounds::<i32, f32>(-2147483904.0_f32));
  // kUpperBound = f32(2^31-1)+1 → 回绕到 2^31 (> MAX, 严格小于判定)
  assert!(macros::is_inbounds::<i32, f32>(2147483520.0_f32));
  assert!(!macros::is_inbounds::<i32, f32>(2147483648.0_f32));
  // i8: 下界 -129 (f64 精确, 开), 上界 128 (f64 精确, 开)
  assert!(macros::is_inbounds::<i8, f64>(127.0));
  assert!(!macros::is_inbounds::<i8, f64>(128.0));
  assert!(!macros::is_inbounds::<i8, f64>(-129.0));
  assert!(macros::is_inbounds::<i8, f64>(-128.0));
  // u32 × f64: MIN=0 → kLowerBound=-1, kLowerBoundIsMin=false (开区间)
  assert!(macros::is_inbounds::<u32, f64>(0.0));
  assert!(macros::is_inbounds::<u32, f64>(4294967295.0));
  assert!(!macros::is_inbounds::<u32, f64>(4294967296.0));
  OK
}

/// RoundDown/RoundUp/make_uint64/is_aligned 规约 (原版 macros.h 语义)
#[test]
fn round_and_align_semantics() -> Void {
  assert_eq!(macros::make_uint64(1, 2), (1_u64 << 32) + 2);
  assert_eq!(macros::round_down(0xFFFFF_u64, 0x1000), 0xFF000);
  assert_eq!(macros::round_up(0xFFFFF_u64, 0x1000), 0x100000);
  // static_cast<T>(-m) 截断语义: 有符号载体同位模式
  assert_eq!(macros::round_down(-15_i32, 16), -16);
  assert_eq!(macros::round_down(15_i32, 16), 0);
  // const 形态与运行期形态一致
  assert_eq!(macros::round_down_c::<16, _>(0xFFFFF_usize), 0xFFFF0);
  assert_eq!(macros::round_up_c::<16, _>(0xFFFF2_usize), 0x100000);
  assert!(macros::is_aligned(0xFFFF0_usize, 16_usize));
  assert!(!macros::is_aligned(0xFFFF2_usize, 16_usize));
  OK
}

/// bounds.h 四函数规约 (原版无独立用例, 按 bounds.h:16-82 语义)
#[test]
fn bounds_semantics() -> Void {
  // IsInRange: 单分支无符号回绕 (含有符号跨界回绕路径)
  assert!(bounds::is_in_range(5_i32, 1, 10));
  assert!(bounds::is_in_range(1_i32, 1, 10));
  assert!(bounds::is_in_range(10_i32, 1, 10));
  assert!(!bounds::is_in_range(0_i32, 1, 10));
  assert!(!bounds::is_in_range(11_i32, 1, 10));
  // 负值域: -5 ∈ [-10, -1]
  assert!(bounds::is_in_range(-5_i32, -10, -1));
  assert!(!bounds::is_in_range(0_i32, -10, -1));
  // u8 全域: 任何值都在 [0, 255] (回绕判定单分支正确性)
  assert!(bounds::is_in_range(255_u8, 0, 255));
  assert!(bounds::is_in_range(0_u8, 0, 255));
  // IsInHalfOpenRange
  assert!(bounds::is_in_half_open_range(9_i32, 1, 10));
  assert!(!bounds::is_in_half_open_range(10_i32, 1, 10));
  assert!(bounds::is_in_half_open_range(0_u8, 0, 255));
  assert!(!bounds::is_in_half_open_range(255_u8, 0, 255));
  // IsInBounds: index+length 回绕亦正确 (bounds.h 注释红线)
  assert!(bounds::is_in_bounds(5_u32, 5, 10));
  assert!(bounds::is_in_bounds(0_u32, 10, 10));
  assert!(!bounds::is_in_bounds(5_u32, 6, 10));
  assert!(!bounds::is_in_bounds(11_u32, 0, 10));
  // 回绕: length 回绕后不满足 length <= max
  assert!(!bounds::is_in_bounds(u32::MAX, u32::MAX, 10));
  // ClampToBounds
  let mut len = 5_u32;
  assert!(bounds::clamp_to_bounds(5, &mut len, 10));
  assert_eq!(len, 5);
  let mut len = 6_u32;
  assert!(!bounds::clamp_to_bounds(5, &mut len, 10));
  assert_eq!(len, 5);
  let mut len = 3_u32;
  assert!(!bounds::clamp_to_bounds(11, &mut len, 10));
  assert_eq!(len, 0);
  OK
}
