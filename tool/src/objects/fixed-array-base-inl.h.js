#!/usr/bin/env bun
// tool/src/objects/fixed-array-base-inl.h.js — fixed-array-base-inl.h 差集对数审计 (每原版文件一脚本红线)
//   v8/src/objects/fixed-array-base-inl.h (82 行) → 落地 v8-objects-arrays/src/fixed_array_base.rs
//   (IsCowArray 参数注入版登记于 fixed-array-base.h.js 脚本头 → 对数落 fixed_array.rs)
//   偏移比对源: v8/src/objects/fixed-array-base.h (kLengthOffset = sizeof(HeapObject/TrustedObject))
//
// 产物: v8-objects-arrays/src/fixed_array_base_inl.rs.skel (对数骨架/合并指引, 审计专用;
//   落地 base.rs 以就地修订为准, 本脚本不覆盖落地文件)
// 规则: inline 方法定义全集机械提取 + 槽位操作/常量/偏移逐项与原文对数 (不符 throw)
//   + 落地对数核对 (每项 def 须在落地文件逐项找到, 缺项 throw 并列票单)
import { readFileSync } from "node:fs";
import { join } from "node:path";
import { banner, readV8, root, writeSrc } from "../../lib.js";

const inl = readV8("src/objects/fixed-array-base-inl.h");
const h = readV8("src/objects/fixed-array-base.h");

// ── 对数断言 (原版锚点 + 机械提取; 任一不符 throw) ────────────────────────
let assertCount = 0;
const fail = (label, got, want) => {
  throw new Error(`对数失配: ${label} got=${JSON.stringify(got)} want=${JSON.stringify(want)}`);
};
const count = (src, re) => (src.match(re) || []).length;

