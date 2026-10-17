//! 原版 type-stats.h/.cc 语义用例 (V8_ENABLE_PRECISE_ZONE_STATS,
//! 对应 AddAllocated/AddDeallocated/MergeWith/Dump 与
//! accounting-allocator.cc:162-175 的并账路径)

#[cfg(feature = "precise-zone-stats")]
use std::any::type_name;

use aok::{OK, Void};
use v8_zone::{AccountingAllocator, Zone, zone_stats_enabled};
#[cfg(feature = "precise-zone-stats")]
use v8_zone::{
  TypeStats, ZoneVector, set_trace_zone_type_stats_enabled, set_zone_stats_enabled,
  trace_zone_type_stats_enabled,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

/// 原版 AddAllocated<TypeTag>/AddDeallocated<TypeTag> 分类账:
/// make<T> 记 T, allocate_array 记 [T], 容器归还记 freed
#[test]
fn zone_type_stats_alloc_dealloc_per_tag() -> Void {
  if !cfg!(feature = "precise-zone-stats") {
    return OK;
  }
  #[cfg(feature = "precise-zone-stats")]
  {
    set_zone_stats_enabled(true);
    let allocator = AccountingAllocator::new();
    let zone = Zone::new(&allocator, "stats");
    let _v = zone.make(0x1234_u32)?;
    let _arr = zone.make_array_with::<u32>(2, &7)?;
    {
      let stats = zone.type_stats();
      let u32_entry = stats.entry(type_name::<u32>()).expect("u32 条目");
      assert_eq!(u32_entry.instance_size, 4);
      // make<u32>: RoundUp(4, 8) = 8 字节, 计数 1 (TypeTag = u32)
      assert_eq!(u32_entry.allocated_bytes, 8);
      assert_eq!(u32_entry.allocation_count, 1);
      let arr_entry = stats.entry(type_name::<[u32]>()).expect("[u32] 条目");
      assert_eq!(arr_entry.instance_size, 1); // 不完整类型记 sizeof(char)
      assert_eq!(arr_entry.allocated_bytes, 8);
      assert_eq!(arr_entry.allocation_count, 1);
    }
    // 容器扩容归还旧块 → deallocated 计入 [i32]
    let mut vec: ZoneVector<'_, '_, i32> = ZoneVector::with_default(&zone, 8)?;
    vec.push_back(9)?;
    {
      let stats = zone.type_stats();
      let arr_entry = stats.entry(type_name::<[i32]>()).expect("[i32] 条目");
      assert_eq!(arr_entry.deallocated_bytes, 32);
    }
    set_zone_stats_enabled(false);
  }
  OK
}

/// 原版 TypeStats::MergeWith: 逐条目累并 (accounting-allocator.cc:162-165)
#[test]
fn zone_type_stats_merge_with() -> Void {
  if !cfg!(feature = "precise-zone-stats") {
    return OK;
  }
  #[cfg(feature = "precise-zone-stats")]
  {
    set_zone_stats_enabled(true);
    let allocator = AccountingAllocator::new();
    let zone1 = Zone::new(&allocator, "s1");
    let _a = zone1.make(1_u64)?;
    let _b = zone1.make(2_u64)?;
    let zone2 = Zone::new(&allocator, "s2");
    let _c = zone2.make(3_u16)?;

    let mut merged = TypeStats::default();
    merged.merge_with(&zone1.type_stats());
    merged.merge_with(&zone2.type_stats());

    let u64_entry = merged.entry(type_name::<u64>()).expect("u64");
    assert_eq!(u64_entry.allocation_count, 2);
    assert_eq!(u64_entry.allocated_bytes, 16);
    let u16_entry = merged.entry(type_name::<u16>()).expect("u16");
    // RoundUp(2, 8) = 8
    assert_eq!(u16_entry.allocated_bytes, 8);
    set_zone_stats_enabled(false);
  }
  OK
}

/// 原版 TraceZoneDestruction 的 type_stats 并账与收尾 Dump
/// (accounting-allocator.cc:162-175): tracing 分配器在 zone 析构时并账,
/// 活跃集清空后 Dump 总表 (trace_zone_type_stats 开关控制)
#[test]
fn zone_type_stats_tracing_merge_and_dump() -> Void {
  if !cfg!(feature = "precise-zone-stats") {
    return OK;
  }
  #[cfg(feature = "precise-zone-stats")]
  {
    set_zone_stats_enabled(true);
    set_trace_zone_type_stats_enabled(true);
    assert!(trace_zone_type_stats_enabled());
    let allocator = AccountingAllocator::with_default_tolerance(false);
    {
      let zone = Zone::new(&allocator, "traced");
      let _a = zone.make(7_u32)?;
    } // zone drop → TraceZoneDestruction → MergeWith → (活跃集空) Dump
    set_trace_zone_type_stats_enabled(false);
    set_zone_stats_enabled(false);
  }
  OK
}

/// 非 tracing 默认路径不受影响 (zone_stats_enabled 关闭时不记账)
#[test]
fn zone_type_stats_disabled_by_default() -> Void {
  let allocator = AccountingAllocator::new();
  let zone = Zone::new(&allocator, "plain");
  let _a = zone.make(1_u32)?;
  assert!(!zone_stats_enabled() || cfg!(feature = "precise-zone-stats"));
  OK
}
