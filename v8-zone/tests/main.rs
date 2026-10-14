//! 原版 zone.h / zone-allocator.h / zone-type-traits.h 补充用例
//! (对应 New<T>/NewVector/CloneVector/Seal/Delete 记账与类型别名面;
//! 覆盖率映射见 readme/zh.md)

use aok::{OK, Void};
use log::info;
use v8_zone::{
  AccountingAllocator, IntVector, Zone, ZoneBoolAllocator, ZoneIntAllocator, ZoneList, ZonePtrList,
  ZoneVector,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 zone.h New<T>/NewVector(len)/NewVector(len,value)/CloneVector 形态
#[test]
fn zone_test_make_and_vector_forms() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  // New<T>: 分配 + 构造
  let v = zone.make(0x1234_u32)?;
  assert_eq!(*v, 0x1234);
  // NewVector(length): 默认值填充
  let d = zone.make_array_default::<i32>(3)?;
  assert_eq!(d, &[0, 0, 0]);
  // NewVector(length, value): 定值填充
  let f = zone.make_array_with::<i32>(2, &7)?;
  assert_eq!(f, &[7, 7]);
  // CloneVector: 复制构造
  let c = zone.clone_vector(&[1, 2, 3])?;
  assert_eq!(c, &[1, 2, 3]);
  // 每笔按 RoundUp(size, 8) 抬升 position (原版 zone.h:59)
  assert_eq!(zone.allocation_size(), 8 + 16 + 8 + 16);
  info!("zone make forms ok");
  OK
}

/// 原版 Zone::Seal (zone.h:166) 标志语义
#[test]
fn zone_test_seal_flag() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  assert!(!zone.is_sealed());
  zone.seal();
  assert!(zone.is_sealed());
  OK
}

/// 原版 Delete 记账: freed 计数仅在 V8_ENABLE_PRECISE_ZONE_STATS 下累计
/// (zone.h:79-100; 非精确模式 freed_size_for_tracing 恒 0, zone.h:198)
#[test]
fn zone_test_delete_freed_tracing() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");
  let mut v: ZoneVector<'_, '_, i32> = ZoneVector::with_default(&zone, 8)?;
  // 触发 EnsureOneMoreCapacity → Grow → DeleteArray(旧存储) (zone-containers.h)
  v.push_back(9)?;
  assert_eq!(v.size(), 9);
  assert_eq!(v.capacity(), 16);
  if cfg!(feature = "precise-zone-stats") {
    // 旧块 RoundUp(8 * sizeof(i32), 8) = 32 计入 freed
    assert_eq!(zone.freed_size_for_tracing(), 32);
    // tracing 总量含全部分配 (32 + 64)
    assert_eq!(zone.allocation_size_for_tracing(), 96);
  } else {
    assert_eq!(zone.freed_size_for_tracing(), 0);
  }
  OK
}

/// 原版 zone-allocator.h:105-106 别名 + zone-type-traits.h:19 ZonePtrList
#[test]
fn zone_test_type_aliases() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "test");

  let ba = ZoneBoolAllocator::new(&zone);
  let p = ba.allocate(1)?;
  // SAFETY: p 为 allocate(1) 刚返回且不再引用
  unsafe { ba.deallocate(p, 1) };
  let ia = ZoneIntAllocator::new(&zone);
  let q = ia.allocate(2)?;
  // SAFETY: q 为 allocate(2) 刚返回且不再引用
  unsafe { ia.deallocate(q, 2) };

  let mut iv: IntVector<'_, '_> = ZoneVector::new(&zone);
  for i in 1..=4 {
    iv.push_back(i)?;
  }
  assert_eq!(iv.iter().sum::<i32>(), 10);

  // ZonePtrList = ZoneList<T*>: 存放指向 zone 内对象的裸指针
  let mut pl: ZonePtrList<'_, '_, i32> = ZoneList::new(2, &zone)?;
  let held = zone.make(42_i32)?;
  let held_ptr: *mut i32 = held;
  pl.add(&held_ptr)?;
  assert_eq!(pl.length(), 1);
  // SAFETY: 指针指向本 zone 内存 (原版 ZoneObject* 语义)
  assert_eq!(unsafe { **pl.first() }, 42);
  OK
}
