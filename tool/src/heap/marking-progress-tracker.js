#!/usr/bin/env bun
// tool/src/heap/marking-progress-tracker.js
// 机械提取 v8/src/heap/marking-progress-tracker.h:
// MarkingProgressTracker — 大对象分块标记进度原子计数器

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/marking-progress-tracker.h");

const anchors = [
  "class MarkingProgressTracker final {",
  "static constexpr size_t kChunkSize = kMaxRegularHeapObjectSize;",
  "overall_chunks_ = (size + kChunkSize - 1) / kChunkSize;",
  "const size_t new_chunk =",
  "std::atomic<size_t> current_chunk_;",
];
for (const a of anchors) if (!hSrc.includes(a)) throw new Error(`锚点缺失: ${a}`);

// 方法清单对数
const methods = ["Enable", "IsEnabled", "GetNextChunkToMark", "TotalNumberOfChunks", "ResetIfEnabled", "GetCurrentChunkForTesting"];
for (const m of methods) if (!hSrc.includes(`${m}(`)) throw new Error(`h 缺方法 ${m}`);
console.log(`marking-progress-tracker: 方法 ${methods.length}, 常量 1 (kChunkSize = kMaxRegularHeapObjectSize)`);

let rust = banner(
  "src/heap/marking-progress-tracker.js",
  "src/heap/marking-progress-tracker.h",
  "大对象分块标记进度跟踪 (启用后不可再禁用)",
);

rust += `use std::sync::atomic::{AtomicUsize, Ordering};

/// v8: src/heap/marking-progress-tracker.h MarkingProgressTracker
/// kChunkSize = kMaxRegularHeapObjectSize (对象按最大常规对象大小分块, 并行认领)
pub struct MarkingProgressTracker {
  /// v8: overall_chunks_ = 0 (0 表示禁用)
  overall_chunks: usize,
  /// v8: current_chunk_ (atomic, acq_rel fetch_add 认领)
  current_chunk: AtomicUsize,
}

impl MarkingProgressTracker {
  /// v8: MarkingProgressTracker::kChunkSize
  pub const K_CHUNK_SIZE: usize = K_MAX_REGULAR_HEAP_OBJECT_SIZE;

  /// v8: MarkingProgressTracker::Enable(size)
  /// overall_chunks_ = (size + kChunkSize - 1) / kChunkSize; current_chunk_ = 0
  pub fn enable(&mut self, size: usize) {
    assert!(!self.is_enabled());
    self.overall_chunks = (size + Self::K_CHUNK_SIZE - 1) / Self::K_CHUNK_SIZE;
    self.current_chunk.store(0, Ordering::Relaxed);
  }

  /// v8: MarkingProgressTracker::IsEnabled — overall_chunks_ != 0
  pub fn is_enabled(&self) -> bool { self.overall_chunks != 0 }

  /// v8: MarkingProgressTracker::GetNextChunkToMark
  /// fetch_add(1, acq_rel), 原文 DCHECK_LT(new_chunk, overall_chunks_)
  pub fn get_next_chunk_to_mark(&self) -> usize {
    let new_chunk = self.current_chunk.fetch_add(1, Ordering::AcqRel);
    assert!(new_chunk < self.overall_chunks);
    new_chunk
  }

  /// v8: MarkingProgressTracker::TotalNumberOfChunks
  pub fn total_number_of_chunks(&self) -> usize { self.overall_chunks }

  /// v8: MarkingProgressTracker::ResetIfEnabled
  pub fn reset_if_enabled(&mut self) {
    if self.is_enabled() { self.current_chunk.store(0, Ordering::Relaxed); }
  }

  /// v8: MarkingProgressTracker::GetCurrentChunkForTesting (relaxed load)
  pub fn get_current_chunk_for_testing(&self) -> usize {
    self.current_chunk.load(Ordering::Relaxed)
  }
}
`;

const p = writeSrc("v8-heap-gc", "marking_progress_tracker.rs.skel", rust);
console.log(`written ${p}`);
