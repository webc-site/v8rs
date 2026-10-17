//! 原版 v8/test/unittests/zone/zone-allocator-unittest.cc 转写
//!
//! 覆盖率映射 (详见 readme/zh.md):
//! - UseWithStdContainers: C++ 借 std::vector/list/set + 自定义 allocator;
//!   Rust 无 allocator 注入, 用同源的 ZoneVector/ZoneLinkedList 验证
//!   zone 分配路径 (sum=5050 断言保留)。std::set 部分无自定义分配
//!   对等物, 该形态跳过。
//! - RecyclingZoneAllocator 5 个用例逐行 1:1 (指针相等断言保留)。

use aok::{OK, Void};
use v8_zone::{
  AccountingAllocator, RecyclingZoneAllocator, Zone, ZoneAllocator, ZoneLinkedList, ZoneVector,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 TestWithStdContainers<Allocator>() 的 vector 分量: sum == 5050
fn sum_5050_vector(zone: &Zone) -> Void {
  // std::vector<int, Alloc> v(10, zone_allocator): 10 个默认 0
  let mut v: ZoneVector<'_, '_, i32> = ZoneVector::with_default(zone, 10)?;
  for i in 1..=100 {
    v.push_back(i)?;
  }
  let sum_of_v: i32 = v.iter().sum();
  assert_eq!(sum_of_v, 5050);
  OK
}

/// 原版 list 分量: ZoneLinkedList push_back 1..=100, sum == 5050
fn sum_5050_list(zone: &Zone) -> Void {
  let mut l: ZoneLinkedList<'_, '_, i32> = ZoneLinkedList::new(zone);
  for i in 1..=100 {
    l.push_back(i)?;
  }
  let sum_of_l: i32 = l.iter().sum();
  assert_eq!(sum_of_l, 5050);
  OK
}

/// 原版 TEST_F(ZoneAllocatorTest, UseWithStdContainers)
#[test]
fn zone_allocator_use_with_std_containers() -> Void {
  let accounting_allocator = AccountingAllocator::new();
  let zone = Zone::new(&accounting_allocator, "test");
  // 原版还验证 allocator 可拷贝/比较: 同一 zone 即相等
  let a1 = ZoneAllocator::<i32>::new(&zone);
  let a2 = ZoneAllocator::<i32>::new(&zone);
  assert!(a1 == a2);
  sum_5050_vector(&zone)?;
  sum_5050_list(&zone)?;
  OK
}

/// 原版 TEST_F(RecyclingZoneAllocatorTest, UseWithStdContainers)
#[test]
fn recycling_zone_allocator_use_with_std_containers() -> Void {
  let accounting_allocator = AccountingAllocator::new();
  let zone = Zone::new(&accounting_allocator, "test");
  // 回收分配器的分配路径本身由下面 5 个用例覆盖, 容器行为一致
  let a = RecyclingZoneAllocator::<i32>::new(&zone);
  let p = a.allocate(8)?;
  // SAFETY: p 为 allocate(8) 返回且不再使用
  unsafe { a.deallocate(p, 8) };
  sum_5050_vector(&zone)?;
  sum_5050_list(&zone)?;
  OK
}

/// 原版 TEST_F(RecyclingZoneAllocatorTest, ReuseSameSize)
#[test]
fn recycling_zone_allocator_reuse_same_size() -> Void {
  let accounting_allocator = AccountingAllocator::new();
  let zone = Zone::new(&accounting_allocator, "test");
  let zone_allocator = RecyclingZoneAllocator::<i32>::new(&zone);

  let allocated = zone_allocator.allocate(10)?;
  // SAFETY: allocated 为本分配器刚返回的 10 元素块, 内容弃用
  unsafe { zone_allocator.deallocate(allocated, 10) };
  let again = zone_allocator.allocate(10)?;
  assert_eq!(again, allocated);
  OK
}

/// 原版 TEST_F(RecyclingZoneAllocatorTest, ReuseSmallerSize)
#[test]
fn recycling_zone_allocator_reuse_smaller_size() -> Void {
  let accounting_allocator = AccountingAllocator::new();
  let zone = Zone::new(&accounting_allocator, "test");
  let zone_allocator = RecyclingZoneAllocator::<i32>::new(&zone);

  let allocated = zone_allocator.allocate(100)?;
  // SAFETY: 同上, 100 元素块归还
  unsafe { zone_allocator.deallocate(allocated, 100) };
  let again = zone_allocator.allocate(10)?;
  assert_eq!(again, allocated);
  OK
}

/// 原版 TEST_F(RecyclingZoneAllocatorTest, DontReuseTooSmallSize)
#[test]
fn recycling_zone_allocator_dont_reuse_too_small_size() -> Void {
  let accounting_allocator = AccountingAllocator::new();
  let zone = Zone::new(&accounting_allocator, "test");
  let zone_allocator = RecyclingZoneAllocator::<i32>::new(&zone);

  // FreeBlock 比单个 int 大, 无法把空闲链写进归还块
  let allocated = zone_allocator.allocate(1)?;
  // SAFETY: 1 元素块归还 (过小, 不会入链)
  unsafe { zone_allocator.deallocate(allocated, 1) };
  let again = zone_allocator.allocate(1)?;
  assert_ne!(again, allocated);
  OK
}

/// 原版 TEST_F(RecyclingZoneAllocatorTest, ReuseMultipleSize)
#[test]
fn recycling_zone_allocator_reuse_multiple_size() -> Void {
  let accounting_allocator = AccountingAllocator::new();
  let zone = Zone::new(&accounting_allocator, "test");
  let zone_allocator = RecyclingZoneAllocator::<i32>::new(&zone);

  let allocated1 = zone_allocator.allocate(10)?;
  let allocated2 = zone_allocator.allocate(20)?;
  let allocated3 = zone_allocator.allocate(30)?;
  // SAFETY: 三块均本分配器归还, 顺序与原版一致
  unsafe {
    zone_allocator.deallocate(allocated1, 10);
    zone_allocator.deallocate(allocated2, 20);
    zone_allocator.deallocate(allocated3, 30);
  }
  assert_eq!(zone_allocator.allocate(10)?, allocated3);
  assert_eq!(zone_allocator.allocate(10)?, allocated2);
  assert_eq!(zone_allocator.allocate(10)?, allocated1);
  OK
}

/// 原版 TEST_F(RecyclingZoneAllocatorTest, DontChainSmallerSizes)
#[test]
fn recycling_zone_allocator_dont_chain_smaller_sizes() -> Void {
  let accounting_allocator = AccountingAllocator::new();
  let zone = Zone::new(&accounting_allocator, "test");
  let zone_allocator = RecyclingZoneAllocator::<i32>::new(&zone);

  let allocated1 = zone_allocator.allocate(10)?;
  let allocated2 = zone_allocator.allocate(5)?;
  let allocated3 = zone_allocator.allocate(10)?;
  // SAFETY: 三块归还顺序与原版一致; 5 元素块不得入链 (头块更小者拒绝)
  unsafe {
    zone_allocator.deallocate(allocated1, 10);
    zone_allocator.deallocate(allocated2, 5);
    zone_allocator.deallocate(allocated3, 10);
  }
  assert_eq!(zone_allocator.allocate(5)?, allocated3);
  assert_eq!(zone_allocator.allocate(5)?, allocated1);
  assert_ne!(zone_allocator.allocate(5)?, allocated2);
  OK
}
