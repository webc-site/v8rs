#!/usr/bin/env bun
// tool/src/heap/base/incremental-marking-schedule.js — incremental_marking_schedule.rs 初版生成
// 组→文件映射 (本脚本管辖):
//   v8/src/heap/base/incremental-marking-schedule.h + incremental-marking-schedule.cc
//   → v8-heap-base/src/incremental_marking_schedule.rs
// 规则:
//   - 常量 (kEstimatedMarkingTime / kStepSizeWhenNotMakingProgress /
//     kEphemeronPairsFlushingRatioIncrements / kTimeDeltaForPredictableSchedule) 机械提取验算
//   - GetNextIncrementalStepDuration 三分支决策骨架
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/heap/base/incremental-marking-schedule.h");
const cc = readV8("src/heap/base/incremental-marking-schedule.cc");

// ---- 常量提取 + 验算 ----
const estTime = h.match(/kEstimatedMarkingTime =\s*\n?\s*v8::base::TimeDelta::FromMilliseconds\((\d+)\)/);
if (!estTime || estTime[1] !== "500") throw new Error(`kEstimatedMarkingTime 应为 500ms, 实得 ${estTime}`);
const stepSize = h.match(/kStepSizeWhenNotMakingProgress = (\d+) \* (\d+)/);
if (!stepSize || Number(stepSize[1]) * Number(stepSize[2]) !== 65536) throw new Error(`kStepSizeWhenNotMakingProgress 应为 64*1024, 实得 ${stepSize}`);
const flushRatio = h.match(/kEphemeronPairsFlushingRatioIncrements = ([\d.]+)/);
if (!flushRatio || flushRatio[1] !== "0.25") throw new Error(`kEphemeronPairsFlushingRatioIncrements 应为 0.25, 实得 ${flushRatio}`);
const predDelta = cc.match(/kTimeDeltaForPredictableSchedule =\s*\n?\s*v8::base::TimeDelta::FromMilliseconds\((\d+)\)/);
if (!predDelta || predDelta[1] !== "1") throw new Error(`kTimeDeltaForPredictableSchedule 应为 1ms, 实得 ${predDelta}`);
const initTarget = h.match(/double ephemeron_pairs_flushing_ratio_target_ = ([\d.]+);/);
if (!initTarget || initTarget[1] !== "0.25") throw new Error("flushing_ratio_target 初值应为 0.25");

// ---- 锚点断言 ----
for (const fn of ["Create", "CreateWithMarkedBytesPerStepForTesting", "NotifyIncrementalMarkingStart", "NotifyConcurrentMarkingStart", "AddMutatorThreadMarkedBytes", "AddConcurrentlyMarkedBytes", "GetNextIncrementalStepDuration", "GetCurrentStepInfo", "ShouldFlushEphemeronPairs", "GetTimeSinceLastConcurrentMarkingUpdate", "SetElapsedTimeForTesting", "marked_bytes", "scheduled_delta_bytes", "is_behind_expectation"]) {
  if (!new RegExp(`\\b${fn}\\s*\\(`).test(h)) throw new Error(`incremental-marking-schedule.h: ${fn} 缺失`);
}
// 决策三分支证据 (cc:112-129)
if (!/kStepSizeWhenNotMakingProgress, min_marked_bytes_per_step_/.test(cc)) throw new Error("cc: 无进展步长分支不符");
if (!/expected_marked_bytes < actual_marked_bytes/.test(cc)) throw new Error("cc: 超前分支不符");
if (!/expected_marked_bytes - actual_marked_bytes/.test(cc)) throw new Error("cc: 追赶分支不符");

