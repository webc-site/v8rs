#!/usr/bin/env bun
// tool/src/heap/concurrent-marking.h.js
// 机械提取 v8/src/heap/concurrent-marking.{h,cc}:
// ConcurrentMarking 方法台账 (调度/汇合/暂停/Flush 族/并发度估计) +
// FetchAndResetConcurrencyEstimate 规则 (estimate ? estimate : 1)。
// 注: 按席位规则 concurrent-marking 不以 marking 开头 → v8-heap-core。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/concurrent-marking.h");
const cc = readV8("src/heap/concurrent-marking.cc");

for (const a of [
  "class V8_EXPORT_PRIVATE ConcurrentMarking {",
  "ConcurrentMarking(Heap* heap, WeakObjects* weak_objects);",
  "void TryScheduleJob(GarbageCollector garbage_collector,",
  "void Join();",
  "void JoinJobForTesting();",
  "bool Pause();",
  "void RescheduleJobIfNeeded(",
  "void FlushNativeContexts(NativeContextStats* main_stats);",
  "void FlushMemoryChunkData();",
  "void FlushPretenuringFeedback();",
  "bool IsStopped();",
  "size_t TotalMarkedBytes();",
  "void set_another_ephemeron_iteration(bool another_ephemeron_iteration) {",
  "GarbageCollector garbage_collector() const {",
  "bool IsWorkLeft() const;",
  "size_t FetchAndResetConcurrencyEstimate() {",
  "const size_t estimate =",
  "estimate_concurrency_.exchange(0, std::memory_order_relaxed);",
  "return estimate ? estimate : 1;",
  "struct TaskState;",
  "class JobTaskMinor;",
  "class JobTaskMajor;",
  "class MinorMarkingState;",
  "size_t GetMajorMaxConcurrency(size_t worker_count);",
  "size_t GetMinorMaxConcurrency(size_t worker_count);",
  "std::atomic<size_t> total_marked_bytes_{0};",
  "std::atomic<size_t> estimate_concurrency_{0};",
]) {
  if (!h.includes(a)) throw new Error(`concurrent-marking.h 锚缺失: ${a}`);
}
for (const a of [
  "void ConcurrentMarking::RunMajor(",
  "size_t ConcurrentMarking::GetMajorMaxConcurrency(",
  "size_t ConcurrentMarking::GetMinorMaxConcurrency(",
  "bool ConcurrentMarking::Pause() {",
  "void ConcurrentMarking::Join() {",
  "void ConcurrentMarking::FlushNativeContexts(",
]) {
  if (!cc.includes(a)) throw new Error(`concurrent-marking.cc 锚缺失: ${a}`);
}
console.log("ConcurrentMarking: 方法 17 + 嵌套类型 4");

let rust = banner(
  "src/heap/concurrent-marking.h.js",
  "src/heap/concurrent-marking.{h,cc}",
  "并发标记: 调度/汇合/暂停台账与并发度估计",
);

rust += `
/// 并发标记器骨架 (对应 class ConcurrentMarking)。
/// v8: src/heap/concurrent-marking.h ConcurrentMarking
pub struct ConcurrentMarkingSkeleton {
  pub total_marked_bytes: core::sync::atomic::AtomicUsize,
  pub another_ephemeron_iteration: core::sync::atomic::AtomicBool,
  pub estimate_concurrency: core::sync::atomic::AtomicUsize,
}

impl ConcurrentMarkingSkeleton {
  /// 对应 FetchAndResetConcurrencyEstimate: exchange(0) 取旧值, 0 则回退 1
  /// v8: src/heap/concurrent-marking.h ConcurrentMarking::FetchAndResetConcurrencyEstimate
  pub fn fetch_and_reset_concurrency_estimate(&self) -> usize {
    let estimate = self.estimate_concurrency.swap(0, core::sync::atomic::Ordering::Relaxed);
    if estimate != 0 {
      estimate
    } else {
      1
    }
  }

  /// 对应 TotalMarkedBytes: relaxed 读累计值
  pub fn total_marked_bytes(&self) -> usize {
    self.total_marked_bytes.load(core::sync::atomic::Ordering::Relaxed)
  }

  /// 对应 set_another_ephemeron_iteration/another_ephemeron_iteration
  pub fn set_another_ephemeron_iteration(&self, v: bool) {
    self.another_ephemeron_iteration.store(v, core::sync::atomic::Ordering::Relaxed);
  }

  pub fn another_ephemeron_iteration(&self) -> bool {
    self.another_ephemeron_iteration.load(core::sync::atomic::Ordering::Relaxed)
  }

  /// 对应 RunMajor 主循环骨架
  /// v8: src/heap/concurrent-marking.cc ConcurrentMarking::RunMajor
  pub fn run_major(&self) {
    unimplemented!("骨架: MarkingWorklists 消费待 marking 族就位");
  }

  /// 对应 Pause: 抢占运行中作业, 返回是否确有并发标记在跑
  /// v8: src/heap/concurrent-marking.cc ConcurrentMarking::Pause
  pub fn pause(&mut self) -> bool {
    unimplemented!("骨架: JobHandle::Cancel/进度同步待任务族就位");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let m = ConcurrentMarkingSkeleton {
    total_marked_bytes: core::sync::atomic::AtomicUsize::new(1234),
    another_ephemeron_iteration: core::sync::atomic::AtomicBool::new(false),
    estimate_concurrency: core::sync::atomic::AtomicUsize::new(3),
  };
  assert_eq!(m.fetch_and_reset_concurrency_estimate(), 3); // 旧值
  assert_eq!(m.fetch_and_reset_concurrency_estimate(), 1); // 清零后回退 1
  assert_eq!(m.total_marked_bytes(), 1234);
  m.set_another_ephemeron_iteration(true);
  assert!(m.another_ephemeron_iteration());
};
`;

const p = writeSrc("v8-heap-core", "concurrent_marking.rs.skel", rust);
console.log(`written ${p}`);
