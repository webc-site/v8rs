//! broker 内联件 — FindRootIndex 排除规则 + 递归互斥守卫
//!
//! 1:1 对应 v8/src/compiler/js-heap-broker-inl.h。守卫类绑定 Isolate 互斥锁,
//! 执行面 (v8-execution-isolate) 未接入, 构造推迟; 语义纪律见各锚点。

use v8_compiler_broker::JSHeapBroker;

/// FindRootIndex — 根索引反查, 三条规则 (js-heap-broker-inl.h):
/// 1. RO 空间判定: 静态根 (V8_STATIC_ROOTS_BOOL) 走 IsInMainCageBase() &&
///    ptr <= StaticReadOnlyRoot::kLastAllocatedRoot, 否则 HeapLayout::InReadOnlySpace
/// 2. 非 RO 且 IsJSReceiver → None (无根常量是 JSReceiver, RO 内无 JSReceiver)
/// 3. root_index_map_.Lookup 命中才 Some
///
/// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::FindRootIndex
pub fn find_root_index<'z>(
  _broker: &JSHeapBroker<'z, '_>,
  _object: v8_compiler_broker::HeapObjectRef<'z>,
) -> Option<u32> {
  todo!() // 推迟台账: RootIndexMap 查表 (v8-roots / broker 根映射面)
}

/// 递归互斥守卫 — depth==0 才真正上锁, 否则只递增计数
/// 成员序: mutex_depth_address → initial_mutex_depth → mutex_guard;
/// 构造时 (*mutex_depth_address_)++
/// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::RecursiveMutexGuardIfNeeded
pub struct RecursiveMutexGuardIfNeeded {
  /// mutex_depth_address_ (指向 broker 内深度计数)
  pub mutex_depth_address: *mut i32,
  /// initial_mutex_depth_ (上锁判定: ==0)
  pub initial_mutex_depth: i32,
}

impl RecursiveMutexGuardIfNeeded {
  /// v8: src/compiler/js-heap-broker-inl.h RecursiveMutexGuardIfNeeded 构造
  pub fn new(_depth_address: *mut i32) -> crate::Result<Self> {
    todo!() // 推迟台账: LocalIsolate parked_mutex 上锁面 (v8-execution-isolate)
  }
}

/// MapUpdater 访问守卫 — 递归转发 broker->isolate()->map_updater_access()
/// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::MapUpdaterGuardIfNeeded
pub struct MapUpdaterGuardIfNeeded {
  /// inner_
  pub inner: RecursiveMutexGuardIfNeeded,
}

/// Boilerplate 迁移守卫 — 递归转发 boilerplate_migration_access()
/// v8: src/compiler/js-heap-broker-inl.h JSHeapBroker::BoilerplateMigrationGuardIfNeeded
pub struct BoilerplateMigrationGuardIfNeeded {
  /// inner_
  pub inner: RecursiveMutexGuardIfNeeded,
}
