//! 原版 v8/test/unittests/base/double-unittest.cc 1:1 迁移
//!
//! 9 个 TEST_F 全落地: 位级黄金值逐字节比对 (golden 场景允许 ==)。

use aok::{OK, Void};
use v8_base::numbers::{diy_fp::DiyFp, double::Double};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// DoubleTest.Uint64Conversions
#[test]
fn double_test_uint64_conversions() -> Void {
  assert_eq!(
    Double::from_u64(0x0123_4567_89AB_CDEF).value().to_bits(),
    0x0123_4567_89AB_CDEF
  );
  assert_eq!(
    Double::from_u64(0x0000_0000_0000_0001).value().to_bits(),
    f64::to_bits(5e-324)
  );
  assert_eq!(
    Double::from_u64(0x7FEF_FFFF_FFFF_FFFF).value(),
    1.7976931348623157e308
  );
  OK
}

/// DoubleTest.AsDiyFp
#[test]
fn double_test_as_diy_fp() -> Void {
  let diy_fp = Double::from_u64(0x0123_4567_89AB_CDEF).as_diy_fp();
  assert_eq!(diy_fp.e(), 0x12 - 0x3FF - 52);
  assert_eq!(diy_fp.f(), 0x0013_4567_89AB_CDEF);

  let diy_fp = Double::from_u64(1).as_diy_fp();
  assert_eq!(diy_fp.e(), -0x3FF - 52 + 1);
  // denormal 无隐藏位
  assert_eq!(diy_fp.f(), 1);

  let diy_fp = Double::from_u64(0x7FEF_FFFF_FFFF_FFFF).as_diy_fp();
  assert_eq!(diy_fp.e(), 0x7FE - 0x3FF - 52);
  assert_eq!(diy_fp.f(), 0x001F_FFFF_FFFF_FFFF);
  OK
}

/// DoubleTest.AsNormalizedDiyFp
#[test]
fn double_test_as_normalized_diy_fp() -> Void {
  let diy_fp = Double::from_u64(0x0123_4567_89AB_CDEF).as_normalized_diy_fp();
  assert_eq!(diy_fp.e(), 0x12 - 0x3FF - 52 - 11);
  assert_eq!(diy_fp.f(), 0x0013_4567_89AB_CDEF << 11);

  let diy_fp = Double::from_u64(1).as_normalized_diy_fp();
  assert_eq!(diy_fp.e(), -0x3FF - 52 + 1 - 63);
  assert_eq!(diy_fp.f(), 0x8000_0000_0000_0000);

  let diy_fp = Double::from_u64(0x7FEF_FFFF_FFFF_FFFF).as_normalized_diy_fp();
  assert_eq!(diy_fp.e(), 0x7FE - 0x3FF - 52 - 11);
  assert_eq!(diy_fp.f(), 0x001F_FFFF_FFFF_FFFF << 11);
  OK
}

/// DoubleTest.IsDenormal
#[test]
fn double_test_is_denormal() -> Void {
  assert!(Double::from_u64(1).is_denormal());
  assert!(Double::from_u64(0x000F_FFFF_FFFF_FFFF).is_denormal());
  assert!(!Double::from_u64(0x0010_0000_0000_0000).is_denormal());
  OK
}

/// DoubleTest.IsSpecial
#[test]
fn double_test_is_special() -> Void {
  assert!(Double::from_double(f64::INFINITY).is_special());
  assert!(Double::from_double(f64::NEG_INFINITY).is_special());
  assert!(Double::from_double(f64::NAN).is_special());
  assert!(Double::from_u64(0xFFF1_2345_0000_0000).is_special());
  // denormal 不算特殊
  assert!(!Double::from_double(5e-324).is_special());
  assert!(!Double::from_double(-5e-324).is_special());
  // 常规值
  assert!(!Double::from_double(0.0).is_special());
  assert!(!Double::from_double(-0.0).is_special());
  assert!(!Double::from_double(1.0).is_special());
  assert!(!Double::from_double(-1.0).is_special());
  assert!(!Double::from_double(1000000.0).is_special());
  assert!(!Double::from_double(-1000000.0).is_special());
  assert!(!Double::from_double(1e23).is_special());
  assert!(!Double::from_double(-1e23).is_special());
  assert!(!Double::from_double(1.7976931348623157e308).is_special());
  assert!(!Double::from_double(-1.7976931348623157e308).is_special());
  OK
}

