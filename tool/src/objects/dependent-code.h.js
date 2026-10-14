#!/usr/bin/env bun
// tool/src/objects/dependent-code.h.js — 覆盖 v8/src/objects/dependent-code.{h,cc}
// → v8-objects-rest/src/dependent_code.rs
// 机械提取: DependencyGroup 位标志 (11 组) + 条目布局常量

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/dependent-code.h");
const ccSrc = readV8("src/objects/dependent-code.cc");

const groupRe = hSrc.match(/enum DependencyGroup\s*\{([\s\S]*?)\n\s*\};/);
if (!groupRe) throw new Error("未找到 DependencyGroup 枚举");

// 逐条提取: kName = 1 << N
const groups = [];
for (const m of groupRe[1].matchAll(/(k\w+)\s*=\s*1\s*<<\s*(\d+)/g)) {
  groups.push({ name: m[1], bit: parseInt(m[2], 10) });
}
const KNOWN = 11;
if (groups.length !== KNOWN) throw new Error(`DependencyGroup 条数 ${groups.length} != ${KNOWN}`);
// 位连续性 (原版顺序 0..10)
groups.forEach((g, i) => {
  if (g.bit !== i) throw new Error(`DependencyGroup 位序失配: ${g.name}=${g.bit} 期望 ${i}`);
});

// cc 侧 DependencyGroupName 表 1:1 对数
const nameFn = ccSrc.match(/const char\* DependentCode::DependencyGroupName\(DependencyGroup group\)\s*\{([\s\S]*?)\n\}/);
if (!nameFn) throw new Error("未找到 DependencyGroupName");
const ccNames = new Map();
for (const m of nameFn[1].matchAll(/case k(\w+):\s*(?:\n\s*)?return "([^"]+)";/g)) {
  ccNames.set("k" + m[1], m[2]);
}
if (ccNames.size !== KNOWN) throw new Error(`DependencyGroupName 表 ${ccNames.size} != ${KNOWN}`);
for (const g of groups) {
  if (!ccNames.has(g.name)) throw new Error(`DependencyGroupName 缺 ${g.name}`);
}

// 条目布局常量 (原版 static constexpr)
const slotsPerEntry = hSrc.match(/kSlotsPerEntry =\s*\n?\s*(\d+);/);
const codeSlotOffset = hSrc.match(/kCodeSlotOffset = (\d+);/);
const groupsSlotOffset = hSrc.match(/kGroupsSlotOffset = (\d+);/);
if (!slotsPerEntry || slotsPerEntry[1] !== "2") throw new Error("kSlotsPerEntry 失配 (应为 2: code + groups)");
if (!codeSlotOffset || codeSlotOffset[1] !== "0") throw new Error("kCodeSlotOffset 失配");
if (!groupsSlotOffset || groupsSlotOffset[1] !== "1") throw new Error("kGroupsSlotOffset 失配");

// kEmptyDependentCode = RootIndex::kEmptyWeakArrayList
const emptyRe = hSrc.match(/kEmptyDependentCode =\s*\n\s*RootIndex::(\w+)/);
if (!emptyRe || emptyRe[1] !== "kEmptyWeakArrayList") throw new Error("kEmptyDependentCode 锚失配");

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/dependent-code.h.js", "src/objects/dependent-code.{h,cc}", "DependentCode 依赖组位标志与条目布局");
rust += `//! DependentCode: {Code, DependencyGroups} 弱数组; 组位标志 11 组。

/// 依赖组位标志 (原版 enum DependencyGroup; 末位须容于 Smi 31 位)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DependencyGroup {
`;
for (const g of groups) {
  rust += `  /// ${g.name} → "${ccNames.get(g.name)}"\n  ${pascal(g.name)} = 1 << ${g.bit},\n`;
}
rust += `}

/// 组位掩码总数 (原版注释: 最后一位必须容于 Smi)。
pub const DEPENDENCY_GROUP_BITS: u32 = ${KNOWN};

/// 每条目槽位数 (code 弱引用 + groups Smi)。
pub const SLOTS_PER_ENTRY: usize = 2;
/// 条目内 code 槽偏移。
pub const CODE_SLOT_OFFSET: usize = 0;
/// 条目内 groups 槽偏移。
pub const GROUPS_SLOT_OFFSET: usize = 1;

/// 空依赖代码表根索引名 (原版 kEmptyDependentCode = RootIndex::kEmptyWeakArrayList)。
pub const EMPTY_DEPENDENT_CODE_ROOT: &str = "kEmptyWeakArrayList";

/// 条目数 → 槽位数 (原版 LengthFor)。
#[inline]
#[must_use]
pub const fn length_for(number_of_entries: usize) -> usize {
  number_of_entries * SLOTS_PER_ENTRY
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DEPENDENCY_GROUP_BITS == 11);
  assert!(DependencyGroup::TransitionGroup as u32 == 1);
  assert!(DependencyGroup::EmptyContextExtensionGroup as u32 == 1 << 10);
  assert!(SLOTS_PER_ENTRY == 2 && CODE_SLOT_OFFSET == 0 && GROUPS_SLOT_OFFSET == 1);
};
`;

const p = writeSrc("v8-objects-rest", "dependent_code.rs", rust);
console.log(`dependent_code.rs: DependencyGroup 11 位组 + cc 名称表 11 条 1:1 + 布局 3 常量, 对数通过`);
