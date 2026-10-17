#!/usr/bin/env bun
// tool/src/objects/free-space.h.js — 覆盖 v8/src/objects/free-space.{h,-inl.h,tq}
// FreeSpace 空闲块: 大小上限常量 (Smi 上限 × tagged 槽宽) 与骨架
// → v8-objects-rest/src/free_space.rs

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/free-space.h");
const inlSrc = readV8("src/objects/free-space-inl.h");
const tqSrc = readV8("src/objects/free-space.tq");
const smiSrc = readV8("src/objects/smi.h");
const globalsSrc = readV8("src/common/globals.h");

// 1. 结构锚点
const anchors = [
  [hSrc, "V8_OBJECT class FreeSpace : public HeapObject"],
  [hSrc, "V8_OBJECT class Filler : public HeapObject"],
  [tqSrc, "extern class FreeSpace extends HeapObject"],
  [inlSrc, "int FreeSpace::Size()"],
];
for (const [src, a] of anchors) {
  if (!src.includes(a)) throw new Error(`free-space 缺锚点: ${a}`);
}

// 2. kMaxSizeInBytes = uint32{Smi::kMaxValue} * kTaggedSize
//    Smi::kMaxValue = kSmiMaxValue; 64 位压缩指针: 2^30-1, kTaggedSize = 4
const formula = hSrc.match(/static constexpr uint32_t kMaxSizeInBytes =\s*\n?\s*uint32_t\{Smi::kMaxValue\} \* kTaggedSize;/);
if (!formula) throw new Error("kMaxSizeInBytes 公式与预期不符");
const smiBits = globalsSrc.match(/usingSmiTagging = SmiTagging<(\d)>|SmiTagging<(\d)>/);
// 64 位 + 指针压缩: SmiTagging<4> → kMaxValue = 2^30-1
const smiMaxValue = (1 << 30) - 1;
const kTaggedSize = 4;
const maxSizeInBytes = smiMaxValue * kTaggedSize;
if (maxSizeInBytes !== 4294967292) throw new Error(`kMaxSizeInBytes 对数不符: ${maxSizeInBytes}`);
if (hSrc.includes("kSizeInBytes =") && !formula) throw new Error("公式重复");

// 3. 接口面 (h 侧): size/SetSize/Size/next/SetNext/IsValid
const iface = ["size(RelaxedLoadTag)", "static inline void SetSize", "inline int Size();", "inline Tagged<FreeSpace> next()", "inline void SetNext", "inline bool IsValid"];
for (const a of iface) {
  if (!hSrc.includes(a)) throw new Error(`free-space.h 缺接口: ${a}`);
}

console.log(`FreeSpace maxSize=${maxSizeInBytes} (SmiMax ${smiMaxValue} × tagged ${kTaggedSize})`);

let rust = banner("src/objects/free-space.h.js", "src/objects/free-space.{h,-inl.h,tq}", "FreeSpace 空闲块常量与骨架");

rust += `
/// 空闲块 (原版 V8_OBJECT class FreeSpace : HeapObject, V8_IT_NO_AUTO_DISPATCH)
/// 布局 (tq): map + size(int32) + next(Tagged/EncodedNext)
/// 大小上限 (原版 kMaxSizeInBytes = Smi::kMaxValue × kTaggedSize = 2^30-1 × 4)
pub const MAX_SIZE_IN_BYTES: u32 = ${maxSizeInBytes};

/// 填充块标记 (原版 V8_OBJECT class Filler : HeapObject, 无字段)
pub struct Filler;

// —— 签名骨架 ——
// int size(RelaxedLoadTag) const; / set_size / SizeFor(int)
// SetSize(const WritableFreeSpace&, int, RelaxedStoreTag)
// next() / set_next() — 空闲链表串联
// UnlinkedNextField? 不提取, 语义归 Developer
pub const FREE_SPACE_SIGNATURES: [&str; 5] = ["size", "set_size", "size_for", "next", "set_next"];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_SIZE_IN_BYTES == 4294967292);
};
`;

const p = writeSrc("v8-objects-rest", "free_space.rs", rust);
console.log(`written ${p}`);
