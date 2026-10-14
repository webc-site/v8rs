#!/usr/bin/env bun
// tool/src/heap/scavenger.js
// 机械提取 v8/src/heap/scavenger{.h,.cc} 族:
// ScavengerCollector/Scavenger 类骨架 + cc 局部常量与结构

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/scavenger.h");
const ccSrc = readV8("src/heap/scavenger.cc");

const anchors = [
  [hSrc, "class ScavengerCollector {"],
  [hSrc, "void CollectGarbage();"],
  [hSrc, "void CompleteSweepingQuarantinedPagesIfNeeded();"],
  [hSrc, "class QuarantinedPageSweeper;"],
  [ccSrc, "enum class ObjectAge { kOld, kYoung };"],
  [ccSrc, "class Scavenger {"],
  [ccSrc, "static constexpr int kScavengedObjectListSegmentSize = 256;"],
  [ccSrc, "struct ScavengedObjectListEntry {"],
  [ccSrc, "struct PinnedObjectEntry {"],
  [ccSrc, "int NumberOfScavengeTasks(Heap* heap) {"],
  [ccSrc, "static constexpr int kMaxScavengerTasks = 8;"],
  [ccSrc, "kProcessPromotedListThreshold ="],
  [ccSrc, "ScavengedObjectList::kMinSegmentSize / 2;"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// 常量真提取
function constInt(src, re) {
  const m = src.match(re);
  if (!m) throw new Error(`常量未找到: ${re}`);
  return parseInt(m[1], 10);
}
const weakSeg = constInt(ccSrc, /kWeakObjectListSegmentSize = (\d+);/);
const copiedSeg = constInt(ccSrc, /kScavengedObjectListSegmentSize = (\d+);/);
const maxTasks = constInt(ccSrc, /kMaxScavengerTasks = (\d+);/);
const emptySeg = constInt(ccSrc, /::heap::base::Worklist<MutablePage\*, (\d+)>;/);
if (weakSeg !== 64 || copiedSeg !== 256 || maxTasks !== 8 || emptySeg !== 64) {
  throw new Error(`常量漂移: weak=${weakSeg} copied=${copiedSeg} tasks=${maxTasks} empty=${emptySeg}`);
}

// visitor 类清单
const visitors = ["ScavengerObjectVisitorBase", "ScavengerCopiedObjectVisitor", "ScavengerPromotedObjectVisitor", "RootScavengeVisitor", "ScavengerJobTask"];
for (const v of visitors) if (!ccSrc.includes(`class ${v}`) && !ccSrc.includes(`${v} :`)) throw new Error(`cc 缺类 ${v}`);
console.log(`scavenger: 常量 4 (${weakSeg}/${copiedSeg}/${maxTasks}/${emptySeg}), visitor 类 ${visitors.length}, 结构 2`);

const camel = (s) => s.replace(/^k/, "");
function emitEnum(rustName, doc, entries) {
  let out = `/// ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum ${rustName} {\n`;
  entries.forEach((k, i) => (out += `  /// ${k}\n  ${camel(k)} = ${i},\n`));
  return out + "}\n\n";
}

let rust = banner(
  "src/heap/scavenger.js",
  "src/heap/scavenger.h, src/heap/scavenger.cc",
  "半空间拷贝 GC: ScavengerCollector 入口与并行任务",
);

rust += emitEnum("ObjectAge", "对象年龄 (cc 局部 enum class ObjectAge)", ["kOld", "kYoung"]);

rust += `/// v8: src/heap/scavenger.cc ScavengedObjectListEntry
pub struct ScavengedObjectListEntry {
  pub heap_object: TaggedHeapObject,
  pub map: TaggedMap,
  /// v8: SafeHeapObjectSize
  pub size: u32,
}

/// v8: src/heap/scavenger.cc PinnedObjectEntry
pub struct PinnedObjectEntry {
  pub address: Address,
  /// v8: MapWord
  pub map_word: MapWord,
  pub size: u32,
  /// v8: metadata (MutablePage*)
  pub metadata: *mut MutablePage,
}

/// v8: src/heap/scavenger.cc Scavenger 常量
pub mod scavenger_consts {
  /// v8: ScavengerWeakObjectsProcessor::kWeakObjectListSegmentSize
  pub const K_WEAK_OBJECT_LIST_SEGMENT_SIZE: usize = ${weakSeg};
  /// v8: Scavenger::kScavengedObjectListSegmentSize
  pub const K_SCAVENGED_OBJECT_LIST_SEGMENT_SIZE: usize = ${copiedSeg};
  /// v8: EmptyChunksList 段大小 (Worklist<MutablePage*, 64>)
  pub const K_EMPTY_CHUNKS_LIST_SEGMENT_SIZE: usize = ${emptySeg};
  /// v8: NumberOfScavengeTasks 内 kMaxScavengerTasks (含主线程)
  pub const K_MAX_SCAVENGER_TASKS: i32 = ${maxTasks};
  /// v8: Scavenger::ShouldEagerlyProcessPromotedList — kMinSegmentSize / 2
  pub const K_PROCESS_PROMOTED_LIST_THRESHOLD: i32 =
    (K_SCAVENGED_OBJECT_LIST_SEGMENT_SIZE / 2) as i32;
}

/// v8: src/heap/scavenger.h ScavengerCollector
pub struct ScavengerCollector {
  /// v8: heap_ (const)
  pub heap: *mut Heap,
  /// v8: quarantined_page_sweeper_
  pub quarantined_page_sweeper: Option<Box<QuarantinedPageSweeper>>,
}

impl ScavengerCollector {
  /// v8: ScavengerCollector::CollectGarbage — 半空间拷贝并行 GC 主入口
  pub fn collect_garbage(&mut self) { todo!() }
  /// v8: ScavengerCollector::CompleteSweepingQuarantinedPagesIfNeeded
  pub fn complete_sweeping_quarantined_pages_if_needed(&mut self) { todo!() }
}

/// v8: src/heap/scavenger.cc Scavenger — 单任务拷贝器
/// (copied_list_/promoted_list_ 为 Worklist Local, 字段面宽, 见原文 280-460 行)
/// v8: ScavengerObjectVisitorBase / ScavengerCopiedObjectVisitor / ScavengerPromotedObjectVisitor
/// v8: RootScavengeVisitor / ScavengerJobTask / ScavengerWeakObjectsProcessor
/// v8: NumberOfScavengeTasks — capacity/MB+1 与核数取 min, 再夹 [1, 8]

// —— 提取不变式断言 ——
const _: () = {
  assert!(ObjectAge::Young as u8 == 1);
  assert!(scavenger_consts::K_MAX_SCAVENGER_TASKS == 8);
  assert!(scavenger_consts::K_PROCESS_PROMOTED_LIST_THRESHOLD == 128);
};
`;

const p = writeSrc("v8-heap-gc", "scavenger.rs.skel", rust);
console.log(`written ${p}`);