// 1) inline 方法定义全集机械提取: `^inline <ret> <Class>::<method>(`
const defs = [...inl.matchAll(/^inline\s+[\w:]+\s+(\w+)::(\w+)\s*\(/gm)].map((m) => `${m[1]}::${m[2]}`);
// 原版全集 (fixed-array-base-inl.h 实测 14 条 def, 含重载):
const DEF_TOTAL = 14;
if (defs.length !== DEF_TOTAL) fail("inline def 总数", defs.length, DEF_TOTAL);
assertCount++;
// 类×方法×重载 逐项对数 (方法名全集断言)
const byQual = {};
for (const d of defs) byQual[d] = (byQual[d] || 0) + 1;
const QUAL_TABLE = {
  "FixedArrayBase::length": 3,
  "FixedArrayBase::ulength": 1,
  "FixedArrayBase::set_length": 2,
  "FixedArrayBase::GetMaxLengthForNewSpaceAllocation": 1,
  "FixedArrayBase::IsCowArray": 1,
  "TrustedFixedArrayBase::length": 3,
  "TrustedFixedArrayBase::ulength": 1,
  "TrustedFixedArrayBase::set_length": 2,
};
for (const [q, want] of Object.entries(QUAL_TABLE)) {
  if (byQual[q] !== want) fail(`def ${q}`, byQual[q], want);
  assertCount++;
}
if (Object.keys(byQual).length !== Object.keys(QUAL_TABLE).length)
  fail("qualified 方法全集大小", Object.keys(byQual).length, Object.keys(QUAL_TABLE).length);
assertCount++;

// 2) 槽位操作对数 (原子读写与原文逐项比对)
for (const [re, want, label] of [
  [/base::AsAtomic32::Acquire_Load\(&length_\)/g, 2, "Acquire_Load(&length_)"],
  [/base::AsAtomic32::Relaxed_Load\(&length_\)/g, 2, "Relaxed_Load(&length_)"],
  [/base::AsAtomic32::Release_Store\(&length_, value\)/g, 2, "Release_Store(&length_)"],
  [/length_ = value;/g, 2, "普通写 length_ = value"],
  [/SafeHeapObjectSize\(length_\)/g, 2, "SafeHeapObjectSize(length_)"],
  [/return length\(\);/g, 2, "ulength 转发 length()"],
  [/ElementsKindToShiftSize\(kind\)/g, 1, "GetMaxLength ElementsKind 依赖 (参数注入)"],
  [/ReadOnlyHeap::EarlyGetReadOnlyRoots/g, 1, "IsCowArray roots 依赖 (参数注入)"],
  [/kMaxRegularHeapObjectSize - FixedArrayBase::kHeaderSize/g, 1, "GetMaxLength 尺寸式"],
]) {
  if (count(inl, re) !== want) fail(label, count(inl, re), want);
  assertCount++;
}

// 3) 偏移逐项比对: base.h kLengthOffset = sizeof(HeapObject/TrustedObject) 公式 ×2
if (count(h, /static constexpr int kLengthOffset = sizeof\((HeapObject|TrustedObject)\);/g) !== 2)
  fail("原版 kLengthOffset 公式", count(h, /static constexpr int kLengthOffset = sizeof\((HeapObject|TrustedObject)\);/g), 2);
assertCount++;

// ── 落地对数核对 (fixed_array_base.rs 应逐项找到 -inl 族方法; 缺项 throw 列票单) ─
const baseRs = readFileSync(join(root, "v8-objects-arrays", "src", "fixed_array_base.rs"), "utf8");
const faRs = readFileSync(join(root, "v8-objects-arrays", "src", "fixed_array.rs"), "utf8");
// 每个 qualified def → 落地 Rust 锚 (双基类同构 → ×2); IsCowArray 参数注入落 fixed_array.rs
const LAND_TABLE = [
  ["FixedArrayBase::length", /unsafe fn length\(&self\) -> u32 \{/g, 2, "base.rs 双 trait"],
  ["(T)FixedArrayBase::ulength", /unsafe fn ulength\(&self\) -> u32 \{/g, 2, "base.rs 双 trait"],
  ["(T)FixedArrayBase::length(Acquire)", /unsafe fn length_acquire\(&self\) -> u32 \{/g, 2, "base.rs 双 trait"],
  ["(T)FixedArrayBase::length(Relaxed)", /unsafe fn length_relaxed\(&self\) -> u32 \{/g, 2, "base.rs 双 trait"],
  ["(T)FixedArrayBase::set_length", /unsafe fn set_length\(&self, value: u32\) \{/g, 2, "base.rs 双 trait"],
  ["(T)FixedArrayBase::set_length(Release)", /unsafe fn set_length_release\(&self, value: u32\) \{/g, 2, "base.rs 双 trait"],
  ["FixedArrayBase::GetMaxLengthForNewSpaceAllocation", /pub const fn max_length_for_new_space_allocation\(shift_size: u32\)/g, 1, "shift 参数注入版"],
  ["FixedArrayBase::IsCowArray", /unsafe fn is_cow_array\(&self, fixed_cow_array_map: Tagged<Map>\)/g, 1, "cow map 参数注入版 (fixed_array.rs)"],
];
const landed = [];
const missing = [];
for (const [label, re, want, note] of LAND_TABLE) {
  const file = label === "FixedArrayBase::IsCowArray" ? faRs : baseRs;
  const got = count(file, re);
  if (got !== want) missing.push(`${label} (${note}): got=${got} want=${want}`);
  else landed.push(`${label} → ${note} ×${got}`);
  assertCount++;
}
if (missing.length) throw new Error(`落地对数缺项 (票单):\n  ${missing.join("\n  ")}`);

// 落地原子序 + 偏移锚逐项 (Acquire/Release 各 ×2; length 槽 = offset_of ×2 + snapshot K_LENGTH_OFFSET)
for (const [re, want, label] of [
  [/Ordering::Acquire/g, 2, "base.rs Acquire 读 ×2 trait"],
  [/Ordering::Release/g, 2, "base.rs Release 写 ×2 trait"],
  [/offset_of!\(FixedArrayBase, length\)/g, 1, "base.rs FixedArrayBase length 槽偏移"],
  [/offset_of!\(TrustedFixedArrayBase, length\)/g, 1, "base.rs TrustedFixedArrayBase length 槽偏移"],
  [/pub const K_LENGTH_OFFSET: usize = K_TAGGED_SIZE;/g, 1, "snapshot 锚 K_LENGTH_OFFSET"],
  [/assert!\(size_of::<TrustedFixedArrayBase>\(\) == size_of::<FixedArrayBase>\(\)\)/g, 1, "双基类布局对数编译期断言"],
]) {
  if (count(baseRs, re) !== want) fail(`落地 ${label}`, count(baseRs, re), want);
  assertCount++;
}
// 覆盖不变式: 原版 14 def = base.rs 12 trait 方法 + GetMaxLength 1 + IsCowArray (fixed_array.rs) 1
const covered = LAND_TABLE.slice(0, 6).reduce((a, [, , want]) => a + want, 0) + 1 + 1;
if (covered !== DEF_TOTAL) fail("覆盖不变式 (def 全覆盖)", covered, DEF_TOTAL);
assertCount++;

// ── 产物: 对数骨架 (合并指引; 审计专用, 不覆盖落地 base.rs) ────────────────
const sigLines = [
  "pub trait FixedArrayBaseExt: Copy {",
  "  pub unsafe fn length(&self) -> u32;          // FixedArrayBase::length (plain→Relaxed)",
  "  pub unsafe fn ulength(&self) -> u32;         // FixedArrayBase::ulength",
  "  pub unsafe fn length_acquire(&self) -> u32;  // length(AcquireLoadTag)",
  "  pub unsafe fn length_relaxed(&self) -> u32;  // length(RelaxedLoadTag)",
  "  pub unsafe fn set_length(&self, value: u32); // set_length (plain→Relaxed)",
  "  pub unsafe fn set_length_release(&self, value: u32); // set_length(ReleaseStoreTag)",
  "}",
  "pub trait TrustedFixedArrayBaseExt: Copy { /* 同上 6 方法, 镜像落地 */ }",
  "impl FixedArrayBase {",
  "  pub const fn max_length_for_new_space_allocation(shift_size: u32) -> usize;",
  "  // GetMaxLengthForNewSpaceAllocation — ElementsKindToShiftSize 参数注入 (h 脚本头登记)",
  "}",
  "// FixedArrayBase::IsCowArray → fixed_array.rs TaggedArrayExt::is_cow_array",
  "//   — ReadOnlyRoots::unchecked_fixed_cow_array_map 参数注入 (h 脚本头登记)",
].join("\n");

const skel = `${banner(
  "src/objects/fixed-array-base-inl.h.js",
  "src/objects/fixed-array-base-inl.h",
  "对数骨架 (审计专用): 14 inline def 全集处置表; 落地以 fixed_array_base.rs 就地修订为准",
)}
//! fixed-array-base-inl.h → fixed_array_base.rs 合并指引
//!
//! 对数处置 (${DEF_TOTAL} def 全覆盖, 0 缺项):
${landed.map((l) => "//! - " + l).join("\n")}

${sigLines}
`;

const p = writeSrc("v8-objects-arrays", "fixed_array_base_inl.rs.skel", skel);
console.log(
  `fixed-array-base-inl.h.js: 对数断言 ${assertCount} 条全过; 原版 ${DEF_TOTAL} inline def ` +
    `(双基类 12 + GetMaxLength + IsCowArray) 落地逐项命中, 缺项 0; 参数注入登记 ×2 ` +
    `(roots cow map / ElementsKind shift); 产物 ${p}`,
);
