// 由 tool/src/heap/base/incremental-marking-schedule.js 从 v8/src/heap/base/incremental-marking-schedule.h + incremental-marking-schedule.cc 生成; 修订就地改, 复跑覆盖
// 增量标记调度: 固定时间窗内按已标记字节追赶

//! IncrementalMarkingSchedule: 假定 kEstimatedMarkingTime=500ms 窗口,
//! 按 elapsed/live 比例推 expected, 落后则放大步长追赶。
//! TimeTicks 空态 (原 IsNull) → Option<TimeTicks>。

use std::{
  cmp,
  sync::atomic::{AtomicUsize, Ordering},
};

use v8_base_platform::{TimeDelta, TimeTicks};

/// 原估计标记总时长 (incremental-marking-schedule.h:51-52, 500ms)。
pub const K_ESTIMATED_MARKING_TIME: TimeDelta = TimeDelta::from_milliseconds(500);
/// 原无进展时的步长 (incremental-marking-schedule.h:56, 64*1024)。
pub const K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS: usize = 65536;
/// 原 ephemeron 对冲刷比例增量 (incremental-marking-schedule.h:117, 0.25)。
pub const K_EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS: f64 = 0.25;
/// 原可预测调度的固定时长 (incremental-marking-schedule.cc:17-19, 1ms)。
const K_TIME_DELTA_FOR_PREDICTABLE_SCHEDULE_MS: i64 = 1;

/// 原 StepInfo (incremental-marking-schedule.h:27-46)。
pub struct StepInfo {
  pub mutator_marked_bytes: usize,
  pub concurrent_marked_bytes: usize,
  pub estimated_live_bytes: usize,
  pub expected_marked_bytes: usize,
  pub elapsed_time: TimeDelta,
}

impl StepInfo {
  /// 原 marked_bytes (h:34-36): mutator + concurrent。
  pub const fn marked_bytes(&self) -> usize {
    self.mutator_marked_bytes + self.concurrent_marked_bytes
  }
  /// 原 scheduled_delta_bytes (h:40-42): 正 = 超前, 负 = 落后。
  pub const fn scheduled_delta_bytes(&self) -> i64 {
    self.marked_bytes() as i64 - self.expected_marked_bytes as i64
  }
  /// 原 is_behind_expectation (h:45)。
  pub const fn is_behind_expectation(&self) -> bool {
    self.scheduled_delta_bytes() < 0
  }
}

/// 原 IncrementalMarkingSchedule (incremental-marking-schedule.h:25)。
pub struct IncrementalMarkingSchedule {
  incremental_marking_start_time: Option<TimeTicks>,
  mutator_thread_marked_bytes: usize,
  concurrently_marked_bytes: AtomicUsize,
  last_estimated_live_bytes: usize,
  ephemeron_pairs_flushing_ratio_target: f64,
  current_step: StepInfo,
  min_marked_bytes_per_step: usize,
  predictable_schedule: bool,
  elapsed_time_override: Option<TimeDelta>,
  last_concurrently_marked_bytes: usize,
  last_concurrently_marked_bytes_update: Option<TimeTicks>,
}

impl IncrementalMarkingSchedule {
  /// 原 Create (cc:24-28): min=0。
  pub fn create(predictable_schedule: bool) -> Self {
    Self::new_inner(0, predictable_schedule)
  }
  /// 原 CreateWithMarkedBytesPerStepForTesting (cc:31-37)。
  pub fn create_with_marked_bytes_per_step_for_testing(
    min_marked_bytes_per_step: usize,
    predictable_schedule: bool,
  ) -> Self {
    Self::new_inner(min_marked_bytes_per_step, predictable_schedule)
  }

  /// 原私有构造 (cc:39-46): 可预测调度恒以 1ms override 驱动。
  fn new_inner(min_marked_bytes_per_step: usize, predictable_schedule: bool) -> Self {
    Self {
      incremental_marking_start_time: None,
      mutator_thread_marked_bytes: 0,
      concurrently_marked_bytes: AtomicUsize::new(0),
      last_estimated_live_bytes: 0,
      ephemeron_pairs_flushing_ratio_target: K_EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS,
      current_step: StepInfo {
        mutator_marked_bytes: 0,
        concurrent_marked_bytes: 0,
        estimated_live_bytes: 0,
        expected_marked_bytes: 0,
        elapsed_time: TimeDelta::new(),
      },
      min_marked_bytes_per_step,
      predictable_schedule,
      elapsed_time_override: predictable_schedule.then_some(TimeDelta::from_milliseconds(
        K_TIME_DELTA_FOR_PREDICTABLE_SCHEDULE_MS,
      )),
      last_concurrently_marked_bytes: 0,
      last_concurrently_marked_bytes_update: None,
    }
  }

  /// 原 NotifyIncrementalMarkingStart (cc:48-53): 幂等。
  pub fn notify_incremental_marking_start(&mut self) {
    if self.incremental_marking_start_time.is_some() {
      return;
    }
    self.incremental_marking_start_time = Some(TimeTicks::now());
  }

  /// 原 NotifyConcurrentMarkingStart (cc:55-60): 幂等。
  pub fn notify_concurrent_marking_start(&mut self) {
    if self.last_concurrently_marked_bytes_update.is_some() {
      return;
    }
    self.last_concurrently_marked_bytes_update = Some(TimeTicks::now());
  }

