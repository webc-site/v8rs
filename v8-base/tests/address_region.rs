//! v8-base/src/address_region.rs 集成测试 — 1:1 对应 address-region-unittest.cc (3 用例)

use aok::{OK, Void};
use v8_base::address_region::AddressRegion;

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn address_region_test_contains() -> Void {
  // 原版 test_cases; -447usize 即 static_cast<Address>(-447) 的位模式
  for &(start, size) in &[(153usize, 771usize), (0, 227), (-447i64 as usize, 447)] {
    let end = start.wrapping_add(size); // exclusive
    let region = AddressRegion::new(start, size);

    // 单地址 contains
    assert!(!region.contains_address(start.wrapping_sub(1041)));
    assert!(!region.contains_address(start.wrapping_sub(1)));
    assert!(!region.contains_address(end));
    assert!(!region.contains_address(end.wrapping_add(1)));
    assert!(!region.contains_address(end.wrapping_add(113)));

    assert!(region.contains_address(start));
    assert!(region.contains_address(start.wrapping_add(1)));
    assert!(region.contains_address(start.wrapping_add(size / 2)));
    assert!(region.contains_address(end.wrapping_sub(1)));

    // 区间 contains
    assert!(!region.contains_region(start.wrapping_sub(1), size));
    assert!(!region.contains_region(start, size + 1));
    assert!(!region.contains_region(start.wrapping_sub(17), 17));
    assert!(!region.contains_region(start.wrapping_sub(17), size * 2));
    assert!(!region.contains_region(end, 1));
    assert!(!region.contains_region(end, 0usize.wrapping_sub(end)));

    assert!(region.contains_region(start, size));
    assert!(region.contains_region(start, 10));
    assert!(region.contains_region(start.wrapping_add(11), 120));
    assert!(region.contains_region(end.wrapping_sub(13), 13));
    assert!(!region.contains_region(end, 0));

    // 零大小查询
    assert!(!region.contains_region(start.wrapping_sub(10), 0));
    assert!(!region.contains_region(start.wrapping_sub(1), 0));
    assert!(!region.contains_region(end, 0));
    assert!(!region.contains_region(end.wrapping_add(10), 0));

    assert!(region.contains_region(start, 0));
    assert!(region.contains_region(start.wrapping_add(10), 0));
    assert!(region.contains_region(end.wrapping_sub(1), 0));
  }
  OK
}

#[test]
fn address_region_test_get_overlap() -> Void {
  const REGION: AddressRegion = AddressRegion::new(100, 20);
  // 无重叠
  assert_eq!(0, REGION.get_overlap(AddressRegion::new(0, 20)).size());
  assert_eq!(0, REGION.get_overlap(AddressRegion::new(120, 20)).size());
  // 相邻
  assert_eq!(0, REGION.get_overlap(AddressRegion::new(80, 20)).size());
  // 简单重叠
  assert_eq!(
    AddressRegion::new(100, 10),
    REGION.get_overlap(AddressRegion::new(90, 20))
  );
  assert_eq!(
    AddressRegion::new(110, 10),
    REGION.get_overlap(AddressRegion::new(110, 20))
  );
  // 完全包含
  assert_eq!(REGION, REGION.get_overlap(AddressRegion::new(90, 40)));
  // 相同
  assert_eq!(REGION, REGION.get_overlap(REGION));
  // 零大小区间
  assert_eq!(0, REGION.get_overlap(AddressRegion::new(0, 0)).size());
  assert_eq!(0, REGION.get_overlap(AddressRegion::new(100, 0)).size());
  assert_eq!(0, REGION.get_overlap(AddressRegion::new(110, 0)).size());
  assert_eq!(0, REGION.get_overlap(AddressRegion::new(120, 0)).size());
  OK
}

#[test]
fn address_region_test_constexpr() -> Void {
  // 原版 static_assert — Rust 以 const 断言承接 (字段访问走 const fn)
  const A: AddressRegion = AddressRegion::new(0, 20);
  const B: AddressRegion = AddressRegion::new(0, 20);
  const _: () = assert!(A.begin() == B.begin() && A.size() == B.size());
  const _: () = assert!(A.size() == 20);
  const _: () = assert!(A.get_overlap(AddressRegion::new(20, 20)).size() == 0);
  OK
}
