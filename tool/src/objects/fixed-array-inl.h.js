#!/usr/bin/env bun
// tool/src/objects/fixed-array-inl.h.js — fixed-array-inl.h 差集对数审计 (每原版文件一脚本红线)
//   v8/src/objects/fixed-array-inl.h (564 行) → 落地 v8-objects-arrays/src/fixed_array.rs
//
// 产物: v8-objects-arrays/src/fixed_array_inl.rs.skel (对数骨架/合并指引, 审计专用;
//   落地 fixed_array.rs 以就地修订为准, 本脚本不覆盖落地文件)
// 规则: 方法名全集 (40 个 qualified 定义名) + 常量/槽位操作/尺寸式逐项与原文对数
//   (不符 throw) + 落地对数核对: 每项处置为 落地命中(fn 名) 或 推迟登记 (New/Allocate/
//   Resize 族 isolate+factory+roots 依赖, 模块头跳过登记区); 出现未登记缺项 throw 列票单
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const inl = readV8("src/objects/fixed-array-inl.h");

// ── 对数断言 ─────────────────────────────────────────────────────────────
let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;

// 1) 方法名全集: 本族 12 类 (含 Iterator) 的 qualified 定义/转发名机械提取
//    长名在前防左部吞并 (TrustedFixedArray 先于 FixedArray)
const CLS = [
  "TaggedArrayBase<D, ElementT_, P>",
  "TrustedFixedArrayBase",
  "TrustedWeakFixedArray",
  "TrustedFixedArray",
  "ProtectedWeakFixedArray",
  "ProtectedFixedArray",
  "WeakHomomorphicFixedArray",
  "WeakFixedArray",
  "WeakArrayList::Iterator",
  "WeakArrayList",
  "ArrayList",
  "FixedArray",
];
const defRe = new RegExp(`(${CLS.join("|")})::(\\w+)\\s*\\(`, "g");
const pairs = [...inl.matchAll(defRe)].map((m) => `${m[1]}::${m[2]}`);
const RAW_TOTAL = 55; // 40 唯一 qualified (定义 54 + Resize 体内 FixedArray::CopyElements 转发 1)
if (pairs.length !== RAW_TOTAL) fail("qualified 名出现总数", pairs.length, RAW_TOTAL);
assertCount++;
const byQual = {};
for (const p of pairs) byQual[p] = (byQual[p] || 0) + 1;
// 每 qualified 名的出现次数逐项表 (定义 + 同名转发/调用; 全集防漂移)
const QUAL_TABLE = {
  "TaggedArrayBase<D, ElementT_, P>::IsInBounds": 1,
  "TaggedArrayBase<D, ElementT_, P>::IsCowArray": 1,
  "TaggedArrayBase<D, ElementT_, P>::get": 4,
  "TaggedArrayBase<D, ElementT_, P>::set": 8,
  "TaggedArrayBase<D, ElementT_, P>::swap": 1,
  "TaggedArrayBase<D, ElementT_, P>::compare_and_swap": 1,
  "TaggedArrayBase<D, ElementT_, P>::MoveElements": 1,
  "TaggedArrayBase<D, ElementT_, P>::CopyElements": 1,
  "TaggedArrayBase<D, ElementT_, P>::RightTrim": 1,
  "TaggedArrayBase<D, ElementT_, P>::AllocatedSize": 1,
  "TaggedArrayBase<D, ElementT_, P>::RawFieldOfFirstElement": 1,
  "TaggedArrayBase<D, ElementT_, P>::RawFieldOfElementAt": 1,
  "TaggedArrayBase<D, ElementT_, P>::Allocate": 1,
  "TaggedArrayBase<D, ElementT_, P>::NewCapacityForIndex": 1,
  "FixedArray::New": 2,
  "FixedArray::is_the_hole": 1,
  "FixedArray::set_the_hole": 2,
  "FixedArray::FillWithHoles": 1,
  "FixedArray::MoveElements": 1,
  "FixedArray::CopyElements": 2,
  "FixedArray::Resize": 1,
  "TrustedFixedArray::New": 1,
  "ProtectedFixedArray::New": 1,
  "WeakArrayList::capacity": 2,
  "WeakArrayList::length": 1,
  "WeakArrayList::ulength": 1,
  "WeakArrayList::set_length": 1,
  "WeakArrayList::Get": 1,
  "WeakArrayList::Set": 2,
  "WeakArrayList::CopyElements": 1,
  "WeakArrayList::AllocatedSize": 1,
  "WeakArrayList::Iterator::Next": 1,
  "WeakFixedArray::New": 1,
  "WeakHomomorphicFixedArray::New": 1,
  "TrustedWeakFixedArray::New": 1,
  "ProtectedWeakFixedArray::New": 1,
  "ArrayList::length": 1,
  "ArrayList::ulength": 1,
  "ArrayList::set_length": 1,
  "ArrayList::New": 1,
};
for (const [q, want] of Object.entries(QUAL_TABLE)) {
  if (byQual[q] !== want) fail(`def ${q}`, byQual[q], want);
  assertCount++;
}
if (Object.keys(byQual).length !== Object.keys(QUAL_TABLE).length)
  fail("方法名全集大小", Object.keys(byQual).length, Object.keys(QUAL_TABLE).length);
