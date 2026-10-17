// 由 tool/src/objects/heap-object-set-map-inl.h.js 从 v8/src/objects/heap-object-set-map-inl.h 生成; 修订就地改, 复跑覆盖
// map 写入族骨架 23 定义; 写屏障/GC 通知语义归 v8-heap-gc

//! HeapObject 的 map 字段写入族 + AllocationWitness 分配见证。
//!
//! 写入矩阵: {默认 | safe_transition} × {带屏障 | 无屏障} ×
//! {Relaxed | Release} × {潜在布局变化 | 安全 map 迁移}; 核心模板
//! `set_map<EmitWriteBarrier, MemoryOrder, IsolateT>` 统一收口。
//! 写屏障 (WriteBarrier::ForValue) 与堆验证 (HeapVerifier) 归 v8-heap-gc。

use std::sync::atomic::{AtomicUsize, Ordering};

use v8_common::AllocationType;

use crate::{
  heap_object::HeapObject,
  map::Map,
  map_word::MapWord,
  tagged::Tagged,
  tagged_constants::HEAP_OBJECT_TAG,
  tagged_field::{TaggedField, full_to_tagged},
};

/// map 写入的验证模式 (heap-object.h enum class VerificationMode)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VerificationMode {
  /// 安全 map 迁移: 对象布局不变, 并发标记器可见。
  SafeMapTransition,
  /// 潜在布局变化: 需要完整验证。
  PotentialLayoutChange,
}

/// 是否发射写屏障 (heap-object.h enum class EmitWriteBarrier)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EmitWriteBarrier {
  Yes,
  No,
}

/// 内存序标签 (原版 RelaxedStoreTag/ReleaseStoreTag 的骨架映射)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StoreOrder {
  Relaxed,
  Release,
}

/// 分配见证: 携带只读根 + 该分配适用的写屏障模式。
#[derive(Debug, Clone, Copy)]
pub struct AllocationWitness {
  /// 分配对象地址。
  pub object: usize,
  /// 写屏障模式 (SKIP/UPDATE)。
  pub write_barrier_mode: u8,
}

/// 跳过写屏障模式常量
pub const SKIP_WRITE_BARRIER: u8 = 0;
/// 更新写屏障模式常量
pub const UPDATE_WRITE_BARRIER: u8 = 1;

impl AllocationWitness {
  /// 按分配类型推导写屏障模式 (只读/年轻代分配跳过屏障)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h AllocationWitness::WriteBarrierModeForAllocation
  #[inline]
  pub fn write_barrier_mode_for_allocation(_object: usize, allocation: u8) -> u8 {
    if allocation == AllocationType::ReadOnly as u8 || allocation == AllocationType::Young as u8 {
      SKIP_WRITE_BARRIER
    } else {
      UPDATE_WRITE_BARRIER
    }
  }
}

impl HeapObject {
  /// 写 map 字段 (默认: 带屏障 + relaxed + 潜在布局变化)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map
  #[inline]
  pub fn set_map_release(&mut self, value: Tagged<Map>, order: StoreOrder) {
    self.set_map_word(value, order);
  }

  /// 写 map 字段 (安全 map 迁移: 布局不变, 验证降级)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_safe_transition
  #[inline]
  pub fn set_map_safe_transition(&mut self, value: Tagged<Map>, order: StoreOrder) {
    self.set_map_word(value, order);
  }

  /// 写 map 字段, 无写屏障 (白对象/不可变值场景)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_no_write_barrier
  #[inline]
  pub fn set_map_no_write_barrier(&mut self, value: Tagged<Map>, order: StoreOrder) {
    self.set_map_word(value, order);
  }

  /// 写 map 字段 (安全迁移 + 无屏障)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_safe_transition_no_write_barrier
  #[inline]
  pub fn set_map_safe_transition_no_write_barrier(
    &mut self,
    value: Tagged<Map>,
    order: StoreOrder,
  ) {
    self.set_map_word(value, order);
  }

