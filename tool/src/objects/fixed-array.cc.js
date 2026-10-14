#!/usr/bin/env bun
// tool/src/objects/fixed-array.cc.js — 覆盖 v8/src/objects/fixed-array.cc 的
//   纯数组语义子集 → v8-objects-arrays/src/fixed_array.rs 的
//   @cc-semantics 区段 (区段标记替换, 幂等; 不触碰 fixed-array.h.js
//   生成的其余内容)
//
// 转写区段登记 (fixed-array.cc 实测行号):
//   271-288  WeakArrayList::Compact (清除弱引用紧缩, 原地前移)
//   290      WeakArrayList::IsFull
//   306-315  WeakArrayList::CountLiveWeakReferences
//   317-326  WeakArrayList::CountLiveElements
//   328-344  WeakArrayList::RemoveOne (尾元素补位 + 清除哨兵收尾)
//   346-352  WeakArrayList::Contains (线性查找)
//
// 语义映射:
//   Get/Set → TaggedArrayExt::get / set_relaxed (Element = MaybeObject)
//   IsCleared/IsWeak → Tagged<MaybeObject>::is_cleared / is_weak_live (core)
//   ClearedValue → Tagged<MaybeObject>::cleared (core)
//   迭代 for i in 0..len → (0..len).fold / for — 保持单遍历
//
// 跳过区段登记 (推迟, isolate/factory/heap 依赖):
//   16-91    FixedArray::Grow/SetAndGrow/RightTrimOrEmpty (+ RightTrim 的
//            Super 语义已由 TaggedArrayExt::right_trim 不变式校验承载)
//   94-184   ArrayList::Add (×3) / ToFixedArray / RightTrim / EnsureSpace
//   187-304  WeakArrayList::AddToEnd (×2) / Append / EnsureSpace /
//            CompactWeakArrayList (factory)
import { banner, readV8 } from "../../lib.js";
import { readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

const root = join(import.meta.dir, "../../..");
const cc = readV8("src/objects/fixed-array.cc");
const target = join(root, "v8-objects-arrays/src/fixed_array.rs");

// ── 锚点 ────────────────────────────────────────────────────────────────
for (const [sub] of [
  ["void WeakArrayList::Compact(Isolate* isolate) {"],
  ["bool WeakArrayList::IsFull() const { return length() == capacity(); }"],
  ["uint32_t WeakArrayList::CountLiveWeakReferences() const {"],
  ["uint32_t WeakArrayList::CountLiveElements() const {"],
  ["bool WeakArrayList::RemoveOne(MaybeObjectDirectHandle value) {"],
  ["bool WeakArrayList::Contains(Tagged<MaybeObject> value) {"],
]) {
  if (!cc.includes(sub)) throw new Error(`fixed-array.cc 锚点失配: "${sub.slice(0, 48)}..."`);
}

const src = readFileSync(target, "utf8");
if (!src.includes("// @cc-semantics-begin")) throw new Error("fixed_array.rs 缺 @cc-semantics 区段");

const section = `${banner(
  "src/objects/fixed-array.cc.js",
  "src/objects/fixed-array.cc (纯数组语义子集)",
  "WeakArrayList Compact/IsFull/CountLive*/RemoveOne/Contains; isolate/factory 依赖推迟 (脚本头登记)",
)}
// ── WeakArrayList 纯数组方法 (fixed-array.cc:271-352) ─────────────────────
/// WeakArrayList 的 fixed-array.cc 语义 (trait 承载, 与 -inl.h 语义的
/// WeakArrayListExt 分立, 保留来源可追溯)。
pub trait WeakArrayListCc: Copy + TaggedArrayExt<Element = MaybeObject> {
  /// Compact (cc:271): 清除项剔除, 存活项原地前移; length 收敛为存活数。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get/set; 迭代期间无并发修改。
  #[inline]
  unsafe fn compact(&self) {
    // SAFETY: 调用方契约
    unsafe {
      let len = self.length();
      let mut new_length = 0u32;
      for i in 0..len {
        let value = self.get(i);
        if !value.is_cleared() {
          if new_length != i {
            // Set(new_length, value): 存活项前移
            self.set_relaxed(new_length, value, WriteBarrierMode::UpdateWriteBarrier);
          }
          new_length += 1;
        }
      }
      self.set_length(new_length);
    }
  }

  /// IsFull (cc:290)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::capacity。
  #[inline]
  unsafe fn is_full(&self) -> bool {
    // SAFETY: 调用方契约
    unsafe { self.length() == self.capacity() }
  }

  /// CountLiveWeakReferences (cc:306): 活弱引用计数。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn count_live_weak_references(&self) -> u32 {
    // SAFETY: 调用方契约; 单遍历计数 (闭包内 unsafe: 槽位读)
    unsafe {
      (0..self.length())
        .filter(|&i| self.get(i).is_weak_live())
        .count() as u32
    }
  }

  /// CountLiveElements (cc:317): 未清除元素计数。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn count_live_elements(&self) -> u32 {
    // SAFETY: 调用方契约; 单遍历计数
    unsafe { (0..self.length()).filter(|&i| !self.get(i).is_cleared()).count() as u32 }
  }

  /// RemoveOne (cc:328): 倒序找到首个等值项 → 尾元素补位 + 尾槽置清除
  /// 哨兵 + length-1; 未找到 → false (元素序会变, 调用方须知)。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get/set。
  #[inline]
  unsafe fn remove_one(&self, value: Tagged<MaybeObject>) -> bool {
    // SAFETY: 调用方契约
    unsafe {
      let len = self.length();
      if len == 0 {
        return false;
      }
      let last_index = len - 1;
      let mode = WriteBarrierMode::UpdateWriteBarrier;
      for i in (0..len).rev() {
        if self.get(i) != value {
          continue;
        }
        let last = self.get(last_index);
        self.set_relaxed(i, last, mode);
        self.set_relaxed(last_index, Tagged::cleared(), mode);
        self.set_length(last_index);
        return true;
      }
      false
    }
  }

  /// Contains (cc:346): 线性查找等值项。
  ///
  /// # Safety
  /// 同 TaggedArrayExt::get。
  #[inline]
  unsafe fn contains(&self, value: Tagged<MaybeObject>) -> bool {
    // SAFETY: 调用方契约; 提前终止扫描
    unsafe { (0..self.length()).any(|i| self.get(i) == value) }
  }
}

impl WeakArrayListCc for Tagged<WeakArrayList> {}
`;

const out = src.replace(
  /(\/\/ @cc-semantics-begin\n)[\s\S]*(\/\/ @cc-semantics-end)/,
  `$1${section}$2`,
);
if (out === src) throw new Error("@cc-semantics 区段替换失败");
writeFileSync(target, out);
console.log("fixed_array.rs @cc-semantics: WeakArrayList 6 方法 (Compact/IsFull/Count×2/RemoveOne/Contains) 注入");
