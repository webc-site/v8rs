// 由 tool/src/heap/cppgc-internal/liveness-broker.js 从 v8/src/heap/cppgc-internal/liveness-broker.h + src/heap/cppgc-internal/liveness-broker.cc 生成; 修订就地改, 复跑覆盖
// 存活查询: 标记位即存活

use core::ffi::c_void;

use crate::{globals::AccessMode, heap_object_header::HeapObjectHeader};

/// v8: src/heap/cppgc-internal/liveness-broker.h LivenessBroker
/// 存活查询门面; 完整版依赖标记状态 (随 marker 批次), 本面提供句柄身份与默认构造。
#[derive(Default)]
pub struct LivenessBroker;

impl LivenessBroker {
  /// v8: src/heap/cppgc-internal/liveness-broker.cc LivenessBroker::IsHeapObjectAliveImpl
  ///
  /// # Safety
  /// `payload` 须指向已分配对象的 payload 起点 (其前置为有效 HeapObjectHeader)。
  pub unsafe fn is_heap_object_alive_impl(&self, payload: *const c_void) -> bool {
    // SAFETY: 由调用契约保证。
    unsafe { is_heap_object_alive_impl(payload) }
  }
}

/// v8: src/heap/cppgc-internal/liveness-broker.cc LivenessBroker::IsHeapObjectAliveImpl
///
/// # Safety
/// `payload` 须指向已分配对象的 payload 起点 (其前置为有效 HeapObjectHeader)。
pub unsafe fn is_heap_object_alive_impl(payload: *const c_void) -> bool {
  // v8: HeapObjectHeader::FromObject(payload).IsMarked()
  // SAFETY: 调用契约保证 payload 前置 8 字节为活对象头。
  unsafe { HeapObjectHeader::from_object(payload) }.is_marked(AccessMode::NonAtomic)
}

/// v8: src/heap/cppgc-internal/liveness-broker.h LivenessBrokerFactory
pub struct LivenessBrokerFactory;

impl LivenessBrokerFactory {
  /// v8: src/heap/cppgc-internal/liveness-broker.cc LivenessBrokerFactory::Create
  pub fn create() -> LivenessBroker {
    LivenessBroker
  }
}
