#!/usr/bin/env bun
// tool/src/heap/parallel-work-item.js
// 机械提取 v8/src/heap/parallel-work-item.h:
// ParallelWorkItem — relaxed acquire 单发工作项

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/parallel-work-item.h");

const anchors = [
  "class ParallelWorkItem {",
  "bool TryAcquire() {",
  "bool IsAcquired() const {",
  "std::memory_order_relaxed) == false;",
  "bool acquire_{false};",
];
for (const a of anchors) if (!hSrc.includes(a)) throw new Error(`锚点缺失: ${a}`);
if (!hSrc.includes("exchange(") || !hSrc.includes("load(")) throw new Error("缺原子操作");

console.log(`parallel-work-item: 方法 2 (TryAcquire/IsAcquired), 字段 1, 内存序 relaxed`);

let rust = banner(
  "src/heap/parallel-work-item.js",
  "src/heap/parallel-work-item.h",
  "并行工作项: 一次性原子认领",
);

rust += `/// v8: src/heap/parallel-work-item.h ParallelWorkItem
/// 原文以 reinterpret_cast<std::atomic<bool>> 访问 bool acquire_;
/// Rust 直接用 AtomicBool, 语义等价 (relaxed 足够: 项内容自 job 起未变)。
#[derive(Default)]
pub struct ParallelWorkItem {
  acquire: std::sync::atomic::AtomicBool,
}

impl ParallelWorkItem {
  pub fn new() -> Self { Self::default() }

  /// v8: ParallelWorkItem::TryAcquire
  /// exchange(true, Relaxed) == false → 认领成功
  pub fn try_acquire(&self) -> bool {
    self.acquire.swap(true, std::sync::atomic::Ordering::Relaxed) == false
  }

  /// v8: ParallelWorkItem::IsAcquired
  pub fn is_acquired(&self) -> bool {
    self.acquire.load(std::sync::atomic::Ordering::Relaxed)
  }
}
`;

const p = writeSrc("v8-heap-core", "parallel_work_item.rs.skel", rust);
console.log(`written ${p}`);