  /// 分配后立即初始化 map (Heap 专用, 屏障模式显式)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_after_allocation
  #[inline]
  pub fn set_map_after_allocation(&mut self, value: Tagged<Map>, _emit: EmitWriteBarrier) {
    self.set_map_word(value, StoreOrder::Relaxed);
  }

  /// 空闲空间填充 map 写入 (WritableFreeSpace 定向写)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::SetFillerMap
  #[inline]
  pub fn set_filler_map(writable_page: usize, value: Tagged<Map>) {
    let loc = writable_page as *const AtomicUsize;
    unsafe {
      (*loc).store(full_to_tagged(value.ptr()), Ordering::Relaxed);
    }
  }

  /// map 槽地址 (MapField 偏移 0)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::map_slot
  #[inline]
  pub fn map_slot(&self) -> usize {
    // 原版 ObjectSlot(MapField::address(this)): map 字段偏移 0, 即对象基址。
    self.field_address(0)
  }

  /// 直写 map word (非 map 语义: GC 期 map word 复用)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_word
  #[inline]
  pub fn set_map_word(&mut self, map: Tagged<Map>, order: StoreOrder) {
    let host = Tagged::<HeapObject>::new((self as *mut HeapObject as usize) | HEAP_OBJECT_TAG);
    let mw = MapWord::from_map(map);
    unsafe {
      match order {
        StoreOrder::Relaxed => TaggedField::<MapWord, 0>::relaxed_store_map_word(host, mw),
        StoreOrder::Release => TaggedField::<MapWord, 0>::release_store_map_word(host, mw),
      }
    }
  }

  /// 直写转发地址 (GC 拷贝期前向指针)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::set_map_word_forwarded
  #[inline]
  pub fn set_map_word_forwarded(&mut self, target: usize, order: StoreOrder) {
    let host = Tagged::<HeapObject>::new((self as *mut HeapObject as usize) | HEAP_OBJECT_TAG);
    let target_obj = Tagged::<HeapObject>::new(target);
    let mw = MapWord::from_forwarding_address(host, target_obj);
    unsafe {
      match order {
        StoreOrder::Relaxed => TaggedField::<MapWord, 0>::relaxed_store_map_word(host, mw),
        StoreOrder::Release => TaggedField::<MapWord, 0>::release_store_map_word(host, mw),
      }
    }
  }

  /// CAS 转发 map word (relaxed, 返回是否换成功)。
  ///
  /// v8: src/objects/heap-object-set-map-inl.h HeapObject::relaxed_compare_and_swap_map_word_forwarded
  #[inline]
  pub fn relaxed_compare_and_swap_map_word_forwarded(
    &mut self,
    old_map_word: u64,
    new_target: usize,
  ) -> bool {
    let host = Tagged::<HeapObject>::new((self as *mut HeapObject as usize) | HEAP_OBJECT_TAG);
    let target_obj = Tagged::<HeapObject>::new(new_target);
    let new_mw = MapWord::from_forwarding_address(host, target_obj);
    let loc = self.map_slot() as *const AtomicUsize;
    let old_tagged = full_to_tagged(old_map_word as usize);
    let new_tagged = full_to_tagged(new_mw.ptr());
    unsafe {
      (*loc)
        .compare_exchange(old_tagged, new_tagged, Ordering::Relaxed, Ordering::Relaxed)
        .is_ok()
    }
  }
}

/// 构造族清单 (map 写入矩阵之外的本文件构造定义)。
pub const CTOR_FAMILY: [&str; 5] = [
  "HeapObject::HeapObject(Tagged<ReadOnly<Map>>)", // 断言 map 非空, SKIP 屏障落 map
  "HeapObject::HeapObject(const AllocationWitness&, Tagged<Map>)", // 按 witness 屏障模式落 map
  "AllocationWitness::AllocationWitness(object, roots, allocation)", // 推导屏障模式
  "AllocationWitness::AllocationWitness(object, roots, write_barrier_mode)", // 显式屏障模式
  "PrimitiveHeapObject::PrimitiveHeapObject(Tagged<ReadOnly<Map>>)", // 转发基类构造
];
