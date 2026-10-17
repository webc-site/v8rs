#!/usr/bin/env bun
// tool/src/heap/incremental-marking-job.h.js
// 机械提取 v8/src/heap/incremental-marking-job.{h,cc}:
// 平台任务驱动的增量标记作业台账 (ScheduleTask/AverageTimeToTask/CurrentTimeToTask)。
// 前缀 incremental-marking → 产物落 v8-heap-gc。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/incremental-marking-job.h");
const cc = readV8("src/heap/incremental-marking-job.cc");

for (const a of [
  "class IncrementalMarkingJob final {",
  "explicit IncrementalMarkingJob(Heap* heap);",
  "IncrementalMarkingJob(const IncrementalMarkingJob&) = delete;",
  "void ScheduleTask();",
  "std::optional<v8::base::TimeDelta> AverageTimeToTask() const;",
  "std::optional<v8::base::TimeDelta> CurrentTimeToTask() const;",
  "class Task;",
  "bool pending_task_ = false;",
]) {
  if (!h.includes(a)) throw new Error(`incremental-marking-job.h 锚缺失: ${a}`);
}
for (const a of [
  "IncrementalMarkingJob::IncrementalMarkingJob(Heap* heap)",
  "void IncrementalMarkingJob::ScheduleTask() {",
  "std::optional<base::TimeDelta> IncrementalMarkingJob::CurrentTimeToTask()",
  "std::optional<v8::base::TimeDelta> IncrementalMarkingJob::AverageTimeToTask()",
]) {
  if (!cc.includes(a)) throw new Error(`incremental-marking-job.cc 锚缺失: ${a}`);
}
// ScheduleTask: pending_task_ 已置或 teardown 中则不重复投递
if (!cc.includes("if (pending_task_ || heap_->IsTearingDown()) {")) throw new Error("ScheduleTask 去重臂缺失");
console.log("IncrementalMarkingJob: 方法 3 + Task 内嵌类");

let rust = banner(
  "src/heap/incremental-marking-job.h.js",
  "src/heap/incremental-marking-job.{h,cc}",
  "增量标记作业: 前台任务调度骨架",
);

rust += `
/// 增量标记作业骨架 (对应 class IncrementalMarkingJob final)。
/// 用平台前台任务 (必要时延迟任务) 驱动 start/step/finalize。
/// v8: src/heap/incremental-marking-job.h IncrementalMarkingJob
pub struct IncrementalMarkingJobSkeleton {
  /// 对应 bool pending_task_
  pub pending_task: bool,
}

impl IncrementalMarkingJobSkeleton {
  /// 对应 ScheduleTask: 任意线程可安全调用; pending_task_ 为真时直接返回
  /// v8: src/heap/incremental-marking-job.cc IncrementalMarkingJob::ScheduleTask
  pub fn schedule_task(&mut self) -> bool {
    if self.pending_task {
      return false;
    }
    self.pending_task = true;
    true
  }

  /// 对应 CurrentTimeToTask: 无待跑任务 → None; 否则 now - scheduled_time_
  /// v8: src/heap/incremental-marking-job.cc IncrementalMarkingJob::CurrentTimeToTask
  #[must_use]
  pub fn current_time_to_task(scheduled: Option<u64>, now: u64) -> Option<u64> {
    scheduled.map(|s| now.saturating_sub(s))
  }

  /// 对应 AverageTimeToTask: 到任务时间的加权滑动平均; 正在跑的任务也计入
  /// v8: src/heap/incremental-marking-job.cc IncrementalMarkingJob::AverageTimeToTask
  #[must_use]
  pub fn average_time_to_task(samples: &[u64]) -> Option<u64> {
    if samples.is_empty() {
      None
    } else {
      Some(samples.iter().sum::<u64>() / samples.len() as u64)
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  let mut job = IncrementalMarkingJobSkeleton { pending_task: false };
  assert!(job.schedule_task());
  assert!(!job.schedule_task());
  job.pending_task = false;
  assert!(job.schedule_task());
  assert_eq!(IncrementalMarkingJobSkeleton::current_time_to_task(None, 100), None);
  assert_eq!(IncrementalMarkingJobSkeleton::current_time_to_task(Some(40), 100), Some(60));
  assert_eq!(IncrementalMarkingJobSkeleton::average_time_to_task(&[]), None);
  assert_eq!(IncrementalMarkingJobSkeleton::average_time_to_task(&[10, 20, 30]), Some(20));
};
`;

const p = writeSrc("v8-heap-gc", "incremental_marking_job.rs.skel", rust);
console.log(`written ${p}`);
