#!/usr/bin/env bun
// tool/src/heap/evacuation-allocator.h.js
// 机械提取 v8/src/heap/evacuation-allocator.{h,cc,-inl.h}:
// 压空分配器台账 (5 路 MainAllocator 惰性 optional + CompactionSpaceKind 枚举,
// 后者定义于 src/common/globals.h) + FreeLast/Allocate 规则锚。

import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/heap/evacuation-allocator.h");
const cc = readV8("src/heap/evacuation-allocator.cc");
const inl = readV8("src/heap/evacuation-allocator-inl.h");
const globals = readV8("src/common/globals.h");

for (const a of [
  "class EvacuationAllocator {",
  "EvacuationAllocator(Heap* heap, CompactionSpaceKind compaction_space_kind);",
  "void Finalize();",
  "inline AllocationResult Allocate(AllocationSpace space,",
  "void FreeLast(AllocationSpace space, Tagged<HeapObject> object,",
  "void FreeLastInMainAllocator(MainAllocator* allocator,",
  "std::optional<MainAllocator> new_space_allocator_;",
  "std::optional<MainAllocator> old_space_allocator_;",
  "std::optional<MainAllocator> code_space_allocator_;",
  "std::optional<MainAllocator> shared_space_allocator_;",
  "std::optional<MainAllocator> trusted_space_allocator_;",
  "CompactionSpaceCollection compaction_spaces_;",
]) {
  if (!h.includes(a)) throw new Error(`evacuation-allocator.h 锚点缺失: ${a}`);
}
for (const a of [
  "EvacuationAllocator::EvacuationAllocator(",
  "void EvacuationAllocator::Finalize() {",
  "void EvacuationAllocator::FreeLast(",
  "void EvacuationAllocator::FreeLastInMainAllocator(",
]) {
  if (!cc.includes(a)) throw new Error(`evacuation-allocator.cc 锚点缺失: ${a}`);
}
if (!inl.includes("AllocationResult EvacuationAllocator::Allocate(")) {
  throw new Error("evacuation-allocator-inl.h Allocate 缺失");
}
// CompactionSpaceKind (globals.h) 值锚
const kind = globals.match(/enum class CompactionSpaceKind \{([^}]*)\};/);
if (!kind) throw new Error("globals.h CompactionSpaceKind 缺失");
const kinds = kind[1].split(",").map((s) => s.trim()).filter(Boolean);
if (kinds.join() !== "kNone,kCompactionSpaceForScavenge,kCompactionSpaceForMarkCompact,kCompactionSpaceForMinorMarkSweep") {
  throw new Error(`CompactionSpaceKind 漂移: ${kinds}`);
}
console.log("EvacuationAllocator: 方法 5 + 分配器槽 5 + CompactionSpaceKind 4 值");

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

let rust = banner(
  "src/heap/evacuation-allocator.h.js",
  "src/heap/evacuation-allocator.{h,cc,-inl.h}",
  "压空分配器: GC 期临时分配与最后一个对象回退",
);

rust += `
/// 压实空间种类 (对应 src/common/globals.h enum class CompactionSpaceKind)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompactionSpaceKind {
`;
kinds.forEach((k, i) => {
  rust += `  /// ${k} = ${i}\n  ${snake(k)} = ${i},\n`;
});
rust += `}

/// 压空分配器骨架 (对应 class EvacuationAllocator)。
/// GC 期间在压实空间分配; Finalize 由主线程把压实空间并回主空间。
/// 5 路 MainAllocator 均为 optional (惰性构造, 按需启用)。
/// v8: src/heap/evacuation-allocator.h EvacuationAllocator
pub struct EvacuationAllocatorSkeleton {
  pub kind: CompactionSpaceKind,
  pub new_space_allocator: Option<()>,
  pub old_space_allocator: Option<()>,
  pub code_space_allocator: Option<()>,
  pub shared_space_allocator: Option<()>,
  pub trusted_space_allocator: Option<()>,
}

impl EvacuationAllocatorSkeleton {
  /// 对应 Allocate: 按空间选路 (new/old/code/shared/trusted 五选一)
  /// v8: src/heap/evacuation-allocator-inl.h EvacuationAllocator::Allocate
  pub fn allocate(&mut self, _space: crate::AllocationSpace, _size: usize) -> Result<(), ()> {
    unimplemented!("骨架: MainAllocator 分配路径待分配器族就位");
  }

  /// 对应 FreeLast: 撤销该空间最近一次分配 (LAB 回退或 free)
  /// v8: src/heap/evacuation-allocator.cc EvacuationAllocator::FreeLast
  pub fn free_last(&mut self, _object: u64, _size: usize) {
    unimplemented!("骨架: FreeLastInMainAllocator 待 LAB 族就位");
  }

  /// 对应 Finalize: 主线程收尾, 压实空间并回主空间
  /// v8: src/heap/evacuation-allocator.cc EvacuationAllocator::Finalize
  pub fn finalize(&mut self) {
    unimplemented!("骨架: MergeAndCompact/Birth 等 compaction 收尾待空间族就位");
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert_eq!(CompactionSpaceKind::NONE as u8, 0);
  assert_eq!(CompactionSpaceKind::COMPACTION_SPACE_FOR_SCAVENGE as u8, 1);
  assert_eq!(CompactionSpaceKind::COMPACTION_SPACE_FOR_MARK_COMPACT as u8, 2);
  assert_eq!(CompactionSpaceKind::COMPACTION_SPACE_FOR_MINOR_MARK_SWEEP as u8, 3);
};
`;

const p = writeSrc("v8-heap-core", "evacuation_allocator.rs.skel", rust);
console.log(`written ${p}`);
