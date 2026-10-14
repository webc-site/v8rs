#!/usr/bin/env bun
// tool/src/objects/deoptimization-data.h.js — 覆盖 v8/src/objects/deoptimization-data.{h,cc}
// → v8-objects-rest/src/deoptimization_data.rs
// 机械提取: DeoptimizationLiteralKind 枚举 (含 Wasm 别名) + 访问器 X-macro 登记
// 475+378 行语义面 (翻译表编解码) 登记跳过。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/objects/deoptimization-data.h");

// DeoptimizationLiteralKind 枚举
const kindRe = hSrc.match(/enum class DeoptimizationLiteralKind : uint8_t\s*\{([\s\S]*?)\n\};/);
if (!kindRe) throw new Error("未找到 DeoptimizationLiteralKind");
const entries = [];
let curDoc = [];
for (const raw of kindRe[1].split("\n")) {
  const line = raw.replace(/\/\/.*$/, "").trim();
  if (!line) continue;
  const m = line.match(/^(k\w+)(?:\s*=\s*(k\w+))?[,]?$/) || line.match(/^(k\w+)(?:\s*=\s*(k\w+))?$/);
  if (!m) throw new Error(`条目不可解析: ${line}`);
  entries.push({ name: m[1], aliasOf: m[2] ?? null });
}
// 对数: 10 条目 (9 顺序 + kWasmInt64 = kSignedBigInt64 别名)
if (entries.length !== 10) throw new Error(`DeoptimizationLiteralKind 条数 ${entries.length} != 10`);
const last = entries[entries.length - 1];
if (last.name !== "kWasmInt64" || last.aliasOf !== "kSignedBigInt64") throw new Error("kWasmInt64 别名失配");

// 顺序值展开 (别名取目标值)
const valueOf = new Map();
let seq = 0;
for (const e of entries) {
  if (e.aliasOf) valueOf.set(e.name, valueOf.get(e.aliasOf));
  else { valueOf.set(e.name, seq); seq++; }
}
if (valueOf.get("kWasmInt64") !== valueOf.get("kSignedBigInt64")) throw new Error("别名值失配");

// 访问器 X-macro 登记 (DECL_ELEMENT_ACCESSORS / DECL_ENTRY_ACCESSORS)
for (const macro of ["DECL_ELEMENT_ACCESSORS", "DECL_ENTRY_ACCESSORS"]) {
  if (!hSrc.includes(`#define ${macro}`)) throw new Error(`未找到 ${macro} X-macro`);
}

const pascal = (s) => s.replace(/^k/, "").toLowerCase().replace(/(^|_)([a-z0-9])/g, (_, p, c) => c.toUpperCase());

let rust = banner("src/objects/deoptimization-data.h.js", "src/objects/deoptimization-data.{h,cc}", "去优化字面量类别枚举");
rust += `//! DeoptimizationData: 去优化字面量类别。
//! 翻译表编解码语义面 (TranslationOpcode/翻译数组) 登记跳过, 由 Developer 就地实现。

/// 去优化字面量类别 (原版 enum class DeoptimizationLiteralKind : uint8_t, 共 ${entries.length} 条目)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeoptimizationLiteralKind {
`;
for (const e of entries) {
  const v = valueOf.get(e.name);
  rust += `  /// ${e.name}${e.aliasOf ? ` (别名 = ${e.aliasOf})` : ""}\n  ${pascal(e.name)} = ${v},\n`;
}
rust += `}

impl DeoptimizationLiteralKind {
  /// 非别名声目数 (别名 kWasmInt64 复用 kSignedBigInt64 值)。
  pub const COUNT: usize = ${seq};

  /// 数值反查 (别名值命中共有条目)。
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
`;
for (const e of entries) {
  if (!e.aliasOf) rust += `      ${valueOf.get(e.name)} => Some(Self::${pascal(e.name)}),\n`;
}
rust += `      _ => None,
    }
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DeoptimizationLiteralKind::Object as u8 == 0);
  assert!(DeoptimizationLiteralKind::Invalid as u8 == 4);
  assert!(DeoptimizationLiteralKind::WasmInt64 as u8 == DeoptimizationLiteralKind::SignedBigInt64 as u8);
};
`;

const p = writeSrc("v8-objects-rest", "deoptimization_data.rs", rust);
console.log(`deoptimization_data.rs: DeoptimizationLiteralKind ${entries.length} 条目 (${seq} 独立值), 对数通过`);
