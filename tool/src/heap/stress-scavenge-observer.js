#!/usr/bin/env bun
// tool/src/heap/stress-scavenge-observer.js
// 机械提取 v8/src/heap/stress-scavenge-observer{.h,.cc} 族:
// StressScavengeObserver — 按 --stress-scavenge 百分比触发 GC

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/stress-scavenge-observer.h");
const ccSrc = readV8("src/heap/stress-scavenge-observer.cc");

const anchors = [
  [hSrc, "class StressScavengeObserver"],
  [hSrc, "void Step(int bytes_allocated, Address soon_object, size_t size) override;"],
  [hSrc, "bool HasRequestedGC() const;"],
  [hSrc, "void RequestedGCDone();"],
  [hSrc, "double MaxNewSpaceSizeReached() const;"],
  [hSrc, "int limit_percentage_;"],
  [hSrc, "bool has_requested_gc_;"],
  [hSrc, "double max_new_space_size_reached_;"],
  [hSrc, "int NextLimit(int min = 0);"],
  [ccSrc, "limit_percentage_ = NextLimit();"],
  [ccSrc, "void StressScavengeObserver::Step("],
  [ccSrc, "limit_percentage_ = NextLimit(static_cast<int>(current_percent));"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

console.log(`stress-scavenge-observer: 字段 5, 方法 6 (Step/HasRequestedGC/RequestedGCDone/MaxNewSpaceSizeReached/NextLimit/构造)`);

let rust = banner(
  "src/heap/stress-scavenge-observer.js",
  "src/heap/stress-scavenge-observer.h, src/heap/stress-scavenge-observer.cc",
  "压力 scavenge 观察者: 随机百分比限幅触发小 GC",
);

rust += `/// v8: src/heap/stress-scavenge-observer.h StressScavengeObserver (AllocationObserver)
pub struct StressScavengeObserver<'a> {
  /// v8: heap_
  pub heap: &'a mut Heap,
  /// v8: limit_percentage_ — NextLimit() 随机重抽
  pub limit_percentage: i32,
  /// v8: has_requested_gc_
  pub has_requested_gc: bool,
  /// v8: max_new_space_size_reached_ — fuzzer-gc-analysis 统计
  pub max_new_space_size_reached: f64,
}

impl StressScavengeObserver<'_> {
  /// v8: StressScavengeObserver::Step — 越限则 RequestGC, trace 输出
  pub fn step(&mut self, bytes_allocated: i32, soon_object: Address, size: usize) { todo!() }
  /// v8: HasRequestedGC / RequestedGCDone
  /// v8: MaxNewSpaceSizeReached
  /// v8: NextLimit(min = 0) — flag:stress-scavenge 随机 [min, 100]
  /// v8: 构造: limit_percentage_ = NextLimit() + trace 输出 "[StressScavenge] %d%% ..."
`;

const p = writeSrc("v8-heap-core", "stress_scavenge_observer.rs.skel", rust);
console.log(`written ${p}`);
