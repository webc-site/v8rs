#!/usr/bin/env bun
// tool/src/heap/heap.h.js
// 机械提取 v8/src/heap/heap.{h,cc} 的规则区: SkipRoot/GCFlag/CompleteSweepingReason/
// HeapState/SlotClearingMode/EmbedderStackStateOrigin/LeaveHeapState/StackScanMode 枚举
// 与 CompleteSweepingReason::ToString 名称表。(heap 类本体为语义实现, 留给 Developer)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/heap.h");

// 枚举体通用提取
function enumBody(name, extra = "") {
  const re = new RegExp(`enum(?: class)? ${name}[^\\{]*\\{([^}]*)\\};`);
  const m = hSrc.match(re);
  if (!m) throw new Error(`未找到枚举 ${name}`);
  return m[1]
    .replace(/\/\/[^\n]*/g, "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

const skipRoot = enumBody("SkipRoot");
if (skipRoot.length !== 10) throw new Error(`SkipRoot ${skipRoot.length}, 期望 10`);
if (skipRoot[0] !== "kExternalStringTable" || skipRoot[9] !== "kReadOnlyBuiltins") {
  throw new Error("SkipRoot 锚点漂移");
}

const gcFlag = enumBody("GCFlag");
if (gcFlag.length !== 4) throw new Error(`GCFlag ${gcFlag.length}, 期望 4`);
if (!gcFlag.includes("kReduceMemoryFootprint = 1 << 0")) throw new Error("GCFlag 位定义漂移");

const sweepReasons = enumBody("CompleteSweepingReason");
if (sweepReasons.length !== 11) throw new Error(`CompleteSweepingReason ${sweepReasons.length}, 期望 11`);

// ToString 名称表 (cc 顺序无关, 按 case 提取)
const strTable = [...hSrc.matchAll(
  /case CompleteSweepingReason::k(\w+):\s*\n\s*return "([^"]+)";/g,
)].map((m) => ({ name: m[1], str: m[2] }));
if (strTable.length !== 11) throw new Error(`ToString 表 ${strTable.length} 条, 期望 11`);

const heapState = enumBody("HeapState");
if (heapState.join() !== "NOT_IN_GC,SCAVENGE,MARK_COMPACT,MINOR_MARK_SWEEP,TEAR_DOWN") {
  throw new Error(`HeapState 漂移: ${heapState}`);
}

const slotClearing = enumBody("SlotClearingMode");
if (slotClearing.join() !== "kCheckSweeping,kUnconditional") throw new Error("SlotClearingMode 漂移");
const embedderOrigin = enumBody("EmbedderStackStateOrigin");
if (embedderOrigin.length !== 2) throw new Error("EmbedderStackStateOrigin 漂移");
const leaveHeap = enumBody("LeaveHeapState");
if (leaveHeap.length !== 2) throw new Error("LeaveHeapState 漂移");
const stackScan = enumBody("StackScanMode");
if (stackScan.join() !== "kNone,kFull,kSelective") throw new Error("StackScanMode 漂移");

// GCFlag 位值
const gcFlagBits = {
  kNoFlags: 0,
  kReduceMemoryFootprint: 1,
  kForced: 2,
  kLastResort: 3,
};

console.log(
  `SkipRoot ${skipRoot.length}, GCFlag ${gcFlag.length}, SweepReason ${sweepReasons.length}, HeapState ${heapState.length}`,
);

let rust = banner(
  "src/heap/heap.h.js",
  "src/heap/heap.{h,cc}",
  "Heap 顶层枚举与名称表 (Heap 类本体为语义实现, 不在本产物)",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

rust += `
/// 根跳过集合 (对应 enum class SkipRoot)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SkipRoot {
`;

skipRoot.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// GC 触发标志位 (对应 enum class GCFlag : uint8_t, 位标志)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum GCFlag {
  #[default]
  /// kNoFlags = 0
  NO_FLAGS = 0,
  /// kReduceMemoryFootprint = 1 << 0
  REDUCE_MEMORY_FOOTPRINT = 1 << 0,
  /// kForced = 1 << 1 (测试 --expose-gc 或 DevTools LowMemoryNotification 强制 GC)
  FORCED = 1 << 1,
  /// kLastResort = 1 << 2
  LAST_RESORT = 1 << 2,
}

/// GCFlag 组合位集 (对应 using GCFlags = base::Flags<GCFlag, uint8_t>)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GCFlags(pub u8);

impl GCFlags {
  /// 对应 base::Flags 的位或
  #[must_use]
  pub const fn contains(self, flag: GCFlag) -> bool {
    (self.0 & flag as u8) == flag as u8
  }

  /// 并入一个标志
  pub fn add(&mut self, flag: GCFlag) {
    self.0 |= flag as u8;
  }
}

/// 完成清扫的原因 (对应 enum class CompleteSweepingReason)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompleteSweepingReason {
`;

sweepReasons.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

impl CompleteSweepingReason {
  /// 名称表 (对应 ToString(CompleteSweepingReason))
  #[inline]
  #[must_use]
  pub const fn to_str(self) -> &'static str {
    match self {
`;

for (const t of strTable) {
  rust += `      Self::${snake("k" + t.name)} => "${t.str}",\n`;
}

rust += `    }
  }
}

/// Heap 主状态 (对应 enum HeapState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum HeapState {
`;

heapState.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// 槽清除方式 (对应 enum class SlotClearingMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SlotClearingMode {
`;

slotClearing.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// Embedder 栈状态来源 (对应 enum class EmbedderStackStateOrigin)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EmbedderStackStateOrigin {
`;

embedderOrigin.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// 离开堆时的状态 (对应 enum class LeaveHeapState)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LeaveHeapState {
`;

leaveHeap.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

/// 栈扫描方式 (对应 enum class StackScanMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StackScanMode {
`;

stackScan.forEach((k, i) => {
  rust += `  /// ${k}\n  ${snake(k)} = ${i},\n`;
});

rust += `}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SkipRoot::EXTERNAL_STRING_TABLE == 0);
  assert!(SkipRoot::READ_ONLY_BUILTINS == 9);
  assert!(GCFlag::REDUCE_MEMORY_FOOTPRINT as u8 == 1);
  assert!(GCFlag::FORCED as u8 == 2);
  assert!(GCFlag::LAST_RESORT as u8 == 4);
  assert!(CompleteSweepingReason::COLLECT_CODE_STATISTICS == 0);
  assert!(CompleteSweepingReason::READ_ONLY == 10);
  assert!(CompleteSweepingReason::MINOR_GC.to_str() == "minor gc");
  assert!(HeapState::NOT_IN_GC == 0);
  assert!(HeapState::TEAR_DOWN == 4);
  assert!(StackScanMode::SELECTIVE == 2);
};
`;
;

const p = writeSrc("v8-heap-core", "heap_enums.rs", rust);
console.log(`written ${p}`);
