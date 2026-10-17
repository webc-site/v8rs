#!/usr/bin/env bun
// tool/src/heap/remembered-set.js
// 机械提取 v8/src/heap/remembered-set{.h,-inl.h} 族:
// RememberedSetOperations / RememberedSet<type> 静态操作面
// (RememberedSetType 枚举在 mutable-page.h, 引用不重复定义)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/remembered-set.h");
const inlSrc = readV8("src/heap/remembered-set-inl.h");
const mpSrc = readV8("src/heap/mutable-page.h"); // RememberedSetType 上下文 (他席)

const anchors = [
  [hSrc, "class RememberedSetOperations {"],
  [hSrc, "static void Insert(SlotSet* slot_set, size_t slot_offset) {"],
  [hSrc, "static int Iterate(SlotSet* slot_set, const MutablePage* chunk,"],
  [hSrc, "static void Remove(SlotSet* slot_set, MutablePage* chunk, Address slot_addr) {"],
  [hSrc, "static void RemoveRange(SlotSet* slot_set, MutablePage* page, Address start,"],
  [hSrc, "static void CheckNoneInRange(SlotSet* slot_set, MemoryChunk* chunk,"],
  [hSrc, "class RememberedSet : public AllStatic {"],
  [hSrc, "static_assert(type == RememberedSetType::OLD_TO_NEW ||"],
  [hSrc, "static void MergeAndDelete(MutablePage* chunk, SlotSet&& other_slot_set) {"],
  [hSrc, "static void MergeAndDeleteTyped(MutablePage* chunk,"],
  [hSrc, "class UpdateTypedSlotHelper {"],
  [inlSrc, "SlotCallbackResult UpdateTypedSlotHelper::UpdateTypedSlot("],
  [inlSrc, "Tagged<HeapObject> UpdateTypedSlotHelper::GetTargetObject("],
  [mpSrc, "enum RememberedSetType {"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// RememberedSetType 真提取 (对数锚)
const rs = mpSrc.match(/enum RememberedSetType \{([^}]*)\};/);
if (!rs) throw new Error("RememberedSetType 未找到");
const rsTypes = rs[1].split(",").map((s) => s.trim()).filter(Boolean);
if (rsTypes[rsTypes.length - 1] !== "NUMBER_OF_REMEMBERED_SET_TYPES") throw new Error("RS 类型末项漂移");
console.log(`remembered-set: RememberedSetType ${rsTypes.length - 1} 实类型 (+1 计数哨兵), 操作面 2 类`);

let rust = banner(
  "src/heap/remembered-set.js",
  "src/heap/remembered-set.h, src/heap/remembered-set-inl.h",
  "记忆集: 老生代→新生代等槽位记录",
);

rust += `/// RememberedSetType 定义于 mutable_page.rs (他席):
/// OLD_TO_NEW / OLD_TO_NEW_BACKGROUND / OLD_TO_OLD / OLD_TO_SHARED /
/// TRUSTED_TO_CODE / TRUSTED_TO_TRUSTED / TRUSTED_TO_SHARED_TRUSTED /
/// SURVIVOR_TO_EXTERNAL_POINTER / NUMBER_OF_REMEMBERED_SET_TYPES
/// 共 ${rsTypes.length - 1} 个实类型。

/// v8: src/heap/remembered-set.h RememberedSetOperations — SlotSet 裸操作
pub mod remembered_set_operations {
  /// v8: Insert<access_mode>(slot_set, slot_offset)
  /// v8: Iterate<access_mode, Callback>(slot_set, chunk, callback, mode)
  ///     → slot_set->Iterate(chunk->ChunkAddress(), 0, BucketsInSlotSet, ...)
  /// v8: Remove(slot_set, chunk, slot_addr) — chunk->Offset 后 Remove
  /// v8: RemoveRange(slot_set, page, start, end, mode) — OffsetMaybeOutOfRange
  /// v8: CheckNoneInRange(slot_set, chunk, start, end)
  ///     末桶计算: BucketForSlot(OffsetMaybeOutOfRange(end) - kTaggedSize) + 1
}

/// v8: src/heap/remembered-set.h RememberedSet<type> (AllStatic)
/// 静态面 (page 级): Insert/MergeAndDelete(仅 OLD_TO_NEW,
/// static_assert)/MergeAndDeleteTyped/DeleteTyped/Contains/CheckNoneInRange/
/// Remove/RemoveRange/Iterate/IterateTyped/EmptyBucketsAfterIterate 等
/// v8: src/heap/remembered-set-inl.h RememberedSet<type> 实现面
/// v8: UpdateTypedSlotHelper — typed slot 按种类 (code entry/target/embedded
///     pointer/cage base) 用 untyped 回调更新
/// v8: src/heap/remembered-set.h UpdateTypedSlotHelper::{UpdateCodeEntry,
///     UpdateCodeTarget, UpdateEmbeddedPointer, UpdateCageBase}
`;

const p = writeSrc("v8-heap-core", "remembered_set.rs.skel", rust);
console.log(`written ${p}`);
