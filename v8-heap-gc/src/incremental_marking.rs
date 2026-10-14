// 增量标记: 步进上限/激活阈值常量与步来源选限
//
// 族: v8/src/heap/incremental-marking.{h,cc}
// 依赖: 无 (纯常量 + 纯函数; StepOrigin 是 cc 侧消费的 canonical 枚举)

/// 增量标记步来源 (对应 enum class StepOrigin, 声明于 incremental-marking.h)。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepOrigin {
  /// kV8: 生成代码/运行时触发
  V8,
  /// kTask: 平台任务触发
  Task,
}

/// 分配观察者步长 (kMajorGC{Young,Old}GenerationAllocationObserverStep)。
pub const K_MAJOR_GC_YOUNG_GENERATION_ALLOCATION_OBSERVER_STEP: usize = 64 * 1024;
pub const K_MAJOR_GC_OLD_GENERATION_ALLOCATION_OBSERVER_STEP: usize = 256 * 1024;

/// 步最大时长 (kMaxStepSizeOnTask = 1ms / kMaxStepSizeOnAllocation = 5ms;
/// static_assert(分配上限 <= kMaxSynchronuousGCOperation))。
pub const K_MAX_STEP_SIZE_ON_TASK_MS: u64 = 1;
pub const K_MAX_STEP_SIZE_ON_ALLOCATION_MS: u64 = 5;

/// 激活阈值 (非 DEBUG: 8MB/8MB; DEBUG 构建为 0 即立即激活)。
/// v8: src/heap/incremental-marking.cc kV8ActivationThreshold / kGlobalActivationThreshold
pub const K_V8_ACTIVATION_THRESHOLD: usize = 8 * 1024 * 1024;
pub const K_GLOBAL_ACTIVATION_THRESHOLD: usize = 8 * 1024 * 1024;
pub const K_V8_ACTIVATION_THRESHOLD_DEBUG: usize = 0;
pub const K_GLOBAL_ACTIVATION_THRESHOLD_DEBUG: usize = 0;

/// 对应 GetMaxDuration(step_origin): kV8 → 分配上限 5ms; kTask → 任务上限 1ms。
/// 注: 原版首分支 `if (v8_flags.predictable) return TimeDelta::Max()` (im.cc:70-72)
/// 属 flag 运行值域, 此处未建模 (无 Max 哨兵), 可预测模式由调用侧直传 step 承载。
/// v8: src/heap/incremental-marking.cc GetMaxDuration
#[must_use]
pub const fn get_max_duration_ms(step_origin: StepOrigin) -> u64 {
  match step_origin {
    StepOrigin::V8 => K_MAX_STEP_SIZE_ON_ALLOCATION_MS,
    StepOrigin::Task => K_MAX_STEP_SIZE_ON_TASK_MS,
  }
}

/// 对应 IsBelowActivationThresholds: 双阈值均 `<=` 才算未达激活线 (upstream AND)。
/// DEBUG 构建阈值 0 → 仅 0 占用满足; 与 upstream 立即激活语义一致。
/// v8: src/heap/incremental-marking.cc:135 IncrementalMarking::IsBelowActivationThresholds
#[must_use]
pub const fn is_below_activation_thresholds(
  v8_size: usize,
  global_size: usize,
  debug_build: bool,
) -> bool {
  let (v8t, gt) = if debug_build {
    (
      K_V8_ACTIVATION_THRESHOLD_DEBUG,
      K_GLOBAL_ACTIVATION_THRESHOLD_DEBUG,
    )
  } else {
    (K_V8_ACTIVATION_THRESHOLD, K_GLOBAL_ACTIVATION_THRESHOLD)
  };
  v8_size <= v8t && global_size <= gt
}

// —— 编译期不变式 (对齐 upstream static_assert) ——
const _: () = {
  assert!(K_MAJOR_GC_YOUNG_GENERATION_ALLOCATION_OBSERVER_STEP == 64 * 1024);
  assert!(K_MAJOR_GC_OLD_GENERATION_ALLOCATION_OBSERVER_STEP == 256 * 1024);
  assert!(get_max_duration_ms(StepOrigin::V8) == 5);
  assert!(get_max_duration_ms(StepOrigin::Task) == 1);
  // DEBUG: 阈值 0 → 任何正占用都不满足 below 条件
  assert!(!is_below_activation_thresholds(1, 1, true));
  // 双阈值皆在 → true
  assert!(is_below_activation_thresholds(1024, 1024, false));
  // 单侧超阈 (v8 侧) → false
  assert!(!is_below_activation_thresholds(9 * 1024 * 1024, 0, false));
  // 边界: 恰等于阈值 → true (upstream `<=`)
  assert!(is_below_activation_thresholds(
    K_V8_ACTIVATION_THRESHOLD,
    K_GLOBAL_ACTIVATION_THRESHOLD,
    false
  ));
};
