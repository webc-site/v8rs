//! 审查环 round4 补强用例 (对抗审查, 与前轮无上下文交叉)
//!
//! 针对第 4 轮审查发现的 3 项容量/记账口径缺陷 (同类于 R3-4/R3-5):
//! 1. from_slice (initializer_list ctor) 容量漂移
//! 2. assign_from 跨 zone 未按原版落入 realloc 分支
//! 3. assign_from realloc 分支旧存储未 DeleteArray (freed 记账漏计)

use aok::{OK, Void};
use v8_zone::{AccountingAllocator, Zone, ZoneVector};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 ZoneVector(initializer_list, Zone*) (zone-containers.h:87-96)
/// 直接 AllocateArray(size): capacity 恰为 size, 不经 kMinCapacity=2 抬升。
#[test]
fn zone_vector_from_slice_exact_capacity() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "from_slice_cap");
  let v1 = ZoneVector::from_slice(&zone, &[7u32])?;
  assert_eq!(v1.size(), 1);
  assert_eq!(
    v1.capacity(),
    1,
    "缺陷: from_slice(1) 容量被 kMinCapacity 抬到 2, 原版恰为 1"
  );
  let v5 = ZoneVector::from_slice(&zone, &[1u32, 2, 3, 4, 5])?;
  assert_eq!(v5.capacity(), 5, "原版 initializer 构造容量恰为 size");
  assert_eq!(v5.as_slice(), &[1, 2, 3, 4, 5]);
  OK
}

/// 原版 operator=(const ZoneVector&) (zone-containers.h:133) 就地覆盖分支
/// 要求 `capacity() >= other.size() && zone_ == other.zone_`; 跨 zone 必须落入
/// realloc 分支 (zone-containers.h:153-161), 容量按 other.capacity() 精确分配。
#[test]
fn zone_vector_assign_from_cross_zone_takes_realloc_branch() -> Void {
  let allocator = AccountingAllocator::new();
  let zone1 = Zone::new(&allocator, "assign_z1");
  let zone2 = Zone::new(&allocator, "assign_z2");
  // zone1: push 5 个 → 高水位 cap 2→4→8
  let mut v: ZoneVector<'_, '_, u32> = ZoneVector::new(&zone1);
  for i in 1..=5u32 {
    v.push_back(i)?;
  }
  assert_eq!(v.capacity(), 8);
  // zone2: initializer 构造 3 个 → cap 恰 3
  let other = ZoneVector::from_slice(&zone2, &[9u32, 8, 7])?;
  assert_eq!(other.capacity(), 3);
  // 容量足够 (8 >= 3) 但跨 zone → 原版走 realloc 分支: cap == other.capacity()
  v.assign_from(&other)?;
  assert_eq!(v.as_slice(), &[9, 8, 7]);
  assert_eq!(
    v.capacity(),
    3,
    "缺陷: 跨 zone assign 走了就地覆盖分支, 原版按 other.capacity() 在本 zone 重新分配"
  );
  OK
}

/// 原版 operator= realloc 分支 (zone-containers.h:151-152) 析构元素后
/// DeleteArray(data_, capacity()) 释放旧存储; 漏调则 freed_size_for_tracing
/// 漏计 (precise-zone-stats 下外部可观察)。
#[test]
fn zone_vector_assign_from_realloc_frees_old_storage() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "assign_free");
  // cap 4, 记账 RoundUp(16,8) = 16
  let mut v = ZoneVector::<u32>::with_default(&zone, 4)?;
  // cap 5, 记账 20; 4 < 5 → realloc 分支
  let other = ZoneVector::from_slice(&zone, &[1u32, 2, 3, 4, 5])?;
  v.assign_from(&other)?;
  assert_eq!(v.as_slice(), &[1, 2, 3, 4, 5]);
  assert_eq!(v.capacity(), 5);
  if cfg!(feature = "precise-zone-stats") {
    // 旧块 4 * 4 = 16 字节必须计入 freed (zone-containers.h:152 DeleteArray)
    assert_eq!(
      zone.freed_size_for_tracing(),
      16,
      "缺陷: realloc 分支旧存储未 DeleteArray, freed 记账漏计"
    );
    // tracing 总量: 16 (with_default) + RoundUp(20,8)=24 (from_slice) + 24 (realloc 新块)
    assert_eq!(zone.allocation_size_for_tracing(), 64);
  }
  // 同 zone 就地分支不得释放存储: 再做一次容量足够的同 zone assign
  let mut dst = ZoneVector::from_slice(&zone, &[0u32; 3])?;
  let src = ZoneVector::from_slice(&zone, &[5u32, 6, 7])?;
  let freed_before = zone.freed_size_for_tracing();
  dst.assign_from(&src)?;
  assert_eq!(
    zone.freed_size_for_tracing(),
    freed_before,
    "就地覆盖分支 (capacity >= size 且同 zone) 不释放存储 (zone-containers.h:133)"
  );
  OK
}
