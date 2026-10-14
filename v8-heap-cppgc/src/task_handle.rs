// 由 tool/src/heap/cppgc-internal/task-handle.h.js 从 v8/src/heap/cppgc-internal/task-handle.h 生成; 修订就地改, 复跑覆盖
// SingleThreadedHandle 单线程任务取消句柄

use std::{cell::Cell, rc::Rc};

/// v8: src/heap/cppgc-internal/task-handle.h:17 NonEmptyTag — 类型级非空证明
pub struct NonEmptyTag;

/// v8: src/heap/cppgc-internal/task-handle.h:16-48 SingleThreadedHandle
/// 空/非空 + 取消位; 原版 `shared_ptr<bool>` → Rust `Rc<Cell<bool>>` (单线程语境, 免原子开销)。
#[derive(Clone, Default)]
pub struct SingleThreadedHandle {
  /// None = 空句柄; Some = 共享取消位。
  is_cancelled: Option<Rc<Cell<bool>>>,
}

impl SingleThreadedHandle {
  /// v8: src/heap/cppgc-internal/task-handle.h:22-23 — NonEmptyTag 构造, 取消位 false
  pub fn new_non_empty(_: NonEmptyTag) -> Self {
    Self {
      is_cancelled: Some(Rc::new(Cell::new(false))),
    }
  }

  /// v8: src/heap/cppgc-internal/task-handle.h:25-28 — Cancel: DCHECK 非空后置位。
  /// 空句柄 Cancel 是契约违规 (原版 DCHECK); Rust 侧 debug_assert 承接, release 静默。
  pub fn cancel(&self) {
    debug_assert!(self.is_cancelled.is_some());
    if let Some(cell) = &self.is_cancelled {
      cell.set(true);
    }
  }

  /// v8: src/heap/cppgc-internal/task-handle.h:30-34 — CancelIfNonEmpty: 空则无操作
  pub fn cancel_if_non_empty(&self) {
    if let Some(cell) = &self.is_cancelled {
      cell.set(true);
    }
  }

  /// v8: src/heap/cppgc-internal/task-handle.h:36-39 — IsCanceled: DCHECK 非空后读
  pub fn is_canceled(&self) -> bool {
    debug_assert!(self.is_cancelled.is_some());
    self.is_cancelled.as_ref().is_some_and(|cell| cell.get())
  }

  /// v8: src/heap/cppgc-internal/task-handle.h:42-44 — 活跃 = 非空 && 未取消
  pub fn is_active(&self) -> bool {
    self.is_cancelled.as_ref().is_some_and(|cell| !cell.get())
  }
}
