#!/usr/bin/env bun
// tool/src/heap/marking-state.js
// 机械提取 v8/src/heap/marking-state{.h,-inl.h} 族:
// MarkingStateBase / MarkingState(ATOMIC) / NonAtomicMarkingState 签名骨架

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/heap/marking-state.h");
const inlSrc = readV8("src/heap/marking-state-inl.h");

const anchors = [
  [hSrc, "class MarkingStateBase {"],
  [hSrc, "class MarkingState final"],
  [hSrc, "class NonAtomicMarkingState final"],
  [hSrc, "V8_INLINE bool TryMark(Tagged<HeapObject> obj);"],
  [hSrc, "V8_INLINE bool TryMarkAndAccountLiveBytes(Tagged<HeapObject> obj);"],
  [hSrc, "V8_INLINE bool IsMarked(const Tagged<HeapObject> obj) const;"],
  [inlSrc, "template <typename ConcreteState, AccessMode access_mode>"],
  [inlSrc, "bool MarkingStateBase<ConcreteState, access_mode>::TryMark("],
  [inlSrc, "return MarkBit::From(isolate_, obj).template Set<access_mode>();"],
  [inlSrc, "return MarkBit::From(isolate_, obj).template Get<access_mode>();"],
];
for (const [src, a] of anchors) if (!src.includes(a)) throw new Error(`锚点缺失: ${a}`);

// inl 方法对数
const inlFns = ["TryMark", "TryMarkAndAccountLiveBytes", "IsMarked", "IsUnmarked"];
for (const f of inlFns) if (!inlSrc.includes(`::${f}(`)) throw new Error(`inl 缺 ${f}`);
console.log(`marking-state: inl 方法 ${inlFns.length} (TryMark/TryMarkAndAccountLiveBytes/IsMarked/IsUnmarked), 类 3`);

let rust = banner(
  "src/heap/marking-state.js",
  "src/heap/marking-state.h, src/heap/marking-state-inl.h",
  "标记状态门面: 原子/非原子两实例化",
);

rust += `/// 访问模式参数: 原文模板参数 AccessMode::ATOMIC / NON_ATOMIC
/// v8: src/heap/marking-state.h MarkingStateBase
pub struct MarkingStateBase {
  /// v8: isolate_
  pub isolate: *const Isolate,
}

/// v8: src/heap/marking-state.h MarkingState (AccessMode::ATOMIC, 标记 visitor 用)
pub type MarkingState = MarkingStateBase;
/// v8: src/heap/marking-state.h NonAtomicMarkingState (AccessMode::NON_ATOMIC)
pub type NonAtomicMarkingState = MarkingStateBase;

impl MarkingStateBase {
  /// v8: src/heap/marking-state-inl.h MarkingStateBase<...,ATOMIC>::TryMark
  /// 语义: bitmap->MarkBitFromAddress(...).template Set<mode>() — 0→1 返回 true
  pub fn try_mark(&self, obj: TaggedHeapObject) -> bool { todo!() }
  /// v8: src/heap/marking-state-inl.h TryMarkAndAccountLiveBytes(obj)
  /// TryMark 成功 → MutablePage::IncrementLiveBytesAtomically
  pub fn try_mark_and_account_live_bytes(&self, obj: TaggedHeapObject) -> bool { todo!() }
  /// v8: src/heap/marking-state-inl.h TryMarkAndAccountLiveBytes(obj, object_size)
  pub fn try_mark_and_account_live_bytes_with_size(&self, obj: TaggedHeapObject, object_size: u32) -> bool { todo!() }
  /// v8: src/heap/marking-state-inl.h IsMarked
  pub fn is_marked(&self, obj: TaggedHeapObject) -> bool { todo!() }
  /// v8: src/heap/marking-state-inl.h IsUnmarked
  pub fn is_unmarked(&self, obj: TaggedHeapObject) -> bool { todo!() }
}
`;

const p = writeSrc("v8-heap-gc", "marking_state.rs.skel", rust);
console.log(`written ${p}`);
