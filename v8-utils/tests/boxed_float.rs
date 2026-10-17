// 上游无 boxed-float 单测; NaN 位模式保持是该包装的核心契约, 补测关键路径
// (取值与 ostreams.h operator<< 文档中的位模式断言一致)。
use aok::{OK, Void};
use v8_utils::{
  boxed_float::{Float32, Float64, HOLE_NAN_INT64, UNDEFINED_NAN_INT64},
  utils::hash64,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn float32_bit_patterns() -> Void {
  let f = Float32::new(1.5f32);
  assert_eq!(f.get_bits(), 1.5f32.to_bits());
  assert_eq!(f.get_scalar(), 1.5);
  assert!(!f.is_nan());
  assert!(!f.is_inf());
  assert!(!f.is_negative());

  let neg = Float32::new(-2.0f32);
  assert!(neg.is_negative());
  assert_eq!(neg.neg().get_scalar(), 2.0);
  assert_eq!(Float32::new(2.0f32).to_negative().get_scalar(), -2.0);

  // 静默 NaN 位构造与 quiet 位 (上游 Float32 无 is_signalling_nan, 断言保持同构)
  let snan = Float32::from_bits(0x7F80_0001);
  assert!(snan.is_nan());
  assert!(!snan.is_quiet_nan());
  let quiet = snan.to_quiet_nan();
  assert!(quiet.is_quiet_nan());

  // 特殊常量
  assert!(Float32::quiet_nan().is_quiet_nan());
  assert!(Float32::infinity().is_inf());
  assert_eq!(Float32::infinity().get_bits(), 0x7F80_0000);
  OK
}

#[test]
fn float64_bit_patterns() -> Void {
  let d = Float64::new(13.37f64);
  assert_eq!(d.get_bits(), 13.37f64.to_bits());
  assert_eq!(d.get_scalar(), 13.37);

  // hole/undefined NaN 位模式 (globals.h)
  assert_eq!(HOLE_NAN_INT64, 0xFFF7FFFF_FFF7FFFF);
  assert_eq!(UNDEFINED_NAN_INT64, 0xFFF6FFFF_FFF6FFFF);
  let hole = Float64::hole_nan();
  assert!(hole.is_nan());
  assert!(hole.is_hole_nan());
  assert!(hole.is_undefined_or_hole_nan());
  assert!(!hole.is_quiet_nan()); // hole NaN 的 quiet 位 (51) 为 0

  let undef = Float64::undefined_nan();
  assert!(undef.is_undefined_nan());

  // 静默 NaN → quiet 化
  let snan = Float64::from_bits(0x7FF0_0000_0000_0001);
  assert!(snan.is_signalling_nan());
  let quiet = snan.to_quiet_nan();
  assert!(quiet.is_quiet_nan());

  // FromMaybeNaN 不丢位
  let nan = Float64::from_maybe_nan(f64::NAN);
  assert!(nan.is_nan());
  assert!(nan.is_quiet_nan());

  // hash: base::hash_value(bits) = hash64 混淆 (boxed-float.h:208), 非原始位模式
  let h = Float64::from_bits(42).hash_value();
  assert_ne!(h, 42);
  assert_eq!(h, hash64(42));
  // 相等位模式 ⇒ 相等散列
  assert_eq!(h, Float64::from_maybe_nan(f64::from_bits(42)).hash_value());
  OK
}

#[test]
fn display_format() -> Void {
  // operator<<: "scalar (0x%08x)"
  assert_eq!(Float32::new(1.5f32).to_string(), "1.5 (0x3fc00000)");
  assert_eq!(Float64::new(1.0f64).to_string(), "1 (0x3ff0000000000000)");
  OK
}
