// 由 tool/src/heap/marking-progress-tracker.js 从 v8/src/heap/marking-progress-tracker.h 收编; 纯逻辑面 1:1 落地。
// 大对象分块标记进度跟踪 (启用后不可再禁用)
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::heap_layout::K_MAX_REGULAR_HEAP_OBJECT_SIZE;

/// 标记进度跟踪器: 大对象按 kMaxRegularHeapObjectSize 分块, 线程原子认领下一块。
/// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker
pub struct MarkingProgressTracker {
  /// v8: overall_chunks_ = 0 (0 表示禁用)
  overall_chunks: usize,
  /// v8: current_chunk_ (atomic, acq_rel fetch_add 认领)
  current_chunk: AtomicUsize,
}

impl Default for MarkingProgressTracker {
  fn default() -> Self {
    Self::new()
  }
}

impl MarkingProgressTracker {
  /// v8: MarkingProgressTracker::kChunkSize = kMaxRegularHeapObjectSize
  pub const K_CHUNK_SIZE: usize = K_MAX_REGULAR_HEAP_OBJECT_SIZE;

  /// 默认构造: 禁用态 (overall_chunks_ = 0)。
  /// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker
  #[must_use]
  pub const fn new() -> Self {
    Self {
      overall_chunks: 0,
      current_chunk: AtomicUsize::new(0),
    }
  }

  /// Enable(size): 分块数向上取整, current_chunk 归零; 原版 DCHECK 禁用态。
  /// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker::Enable
  pub fn enable(&mut self, size: usize) {
    debug_assert!(!self.is_enabled());
    self.overall_chunks = size.div_ceil(Self::K_CHUNK_SIZE);
    self.current_chunk.store(0, Ordering::Relaxed);
  }

  /// IsEnabled: overall_chunks_ != 0。
  /// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker::IsEnabled
  #[must_use]
  pub fn is_enabled(&self) -> bool {
    self.overall_chunks != 0
  }

  /// GetNextChunkToMark: fetch_add(1, acq_rel) 认领, 原版 DCHECK_LT 越界。
  /// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker::GetNextChunkToMark
  pub fn get_next_chunk_to_mark(&self) -> usize {
    let new_chunk = self.current_chunk.fetch_add(1, Ordering::AcqRel);
    debug_assert!(new_chunk < self.overall_chunks);
    new_chunk
  }

  /// TotalNumberOfChunks。
  /// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker::TotalNumberOfChunks
  #[must_use]
  pub fn total_number_of_chunks(&self) -> usize {
    self.overall_chunks
  }

  /// ResetIfEnabled: 启用态才把 current_chunk 归零。
  /// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker::ResetIfEnabled
  pub fn reset_if_enabled(&mut self) {
    if self.is_enabled() {
      self.current_chunk.store(0, Ordering::Relaxed);
    }
  }

  /// GetCurrentChunkForTesting: relaxed load。
  /// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker::GetCurrentChunkForTesting
  #[must_use]
  pub fn get_current_chunk_for_testing(&self) -> usize {
    self.current_chunk.load(Ordering::Relaxed)
  }
}

// —— 布局不变式 ——
const _: () = {
  assert!(MarkingProgressTracker::K_CHUNK_SIZE == 131072);
};
