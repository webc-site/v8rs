#!/usr/bin/env bun
// tool/src/wasm/well-known-imports.cc.js
// 机械提取 v8/src/wasm/well-known-imports.cc 的差集:
// WellKnownImportName 的 case→字符串名表 (含 fall-through 共享), Update/Initialize 语义。
// 头侧枚举在 v8-wasm-core/src/well_known_imports.rs (well-known-imports.h.js)。

import { readV8, writeSrc, banner } from "../../lib.js";

const cc = readV8("src/wasm/well-known-imports.cc");
const h = readV8("src/wasm/well-known-imports.h");

// ---------- 锚断言 ----------
const anchors = [
  ["include 自头", cc.includes('#include "src/wasm/well-known-imports.h"')],
  ["Name 函数", cc.includes("const char* WellKnownImportName(WellKnownImport wki) {")],
  ["Update", cc.includes("WellKnownImportsList::UpdateResult WellKnownImportsList::Update(")],
  ["Initialize", cc.includes("void WellKnownImportsList::Initialize(")],
];
for (const [n, ok] of anchors) {
  if (!ok) throw new Error(`锚点缺失: ${n}`);
}

// ---------- case→名字表 (fall-through: 连续 case 共享一个 return) ----------
const fnRegion = cc.slice(
  cc.indexOf("const char* WellKnownImportName"),
  cc.indexOf("WellKnownImportsList::UpdateResult"),
);
const entries = []; // {cases: [...], name: string}
let pending = [];
for (const line of fnRegion.split("\n")) {
  const c = line.match(/case WellKnownImport::k(\w+):/);
  if (c) { pending.push(c[1]); continue; }
  const r = line.match(/return "((?:[^"\\]|\\.)*)";/);
  if (r && pending.length) {
    entries.push({ cases: pending, name: r[1] });
    pending = [];
  }
}
const caseCount = entries.reduce((n, e) => n + e.cases.length, 0);
if (caseCount !== 79) throw new Error(`case 总数 ${caseCount}, 期望 79`);
if (entries.length !== 60) throw new Error(`名字数 ${entries.length}, 期望 60`);
if (entries[0].name !== "uninstantiated" || entries[2].name !== "LinkError") {
  throw new Error("表头锚点漂移");
}

// ---------- 对数: 头侧枚举 vs switch case 差集 ----------
const enumRegion = h.slice(h.indexOf("enum class WellKnownImport"), h.indexOf("};", h.indexOf("enum class WellKnownImport")));
const enumNames = [...enumRegion.matchAll(/^\s*k(\w+)\s*(?:=\s*[\w]+)?,/gm)].map((m) => m[1]);
const caseSet = new Set(entries.flatMap((e) => e.cases));
const enumOnly = enumNames.filter((n) => !caseSet.has(n)); // 如 kFirstCompileTimeImport 别名
for (const c of caseSet) {
  if (!enumNames.includes(c)) throw new Error(`switch case 无枚举项: ${c}`);
}

// ---------- Update 语义锚 ----------
if (!cc.includes("return kFoundIncompatibility;")) throw new Error("kFoundIncompatibility 返回缺失");
if (!cc.includes("return kOK;")) throw new Error("kOK 返回缺失");
if (!cc.includes("IsCompileTimeImport(statuses_[j].load(std::memory_order_relaxed))")) {
  throw new Error("compile-time import 保留语义缺失");
}

console.log(
  `well-known-imports.cc: case=${caseCount} 名字=${entries.length} 枚举=${enumNames.length} 仅枚举别名=${enumOnly.length} (${enumOnly.join(",")})`,
);

// ---------- 生成 Rust ----------
let rust = `${banner(
  "src/wasm/well-known-imports.cc.js",
  "src/wasm/well-known-imports.cc",
  "WellKnownImportName 名字表 + Update/Initialize 语义",
)}
//! well-known 导入的名字台账与状态更新规则; 枚举本体在 well_known_imports.rs。

/// WellKnownImportName 名字表 (${entries.length} 名 / ${caseCount} case, fall-through 共享名字)
/// v8: src/wasm/well-known-imports.cc WellKnownImportName
#[rustfmt::skip]
pub static WELL_KNOWN_IMPORT_NAMES: &[(&[&str], &str)] = &[
`;
for (const e of entries) {
  rust += `  (&[${e.cases.map((c) => `"${c}"`).join(", ")}], "${e.name}"),\n`;
}
rust += `];

/// 头侧枚举中不出现在 switch 的别名 (与 kStringCast 等同值)
/// v8: src/wasm/well-known-imports.h WellKnownImport
#[rustfmt::skip]
pub static ENUM_ONLY_ALIASES: &[&str] = &[
`;
for (const a of enumOnly) {
  rust += `  "${a}",\n`;
}
rust += `];

/// WellKnownImportsList::Update 规则
/// v8: src/wasm/well-known-imports.cc WellKnownImportsList::Update
pub mod update_rules {
    /// 旧值为 kGeneric: 保持不变
    pub const GENERIC_STAYS: bool = true;
    /// 旧值等于新值: 保持不变
    pub const SAME_STAYS: bool = true;
    /// 旧值为 kUninstantiated: 直接写入新值
    pub const UNINITIALIZED_OVERWRITES: bool = true;
    /// 其它冲突: 除 compile-time import (保留状态) 外全部回落 kGeneric, 返回 kFoundIncompatibility
    pub const CONFLICT_FALLS_BACK_TO_GENERIC: bool = true;
    /// 全部兼容时返回 kOK
    pub const ALL_COMPATIBLE_RETURNS_OK: bool = true;
    /// 读写均为 memory_order_relaxed
    pub const MEMORY_ORDER_RELAXED: bool = true;
}

/// WellKnownImportsList::Initialize: 逐项写入 (要求旧值全为 kUninstantiated)
/// v8: src/wasm/well-known-imports.cc WellKnownImportsList::Initialize
pub const INITIALIZE_EXPECTS_UNINSTANTIATED: bool = true;

// —— 提取不变式断言 ——
const _: () = {
    assert!(WELL_KNOWN_IMPORT_NAMES.len() == 60);
    assert!(WELL_KNOWN_IMPORT_NAMES.iter().map(|(cs, _)| cs.len()).sum::<usize>() == 79);
    assert!(WELL_KNOWN_IMPORT_NAMES[0].1 == "uninstantiated");
    assert!(WELL_KNOWN_IMPORT_NAMES[2].1 == "LinkError");
    assert!(INITIALIZE_EXPECTS_UNINSTANTIATED);
};
`;

const p = writeSrc("v8-wasm-core", "well_known_imports_cc.rs.skel", rust);
console.log(`written ${p}`);
