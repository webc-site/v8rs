#!/usr/bin/env bun
// tool/src/heap/marking-barrier.js
// 机械提取 v8/src/heap/marking-barrier{.h,.cc,-inl.h} 族:
// MarkingBarrier 签名骨架 + 方法清单 (MarkingMode 见 mutable-page.h, 引用不重复定义)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/marking-barrier.h");
const ccSrc = readV8("src/heap/marking-barrier.cc");
const inlSrc = readV8("src/heap/marking-barrier-inl.h");

const anchors = [
  [hSrc, "class MarkingBarrier {"],
  [hSrc, "void Activate(bool is_compacting, MarkingMode marking_mode);"],
  [hSrc, "static void ActivateAll(Heap* heap, bool is_compacting);"],
  [hSrc, "static void ActivateYoung(Heap* heap);"],
  [hSrc, "bool is_minor() const { return marking_mode_ == MarkingMode::kMinorMarking; }"],
  [hSrc, "void Write(Tagged<HeapObject> host, IndirectPointerSlot slot);"],
  [hSrc, "void Write(Tagged<JSArrayBuffer> host, ArrayBufferExtension*);"],
  [hSrc, "void WriteWithoutHost(Tagged<HeapObject> value);"],
  [hSrc, "MarkingMode marking_mode_ = MarkingMode::kNoMarking;"],
  [inlSrc, "void MarkingBarrier::MarkValue("],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// cc 方法清单 (对数)
const ccMethods = [
  "MarkingBarrier::MarkingBarrier", "MarkingBarrier::~MarkingBarrier",
  "MarkingBarrier::Write(Tagged<HeapObject> host, IndirectPointerSlot slot)",
  "MarkingBarrier::WriteWithoutHost", "MarkingBarrier::Write(Tagged<InstructionStream> host",
  "MarkingBarrier::Write(Tagged<JSArrayBuffer> host", "MarkingBarrier::RecordRelocSlot",
  "MarkingBarrier::ActivateAll", "MarkingBarrier::ActivateYoung", "MarkingBarrier::Activate(bool",
  "MarkingBarrier::ActivateShared", "MarkingBarrier::DeactivateAll", "MarkingBarrier::DeactivateYoung",
  "MarkingBarrier::Deactivate()", "MarkingBarrier::DeactivateShared", "MarkingBarrier::PublishAll",
  "MarkingBarrier::PublishYoung", "MarkingBarrier::PublishIfNeeded", "MarkingBarrier::PublishSharedIfNeeded",
  "MarkingBarrier::IsCurrentMarkingBarrier",
];
for (const m of ccMethods) if (!ccSrc.includes(m)) throw new Error(`cc 缺 ${m}`);
console.log(`marking-barrier: cc 方法 ${ccMethods.length}, inl 内联 4 (MarkValue/MarkValueShared/MarkValueLocal/MarkRange 见锚)`);

let rust = banner(
  "src/heap/marking-barrier.js",
  "src/heap/marking-barrier.h, src/heap/marking-barrier.cc, src/heap/marking-barrier-inl.h",
  "增量标记写屏障: 激活/发布/写槽记录",
);

rust += `//! MarkingMode (kNoMarking/kMinorMarking/kMajorMarking) 定义于 mutable_page.rs, 此处引用。

/// v8: src/heap/marking-barrier.h MarkingBarrier
/// 字段: heap_/major_collector_/minor_collector_/incremental_marking_/
///       current_worklists_/shared_heap_worklists_/marking_state_/
///       typed_slots_map_/is_compacting_/is_activated_/
///       is_main_thread_barrier_/uses_shared_heap_/is_shared_space_isolate_/
///       marking_mode_
pub struct MarkingBarrier {
  /// v8: MarkingBarrier::heap_
  pub heap: *mut Heap,
  /// v8: MarkingBarrier::major_collector_
  pub major_collector: *mut MarkCompactCollector,
  /// v8: MarkingBarrier::minor_collector_
  pub minor_collector: *mut MinorMarkSweepCollector,
  /// v8: MarkingBarrier::incremental_marking_
  pub incremental_marking: *mut IncrementalMarking,
  /// v8: MarkingBarrier::is_compacting_ = false
  pub is_compacting: bool,
  /// v8: MarkingBarrier::is_activated_ = false
  pub is_activated: bool,
  /// v8: MarkingBarrier::is_main_thread_barrier_ (const)
  pub is_main_thread_barrier: bool,
  /// v8: MarkingBarrier::uses_shared_heap_ (const)
  pub uses_shared_heap: bool,
  /// v8: MarkingBarrier::is_shared_space_isolate_ (const)
  pub is_shared_space_isolate: bool,
  /// v8: MarkingBarrier::marking_mode_ = kNoMarking
  pub marking_mode: MarkingMode,
}

impl MarkingBarrier {
  /// v8: MarkingBarrier::MarkingBarrier(LocalHeap*)
  pub fn new(local_heap: &mut LocalHeap) -> Self { todo!() }
  /// v8: MarkingBarrier::Activate(bool is_compacting, MarkingMode)
  pub fn activate(&mut self, is_compacting: bool, marking_mode: MarkingMode) { todo!() }
  /// v8: MarkingBarrier::Deactivate
  pub fn deactivate(&mut self) { todo!() }
  /// v8: MarkingBarrier::PublishIfNeeded
  pub fn publish_if_needed(&mut self) { todo!() }
  /// v8: MarkingBarrier::ActivateShared / DeactivateShared / PublishSharedIfNeeded
  pub fn activate_shared(&mut self) { todo!() }
  pub fn deactivate_shared(&mut self) { todo!() }
  pub fn publish_shared_if_needed(&mut self) { todo!() }
  /// v8: MarkingBarrier::ActivateAll(Heap*, bool) (static)
  pub fn activate_all(heap: &mut Heap, is_compacting: bool) { todo!() }
  /// v8: MarkingBarrier::DeactivateAll(Heap*) (static)
  pub fn deactivate_all(heap: &mut Heap) { todo!() }
  /// v8: MarkingBarrier::PublishAll(Heap*) (static)
  pub fn publish_all(heap: &mut Heap) { todo!() }
  /// v8: MarkingBarrier::ActivateYoung(Heap*) / DeactivateYoung / PublishYoung (static)
  pub fn activate_young(heap: &mut Heap) { todo!() }
  pub fn deactivate_young(heap: &mut Heap) { todo!() }
  pub fn publish_young(heap: &mut Heap) { todo!() }
  /// v8: MarkingBarrier::Write (模板槽位/间接指针槽/RelocInfo/ArrayBufferExtension)
  /// v8: MarkingBarrier::WriteWithoutHost
  /// v8: MarkingBarrier::MarkValue / MarkValueShared / MarkValueLocal / MarkRange (-inl.h)
  /// v8: MarkingBarrier::RecordRelocSlot
  /// v8: MarkingBarrier::is_minor / is_not_major / is_major / is_compacting(-inl.h)
  /// v8: MarkingBarrier::IsCurrentMarkingBarrier
  /// v8: MarkingBarrier::AssertMarkingIsActivated (#ifdef DEBUG)
  /// v8: MarkingBarrier::IsMarked (#if V8_VERIFY_WRITE_BARRIERS)
  fn write(&mut self) { todo!() }
}
`;

const p = writeSrc("v8-heap-gc", "marking_barrier.rs.skel", rust);
console.log(`written ${p}`);
