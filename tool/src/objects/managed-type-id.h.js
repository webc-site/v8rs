#!/usr/bin/env bun
// tool/src/objects/managed-type-id.h.js — 覆盖 v8/src/objects/managed-type-id.h
// → v8-objects-rest/src/managed_type_id.rs
// 机械提取: ManagedTypeId → 名称表 (ToString switch 逐 case 1:1)

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/managed-type-id.h");
const internalSrc = readV8("include/v8-internal.h");

// 枚举定义侧 (include/v8-internal.h) — 原版侧锚
const enumRe = internalSrc.match(/enum class ManagedTypeId\s*:\s*(\w+)\s*\{([\s\S]*?)\n\};/);
if (!enumRe) throw new Error("未找到 ManagedTypeId 枚举定义 (include/v8-internal.h)");
const reprType = enumRe[1];
const enumNames = [...enumRe[2].matchAll(/(k\w+)/g)].map((m) => m[1]);

// ToString switch case 侧 (managed-type-id.h)
const switchRe = hSrc.match(/switch \(type_id\)\s*\{([\s\S]*?)\n\s*\}\s*\n\}/);
if (!switchRe) throw new Error("未找到 ToString switch");
const caseNames = [...switchRe[1].matchAll(/case ManagedTypeId::(k\w+):/g)].map((m) => m[1]);
const strNames = [...switchRe[1].matchAll(/return "([^"]+)";/g)].map((m) => m[1]);

// 对数: case 数 == 字符串数 == 枚举变体数
if (caseNames.length !== strNames.length) throw new Error(`case ${caseNames.length} != return ${strNames.length}`);
if (caseNames.length !== enumNames.length) {
  throw new Error(`ToString ${caseNames.length} 条 != 枚举 ${enumNames.length} 变体 (漏枚举侧对数)`);
}
for (let i = 0; i < caseNames.length; i++) {
  if (caseNames[i] !== enumNames[i]) throw new Error(`顺序失配 @${i}: ${caseNames[i]} vs ${enumNames[i]}`);
}

let rust = banner("src/objects/managed-type-id.h.js", "src/objects/managed-type-id.h", "ManagedTypeId 名称表 (枚举 1:1)");
rust += `//! ManagedTypeId 名称表: C++ 侧 \`ToString(ManagedTypeId)\` switch 的 1:1 移植。
//! 枚举定义在 include/v8-internal.h; 共 ${caseNames.length} 项。

/// ManagedTypeId 每个变体的显示名 (原版 ToString)。
#[rustfmt::skip]
pub const MANAGED_TYPE_ID_NAMES: [(&str, &str); ${caseNames.length}] = [
`;
for (let i = 0; i < caseNames.length; i++) {
  rust += `  ("${caseNames[i]}", "${strNames[i]}"),\n`;
}
rust += `];

/// 按 k 名查显示名。
#[inline]
#[must_use]
pub fn to_string(type_id: &str) -> Option<&'static str> {
  MANAGED_TYPE_ID_NAMES.iter().find(|(k, _)| *k == type_id).map(|(_, n)| *n)
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(MANAGED_TYPE_ID_NAMES.len() == ${caseNames.length});
  assert!(MANAGED_TYPE_ID_NAMES[0] == ("${caseNames[0]}", "${strNames[0]}"));
  assert!(MANAGED_TYPE_ID_NAMES[${caseNames.length - 1}] == ("${caseNames[caseNames.length - 1]}", "${strNames[strNames.length - 1]}"));
};
`;

const p = writeSrc("v8-objects-rest", "managed_type_id.rs", rust);
console.log(`managed_type_id.rs: 名称表 ${caseNames.length} 条 (与 v8-internal.h 枚举 1:1), 对数通过`);
