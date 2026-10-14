#!/usr/bin/env bun
// tool/src/heap/pretenuring-handler.js
// 机械提取 v8/src/heap/pretenuring-handler{.h,.cc,-inl.h} 族:
// PretenuringHandler 常量/枚举/方法骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/pretenuring-handler.h");
const ccSrc = readV8("src/heap/pretenuring-handler.cc");
const inlSrc = readV8("src/heap/pretenuring-handler-inl.h");

const anchors = [
  [hSrc, "class PretenuringHandler final {"],
  [hSrc, "static constexpr int kInitialFeedbackCapacity = 256;"],
  [hSrc, "using PretenuringFeedbackMap ="],
  [hSrc, "enum FindMementoMode { kForRuntime, kForGC };"],
  [hSrc, "static inline Tagged<AllocationMemento> FindAllocationMemento("],
  [ccSrc, "void PretenuringHandler::ProcessPretenuringFeedback("],
  [ccSrc, "void PretenuringHandler::MergeAllocationSitePretenuringFeedback("],
  [inlSrc, "PretenuringHandler::FindAllocationMemento"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

const cap = parseInt(hSrc.match(/kInitialFeedbackCapacity = (\d+);/)[1], 10);
if (cap !== 256) throw new Error("kInitialFeedbackCapacity 漂移");
const mode = hSrc.match(/enum FindMementoMode \{([^}]*)\};/)[1].split(",").map((s) => s.trim());
if (mode.join() !== "kForRuntime,kForGC") throw new Error(`FindMementoMode 漂移: ${mode}`);
console.log(`pretenuring-handler: kInitialFeedbackCapacity=${cap}, FindMementoMode ${mode.length}`);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
let rust = banner(
  "src/heap/pretenuring-handler.js",
  "src/heap/pretenuring-handler.h, src/heap/pretenuring-handler.cc, src/heap/pretenuring-handler-inl.h",
  "预提晋升: memento 反馈采样与决策",
);

rust += `/// v8: src/heap/pretenuring-handler.h FindMementoMode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FindMementoMode {
`;
mode.forEach((k, i) => (rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`));
rust += `}

/// v8: src/heap/pretenuring-handler.h PretenuringHandler
pub struct PretenuringHandler {
  /// v8: heap_
  pub heap: *mut Heap,
  /// v8: allocations_ / PretenuringFeedbackMap feedback (清后保留容量)
  /// v8: byte_counts_ / byte_sizes_ (按 age 分桶)
}

impl PretenuringHandler {
  /// v8: kInitialFeedbackCapacity = 256
  pub const K_INITIAL_FEEDBACK_CAPACITY: i32 = ${cap};

  /// v8: FindAllocationMemento<mode> — memento 紧邻对象后 (-inl.h)
  /// v8: CalculatePretenuringFeedback — 由 memento 采样推晋升阈值 (cc)
  /// v8: PretenuringHandler 构造/析构 (cc)
  /// v8: Clear / MergeData / ResetAll data 面见 cc
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FindMementoMode::FOR_GC == 1);
  assert!(PretenuringHandler::K_INITIAL_FEEDBACK_CAPACITY == 256);
};
`;

const p = writeSrc("v8-heap-core", "pretenuring_handler.rs.skel", rust);
console.log(`written ${p}`);
