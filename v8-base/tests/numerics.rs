//! v8-base/src/numerics/* 集成测试 — safe_conversions / checked_math / clamped_math / angle_conversions
//!
//! 覆盖数值族红线: 饱和/检查边界语义, `is_value_in_range`, 不得把静默截断当正确.
//! 原版对应 checked_math/clamped_math/safe_conversions/angle_conversions 边界用例.
//! `checked_cast` 越界与 `safe_unsigned_abs(INT_MIN)` 触发 DCHECK, 不在 release-safe 测试中断言.

use core::f64::consts::PI;

use aok::{OK, Void};
use v8_base::numerics::{
  angle_conversions::{deg_to_rad, rad_to_deg},
  checked_math::{CheckedInt32, CheckedUint32},
  clamped_math::{ClampedInt32, ClampedUint32},
  safe_conversions::{
    clamp_ceil_i32, clamp_floor_i32, clamp_round_i32, is_value_in_range_for, safe_unsigned_abs,
    saturated_cast, saturated_cast_f64, try_cast,
  },
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn safe_conversions_range_and_try() -> Void {
  // is_value_in_range_for: 目标类型能否容纳
  assert!(is_value_in_range_for::<i32, i64>(100));
  assert!(!is_value_in_range_for::<i32, i64>(i64::from(i32::MAX) + 1));
  assert!(is_value_in_range_for::<u8, i32>(255));
  assert!(!is_value_in_range_for::<u8, i32>(-1));
  // try_cast 越界返回 None (不截断)
  assert_eq!(Some(200i32), try_cast::<i32, i64>(200));
  assert_eq!(None, try_cast::<i8, i32>(200));
  assert_eq!(None, try_cast::<u32, i32>(-1));
  OK
}

#[test]
fn safe_conversions_saturated() -> Void {
  // 整数饱和: 越界钳制到目标边界
  assert_eq!(i32::MAX, saturated_cast::<i32, _>(i64::from(i32::MAX) + 1));
  assert_eq!(i32::MIN, saturated_cast::<i32, _>(i64::from(i32::MIN) - 1));
  assert_eq!(u8::MAX, saturated_cast::<u8, _>(300i32));
  assert_eq!(0u8, saturated_cast::<u8, _>(-5i32));
  // float→int 饱和, NaN→0
  assert_eq!(0i32, saturated_cast_f64::<i32>(f64::NAN));
  assert_eq!(i32::MAX, saturated_cast_f64::<i32>(1e30));
  assert_eq!(i32::MIN, saturated_cast_f64::<i32>(-1e30));
  assert_eq!(7i32, saturated_cast_f64::<i32>(7.9));
  OK
}

#[test]
fn safe_conversions_clamp_and_abs() -> Void {
  // ClampFloor/Ceil/Round (i32)
  assert_eq!(2, clamp_floor_i32(2.7));
  assert_eq!(-3, clamp_floor_i32(-2.3));
  assert_eq!(3, clamp_ceil_i32(2.1));
  assert_eq!(-2, clamp_ceil_i32(-2.7));
  assert_eq!(3, clamp_round_i32(2.5));
  assert_eq!(-3, clamp_round_i32(-2.5));
  assert_eq!(2, clamp_round_i32(2.4));
  // SafeUnsignedAbs 正常值
  assert_eq!(5u32, safe_unsigned_abs(-5i32));
  assert_eq!(5u32, safe_unsigned_abs(5i32));
  assert_eq!(7u64, safe_unsigned_abs(-7i64));
  OK
}

#[test]
fn checked_math_boundaries() -> Void {
  let a = CheckedInt32::new(i32::MAX);
  let one = CheckedInt32::new(1);
  // 加法溢出 → 失效
  assert!(a.add(&one).has_overflowed());
  assert!(a.add(&one).value().is_none());
  // 正常加法
  let b = CheckedInt32::new(40);
  let c = CheckedInt32::new(2);
  assert_eq!(Some(42), b.add(&c).value());
  assert_eq!(Some(38), b.subtract(&c).value());
  assert_eq!(Some(80), b.multiply(&c).value());
  assert_eq!(Some(20), b.divide(&c).value());
  // 除零 → 失效
  assert!(b.divide(&CheckedInt32::new(0)).has_overflowed());
  // INT32_MIN / -1 → 溢出
  assert!(
    CheckedInt32::new(i32::MIN)
      .divide(&CheckedInt32::new(-1))
      .has_overflowed()
  );

  // 无符号减法下溢 → 失效; 失效传播
  let u = CheckedUint32::new(0);
  assert!(u.subtract(&CheckedUint32::new(1)).has_overflowed());
  let inv = CheckedUint32::invalid(5);
  assert!(inv.add(&u).has_overflowed());
  OK
}

#[test]
fn clamped_math_boundaries() -> Void {
  let mx = ClampedInt32::new(i32::MAX);
  let one = ClampedInt32::new(1);
  // 饱和: 溢出钳制到边界, 始终有效取值
  assert_eq!(i32::MAX, mx.add(&one).value());
  assert_eq!(i32::MIN, ClampedInt32::new(i32::MIN).subtract(&one).value());
  assert_eq!(
    i32::MAX,
    ClampedInt32::new(100000)
      .multiply(&ClampedInt32::new(100000))
      .value()
  );
  assert_eq!(
    21,
    ClampedInt32::new(42).divide(&ClampedInt32::new(2)).value()
  );
  // 除零 → 原版 ClampedDivide 结果为 0
  assert_eq!(
    0,
    ClampedInt32::new(42).divide(&ClampedInt32::new(0)).value()
  );

  // 无符号饱和
  assert_eq!(
    0u32,
    ClampedUint32::new(0)
      .subtract(&ClampedUint32::new(1))
      .value()
  );
  assert_eq!(
    u32::MAX,
    ClampedUint32::new(u32::MAX)
      .add(&ClampedUint32::new(1))
      .value()
  );
  OK
}

#[test]
fn angle_conversions_boundaries() -> Void {
  assert_eq!(0.0, deg_to_rad(0.0f64));
  assert_eq!(0.0, rad_to_deg(0.0f64));
  let r: f64 = deg_to_rad(180.0);
  assert!((r - PI).abs() < 1e-15);
  let d: f64 = rad_to_deg(PI);
  assert!((d - 180.0).abs() < 1e-12);
  // f32 版本
  assert_eq!(0.0f32, deg_to_rad(0.0f32));
  OK
}
