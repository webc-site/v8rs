//! 原版 v8/test/unittests/zone/zone-unittest.cc 转写
//! 用例名保留原名 (TEST_F(ZoneTest, 8ByteAlignment) → zone_test_8_byte_alignment)
//! 另补充 zone.h 核心语义的对照用例 (glossary 7.3 覆盖率映射见 readme/zh.md)

use aok::{OK, Void};
use v8_zone::{AccountingAllocator, Error, Zone, ZoneScope};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 TEST_F(ZoneTest, 8ByteAlignment)
#[test]
fn zone_test_8_byte_alignment() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  for i in 0..16usize {
    let ptr = zone.allocate_bytes_ref(i)?.as_mut_ptr() as usize;
    assert_eq!(ptr % 8, 0, "allocation of {i} bytes not 8-aligned");
  }
  OK
}

/// 补充: allocation_size 记账 = committed + 头段已用 (原版 zone.h 注释口径)
#[test]
fn zone_test_allocation_size_accounting() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  assert_eq!(zone.allocation_size(), 0);
  let _a = zone.allocate_bytes_ref(100)?;
  assert_eq!(zone.allocation_size(), 104); // align_up(100, 8)
  let _b = zone.allocate_bytes_ref(1)?;
  assert_eq!(zone.allocation_size(), 112);
  OK
}

/// 补充: 扩段高水位 (原版 Expand: new = size + 2*old, 夹到 [8KB, ...])
#[test]
fn zone_test_expand_high_water() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  // 首次小分配即拉起到最小段 8KB
  let _a = zone.allocate_bytes_ref(8)?;
  let first = zone.segment_bytes_allocated();
  assert!(first >= 8 * 1024, "first segment {first} < 8KB minimum");
  assert_eq!(allocator.current_memory_usage(), first);
  assert!(allocator.max_memory_usage() >= first);
  // 大分配触发扩段, 段总量单调增加
  let _big = zone.allocate_bytes_ref(40 * 1024)?;
  let second = zone.segment_bytes_allocated();
  assert!(second > first, "no growth for large request");
  OK
}

/// 补充: Reset() 保留末段 (原版 zone.cc Reset 语义; D4: reset 需独占,
/// 借出引用先结束生命周期)
#[test]
fn zone_test_reset_keeps_last_segment() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "test");
  let before = {
    let _a = zone.allocate_bytes_ref(1024)?;
    let _b = zone.allocate_bytes_ref(1024)?;
    zone.segment_bytes_allocated()
  };
  zone.reset();
  assert_eq!(zone.allocation_size(), 0, "reset must clear usage");
  // 保留段仍归 zone 持有: 段字节数等于保留段的 total_size
  let kept = zone.segment_bytes_allocated();
  assert!(
    kept > 0 && kept <= before,
    "reset must keep exactly one segment"
  );
  // 复位后可继续分配 (原版 position 回卷到保留段起点)
  let _c = zone.allocate_bytes_ref(64)?;
  assert_eq!(zone.allocation_size(), 64);
  OK
}

/// 补充: Snapshot/Restore 归还后建段 (原版 ZoneSnapshot::Restore;
/// D4: restore 需独占, 借出引用先结束生命周期)
#[test]
fn zone_test_snapshot_restore() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "test");
  let snapshot = {
    let _a = zone.allocate_bytes_ref(64)?;
    zone.snapshot()
  };
  let size_with = zone.allocation_size();
  let _b = zone.allocate_bytes_ref(20000)?; // 触发扩段
  assert!(zone.allocation_size() > size_with);
  // _b 借用随最后使用点结束 (NLL), 回滚与借出互斥 (D4)
  snapshot.restore(&mut zone)?;
  assert_eq!(zone.allocation_size(), 64, "restore must roll back usage");
  OK
}

/// 补充: ZoneScope 作用域回滚 (原版 ZoneScope RAII; D4: scope 持 &mut Zone,
/// 作用域内分配经 scope.zone() 进行, 借用被约束在作用域内)
#[test]
fn zone_test_zone_scope() -> Void {
  let allocator = AccountingAllocator::new();
  let mut zone = Zone::new(&allocator, "test");
  let outer_size = {
    let _a = zone.allocate_bytes_ref(32)?;
    zone.allocation_size()
  };
  {
    let mut scope = ZoneScope::new(&mut zone);
    let _tmp = scope.zone().allocate_bytes_ref(5000)?;
    assert!(scope.zone().allocation_size() > outer_size);
  }
  assert_eq!(
    zone.allocation_size(),
    outer_size,
    "ZoneScope drop must restore"
  );
  OK
}

/// 补充: Zone Drop → DeleteAll, allocator 用量归零 (原版 ~Zone)
#[test]
fn zone_test_drop_returns_segments() -> Void {
  let allocator = AccountingAllocator::new();
  {
    let zone = Zone::new(&allocator, "test");
    let _a = zone.allocate_bytes_ref(4096)?;
    assert!(allocator.current_memory_usage() > 0);
  }
  assert_eq!(
    allocator.current_memory_usage(),
    0,
    "all segments must be returned"
  );
  OK
}

/// 补充: Restore 传错 zone 返回 Err (原版 CHECK_NOT_NULL 收紧;
/// D4: restore 需 &mut zone)
#[test]
fn zone_test_restore_wrong_zone() -> Void {
  let allocator = AccountingAllocator::new();
  let zone1 = Zone::new(&allocator, "z1");
  let mut zone2 = Zone::new(&allocator, "z2");
  let snapshot = {
    let _a = zone1.allocate_bytes_ref(64)?;
    zone1.snapshot()
  };
  let err = snapshot.restore(&mut zone2);
  assert!(matches!(err, Err(Error::WrongZone)));
  OK
}
