// 由 tool/src/heap/cppgc-internal/compaction-worklists.js 从 v8/src/heap/cppgc-internal/compaction-worklists.h + src/heap/cppgc-internal/compaction-worklists.cc 生成; 修订就地改, 复跑覆盖
// 可移动引用槽位工作列表
// 推迟台账: 原版 `heap::base::Worklist<MovableReference*, 256>` 为分段固定容量并发队列, 依赖 base
// worklist 绑定面 (他 crate, 本席禁触), 暂以单线程 Vec 承接元素身份, clear 语义等价 (全量清空)。

use core::ffi::c_void;

/// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists::MovableReference
pub type MovableReference = *const c_void;

/// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists::MovableReferencesWorklist
/// = heap::base::Worklist<MovableReference*, 256> — 每本地段 256 项 (Vec 暂承, 见件首台账)。
pub type MovableReferencesWorklist = Vec<MovableReference>;

/// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists
#[derive(Default)]
pub struct CompactionWorklists {
  /// v8: movable_slots_worklist_
  movable_slots_worklist: MovableReferencesWorklist,
}

impl CompactionWorklists {
  /// v8: src/heap/cppgc-internal/compaction-worklists.h CompactionWorklists::movable_slots_worklist
  pub fn movable_slots_worklist(&mut self) -> &mut MovableReferencesWorklist {
    &mut self.movable_slots_worklist
  }

  /// 对应原版 const 重载
  pub fn movable_slots_worklist_ref(&self) -> &MovableReferencesWorklist {
    &self.movable_slots_worklist
  }

  /// v8: src/heap/cppgc-internal/compaction-worklists.cc CompactionWorklists::ClearForTesting
  pub fn clear_for_testing(&mut self) {
    self.movable_slots_worklist.clear();
  }
}
