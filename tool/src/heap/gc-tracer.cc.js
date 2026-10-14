#!/usr/bin/env bun
// tool/src/heap/gc-tracer.cc.js
// 机械提取 v8/src/heap/gc-tracer.cc + gc-tracer-inl.h (差集: .h 由既有 gc-tracer.h.js 覆盖):
// ToString 常量表 (Event::Type 双名/State/Priority) + 速度界常量 (BoundedAverageSpeed/
// BoundedThroughput) + cc 方法清单计数 + inl 计时规则。
// gc-tracer 族按 todo 看板落 v8-heap-gc (与既有 gc_tracer.rs 同族)。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/heap/gc-tracer.cc");
const inl = readV8("src/heap/gc-tracer-inl.h");

// —— ToString(Event::Type) 双名表逐项锚 ——
const typeTable = [
  ["SCAVENGER", "s", "Scavenge"],
  ["MARK_COMPACTOR", "mc", "Mark-Compact"],
  ["INCREMENTAL_MARK_COMPACTOR", "mc", "Incremental Mark-Compact"],
  ["MINOR_MARK_SWEEPER", "mms", "Minor Mark-Sweep"],
  ["INCREMENTAL_MINOR_MARK_SWEEPER", "mms", "Incremental Minor Mark-Sweep"],
  ["START", "st", "Start"],
];
for (const [k, short, long] of typeTable) {
  if (!cc.includes(`case GCTracer::Event::Type::${k}:`)) throw new Error(`Type 臂缺 ${k}`);
  if (!cc.includes(`? "${short}" : "${long}"`)) throw new Error(`Type 名漂移: ${k}`);
}
// —— State 表 ——
const stateTable = [["MARKING", "Marking"], ["ATOMIC", "Atomic"], ["SWEEPING", "Sweeping"]];
for (const [k, s] of stateTable) {
  if (!cc.includes(`case GCTracer::Event::State::${k}:`) || !cc.includes(`return "${s}";`)) {
    throw new Error(`State 表漂移: ${k}`);
  }
}
// —— Priority 表 ——
const prioTable = [["kUserBlocking", "UserBlocking"], ["kUserVisible", "UserVisible"], ["kBestEffort", "BestEffort"]];
for (const [k, s] of prioTable) {
  if (!cc.includes(`case v8::Isolate::Priority::${k}:`) || !cc.includes(`return "${s}";`)) {
    throw new Error(`Priority 表漂移: ${k}`);
  }
}
// —— 速度界常量 ——
for (const a of [
  "constexpr size_t kMinNonEmptySpeedInBytesPerMs = 1;",
  "constexpr size_t kMaxSpeedInBytesPerMs = GB;",
  "std::optional<double> BoundedAverageSpeed(",
  "double BoundedThroughput(const ::heap::base::SmoothedBytesAndDuration& buffer) {",
  "return std::min(buffer.GetThroughput(), kMaxSpeedInBytesPerMs);",
]) {
  if (!cc.includes(a)) throw new Error(`gc-tracer.cc 常量锚缺失: ${a}`);
}
// —— cc 方法清单 (GCTracer::Xxx / GCTracer::Nested::Xxx 定义引用) ——
const refs = [...new Set([...cc.matchAll(/GCTracer::(\w+)(?:::(\w+))?\s*\(/g)].map((m) => m[2] || m[1]))];
if (refs.length < 40) throw new Error(`GCTracer 方法 ${refs.length}, 期望 >= 40`);
for (const must of ["StartCycle", "StopCycle", "ReportFullCycleToRecorder", "SampleAllocation", "RecordMutatorUtilization"]) {
  if (!refs.includes(must)) throw new Error(`方法清单缺 ${must}`);
}
// —— inl 计时规则锚 ——
for (const a of [
  "GCTracer::Scope::Scope(GCTracer* tracer, ScopeId scope, ThreadKind thread_kind)",
  "GCTracer::Scope::~Scope() {",
  "constexpr const char* GCTracer::Scope::Name(ScopeId id) {",
  "constexpr bool GCTracer::Scope::NeedsYoungEpoch(ScopeId id) {",
]) {
  if (!inl.includes(a)) throw new Error(`gc-tracer-inl.h 锚缺失: ${a}`);
}
console.log(`GCTracer.cc: Type 表 6 + State 表 3 + Priority 表 3 + 方法 ${refs.length}`);

let rust = banner(
  "src/heap/gc-tracer.cc.js",
  "src/heap/gc-tracer.{cc,-inl.h}",
  "GC 追踪: 事件名常量表与速度界",
);

rust += `
/// 事件类型名表 (对应 ToString(GCTracer::Event::Type, short_name) 双名)。
/// v8: src/heap/gc-tracer.cc ToString(Event::Type)
#[must_use]
pub fn event_type_to_string(type_: &str, short_name: bool) -> &'static str {
  match type_ {
${typeTable.map(([k, s, l]) => `    "${k}" => if short_name { "${s}" } else { "${l}" },`).join("\n")}
    _ => unreachable!("Event::Type 全枚举"),
  }
}

/// 事件状态名 (对应 ToString(GCTracer::Event::State))。
/// v8: src/heap/gc-tracer.cc ToString(Event::State)
#[must_use]
pub fn event_state_to_string(state: &str) -> &'static str {
  match state {
${stateTable.map(([k, s]) => `    "${k}" => "${s}",`).join("\n")}
    "NOT_RUNNING" => "Not running",
    _ => unreachable!("Event::State 全枚举"),
  }
}

/// isolate 优先级名 (对应 ToString(v8::Isolate::Priority))。
/// v8: src/heap/gc-tracer.cc ToString(Priority)
#[must_use]
pub fn priority_to_string(priority: &str) -> &'static str {
  match priority {
${prioTable.map(([k, s]) => `    "${k}" => "${s}",`).join("\n")}
    _ => unreachable!("Priority 全枚举"),
  }
}

/// 速度界 (对应 BoundedAverageSpeed 常量: 非空下限 1 B/ms, 上限 1 GB/ms)。
pub const K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS: f64 = 1.0;
pub const K_MAX_SPEED_IN_BYTES_PER_MS: f64 = 1024.0 * 1024.0 * 1024.0;

/// 对应 BoundedAverageSpeed: 环形缓冲均值夹在 [1, GB] 内。
/// v8: src/heap/gc-tracer.cc BoundedAverageSpeed
#[must_use]
pub fn bounded_average_speed(raw: Option<f64>) -> Option<f64> {
  raw.map(|v| v.clamp(K_MIN_NON_EMPTY_SPEED_IN_BYTES_PER_MS, K_MAX_SPEED_IN_BYTES_PER_MS))
}

/// 对应 BoundedThroughput: 平滑吞吐上限 GB/ms。
/// v8: src/heap/gc-tracer.cc BoundedThroughput
#[must_use]
pub fn bounded_throughput(raw: f64) -> f64 {
  raw.min(K_MAX_SPEED_IN_BYTES_PER_MS)
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(event_type_to_string("SCAVENGER", false), "Scavenge");
  assert_eq!(event_type_to_string("SCAVENGER", true), "s");
  assert_eq!(event_type_to_string("INCREMENTAL_MARK_COMPACTOR", true), "mc");
  assert_eq!(event_type_to_string("INCREMENTAL_MINOR_MARK_SWEEPER", false), "Incremental Minor Mark-Sweep");
  assert_eq!(event_type_to_string("START", true), "st");
  assert_eq!(event_state_to_string("MARKING"), "Marking");
  assert_eq!(event_state_to_string("SWEEPING"), "Sweeping");
  assert_eq!(priority_to_string("kBestEffort"), "BestEffort");
  assert_eq!(bounded_average_speed(Some(0.5)), Some(1.0)); // 下限兜底
  assert_eq!(bounded_average_speed(Some(1e12)), Some(K_MAX_SPEED_IN_BYTES_PER_MS));
  assert_eq!(bounded_average_speed(None), None);
  assert_eq!(bounded_throughput(2e12), K_MAX_SPEED_IN_BYTES_PER_MS);
  assert_eq!(bounded_throughput(123.0), 123.0);
};
`;

const p = writeSrc("v8-heap-gc", "gc_tracer_cc.rs.skel", rust);
console.log(`written ${p}`);
