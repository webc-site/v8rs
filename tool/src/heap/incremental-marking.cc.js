#!/usr/bin/env bun
// tool/src/heap/incremental-marking.cc.js
// 机械提取 v8/src/heap/incremental-marking.cc (仅 cc; .h/-inl.h 在 b 半区表):
// 步进/激活常量 (观察者步长/最大步时长/激活阈值 DEBUG 双臂) + GetMaxDuration
// 按 StepOrigin 选上限规则 + 方法清单计数对数。
// 前缀 incremental-marking → 产物落 v8-heap-gc。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/heap/incremental-marking.cc");

// —— 常量锚 (带数值) ——
for (const a of [
  "static constexpr size_t kMajorGCYoungGenerationAllocationObserverStep = 64 * KB;",
  "static constexpr size_t kMajorGCOldGenerationAllocationObserverStep = 256 * KB;",
  "static constexpr v8::base::TimeDelta kMaxStepSizeOnTask =",
  "v8::base::TimeDelta::FromMilliseconds(1);",
  "static constexpr v8::base::TimeDelta kMaxStepSizeOnAllocation =",
  "v8::base::TimeDelta::FromMilliseconds(5);",
  "static_assert(kMaxStepSizeOnAllocation <= kMaxSynchronuousGCOperation);",
  "static constexpr size_t kV8ActivationThreshold = 8 * MB;",
  "static constexpr size_t kGlobalActivationThreshold = 8 * MB;",
]) {
  if (!cc.includes(a)) throw new Error(`incremental-marking.cc 常量锚缺失: ${a}`);
}
// DEBUG 双臂 (阈值 0)
if (!/#else\nstatic constexpr size_t kV8ActivationThreshold = 0;/.test(cc)) {
  throw new Error("DEBUG 阈值双臂缺失");
}
// —— 方法清单 (cc 内 IncrementalMarking:: 成员定义) ——
const methods = [...new Set([...cc.matchAll(/^[\w:<>,&* ]*IncrementalMarking::(\w+)\(/gm)].map((m) => m[1]))];
if (methods.length < 28) throw new Error(`IncrementalMarking 方法 ${methods.length}, 期望 >= 28`);
for (const must of ["Start", "Stop", "Step", "MarkRoots", "StartBlackAllocation", "ShouldFinalize", "GetScheduledBytes"]) {
  if (!methods.includes(must)) throw new Error(`方法清单缺 ${must}`);
}
// GetMaxDuration 规则锚 (StepOrigin → 上限)
for (const a of [
  "base::TimeDelta GetMaxDuration(StepOrigin step_origin) {",
  "case StepOrigin::kV8:",
  "case StepOrigin::kTask:",
  "return kMaxStepSizeOnAllocation;",
  "return kMaxStepSizeOnTask;",
]) {
  if (!cc.includes(a)) throw new Error(`GetMaxDuration 锚缺失: ${a}`);
}
console.log(`IncrementalMarking.cc: 常量 7 (含 DEBUG 双臂) + 方法 ${methods.length}`);

let rust = banner(
  "src/heap/incremental-marking.cc.js",
  "src/heap/incremental-marking.cc",
  "增量标记: 步进上限/激活阈值常量与步来源选限",
);

rust += `
/// 增量标记步来源 (对应 enum class StepOrigin, 声明于 incremental-marking.h —
/// b 半区族脚本; 此处登记 cc 侧消费面)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOrigin {
  /// kV8: 生成代码/运行时触发
  V8,
  /// kTask: 平台任务触发
  TASK,
}

/// 分配观察者步长 (kMajorGC{Young,Old}GenerationAllocationObserverStep)。
pub const K_MAJOR_GC_YOUNG_GENERATION_ALLOCATION_OBSERVER_STEP: usize = 64 * 1024;
pub const K_MAJOR_GC_OLD_GENERATION_ALLOCATION_OBSERVER_STEP: usize = 256 * 1024;

/// 步最大时长 (kMaxStepSizeOnTask = 1ms / kMaxStepSizeOnAllocation = 5ms;
/// static_assert(分配上限 <= kMaxSynchronuousGCOperation))。
pub const K_MAX_STEP_SIZE_ON_TASK_MS: u64 = 1;
pub const K_MAX_STEP_SIZE_ON_ALLOCATION_MS: u64 = 5;

/// 激活阈值 (非 DEBUG: 8MB/8MB; DEBUG 构建为 0 即立即激活)。
pub const K_V8_ACTIVATION_THRESHOLD: usize = 8 * 1024 * 1024;
pub const K_GLOBAL_ACTIVATION_THRESHOLD: usize = 8 * 1024 * 1024;
pub const K_V8_ACTIVATION_THRESHOLD_DEBUG: usize = 0;
pub const K_GLOBAL_ACTIVATION_THRESHOLD_DEBUG: usize = 0;

/// 对应 GetMaxDuration(step_origin): kV8 → 分配上限 5ms; kTask → 任务上限 1ms。
/// v8: src/heap/incremental-marking.cc GetMaxDuration
#[must_use]
pub fn get_max_duration_ms(step_origin: StepOrigin) -> u64 {
  match step_origin {
    StepOrigin::V8 => K_MAX_STEP_SIZE_ON_ALLOCATION_MS,
    StepOrigin::TASK => K_MAX_STEP_SIZE_ON_TASK_MS,
  }
}

/// 对应 IsBelowActivationThresholds: 任一超阈值即到激活线
/// (v8 与 global 双阈值, DEBUG 构建阈值 0 → 恒不阻拦)。
/// v8: src/heap/incremental-marking.cc IncrementalMarking::IsBelowActivationThresholds
#[must_use]
pub fn is_below_activation_thresholds(v8_size: usize, global_size: usize, debug_build: bool) -> bool {
  let (v8t, gt) = if debug_build {
    (K_V8_ACTIVATION_THRESHOLD_DEBUG, K_GLOBAL_ACTIVATION_THRESHOLD_DEBUG)
  } else {
    (K_V8_ACTIVATION_THRESHOLD, K_GLOBAL_ACTIVATION_THRESHOLD)
  };
  v8_size < v8t || global_size < gt
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(K_MAJOR_GC_YOUNG_GENERATION_ALLOCATION_OBSERVER_STEP, 64 * 1024);
  assert_eq!(K_MAJOR_GC_OLD_GENERATION_ALLOCATION_OBSERVER_STEP, 256 * 1024);
  assert_eq!(get_max_duration_ms(StepOrigin::V8), 5);
  assert_eq!(get_max_duration_ms(StepOrigin::TASK), 1);
  // DEBUG: 阈值 0 → 任何正占用都不低于阈值
  assert!(!is_below_activation_thresholds(1, 1, true));
  assert!(is_below_activation_thresholds(1, 1, false));
  assert!(!is_below_activation_thresholds(9 * 1024 * 1024, 0, false));
};
`;

const p = writeSrc("v8-heap-gc", "incremental_marking_cc.rs.skel", rust);
console.log(`written ${p}`);
