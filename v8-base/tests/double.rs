//! v8-base/src/numbers/double.rs 集成测试 — 1:1 对应 double-unittest.cc (9 用例)

use aok::{OK, Void};
use v8_base::numbers::double::Double;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn double_test_uint64_conversions() -> Void {
  // 字节序核对
  let ordered = 0x0123_4567_89AB_CDEFu64;
  assert_eq!(3.512700564088504e-303, Double::from_u64(ordered).value());

  let min_double64 = 0x0000_0000_0000_0001u64;
  assert_eq!(5e-324, Double::from_u64(min_double64).value());

  let max_double64 = 0x7FEF_FFFF_FFFF_FFFFu64;
  assert_eq!(
    1.7976931348623157e308,
    Double::from_u64(max_double64).value()
  );
  OK
}

#[test]
fn double_test_as_diy_fp() -> Void {
  let ordered = 0x0123_4567_89AB_CDEFu64;
  let diy_fp = Double::from_u64(ordered).as_diy_fp();
  assert_eq!(0x12 - 0x3FF - 52, diy_fp.e());
  // 52 位尾数 + 第 52 位的隐含 1
  assert_eq!(0x0013_4567_89AB_CDEFu64, diy_fp.f());

  let min_double64 = 0x0000_0000_0000_0001u64;
  let diy_fp = Double::from_u64(min_double64).as_diy_fp();
  assert_eq!(-0x3FF - 52 + 1, diy_fp.e());
  // denormal 无隐含位
  assert_eq!(1, diy_fp.f());

  let max_double64 = 0x7FEF_FFFF_FFFF_FFFFu64;
  let diy_fp = Double::from_u64(max_double64).as_diy_fp();
  assert_eq!(0x7FE - 0x3FF - 52, diy_fp.e());
  assert_eq!(0x001F_FFFF_FFFF_FFFFu64, diy_fp.f());
  OK
}

#[test]
fn double_test_as_normalized_diy_fp() -> Void {
  let ordered = 0x0123_4567_89AB_CDEFu64;
  let diy_fp = Double::from_u64(ordered).as_normalized_diy_fp();
  assert_eq!(0x12 - 0x3FF - 52 - 11, diy_fp.e());
  assert_eq!(0x0013_4567_89AB_CDEFu64 << 11, diy_fp.f());

  let min_double64 = 0x0000_0000_0000_0001u64;
  let diy_fp = Double::from_u64(min_double64).as_normalized_diy_fp();
  assert_eq!(-0x3FF - 52 + 1 - 63, diy_fp.e());
  assert_eq!(0x8000_0000_0000_0000u64, diy_fp.f());

  let max_double64 = 0x7FEF_FFFF_FFFF_FFFFu64;
  let diy_fp = Double::from_u64(max_double64).as_normalized_diy_fp();
  assert_eq!(0x7FE - 0x3FF - 52 - 11, diy_fp.e());
  assert_eq!(0x001F_FFFF_FFFF_FFFFu64 << 11, diy_fp.f());
  OK
}

#[test]
fn double_test_is_denormal() -> Void {
  let min_double64 = 0x0000_0000_0000_0001u64;
  assert!(Double::from_u64(min_double64).is_denormal());
  let bits = 0x000F_FFFF_FFFF_FFFFu64;
  assert!(Double::from_u64(bits).is_denormal());
  let bits = 0x0010_0000_0000_0000u64;
  assert!(!Double::from_u64(bits).is_denormal());
  OK
}

#[test]
fn double_test_is_special() -> Void {
  assert!(Double::from_double(f64::INFINITY).is_special());
  assert!(Double::from_double(f64::NEG_INFINITY).is_special());
  assert!(Double::from_double(f64::NAN).is_special());
  let bits = 0xFFF1_2345_0000_0000u64;
  assert!(Double::from_u64(bits).is_special());
  // denormal 不算特殊值
  assert!(!Double::from_double(5e-324).is_special());
  assert!(!Double::from_double(-5e-324).is_special());
  assert!(!Double::from_double(0.0).is_special());
  assert!(!Double::from_double(-0.0).is_special());
  assert!(!Double::from_double(1.0).is_special());
  assert!(!Double::from_double(-1.0).is_special());
  assert!(!Double::from_double(1_000_000.0).is_special());
  assert!(!Double::from_double(-1_000_000.0).is_special());
  assert!(!Double::from_double(1e23).is_special());
  assert!(!Double::from_double(-1e23).is_special());
  assert!(!Double::from_double(1.7976931348623157e308).is_special());
  assert!(!Double::from_double(-1.7976931348623157e308).is_special());
  OK
}

#[test]
fn double_test_is_infinite() -> Void {
  assert!(Double::from_double(f64::INFINITY).is_infinite());
  assert!(Double::from_double(f64::NEG_INFINITY).is_infinite());
  assert!(!Double::from_double(f64::NAN).is_infinite());
  assert!(!Double::from_double(0.0).is_infinite());
  assert!(!Double::from_double(-0.0).is_infinite());
  assert!(!Double::from_double(1.0).is_infinite());
  assert!(!Double::from_double(-1.0).is_infinite());
  let min_double64 = 0x0000_0000_0000_0001u64;
  assert!(!Double::from_u64(min_double64).is_infinite());
  OK
}