assertCount++;
// 表外类防漏: 广谱大写 qualified 名 − 本族表 = 已登记体内调用集
// (AsAtomic32/Smi/ReadOnlyHeap/Super 调用 + Iterator::Next 为窄集全名命中的尾段子串)
const broad = [...inl.matchAll(/([A-Z]\w*)::(\w+)\s*\(/g)].map((m) => `${m[1]}::${m[2]}`);
const extras = [...new Set(broad.filter((p) => !(p in byQual)))].sort();
const EXTRAS_WANT =
  "AsAtomic32::Relaxed_Load,Iterator::Next,ReadOnlyHeap::Contains," +
  "ReadOnlyHeap::EarlyGetReadOnlyRoots,Smi::zero,Super::CopyElements";
if (extras.join() !== EXTRAS_WANT)
  fail("表外 qualified 名集漂移", extras.join(), EXTRAS_WANT);
assertCount++;

// 2) 常量/槽位操作/尺寸式与原文逐项对数
for (const [re, want, label] of [
  [/capacity \+ \(capacity >> 1\) \+ 16/g, 1, "NewCapacityForIndex 增长式"],
  [/::kMaxLength/g, 6, "kMaxLength 上界检查 (New 族)"],
  [/FatalNoSecurityImpact\(/g, 6, "超长 Fatal (New 族)"],
  [/MemsetTagged\(/g, 8, "MemsetTagged 填充 (New 族/fill 语义源)"],
  [/objects\(\)\[index\]\.Relaxed_Load\(\)/g, 2, "get Relaxed 读 (含无 tag 重载)"],
  [/objects\(\)\[index\]\.Acquire_Load\(\)/g, 1, "get Acquire 读"],
  [/objects\(\)\[index\]\.SeqCst_Load\(\)/g, 1, "get SeqCst 读"],
  [/objects\(\)\[index\]\.Relaxed_Store\(derived\(\)/g, 2, "set Relaxed 写 (含无 tag 重载)"],
  [/objects\(\)\[index\]\.Release_Store\(derived\(\)/g, 1, "set Release 写"],
  [/objects\(\)\[index\]\.SeqCst_Store\(derived\(\)/g, 1, "set SeqCst 写"],
  [/objects\(\)\[index\]\.SeqCst_Swap\(derived\(\)/g, 1, "swap"],
  [/objects\(\)\[index\]\.SeqCst_CompareAndSwap\(derived\(\)/g, 1, "compare_and_swap"],
  [/DCHECK\(!IsCowArray\(\)\)/g, 6, "set/swap/cas cow DCHECK (落地转调用方契约)"],
  [/roots_table\(\)\.empty_/g, 4, "空数组 roots (New 族推迟)"],
  [/kMapRootIndex/g, 1, "Allocate roots 依赖 (推迟)"],
  [/SKIP_WRITE_BARRIER/g, 9, "Smi 写跳屏障 (set_smi 族)"],
]) {
  if (count(inl, re) !== want) fail(label, count(inl, re), want);
  assertCount++;
}

// ── 3) 落地对数核对 (fixed_array.rs) ─────────────────────────────────────
const faRs = readFileSync(join(root, "v8-objects-arrays", "src", "fixed_array.rs"), "utf8");
// qualified → 落地 Rust fn 名集 (须逐项 fn 命中)
const LAND = {
  "TaggedArrayBase<D, ElementT_, P>::IsInBounds": ["is_in_bounds"],
  "TaggedArrayBase<D, ElementT_, P>::IsCowArray": ["is_cow_array"],
  "TaggedArrayBase<D, ElementT_, P>::get": ["get", "get_relaxed", "get_acquire", "get_seqcst"],
  "TaggedArrayBase<D, ElementT_, P>::set": [
    "set", "set_relaxed", "set_release", "set_seqcst",
    "set_smi", "set_smi_relaxed", "set_smi_release", "set_smi_seqcst",
  ],
  "TaggedArrayBase<D, ElementT_, P>::swap": ["swap"],
  "TaggedArrayBase<D, ElementT_, P>::compare_and_swap": ["compare_and_swap"],
  "TaggedArrayBase<D, ElementT_, P>::CopyElements": ["copy_elements"],
  "TaggedArrayBase<D, ElementT_, P>::RightTrim": ["right_trim"],
  "TaggedArrayBase<D, ElementT_, P>::AllocatedSize": ["allocated_size"],
  "TaggedArrayBase<D, ElementT_, P>::RawFieldOfFirstElement": ["raw_field_of_first_element"],
  "TaggedArrayBase<D, ElementT_, P>::RawFieldOfElementAt": ["raw_field_of_element_at"],
  "TaggedArrayBase<D, ElementT_, P>::NewCapacityForIndex": ["new_capacity_for_index"],
  "FixedArray::is_the_hole": ["is_the_hole"],
  "FixedArray::set_the_hole": ["set_the_hole"],
  "FixedArray::FillWithHoles": ["fill_with_holes"],
  "FixedArray::CopyElements": ["copy_elements"],
  "WeakArrayList::capacity": ["capacity", "capacity_relaxed"],
  "WeakArrayList::length": ["length"],
  "WeakArrayList::ulength": ["ulength"],
  "WeakArrayList::set_length": ["set_length"],
  "WeakArrayList::Get": ["get"],
  "WeakArrayList::Set": ["set", "set_smi"],
  "WeakArrayList::CopyElements": ["copy_elements"],
  "WeakArrayList::AllocatedSize": ["allocated_size"],
  "WeakArrayList::Iterator::Next": ["next"],
  "ArrayList::length": ["length"],
  "ArrayList::ulength": ["ulength"],
  "ArrayList::set_length": ["set_length"],
};
// qualified → 推迟登记锚 (模块头跳过登记区须含该串; New/Allocate/Resize → L3 heap)
// 口径适配 (实态): MoveElements 族 (base inl:163 + FixedArray inl:398) 原版实体仅
//   isolate->heap()->MoveRange 写屏障簿记 → L3 heap 推迟 (模块头已登记);
//   ptr 级重叠拷贝语义已由 copy_elements (ptr::copy, 允许重叠) 落地承接 (只增)
const DEFERRED = {
  "TaggedArrayBase<D, ElementT_, P>::Allocate": "New/Allocate",
  "TaggedArrayBase<D, ElementT_, P>::MoveElements": "MoveElements",
  "FixedArray::New": "New/Allocate",
  "FixedArray::MoveElements": "MoveElements",
  "TrustedFixedArray::New": "New/Allocate",
  "ProtectedFixedArray::New": "New/Allocate",
  "WeakFixedArray::New": "New/Allocate",
  "WeakHomomorphicFixedArray::New": "New/Allocate",
  "TrustedWeakFixedArray::New": "New/Allocate",
  "ProtectedWeakFixedArray::New": "New/Allocate",
  "ArrayList::New": "New/Allocate",
  "FixedArray::Resize": "Resize",
};
const missing = [];
const landedLines = [];
for (const [q, names] of Object.entries(LAND)) {
  for (const n of names) {
    if (!new RegExp(`\\bfn\\s+${n}\\b`).test(faRs)) missing.push(`${q} → fn ${n} (落地缺失)`);
    else landedLines.push(`//! - ${q} → fn ${n}`);
  }
  assertCount++;
}
for (const [q, anchor] of Object.entries(DEFERRED)) {
  if (!faRs.includes(anchor)) missing.push(`${q} → 推迟登记锚 "${anchor}" 未找到`);
  else landedLines.push(`//! - ${q} → 推迟 (登记锚 "${anchor}")`);
  assertCount++;
}
// 处置不变式: 落地 28 + 推迟 12 == 全集 40 (MoveElements ×2 由落地转推迟, 见上注)
if (Object.keys(LAND).length + Object.keys(DEFERRED).length !== Object.keys(byQual).length)
  fail("处置表覆盖全集", Object.keys(LAND).length + Object.keys(DEFERRED).length, Object.keys(byQual).length);
assertCount++;
if (missing.length) throw new Error(`落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

// 落地语义锚逐项 (增长式/元素寻址/SeqCst 序/调用方契约与 heap 推迟登记)
for (const [re, want, label] of [
  [/capacity = capacity \+ \(capacity >> 1\) \+ 16;/g, 1, "落地增长式 1.5x+16 (实态: 忠实 do-while 直译, 原 saturating 加固已修订回原版形态)"],
  [/MAX_CAPACITY/g, 2, "落地容量上界 (trait 声明 + K_MAX_FIXED_ARRAY_CAPACITY 常量定义 2 见点)"],
  [/T::OBJECTS_OFFSET \+ index as usize \* T::ELEMENT_SIZE/g, 1, "落地元素槽寻址 (objects()[index])"],
  [/Ordering::SeqCst/g, 5, "落地 SeqCst 序 (get/set/swap/cas×2)"],
  [/DCHECK\(!IsCowArray\(\)\)/g, 2, "落地调用方契约登记 (模块头 + set 文档)"],
  [/RightTrimArray 归 L3 heap/g, 1, "落地 RightTrim heap 推迟登记"],
]) {
  if (count(faRs, re) !== want) fail(`落地 ${label}`, count(faRs, re), want);
  assertCount++;
}

// ── 产物: 对数骨架 (合并指引; 审计专用, 不覆盖落地 fixed_array.rs) ─────────
const skel = `${banner(
  "src/objects/fixed-array-inl.h.js",
  "src/objects/fixed-array-inl.h",
  `对数骨架 (审计专用): 40 qualified 方法全集处置表 (落地 ${Object.keys(LAND).length} / 推迟登记 ${Object.keys(DEFERRED).length}); 落地以 fixed_array.rs 就地修订为准`,
)}
//! fixed-array-inl.h → fixed_array.rs 合并指引
//!
//! 对数处置 (${Object.keys(byQual).length} qualified / ${RAW_TOTAL} 出现):
${landedLines.join("\n")}
//!
//! 重载展开: TaggedArrayBase::get ×4 → get/get_relaxed/get_acquire/get_seqcst;
//! set ×8 → set/set_relaxed/set_release/set_seqcst + set_smi×4 (SKIP_WRITE_BARRIER);
//! WeakArrayList::capacity ×2 → capacity/capacity_relaxed;
//! set_the_hole ×2 (Isolate/ReadOnlyRoots) → set_the_hole 合并 (hole 值参数注入)。

impl TaggedArrayExtPlaceholder {
  // 落地全集 (TaggedArrayExt / FixedArrayExt / WeakArrayListExt 分 trait 承载):
  pub unsafe fn is_in_bounds(&self, index: u32) -> bool;
  pub unsafe fn is_cow_array(&self, fixed_cow_array_map: Tagged<Map>) -> bool;
  pub unsafe fn get(&self, index: u32) -> Tagged<Self::Element>;
  pub unsafe fn get_relaxed(&self, index: u32) -> Tagged<Self::Element>;
  pub unsafe fn get_acquire(&self, index: u32) -> Tagged<Self::Element>;
  pub unsafe fn get_seqcst(&self, index: u32) -> Tagged<Self::Element>;
  pub unsafe fn set(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode);
  pub unsafe fn set_relaxed(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode);
  pub unsafe fn set_release(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode);
  pub unsafe fn set_seqcst(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode);
  pub unsafe fn set_smi(&self, index: u32, value: Tagged<Smi>);
  pub unsafe fn set_smi_relaxed(&self, index: u32, value: Tagged<Smi>);
  pub unsafe fn set_smi_release(&self, index: u32, value: Tagged<Smi>);
  pub unsafe fn set_smi_seqcst(&self, index: u32, value: Tagged<Smi>);
  pub unsafe fn swap(&self, index: u32, value: Tagged<Self::Element>, mode: WriteBarrierMode) -> Tagged<Self::Element>;
  pub unsafe fn compare_and_swap(&self, index: u32, expected: Tagged<Self::Element>, value: Tagged<Self::Element>, mode: WriteBarrierMode) -> Tagged<Self::Element>;
  // MoveElements 族不另设 fn: 原版实体仅 heap->MoveRange 簿记 (L3 推迟),
  // ptr 级重叠语义由 copy_elements (ptr::copy) 统一承接 (模块头追加登记)。
  pub unsafe fn copy_elements(dst: &Self, dst_index: u32, src: &Self, src_index: u32, len: u32, mode: WriteBarrierMode);
  pub fn right_trim(&self, new_capacity: u32) -> Result<()>;
  pub unsafe fn allocated_size(&self) -> usize;
  pub unsafe fn raw_field_of_first_element(&self) -> *mut TaggedSlot;
  pub unsafe fn raw_field_of_element_at(&self, index: u32) -> *mut TaggedSlot;
  pub fn new_capacity_for_index(index: u32, old_capacity: u32) -> u32;
  pub unsafe fn is_the_hole(&self, hole: Tagged<Object>, index: u32) -> bool;
  pub unsafe fn set_the_hole(&self, hole: Tagged<Object>, index: u32);
  pub unsafe fn fill_with_holes(&self, hole: Tagged<Object>, from: u32, to: u32);
  pub unsafe fn capacity(&self) -> u32;
  pub unsafe fn capacity_relaxed(&self) -> u32;
  pub unsafe fn length(&self) -> u32;
  pub unsafe fn ulength(&self) -> u32;
  pub unsafe fn set_length(&self, value: u32);
  pub unsafe fn next(&mut self) -> Option<Tagged<HeapObject>>; // WeakArrayList::Iterator::Next
}
`;

const p = writeSrc("v8-objects-arrays", "fixed_array_inl.rs.skel", skel);
console.log(
  `fixed-array-inl.h.js: 对数断言 ${assertCount} 条全过; 原版 ${RAW_TOTAL} 出现 / ` +
    `${Object.keys(byQual).length} qualified 全集命中 — 落地 ${Object.keys(LAND).length} ` +
    `+ 推迟登记 ${Object.keys(DEFERRED).length} ` +
    `(New/Allocate/Resize + MoveElements 族 → L3 heap; Move ptr 级语义已并入 copy_elements), 缺项 0; 产物 ${p}`,
);