const body = `${banner("src/heap/base/incremental-marking-schedule.js", "src/heap/base/incremental-marking-schedule.h + incremental-marking-schedule.cc", "增量标记调度: 固定时间窗内按已标记字节追赶")}
//! IncrementalMarkingSchedule: 假定 kEstimatedMarkingTime=500ms 窗口,
//! 按 elapsed/live 比例推 expected, 落后则放大步长追赶。
//! TimeTicks 空态 (原 IsNull) → Option<TimeTicks>。

use std::{cmp, sync::atomic::{AtomicUsize, Ordering}};

use v8_base_platform::{TimeDelta, TimeTicks};

/// 原估计标记总时长 (机械提取: ${estTime[1]}ms)。
pub const K_ESTIMATED_MARKING_TIME_MS: i64 = ${estTime[1]};
/// 原无进展时的步长 (机械提取: 64*1024=${Number(stepSize[1]) * Number(stepSize[2])})。
pub const K_STEP_SIZE_WHEN_NOT_MAKING_PROGRESS: usize = ${Number(stepSize[1]) * Number(stepSize[2])};
/// 原 ephemeron 对冲刷比例增量 (机械提取: ${flushRatio[1]})。
pub const K_EPHEMERON_PAIRS_FLUSHING_RATIO_INCREMENTS: f64 = ${flushRatio[1]};
/// 原可预测调度的固定时长 (cc 机械提取: ${predDelta[1]}ms)。
const K_TIME_DELTA_FOR_PREDICTABLE_SCHEDULE_MS: i64 = ${predDelta[1]};

/// 原 StepInfo (incremental-marking-schedule.h:27-46)。
pub struct StepInfo {
  pub mutator_marked_bytes: usize,
  pub concurrent_marked_bytes: usize,
  pub estimated_live_bytes: usize,
  pub expected_marked_bytes: usize,
  pub elapsed_time: TimeDelta,
}

impl StepInfo {
  /// 原 marked_bytes: mutator + concurrent。
  pub const fn marked_bytes(&self) -> usize { todo!("就地修订") }
  /// 原 scheduled_delta_bytes: 正 = 超前, 负 = 落后。
  pub const fn scheduled_delta_bytes(&self) -> i64 { todo!("就地修订") }
  /// 原 is_behind_expectation。
  pub const fn is_behind_expectation(&self) -> bool { todo!("就地修订") }
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
  pub fn create(predictable_schedule: bool) -> Self { todo!("就地修订") }
  /// 原 CreateWithMarkedBytesPerStepForTesting (cc:31-37)。
  pub fn create_with_marked_bytes_per_step_for_testing(min_marked_bytes_per_step: usize, predictable_schedule: bool) -> Self { todo!("就地修订") }
  /// 原 NotifyIncrementalMarkingStart (cc:48-53): 幂等。
  pub fn notify_incremental_marking_start(&mut self) { todo!("就地修订") }
  /// 原 NotifyConcurrentMarkingStart (cc:55-60): 幂等。
  pub fn notify_concurrent_marking_start(&mut self) { todo!("就地修订") }
  /// 原 AddMutatorThreadMarkedBytes (cc:62-65)。
  pub fn add_mutator_thread_marked_bytes(&mut self, marked_bytes: usize) { todo!("就地修订") }
  /// 原 AddConcurrentlyMarkedBytes (cc:67-70): relaxed fetch_add。
  pub fn add_concurrently_marked_bytes(&self, marked_bytes: usize) { todo!("就地修订") }
  /// 原 GetNextIncrementalStepDuration (cc:99-130), 三分支:
  ///   1) 增量 < kStepSizeWhenNotMakingProgress → max(无进展步长, min_step)
  ///   2) expected < actual (超前) → min_step
  ///   3) 落后 → max(min_step, expected - actual)
  pub fn get_next_incremental_step_duration(&mut self, estimated_live_bytes: usize) -> usize { todo!("就地修订") }
  /// 原 GetCurrentStepInfo (cc:94-97)。
  pub fn current_step_info(&self) -> &StepInfo { todo!("就地修订") }
  /// 原 ShouldFlushEphemeronPairs (cc:134-142): 每 25% 总量冲刷一次。
  pub fn should_flush_ephemeron_pairs(&mut self) -> bool { todo!("就地修订") }
  /// 原 GetTimeSinceLastConcurrentMarkingUpdate (cc:144-158)。
  pub fn time_since_last_concurrent_marking_update(&mut self) -> TimeDelta { todo!("就地修订") }
  /// 原 min_marked_bytes_per_step (h:108-110)。
  pub const fn min_marked_bytes_per_step(&self) -> usize { todo!("就地修订") }
  /// 原 SetElapsedTimeForTesting (cc:160-163)。
  pub fn set_elapsed_time_for_testing(&mut self, elapsed_time: TimeDelta) { todo!("就地修订") }
  /// 原 GetElapsedTimeSinceMarkingStart (cc:80-92, private): override 一次性消费。
  fn elapsed_time_since_marking_start(&mut self) -> TimeDelta { todo!("就地修订") }
  /// 原 GetOverallMarkedBytes (cc:72-74, private)。
  fn overall_marked_bytes(&self) -> usize { todo!("就地修订") }
}
`;

const p = writeSrc("v8-heap-base", "incremental_marking_schedule.rs", body);
console.log(`incremental-marking-schedule.js: ${p} (常量 5 项验算 + 方法 14 项锚点 + 三分支公式核对通过)`);