#[test]
fn double_test_sign() -> Void {
  assert_eq!(1, Double::from_double(1.0).sign());
  assert_eq!(1, Double::from_double(f64::INFINITY).sign());
  assert_eq!(-1, Double::from_double(f64::NEG_INFINITY).sign());
  assert_eq!(1, Double::from_double(0.0).sign());
  assert_eq!(-1, Double::from_double(-0.0).sign());
  let min_double64 = 0x0000_0000_0000_0001u64;
  assert_eq!(1, Double::from_u64(min_double64).sign());
  OK
}

#[test]
fn double_test_normalized_boundaries() -> Void {
  // 1.5: 非 2^p 型尾数, 两侧边界等距
  let diy_fp = Double::from_double(1.5).as_normalized_diy_fp();
  let (boundary_minus, boundary_plus) = Double::from_double(1.5).normalized_boundaries();
  assert_eq!(diy_fp.e(), boundary_minus.e());
  assert_eq!(diy_fp.e(), boundary_plus.e());
  assert_eq!(
    diy_fp.f() - boundary_minus.f(),
    boundary_plus.f() - diy_fp.f()
  );
  assert_eq!(1 << 10, diy_fp.f() - boundary_minus.f());

  // 1.0: 2^p 型尾数, 下边界距离减半
  let diy_fp = Double::from_double(1.0).as_normalized_diy_fp();
  let (boundary_minus, boundary_plus) = Double::from_double(1.0).normalized_boundaries();
  assert_eq!(diy_fp.e(), boundary_minus.e());
  assert_eq!(diy_fp.e(), boundary_plus.e());
  assert!(boundary_plus.f() - diy_fp.f() > diy_fp.f() - boundary_minus.f());
  assert_eq!(1 << 9, diy_fp.f() - boundary_minus.f());
  assert_eq!(1 << 10, boundary_plus.f() - diy_fp.f());

  // 最小 denormal: 等距但距离巨大
  let min_double64 = 0x0000_0000_0000_0001u64;
  let d = Double::from_u64(min_double64);
  let diy_fp = d.as_normalized_diy_fp();
  let (boundary_minus, boundary_plus) = d.normalized_boundaries();
  assert_eq!(diy_fp.e(), boundary_minus.e());
  assert_eq!(diy_fp.e(), boundary_plus.e());
  assert_eq!(
    diy_fp.f() - boundary_minus.f(),
    boundary_plus.f() - diy_fp.f()
  );
  assert_eq!(1u64 << 62, diy_fp.f() - boundary_minus.f());

  // 最小正规数: 唯一例外 — 2^p 型尾数仍等距
  let smallest_normal64 = 0x0010_0000_0000_0000u64;
  let d = Double::from_u64(smallest_normal64);
  let diy_fp = d.as_normalized_diy_fp();
  let (boundary_minus, boundary_plus) = d.normalized_boundaries();
  assert_eq!(diy_fp.e(), boundary_minus.e());
  assert_eq!(diy_fp.e(), boundary_plus.e());
  assert_eq!(
    diy_fp.f() - boundary_minus.f(),
    boundary_plus.f() - diy_fp.f()
  );
  assert_eq!(1 << 10, diy_fp.f() - boundary_minus.f());

  // 最大 denormal
  let largest_denormal64 = 0x000F_FFFF_FFFF_FFFFu64;
  let d = Double::from_u64(largest_denormal64);
  let diy_fp = d.as_normalized_diy_fp();
  let (boundary_minus, boundary_plus) = d.normalized_boundaries();
  assert_eq!(diy_fp.e(), boundary_minus.e());
  assert_eq!(diy_fp.e(), boundary_plus.e());
  assert_eq!(
    diy_fp.f() - boundary_minus.f(),
    boundary_plus.f() - diy_fp.f()
  );
  assert_eq!(1 << 11, diy_fp.f() - boundary_minus.f());

  // 最大 double
  let max_double64 = 0x7FEF_FFFF_FFFF_FFFFu64;
  let d = Double::from_u64(max_double64);
  let diy_fp = d.as_normalized_diy_fp();
  let (boundary_minus, boundary_plus) = d.normalized_boundaries();
  assert_eq!(diy_fp.e(), boundary_minus.e());
  assert_eq!(diy_fp.e(), boundary_plus.e());
  assert_eq!(
    diy_fp.f() - boundary_minus.f(),
    boundary_plus.f() - diy_fp.f()
  );
  assert_eq!(1 << 10, diy_fp.f() - boundary_minus.f());
  OK
}

#[test]
fn double_test_next_double() -> Void {
  assert_eq!(4e-324, Double::from_double(0.0).next_double());
  assert_eq!(0.0, Double::from_double(-0.0).next_double());
  assert_eq!(-0.0, Double::from_double(-4e-324).next_double());
  let d0 = Double::from_double(-4e-324);
  let d1 = Double::from_double(d0.next_double());
  let d2 = Double::from_double(d1.next_double());
  assert_eq!(-0.0, d1.value());
  assert_eq!(0.0, d2.value());
  assert_eq!(4e-324, d2.next_double());
  assert_eq!(
    -1.7976931348623157e308,
    Double::from_double(f64::NEG_INFINITY).next_double()
  );
  assert_eq!(
    f64::INFINITY,
    Double::from_u64(0x7FEF_FFFF_FFFF_FFFF).next_double()
  );
  OK
}
