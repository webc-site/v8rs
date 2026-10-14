#!/usr/bin/env bun
// tool/src/heap/heap.cc.js
// 机械提取 v8/src/heap/heap.cc + heap-inl.h (差集: heap.h 由既有 heap.h.js 覆盖):
// ComputeMutatorUtilization 公式 / 无效 MarkCompact 连击上限 / 高碎片判定
// (committed - used > used + 16MB) / crash key 前缀 / 方法清单计数对数。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/heap/heap.cc");
const inl = readV8("src/heap/heap-inl.h");

// —— ComputeMutatorUtilization 公式锚 ——
for (const a of [
  "double ComputeMutatorUtilizationImpl(double mutator_speed,",
  "constexpr double kMinMutatorUtilization = 0.0;",
  "constexpr double kConservativeGcSpeedInBytesPerMillisecond = 200000;",
  "if (mutator_speed == 0) return kMinMutatorUtilization;",
  "if (!gc_speed) gc_speed = kConservativeGcSpeedInBytesPerMillisecond;",
  "return *gc_speed / (mutator_speed + *gc_speed);",
]) {
  if (!cc.includes(a)) throw new Error(`heap.cc 公式锚缺失: ${a}`);
}
// —— 高/低分配速率与无效 GC 阈值 ——
for (const a of [
  "constexpr double kHighMutatorUtilization = 0.993;",
  "return mu > kHighMutatorUtilization;",
  "static constexpr int kMaxConsecutiveIneffectiveMarkCompacts = 4;",
  "if (++consecutive_ineffective_mark_compacts_ ==",
  'FatalProcessOutOfMemory("Ineffective mark-compacts near heap limit");',
]) {
  if (!cc.includes(a)) throw new Error(`heap.cc 阈值锚缺失: ${a}`);
}
// —— 高碎片判定 ——
for (const a of [
  "if (committed < used) return false;",
  "constexpr size_t kSlack = 16 * MB;",
  "// Fragmentation is high if committed > 2 * used + kSlack.",
  "return committed - used > used + kSlack;",
]) {
  if (!cc.includes(a)) throw new Error(`heap.cc 碎片锚缺失: ${a}`);
}
// —— crash key ——
if (!cc.includes('constexpr char kCrashKeyPrefix[] = "v8-oom-";')) throw new Error("crash key 前缀漂移");
// —— 方法清单 (Heap::Xxx 定义引用) ——
const refs = [...new Set([...cc.matchAll(/\bHeap::(\w+)\s*\(/g)].map((m) => m[1]))];
if (refs.length < 150) throw new Error(`Heap 方法 ${refs.length}, 期望 >= 150`);
for (const must of ["CollectGarbage", "PerformGarbageCollection", "ConfigureHeap", "IsGCWithStack", "CheckIneffectiveMarkCompact", "HasHighFragmentation"]) {
  if (!refs.includes(must)) throw new Error(`方法清单缺 ${must}`);
}
// —— inl 台账 ——
for (const a of [
  "Isolate* Heap::isolate() const { return Isolate::FromHeap(this); }",
  "bool Heap::IsMainThread() const {",
  "uint64_t Heap::external_memory() const {",
  "RootsTable& Heap::roots_table() { return isolate()->roots_table(); }",
  "void Heap::SetRootMaterializedObjects(Tagged<FixedArray> objects) {",
]) {
  if (!inl.includes(a)) throw new Error(`heap-inl.h 锚缺失: ${a}`);
}
console.log(`Heap.cc: 公式 1 + 阈值 4 + 方法 ${refs.length}`);

let rust = banner(
  "src/heap/heap.cc.js",
  "src/heap/{heap.cc,heap-inl.h}",
  "堆主体: mutator 利用率公式/无效 GC 连击/碎片判定",
);

rust += `
/// mutator 利用率 (对应 ComputeMutatorUtilizationImpl)。
/// mutator_speed == 0 → 0; 无 gc_speed → 保守 200000 B/ms;
/// 公式 gc_speed / (mutator_speed + gc_speed)。
/// v8: src/heap/heap.cc ComputeMutatorUtilizationImpl
#[must_use]
pub fn compute_mutator_utilization(mutator_speed: f64, gc_speed: Option<f64>) -> f64 {
  const K_MIN_MUTATOR_UTILIZATION: f64 = 0.0;
  const K_CONSERVATIVE_GC_SPEED_IN_BYTES_PER_MILLISECOND: f64 = 200000.0;
  if mutator_speed == 0.0 {
    return K_MIN_MUTATOR_UTILIZATION;
  }
  let gc_speed = gc_speed.unwrap_or(K_CONSERVATIVE_GC_SPEED_IN_BYTES_PER_MILLISECOND);
  gc_speed / (mutator_speed + gc_speed)
}

/// 低分配速率判定阈值 (kHighMutatorUtilization = 0.993, HasLow{Young,Old}GenerationAllocationRate)。
pub const K_HIGH_MUTATOR_UTILIZATION: f64 = 0.993;

/// 无效 MarkCompact 连击上限 (kMaxConsecutiveIneffectiveMarkCompacts = 4)。
/// 连续第 4 次触发 near-heap-limit 回调 / OOM 崩溃。
pub const K_MAX_CONSECUTIVE_INEFFECTIVE_MARK_COMPACTS: i32 = 4;

/// 对应 CheckIneffectiveMarkCompact 的连击状态机:
/// 无效 → 计数 +1, 到 4 走终局 (回调提限则清零); 有效 → 清零。
/// v8: src/heap/heap.cc Heap::CheckIneffectiveMarkCompact
#[must_use]
pub fn step_ineffective_mark_compact(counter: i32, is_ineffective: bool, near_limit_callback_raised: bool) -> i32 {
  if !is_ineffective {
    return 0;
  }
  let next = counter + 1;
  if next == K_MAX_CONSECUTIVE_INEFFECTIVE_MARK_COMPACTS {
    // 回调提限 → 清零; 否则 FatalProcessOutOfMemory (调用方处置)
    if near_limit_callback_raised {
      0
    } else {
      next
    }
  } else {
    next
  }
}

/// 高碎片判定 (对应 HasHighFragmentation: committed - used > used + 16MB;
/// committed < used (后台分配暂时倒挂) 直接判低)。
/// v8: src/heap/heap.cc Heap::HasHighFragmentation
#[must_use]
pub fn has_high_fragmentation(committed: usize, used: usize) -> bool {
  if committed < used {
    return false;
  }
  const K_SLACK: usize = 16 * 1024 * 1024;
  committed - used > used + K_SLACK
}

/// OOM crash key 前缀 (kCrashKeyPrefix = "v8-oom-")。
pub const K_CRASH_KEY_PREFIX: &str = "v8-oom-";

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(compute_mutator_utilization(0.0, Some(100.0)), 0.0);
  // 保守 gc_speed: 200000 / (200000 + 200000) = 0.5
  assert_eq!(compute_mutator_utilization(200000.0, None), 0.5);
  assert_eq!(compute_mutator_utilization(300000.0, Some(100000.0)), 0.25);
  assert_eq!(K_HIGH_MUTATOR_UTILIZATION, 0.993);
  assert_eq!(K_MAX_CONSECUTIVE_INEFFECTIVE_MARK_COMPACTS, 4);
  assert_eq!(step_ineffective_mark_compact(0, false, false), 0);
  assert_eq!(step_ineffective_mark_compact(0, true, false), 1);
  assert_eq!(step_ineffective_mark_compact(3, true, true), 0); // 回调提限清零
  assert_eq!(step_ineffective_mark_compact(3, true, false), 4); // 终局
  assert!(!has_high_fragmentation(100, 200)); // 倒挂 → 低
  assert!(!has_high_fragmentation(300, 100)); // 差 200 远小于 used+16MB → 低
  assert!(has_high_fragmentation(100 + 2 * 32 * 1024 * 1024, 32 * 1024 * 1024 + 1));
  assert_eq!(K_CRASH_KEY_PREFIX, "v8-oom-");
};
`;

const p = writeSrc("v8-heap-core", "heap_cc.rs.skel", rust);
console.log(`written ${p}`);
