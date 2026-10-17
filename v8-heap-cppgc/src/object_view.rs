// 由 tool/src/heap/cppgc-internal/object-view.h.js 从 v8/src/heap/cppgc-internal/object-view.h 生成; 修订就地改, 复跑覆盖
// ObjectView 对象边界访问器 — Start/End/Size 三查询
// 模板面承接: 原版 `ObjectView<AccessMode mode>` → Rust 运行期 mode 字段 (同 heap-object-header)。
// 推迟台账: 大对象臂依赖 LargePage::PayloadEnd/ObjectSize (BasePage::FromPayload 回溯),
// 随 heap_page 批次挂载; 本面对大对象查询暂留占位。

use crate::{
  globals::{AccessMode, Address},
  heap_object_header::HeapObjectHeader,
};

/// v8: src/heap/cppgc-internal/object-view.h:19-57 ObjectView<AccessMode>
/// 不保活对象, 只在对象边界内取头; 大对象判定在构造时一次求值。
pub struct ObjectView<'a> {
  header: &'a HeapObjectHeader,
  mode: AccessMode,
  is_large_object: bool, // v8: header_.IsLargeObject<mode>()
}

impl<'a> ObjectView<'a> {
  /// v8: src/heap/cppgc-internal/object-view.h:33-40 — 构造: 大对象判定。
  /// 原版另有 `BasePage::FromPayload` 存 base_page_ 与 Start+Size==End 不变式 DCHECK,
  /// 二者随 heap_page 批次承接 (见件首台账)。
  pub fn new(header: &'a HeapObjectHeader, mode: AccessMode) -> Self {
    let is_large_object = header.is_large_object(mode);
    Self {
      header,
      mode,
      is_large_object,
    }
  }

  /// v8: src/heap/cppgc-internal/object-view.h:42-45 — Start = header_.ObjectStart()
  pub fn start(&self) -> Address {
    self.header.object_start()
  }

  /// v8: src/heap/cppgc-internal/object-view.h:47-51 — End: 大对象 PayloadEnd / 普通对象 ObjectEnd
  pub fn end(&self) -> Address {
    if self.is_large_object {
      todo!("LargePage::PayloadEnd 随 heap_page 批次")
    } else {
      self.header.object_end(self.mode)
    }
  }

  /// v8: src/heap/cppgc-internal/object-view.h:53-57 — Size: 大对象 LargePage::ObjectSize / 普通对象 ObjectSize
  pub fn size(&self) -> usize {
    if self.is_large_object {
      todo!("LargePage::ObjectSize 随 heap_page 批次")
    } else {
      self.header.object_size(self.mode)
    }
  }
}
