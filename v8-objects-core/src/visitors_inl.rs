// 由 tool/src/objects/visitors-inl.h.js 从 v8/src/objects/visitors-inl.h 生成; 修订就地改, 复跑覆盖
// 客户端访客共享堆过滤语义; trait/Root 表在 visitors.rs

//! 访客内联实现 (原版 visitors-inl.h)
//!
//! 客户端 (client isolate) 访客在多 isolate 共享堆下只把"可共享对象"
//! 转发给真实访客: `IsSharedHeapObject = IsHeapObject && InWritableSharedSpace`.

use crate::{
  heap_object::HeapObject,
  object_predicates_inl::is_shared,
  tagged::{Object, Tagged},
};

/// cage 基址对 (压缩开: 主 cage + 代码 cage; 基线非压缩为空对)
#[derive(Debug, Clone, Copy, Default)]
pub struct CageBases {
  /// 主指针压缩 cage 基址 (非压缩基线 0)
  pub cage_base: usize,
  /// 代码 cage 基址 (V8_EXTERNAL_CODE_SPACE 开时区别于主 cage)
  pub code_cage_base: usize,
}

impl CageBases {
  /// 双 cage 直构 (原版 ObjectVisitorWithCageBases(cage_base, code_cage_base))
  #[inline]
  pub const fn new(cage_base: usize, code_cage_base: usize) -> Self {
    Self {
      cage_base,
      code_cage_base,
    }
  }

  /// 从 Isolate 取双 cage (原版 ObjectVisitorWithCageBases(Isolate*))
  #[inline]
  pub const fn from_isolate(cage_base: usize, code_cage_base: usize) -> Self {
    Self::new(cage_base, code_cage_base)
  }

  /// 从 Heap 取 (原版 ObjectVisitorWithCageBases(Heap*): 经 Isolate::FromHeap)
  #[inline]
  pub const fn from_heap(cage_base: usize, code_cage_base: usize) -> Self {
    Self::from_isolate(cage_base, code_cage_base)
  }
}

/// 共享堆对象判定 (原版 ClientRootVisitor/ClientObjectVisitor::IsSharedHeapObject, 静态)
#[inline]
pub fn is_shared_heap_object(object: Tagged<Object>) -> bool {
  if !object.is_heap_object() {
    return false;
  }
  is_shared(object)
}

/// 运行中代码根访问 (客户端版仅 DEBUG 断言: 代码槽不得在可写共享空间)
#[inline]
pub fn client_root_visitor_visit_running_code(_code_slot: usize, _maybe_istream_slot: usize) {
  // DEBUG 下双槽 InWritableSharedSpace 断言, release 空实现
}

/// map 指针访问 (客户端版: 宿主 map 非共享则不转发)
#[inline]
pub fn client_object_visitor_visit_map_pointer<F>(host: Tagged<HeapObject>, mut actual_visitor: F)
where
  F: FnMut(Tagged<HeapObject>),
{
  unsafe {
    let map = host.map();
    if is_shared_heap_object(map.cast()) {
      actual_visitor(host);
    }
  }
}

/// 指令流指针访问 (客户端版仅 DEBUG 断言: 指令流不可在可写共享空间)
#[inline]
pub fn client_object_visitor_visit_instruction_stream_pointer(_host: usize, _slot: usize) {
  // DEBUG 断言 InWritableSharedSpace(istream) 不成立
}

/// 代码目标访问 (客户端版仅 DEBUG 断言: 重定位目标不可在可写共享空间)
#[inline]
pub fn client_object_visitor_visit_code_target(_host: usize, _rinfo: usize) {
  // DEBUG 断言 target 不在 InWritableSharedSpace
}

/// 嵌入指针访问 (客户端版: 目标非共享则不转发)
#[inline]
pub fn client_object_visitor_visit_embedded_pointer<F>(
  target: Tagged<Object>,
  mut actual_visitor: F,
) where
  F: FnMut(Tagged<Object>),
{
  if is_shared_heap_object(target) {
    actual_visitor(target);
  }
}
