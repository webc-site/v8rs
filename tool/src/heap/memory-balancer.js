#!/usr/bin/env bun
// tool/src/heap/memory-balancer.js
// 机械提取 v8/src/heap/memory-balancer{.h,.cc} 族:
// MemoryBalancer EWMA 常量与签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/memory-balancer.h");
const ccSrc = readV8("src/heap/memory-balancer.cc");

const anchors = [
  [hSrc, "class MemoryBalancer {"],
  [hSrc, "static constexpr double kMajorAllocationDecayRate = 0.95;"],
  [hSrc, "static constexpr double kMajorGCDecayRate = 0.5;"],
  [hSrc, "void UpdateAllocationRate(size_t major_allocation_bytes,"],
  [hSrc, "void UpdateGCSpeed(size_t major_gc_bytes, base::TimeDelta major_gc_duration);"],
  [hSrc, "void HeartbeatUpdate();"],
  [hSrc, "void RecomputeLimits(size_t embedder_allocation_limit, base::TimeTicks time);"],
  [hSrc, "class SmoothedBytesAndDuration {"],
  [hSrc, "double rate() const { return bytes_ / duration_; }"],
  [hSrc, "class HeartbeatTask : public CancelableTask {"],
  [ccSrc, "void MemoryBalancer::RefreshLimit() {"],
  [ccSrc, "void MemoryBalancer::PostHeartbeatTask() {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// 常量真提取
const allocDecay = parseFloat(hSrc.match(/kMajorAllocationDecayRate = ([\d.]+);/)[1]);
const gcDecay = parseFloat(hSrc.match(/kMajorGCDecayRate = ([\d.]+);/)[1]);
if (allocDecay !== 0.95 || gcDecay !== 0.5) throw new Error(`decay 漂移: ${allocDecay}/${gcDecay}`);
// EWMA 公式锚
if (!hSrc.includes("bytes_ * decay_rate + static_cast<double>(bytes) * (1 - decay_rate);")) {
  throw new Error("EWMA 公式漂移");
}
console.log(`memory-balancer: decay 常量 2 (${allocDecay}/${gcDecay}), 类 2, EWMA 公式锚 1`);

let rust = banner(
  "src/heap/memory-balancer.js",
  "src/heap/memory-balancer.h, src/heap/memory-balancer.cc",
  "内存均衡器: EWMA 平滑分配率/GC 速度并重算限额",
);

rust += `/// v8: src/heap/memory-balancer.h MemoryBalancer::SmoothedBytesAndDuration
/// EWMA: x' = x * decay + new * (1 - decay); rate() = bytes / duration
pub struct SmoothedBytesAndDuration {
  bytes: f64,
  duration: f64,
}

impl SmoothedBytesAndDuration {
  pub fn new(bytes: usize, duration: f64) -> Self {
    Self { bytes: bytes as f64, duration }
  }
  /// v8: SmoothedBytesAndDuration::Update
  pub fn update(&mut self, bytes: usize, duration: f64, decay_rate: f64) {
    self.bytes = self.bytes * decay_rate + bytes as f64 * (1.0 - decay_rate);
    self.duration = self.duration * decay_rate + duration * (1.0 - decay_rate);
  }
  /// v8: SmoothedBytesAndDuration::rate — bytes/ms
  pub fn rate(&self) -> f64 { self.bytes / self.duration }
}

/// v8: src/heap/memory-balancer.h MemoryBalancer
pub struct MemoryBalancer {
  /// v8: heap_
  pub heap: *mut Heap,
  /// v8: live_memory_after_gc_ = 0
  pub live_memory_after_gc: usize,
  /// v8: embedder_allocation_limit_ = 0
  pub embedder_allocation_limit: usize,
  /// v8: major_allocation_rate_ / major_gc_speed_ (optional)
  /// v8: last_measured_memory_ = 0 / last_measured_at_
  /// v8: heartbeat_task_started_ = false
  pub heartbeat_task_started: bool,
}

impl MemoryBalancer {
  /// v8: kMajorAllocationDecayRate
  pub const K_MAJOR_ALLOCATION_DECAY_RATE: f64 = ${allocDecay};
  /// v8: kMajorGCDecayRate
  pub const K_MAJOR_GC_DECAY_RATE: f64 = ${gcDecay};

  /// v8: MemoryBalancer::UpdateAllocationRate
  pub fn update_allocation_rate(&mut self, bytes: usize, duration: TimeDelta) { todo!() }
  /// v8: MemoryBalancer::UpdateGCSpeed
  pub fn update_gc_speed(&mut self, bytes: usize, duration: TimeDelta) { todo!() }
  /// v8: MemoryBalancer::HeartbeatUpdate — 心跳测差分推分配率
  pub fn heartbeat_update(&mut self) { todo!() }
  /// v8: MemoryBalancer::RecomputeLimits — 由速率与上界推旧生代限额
  pub fn recompute_limits(&mut self, embedder_allocation_limit: usize, time: TimeTicks) { todo!() }
  /// v8: MemoryBalancer::RefreshLimit / PostHeartbeatTask (cc)
}

/// v8: src/heap/memory-balancer.h HeartbeatTask — CancelableTask, RunInternal 调 HeartbeatUpdate
/// v8: src/heap/memory-balancer.cc HeartbeatTask::RunInternal
`;

const p = writeSrc("v8-heap-core", "memory_balancer.rs.skel", rust);
console.log(`written ${p}`);
