//! incremental-marking-schedule-unittest.cc 1:1 迁移。
use aok::{OK, Void};
use v8_base_platform::TimeDelta;
use v8_heap_base::{
  IncrementalMarkingSchedule, K_ESTIMATED_MARKING_TIME, K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

const K_ZERO_BYTES_STEP: usize = 0;
const K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP: usize = K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS;
const K_ESTIMATED_LIVE_SIZE: usize = 100 * K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP;

fn k_half_estimated_marking_time() -> TimeDelta {
  TimeDelta::from_milliseconds_d(K_ESTIMATED_MARKING_TIME.in_milliseconds_f() * 0.5)
}

// TEST_F(FirstStepReturnsDefaultDuration) cc:34-42
#[test]
fn first_step_returns_default_duration() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create_with_marked_bytes_per_step_for_testing(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    false,
  );
  schedule.notify_incremental_marking_start();
  schedule.set_elapsed_time_for_testing(TimeDelta::from_milliseconds(0));
  assert_eq!(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  OK
}

// TEST_F(EmptyStepDuration) cc:44-54
#[test]
fn empty_step_duration() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create(false);
  schedule.notify_incremental_marking_start();
  schedule.set_elapsed_time_for_testing(TimeDelta::from_milliseconds(0));
  // 做一些进展以避开无进展步长分支:
  schedule.add_mutator_thread_marked_bytes(K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS);
  assert_eq!(
    K_ZERO_BYTES_STEP,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  OK
}

// TEST_F(NoTimePassedReturnsMinimumDuration) cc:56-68
#[test]
fn no_time_passed_returns_minimum_duration() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create_with_marked_bytes_per_step_for_testing(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    false,
  );
  schedule.notify_incremental_marking_start();
  // 告知 oracle 非首步:
  schedule.add_mutator_thread_marked_bytes(K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP);
  schedule.set_elapsed_time_for_testing(TimeDelta::from_milliseconds(0));
  assert_eq!(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  OK
}

// TEST_F(OracleDoesntExccedMaximumStepDuration) cc:70-85
#[test]
fn oracle_doesnt_exceed_maximum_step_duration() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create_with_marked_bytes_per_step_for_testing(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    false,
  );
  schedule.notify_incremental_marking_start();
  // 增量标记字节 >= 无进展步长, 否则会命中无进展分支:
  schedule.add_mutator_thread_marked_bytes(K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS);
  schedule.set_elapsed_time_for_testing(K_ESTIMATED_MARKING_TIME);
  assert_eq!(
    K_ESTIMATED_LIVE_SIZE - K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  OK
}

// TEST_F(AheadOfScheduleReturnsMinimumDuration) cc:87-98
#[test]
fn ahead_of_schedule_returns_minimum_duration() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create_with_marked_bytes_per_step_for_testing(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    false,
  );
  schedule.notify_incremental_marking_start();
  schedule.add_mutator_thread_marked_bytes(K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP);
  schedule.add_concurrently_marked_bytes((0.6 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.set_elapsed_time_for_testing(k_half_estimated_marking_time());
  assert_eq!(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  OK
}

// TEST_F(AheadOfScheduleReturnsMinimumDurationZeroStep) cc:100-110
#[test]
fn ahead_of_schedule_returns_minimum_duration_zero_step() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create(false);
  schedule.notify_incremental_marking_start();
  schedule.add_mutator_thread_marked_bytes(K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP);
  schedule.add_concurrently_marked_bytes((0.6 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.set_elapsed_time_for_testing(k_half_estimated_marking_time());
  assert_eq!(
    K_ZERO_BYTES_STEP,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  OK
}

// TEST_F(BehindScheduleReturnsDelta) cc:112-130
#[test]
fn behind_schedule_returns_delta() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create_with_marked_bytes_per_step_for_testing(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    false,
  );
  schedule.notify_incremental_marking_start();
  schedule.add_mutator_thread_marked_bytes((0.1 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.add_concurrently_marked_bytes((0.25 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.set_elapsed_time_for_testing(k_half_estimated_marking_time());
  assert_eq!(
    (0.15 * K_ESTIMATED_LIVE_SIZE as f64) as usize,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  schedule.add_concurrently_marked_bytes((0.05 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.set_elapsed_time_for_testing(k_half_estimated_marking_time());
  assert_eq!(
    (0.1 * K_ESTIMATED_LIVE_SIZE as f64) as usize,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  schedule.add_concurrently_marked_bytes((0.05 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.set_elapsed_time_for_testing(k_half_estimated_marking_time());
  assert_eq!(
    (0.05 * K_ESTIMATED_LIVE_SIZE as f64) as usize,
    schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE)
  );
  OK
}

// TEST_F(GetCurrentStepInfo) cc:132-148
#[test]
fn get_current_step_info() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create_with_marked_bytes_per_step_for_testing(
    K_MINIMUM_MARKED_BYTES_PER_INCREMENTAL_STEP,
    false,
  );
  schedule.notify_incremental_marking_start();
  schedule.add_mutator_thread_marked_bytes((0.3 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.add_concurrently_marked_bytes((0.4 * K_ESTIMATED_LIVE_SIZE as f64) as usize);
  schedule.set_elapsed_time_for_testing(k_half_estimated_marking_time());
  schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE);
  let step_info = schedule.current_step_info();
  assert_eq!(step_info.elapsed_time, k_half_estimated_marking_time());
  assert_eq!(
    step_info.mutator_marked_bytes,
    (0.3 * K_ESTIMATED_LIVE_SIZE as f64) as usize
  );
  assert_eq!(
    step_info.concurrent_marked_bytes,
    (0.4 * K_ESTIMATED_LIVE_SIZE as f64) as usize
  );
  assert_eq!(
    step_info.marked_bytes(),
    (0.7 * K_ESTIMATED_LIVE_SIZE as f64) as usize
  );
  assert_eq!(step_info.estimated_live_bytes, K_ESTIMATED_LIVE_SIZE);
  assert_ne!(step_info.scheduled_delta_bytes(), 0);
  OK
}

// API 语义补充: ephemeron 冲刷节奏 + 并发字节时戳 + 可预测调度
#[test]
fn ephemeron_flush_and_concurrent_timestamp() -> Void {
  let mut schedule = IncrementalMarkingSchedule::create(false);
  schedule.notify_incremental_marking_start();
  schedule.set_elapsed_time_for_testing(TimeDelta::from_milliseconds(0));
  // 先落 last_estimated_live_bytes (flush 比例的分母)
  schedule.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE);
  // 未达标: 不冲刷
  schedule.add_mutator_thread_marked_bytes(10);
  assert!(!schedule.should_flush_ephemeron_pairs());
  // 达 25%: 冲刷一次, 目标升至 50%
  schedule.add_mutator_thread_marked_bytes(K_ESTIMATED_LIVE_SIZE / 4);
  assert!(schedule.should_flush_ephemeron_pairs());
  assert!(!schedule.should_flush_ephemeron_pairs());
  // 并发字节推进后时戳刷新, 间隔为零
  schedule.notify_concurrent_marking_start();
  schedule.add_concurrently_marked_bytes(1);
  assert!(
    schedule
      .time_since_last_concurrent_marking_update()
      .is_zero()
  );
  // 可预测调度: override 每步重置为 1ms
  let mut pred = IncrementalMarkingSchedule::create(true);
  pred.notify_incremental_marking_start();
  let d1 = pred.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE);
  let d2 = pred.get_next_incremental_step_duration(K_ESTIMATED_LIVE_SIZE);
  assert_eq!(d1, d2);
  assert_eq!(pred.min_marked_bytes_per_step(), 0);
  OK
}
