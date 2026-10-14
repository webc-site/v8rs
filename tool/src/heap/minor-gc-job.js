#!/usr/bin/env bun
// tool/src/heap/minor-gc-job.js
// 机械提取 v8/src/heap/minor-gc-job{.h,.cc} 族:
// MinorGCJob — 后台小 GC 任务调度

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/minor-gc-job.h");
const ccSrc = readV8("src/heap/minor-gc-job.cc");

const anchors = [
  [hSrc, "class MinorGCJob final {"],
  [hSrc, "void TryScheduleTask();"],
  [hSrc, "void CancelTaskIfScheduled();"],
  [hSrc, "return current_task_id_ != CancelableTaskManager::kInvalidTaskId;"],
  [hSrc, "std::unique_ptr<AllocationObserver> minor_gc_task_observer_;"],
  [ccSrc, "MinorGCJob::Task"],
  [ccSrc, "MinorGCJob::TryScheduleTask"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

console.log(`minor-gc-job: 类 1 + 内嵌 Task 1, 方法 3 (TryScheduleTask/CancelTaskIfScheduled/IsScheduled)`);

let rust = banner(
  "src/heap/minor-gc-job.js",
  "src/heap/minor-gc-job.h, src/heap/minor-gc-job.cc",
  "小 GC 后台任务: 按分配量调度/取消",
);

rust += `/// v8: src/heap/minor-gc-job.h MinorGCJob
pub struct MinorGCJob {
  /// v8: heap_ (const)
  pub heap: *mut Heap,
  /// v8: current_task_id_ = kInvalidTaskId
  pub current_task_id: u64, // CancelableTaskManager::Id
  /// v8: minor_gc_task_observer_ — AllocationObserver 累计字节触发 TryScheduleTask
  pub minor_gc_task_observer: Option<Box<AllocationObserver>>,
}

impl MinorGCJob {
  /// v8: MinorGCJob::MinorGCJob(Heap*) V8_NOEXCEPT — 构造时挂 observer
  pub fn new(heap: *mut Heap) -> Self { todo!() }
  /// v8: MinorGCJob::TryScheduleTask — 未调度且允许后台 GC 时投 Task
  pub fn try_schedule_task(&mut self) { todo!() }
  /// v8: MinorGCJob::CancelTaskIfScheduled — 撤销未跑任务
  pub fn cancel_task_if_scheduled(&mut self) { todo!() }
  /// v8: MinorGCJob::IsScheduled
  pub fn is_scheduled(&self) -> bool { self.current_task_id != K_INVALID_TASK_ID }
}

/// v8: src/heap/minor-gc-job.cc MinorGCJob::Task — CancelableTask, RunInternal 触发 minor GC
/// v8: src/heap/minor-gc-job.cc Task::RunInternal
`;

const p = writeSrc("v8-heap-core", "minor_gc_job.rs.skel", rust);
console.log(`written ${p}`);
