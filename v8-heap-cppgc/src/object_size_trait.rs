// 由 tool/src/heap/cppgc-internal/object-size-trait.cc.js 从 v8/src/heap/cppgc-internal/object-size-trait.cc 生成; 修订就地改, 复跑覆盖
// BaseObjectSizeTrait 两静态尺寸查询 (声明面在 include/cppgc/object-size-trait.h)
// 推迟台账: mixin 臂依赖 BasePage::FromPayload + ObjectHeaderFromInnerAddress, 随 heap_page 批次挂载。

use core::ffi::c_void;

use crate::{globals::AccessMode, heap_object_header::HeapObjectHeader, object_view::ObjectView};

/// v8: src/heap/cppgc-internal/object-size-trait.cc:15-31 BaseObjectSizeTrait
/// 两臂均 AccessMode::kAtomic (并发安全读)。
pub struct BaseObjectSizeTrait;

impl BaseObjectSizeTrait {
  /// v8: src/heap/cppgc-internal/object-size-trait.cc:15-19 — GC 对象: 头 → 视图 → 尺寸
  ///
  /// # Safety
  /// `object` 须指向已分配对象 payload 起点 (其前置为有效 HeapObjectHeader)。
  pub unsafe fn get_object_size_for_garbage_collected(object: *const c_void) -> usize {
    // SAFETY: 调用契约保证 object 前置为活对象头 (原版 reinterpret_cast 同形)。
    let header = unsafe { HeapObjectHeader::from_object(object) };
    ObjectView::new(header, AccessMode::Atomic).size()
  }

  /// v8: src/heap/cppgc-internal/object-size-trait.cc:22-31 — mixin 内地址: 页回溯头, 断言非大对象
  /// 原版 DCHECK(!header.IsLargeObject<kAtomic>()); 页回溯依赖 BasePage, 随 heap_page 批次。
  pub fn get_object_size_for_garbage_collected_mixin(_address: *const c_void) -> usize {
    todo!("BasePage::FromPayload + ObjectHeaderFromInnerAddress 随 heap_page 批次")
  }
}