/// DoubleTest.IsInfinite
#[test]
fn double_test_is_infinite() -> Void {
  assert!(Double::from_double(f64::INFINITY).is_infinite());
  assert!(Double::from_double(f64::NEG_INFINITY).is_infinite());
  assert!(!Double::from_double(f64::NAN).is_infinite());
  assert!(!Double::from_double(0.0).is_infinite());
  assert!(!Double::from_double(-0.0).is_infinite());
  assert!(!Double::from_double(1.0).is_infinite());
  assert!(!Double::from_double(-1.0).is_infinite());
  assert!(!Double::from_u64(1).is_infinite());
  OK
}

/// DoubleTest.Sign
#[test]
fn double_test_sign() -> Void {
  assert_eq!(Double::from_double(1.0).sign(), 1);
  assert_eq!(Double::from_double(f64::INFINITY).sign(), 1);
  assert_eq!(Double::from_double(f64::NEG_INFINITY).sign(), -1);
  assert_eq!(Double::from_double(0.0).sign(), 1);
  assert_eq!(Double::from_double(-0.0).sign(), -1);
  assert_eq!(Double::from_u64(1).sign(), 1);
  OK
}

/// DoubleTest.NormalizedBoundaries
#[test]
fn double_test_normalized_boundaries() -> Void {
  let check = |bits: u64, minus_dist: u64, plus_dist: Option<u64>| {
    let d = Double::from_u64(bits);
    let diy_fp = d.as_normalized_diy_fp();
    let (boundary_minus, boundary_plus) = d.normalized_boundaries();
    assert_eq!(diy_fp.e(), boundary_minus.e());
    assert_eq!(diy_fp.e(), boundary_plus.e());
    match plus_dist {
      None => {
        // 等距边界
        assert_eq!(
          diy_fp.f() - boundary_minus.f(),
          boundary_plus.f() - diy_fp.f()
        );
      }
      Some(pd) => {
        // 2^p 型 significand: 下边界距离减半
        assert!(boundary_plus.f() - diy_fp.f() > diy_fp.f() - boundary_minus.f());
        assert_eq!(diy_fp.f() - boundary_minus.f(), minus_dist);
        assert_eq!(boundary_plus.f() - diy_fp.f(), pd);
      }
    }
    if plus_dist.is_none() {
      assert_eq!(diy_fp.f() - boundary_minus.f(), minus_dist);
    }
    let _: DiyFp = boundary_minus;
  };

  // 1.5: 非幂型, 等距
  let d = Double::from_double(1.5);
  let diy_fp = d.as_normalized_diy_fp();
  let (bm, bp) = d.normalized_boundaries();
  assert_eq!(diy_fp.e(), bm.e());
  assert_eq!(diy_fp.e(), bp.e());
  assert_eq!(diy_fp.f() - bm.f(), bp.f() - diy_fp.f());
  assert_eq!(diy_fp.f() - bm.f(), 1 << 10);

  // 1.0: 幂型, 下边界减半
  check(f64::to_bits(1.0), 1 << 9, Some(1 << 10));
  // min denormal: 等距, 距离 2^62
  check(1, 1 << 62, None);
  // smallest normal: 虽为幂型但等距 (唯一例外)
  check(0x0010_0000_0000_0000, 1 << 10, None);
  // largest denormal
  check(0x000F_FFFF_FFFF_FFFF, 1 << 11, None);
  // max double
  check(0x7FEF_FFFF_FFFF_FFFF, 1 << 10, None);
  OK
}

/// DoubleTest.NextDouble
#[test]
fn double_test_next_double() -> Void {
  assert_eq!(Double::from_double(0.0).next_double(), 4e-324);
  assert_eq!(Double::from_double(-0.0).next_double(), 0.0);
  assert_eq!(
    Double::from_double(-4e-324).next_double(),
    0.0f64.copysign(-1.0)
  );
  let d0 = Double::from_double(-4e-324);
  let d1 = Double::from_double(d0.next_double());
  let d2 = Double::from_double(d1.next_double());
  assert_eq!(d1.value().to_bits(), f64::to_bits(-0.0));
  assert_eq!(d2.value(), 0.0);
  assert_eq!(d2.next_double(), 4e-324);
  assert_eq!(
    Double::from_double(f64::NEG_INFINITY).next_double(),
    -1.7976931348623157e308
  );
  assert_eq!(
    Double::from_u64(0x7FEF_FFFF_FFFF_FFFF).next_double(),
    f64::INFINITY
  );
  OK
}