  /// 原 AddMutatorThreadMarkedBytes (cc:62-65)。
  pub fn add_mutator_thread_marked_bytes(&mut self, marked_bytes: usize) {
    self.mutator_thread_marked_bytes += marked_bytes;
  }

  /// 原 AddConcurrentlyMarkedBytes (cc:67-70): relaxed fetch_add。
  pub fn add_concurrently_marked_bytes(&self, marked_bytes: usize) {
    self
      .concurrently_marked_bytes
      .fetch_add(marked_bytes, Ordering::Relaxed);
  }

  /// 原 GetNextIncrementalStepDuration (cc:99-130), 三分支:
  ///   1) 增量 < kStepSizeWhenNotMakingProgress → max(无进展步长, min_step)
  ///   2) expected < actual (超前) → min_step
  ///   3) 落后 → max(min_step, expected - actual)
  pub fn get_next_incremental_step_duration(&mut self, estimated_live_bytes: usize) -> usize {
    self.last_estimated_live_bytes = estimated_live_bytes;
    debug_assert!(self.incremental_marking_start_time.is_some());
    let elapsed_time = self.elapsed_time_since_marking_start();
    let last_marked_bytes = self.current_step.marked_bytes();
    let actual_marked_bytes = self.overall_marked_bytes();
    let expected_marked_bytes = (estimated_live_bytes as f64 * elapsed_time.in_milliseconds_f()
      / K_ESTIMATED_MARKING_TIME.in_milliseconds_f())
    .ceil() as usize;
    // 原版 current_step_ 聚合赋值 (cc:113-115)。
    self.current_step = StepInfo {
      mutator_marked_bytes: self.mutator_thread_marked_bytes,
      concurrent_marked_bytes: self.concurrently_marked_bytes.load(Ordering::Relaxed),
      estimated_live_bytes,
      expected_marked_bytes,
      elapsed_time,
    };
    if actual_marked_bytes >= last_marked_bytes
      && actual_marked_bytes - last_marked_bytes < K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS
    {
      return cmp::max(
        K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS,
        self.min_marked_bytes_per_step,
      );
    }
    if expected_marked_bytes < actual_marked_bytes {
      // 超前: 增量标记取最小步长。
      return self.min_marked_bytes_per_step;
    }
    // 落后: 追赶 (expected - actual)。
    cmp::max(
      self.min_marked_bytes_per_step,
      expected_marked_bytes - actual_marked_bytes,
    )
  }

  /// 原 GetCurrentStepInfo (cc:94-97)。
  pub const fn current_step_info(&self) -> &StepInfo {
    &self.current_step
  }

  /// 原 ShouldFlushEphemeronPairs (cc:134-142): 每 25% 总量冲刷一次。
  pub fn should_flush_ephemeron_pairs(&mut self) -> bool {
    if (self.overall_marked_bytes() as f64)
      < self.ephemeron_pairs_flushing_ratio_target * self.last_estimated_live_bytes as f64
    {
      return false;
    }
    self.ephemeron_pairs_flushing_ratio_target += K_EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS;
    true
  }

  /// 原 GetTimeSinceLastConcurrentMarkingUpdate (cc:144-158):
  /// 并发字节有推进 → 刷新时戳并返回零; 未启动并发 → 返回零 (同推进信号)。
  pub fn time_since_last_concurrent_marking_update(&mut self) -> TimeDelta {
    let current_concurrently_marked_bytes = self.concurrently_marked_bytes.load(Ordering::Relaxed);
    if current_concurrently_marked_bytes > self.last_concurrently_marked_bytes {
      self.last_concurrently_marked_bytes = current_concurrently_marked_bytes;
      self.last_concurrently_marked_bytes_update = Some(TimeTicks::now());
      return TimeDelta::new();
    }
    match self.last_concurrently_marked_bytes_update {
      None => TimeDelta::new(),
      Some(update) => TimeTicks::now().minus(update),
    }
  }

  /// 原 min_marked_bytes_per_step (h:108-110)。
  pub const fn min_marked_bytes_per_step(&self) -> usize {
    self.min_marked_bytes_per_step
  }

  /// 原 SetElapsedTimeForTesting (cc:160-163)。
  pub fn set_elapsed_time_for_testing(&mut self, elapsed_time: TimeDelta) {
    self.elapsed_time_override = Some(elapsed_time);
  }

  /// 原 GetElapsedTimeSinceMarkingStart (cc:80-92, private): override 一次性消费;
  /// 可预测调度消费后重置回 1ms。
  fn elapsed_time_since_marking_start(&mut self) -> TimeDelta {
    if let Some(elapsed_time) = self.elapsed_time_override {
      self.elapsed_time_override = if self.predictable_schedule {
        Some(TimeDelta::from_milliseconds(
          K_TIME_DELTA_FOR_PREDICTABLE_SCHEDULE_MS,
        ))
      } else {
        None
      };
      return elapsed_time;
    }
    match self.incremental_marking_start_time {
      Some(start) => TimeTicks::now().minus(start),
      None => TimeDelta::new(),
    }
  }

  /// 原 GetOverallMarkedBytes (cc:72-74, private)。
  fn overall_marked_bytes(&self) -> usize {
    self.mutator_thread_marked_bytes + self.concurrently_marked_bytes.load(Ordering::Relaxed)
  }
}
