// 增量标记作业: 前台任务调度骨架
//
// 族: v8/src/heap/incremental-marking-job.{h,cc}
// 说明: 平台 TaskRunner 依赖尚未冻结, 此处只登记调度状态位的纯骨架 + 时间统计纯函数;
//       ScheduleTask 与真平台的 PostIdleTask/PostTask 桥接在 heap-core/platform 面冻结后启动。

/// 增量标记作业骨架 (对应 class IncrementalMarkingJob final)。
/// 用平台前台任务 (必要时延迟任务) 驱动 start/step/finalize。
/// v8: src/heap/incremental-marking-job.h IncrementalMarkingJob
#[derive(Debug, Default)]
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

  /// 对应 CurrentTimeToTask: 无待跑任务 → None; 否则 now - scheduled_time_。
  /// v8: src/heap/incremental-marking-job.cc IncrementalMarkingJob::CurrentTimeToTask
  #[must_use]
  pub fn current_time_to_task(scheduled: Option<u64>, now: u64) -> Option<u64> {
    scheduled.map(|s| now.saturating_sub(s))
  }

  /// 对应 AverageTimeToTask: 到任务时间的加权滑动平均; 正在跑的任务也计入。
  /// 简化: 以算术平均代替 upstream 的加权滑动窗口 (窗口参数依赖 heap flag)。
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

#[cfg(test)]
mod tests {
  use super::IncrementalMarkingJobSkeleton;

  #[test]
  fn schedule_task_marks_pending_and_rejects_reentry() {
    let mut job = IncrementalMarkingJobSkeleton::default();
    assert!(job.schedule_task());
    assert!(!job.schedule_task());
    job.pending_task = false;
    assert!(job.schedule_task());
  }

  #[test]
  fn current_time_to_task_handles_no_schedule() {
    assert_eq!(
      IncrementalMarkingJobSkeleton::current_time_to_task(None, 100),
      None
    );
    assert_eq!(
      IncrementalMarkingJobSkeleton::current_time_to_task(Some(40), 100),
      Some(60)
    );
    // 上游使用 saturating_sub, 早于 scheduled 的 now 视为 0
    assert_eq!(
      IncrementalMarkingJobSkeleton::current_time_to_task(Some(150), 100),
      Some(0)
    );
  }

  #[test]
  fn average_time_to_task_empty_returns_none() {
    assert_eq!(
      IncrementalMarkingJobSkeleton::average_time_to_task(&[]),
      None
    );
    assert_eq!(
      IncrementalMarkingJobSkeleton::average_time_to_task(&[10, 20, 30]),
      Some(20)
    );
  }
}
