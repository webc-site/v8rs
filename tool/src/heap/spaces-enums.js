#!/usr/bin/env bun
// tool/src/heap/spaces-enums.js
// 打包提取 heap 空间族小枚举:
// memory-allocator.h (AllocationMode/FreeMode), paged-spaces.h (DestinationHeap/State),
// read-only-spaces.h (SealMode), read-only-heap.h (SkipFreeSpaceOrFiller),
// new-spaces.h (SemiSpaceId), memory-chunk.h 已有产物覆盖 Flag (禁碰)。
// 产物落 v8-heap-core/src/spaces_enums.rs。

import { readV8, writeSrc, banner } from "../../lib.js";

const memAlloc = readV8("src/heap/memory-allocator.h");
const paged = readV8("src/heap/paged-spaces.h");
const roSpaces = readV8("src/heap/read-only-spaces.h");
const roHeap = readV8("src/heap/read-only-heap.h");
const newSpaces = readV8("src/heap/new-spaces.h");

function enumBody(src, name) {
  const m = src.match(new RegExp(`enum(?: class)? ${name}[^\\{]*\\{([^}]*)\\};`, "s"));
  if (!m) throw new Error(`未找到枚举 ${name}`);
  return m[1]
    .replace(/\/\/[^\n]*/g, "")
    .split(",")
    .map((s) => s.trim())
    .filter(Boolean);
}

const allocMode = enumBody(memAlloc, "AllocationMode");
if (allocMode.join() !== "kRegular,kTryDelayedAndPooled") throw new Error("AllocationMode 漂移");
const freeMode = enumBody(memAlloc, "FreeMode");
if (freeMode.length !== 4 || freeMode[0] !== "kImmediately" || freeMode[3] !== "kDelayThenPool") {
  throw new Error(`FreeMode 漂移: ${freeMode}`);
}

const dstHeap = enumBody(paged, "DestinationHeap");
if (dstHeap.join() !== "kSameHeap,kSharedSpaceHeap") throw new Error("DestinationHeap 漂移");
const pagedState = enumBody(paged, "State");
if (pagedState.length !== 7 || pagedState[0] !== "kOldSpace" || pagedState[6] !== "kFinished") {
  throw new Error(`PagedSpace State 漂移: ${pagedState}`);
}

const sealMode = enumBody(roSpaces, "SealMode");
if (sealMode.join() !== "kDetachFromHeap,kDetachFromHeapAndUnregisterMemory,kDoNotDetachFromHeap") {
  throw new Error("SealMode 漂移");
}

const skipFiller = enumBody(roHeap, "SkipFreeSpaceOrFiller");
if (skipFiller.join() !== "kYes,kNo") throw new Error("SkipFreeSpaceOrFiller 漂移");

const semiSpace = enumBody(newSpaces, "SemiSpaceId");
if (semiSpace.join() !== "kFromSpace = 0,kToSpace = 1") throw new Error("SemiSpaceId 漂移");

console.log("空间族枚举 7 个");

let rust = banner(
  "src/heap/spaces-enums.js",
  "src/heap/{memory-allocator,paged-spaces,read-only-spaces,read-only-heap,new-spaces}.h",
  "空间族枚举打包",
);

const snake = (s) => s.replace(/^k/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();

function emitEnum(rustName, doc, entries, repr = "u8") {
  let out = `
/// ${doc}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(${repr})]
pub enum ${rustName} {
`;
  entries.forEach((raw, i) => {
    const hasVal = raw.includes("=");
    const [n, v] = hasVal ? raw.split("=").map((x) => x.trim()) : [raw, String(i)];
    out += `  /// ${raw}\n  ${snake(n)} = ${v},\n`;
  });
  return out + `}\n`;
}

rust += emitEnum("MemoryAllocationMode", "页分配方式 (对应 MemoryAllocator::AllocationMode)", allocMode);
rust += emitEnum("MemoryFreeMode", "页释放方式 (对应 MemoryAllocator::FreeMode)", freeMode);
rust += emitEnum("CompactionDestinationHeap", "压实合并目标堆 (对应 CompactionSpace::DestinationHeap)", dstHeap);
rust += emitEnum("PagedSpaceIteratorState", "空间迭代器状态 (对应 SpaceIterator::State)", pagedState);
rust += emitEnum("ReadOnlySealMode", "只读空间封存方式 (对应 ReadOnlySpace::SealMode)", sealMode);
rust += emitEnum("SkipFreeSpaceOrFiller", "迭代是否跳过空闲区/填充对象 (对应 SkipFreeSpaceOrFiller)", skipFiller);
rust += emitEnum("SemiSpaceId", "半空间标识 (对应 enum SemiSpaceId)", semiSpace);

rust += `
// —— 提取不变式断言 ——
const _: () = {
  assert!(MemoryAllocationMode::TRY_DELAYED_AND_POOLED == 1);
  assert!(MemoryFreeMode::POOL == 1);
  assert!(CompactionDestinationHeap::SHARED_SPACE_HEAP == 1);
  assert!(PagedSpaceIteratorState::FINISHED == 6);
  assert!(ReadOnlySealMode::DO_NOT_DETACH_FROM_HEAP == 2);
  assert!(SkipFreeSpaceOrFiller::NO == 1);
  assert!(SemiSpaceId::TO_SPACE == 1);
};
`;
;

const p = writeSrc("v8-heap-core", "spaces_enums.rs", rust);
console.log(`written ${p}`);
