#!/usr/bin/env bun
// tool/src/wasm/wasm-builtin-list.h.js
// 机械提取 v8/src/wasm/wasm-builtin-list.h:
// WASM_BUILTINS_WITH_JUMP_TABLE_SLOT(V, VTRAP) 全表 (陷阱 + 远跳 builtin),
// WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT 与 WASM_BUILTIN_LIST 组合宏。

import { readV8, writeSrc, banner } from "../../lib.js";

const hSrc = readV8("src/wasm/wasm-builtin-list.h");
const globalsSrc = readV8("src/common/globals.h");

// 合并续行的宏体
function macroBody(src, name) {
  const lines = src.split("\n");
  for (let i = 0; i < lines.length; i++) {
    const m = lines[i].match(new RegExp(`^#define\\s+${name}\\(([^)]*)\\)\\s*(.*)$`));
    if (!m) continue;
    let body = m[2];
    while (body.endsWith("\\")) body = body.slice(0, -1) + " " + (lines[++i] ?? "").trim();
    return body.replace(/\/\*[\s\S]*?\*\//g, " ");
  }
  throw new Error(`未找到宏 ${name}`);
}

// FOREACH_WASM_TRAPREASON (陷阱段)
const trapBody = macroBody(globalsSrc, "FOREACH_WASM_TRAPREASON");
const trapReasons = [...trapBody.matchAll(/V\((\w+)\)/g)].map((m) => m[1]);
if (trapReasons.length !== 21) throw new Error(`TrapReason ${trapReasons.length}, 期望 21`);

// WASM_BUILTINS_WITH_JUMP_TABLE_SLOT(V, VTRAP)
const withSlotBody = macroBody(hSrc, "WASM_BUILTINS_WITH_JUMP_TABLE_SLOT");
// 展开为有序条目: {kind: "trap"|"builtin", name}
const entries = [];
{
  const body = withSlotBody
    .replace(/FOREACH_WASM_TRAPREASON\s*\(\s*VTRAP\s*\)/g, "\0TRAPS\0");
  const tokens = body.split(/(?=[A-Z_]\w*\s*\()|(?=\0TRAPS\0)/);
  // 简化: 逐段扫描
  let rest = body;
  const trapRe = /\0TRAPS\0/;
  const itemRe = /(V|VTRAP)\(\s*(\w+)\s*\)/g;
  let trapInserted = false;
  let m;
  // 按出现顺序插入: 先找 TRAPS 标记位置, 再合并 V/VTRAP 的位置排序
  const events = [];
  if (trapRe.test(rest)) {
    events.push({ pos: rest.search(trapRe), kind: "traps" });
  }
  while ((m = itemRe.exec(rest)) !== null) {
    events.push({ pos: m.index, kind: m[1] === "VTRAP" ? "trap" : "builtin", name: m[2] });
  }
  events.sort((a, b) => a.pos - b.pos);
  for (const e of events) {
    if (e.kind === "traps") {
      for (const t of trapReasons) entries.push({ kind: "trap", name: t });
    } else {
      entries.push({ kind: e.kind, name: e.name });
    }
  }
}
if (entries.length !== 21 + 134) throw new Error(`跳表 builtin ${entries.length} 条, 期望 155 (21 陷阱 + 134 builtin)`);
if (entries[0].kind !== "trap" || entries[0].name !== "TrapUnreachable") throw new Error("首条应为 TrapUnreachable");
if (entries[21].name !== "WasmCompileLazy") throw new Error("第 22 条应为 WasmCompileLazy (原版注释: 与 TrapId 对齐)");

// WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT
const withoutBody = macroBody(hSrc, "WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT");
const withoutList = [...withoutBody.matchAll(/V\(\s*(\w+)\s*\)/g)].map((m) => m[1]);

// WASM_BUILTIN_LIST = WITH + WITHOUT
const builtinListBody = macroBody(hSrc, "WASM_BUILTIN_LIST");
if (!builtinListBody.includes("WASM_BUILTINS_WITH_JUMP_TABLE_SLOT(V, VTRAP)") ||
    !builtinListBody.includes("WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT(V)")) {
  throw new Error("WASM_BUILTIN_LIST 组合漂移");
}

console.log(`跳表 ${entries.length} (trap 21 + builtin 134), 无跳表 ${withoutList.length}`);

let rust = banner(
  "src/wasm/wasm-builtin-list.h.js",
  "src/wasm/wasm-builtin-list.h",
  "wasm builtin 全表: 远跳表槽位序 (陷阱段对齐 compiler::TrapId) + 非跳表 builtin",
);

rust += `
/// 跳表槽位条目 (陷阱 + 普通 builtin, 顺序 = 原版 WASM_BUILTINS_WITH_JUMP_TABLE_SLOT 展开)
/// 注: 前 21 项为 FOREACH_WASM_TRAPREASON (VTRAP), 与 compiler::TrapId 序号对齐, 顺序敏感。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WasmFarJumpBuiltin {
`;

for (const [i, e] of entries.entries()) {
  const rustName = e.kind === "trap"
    ? "THROW_WASM_" + e.name.replace(/^Trap/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()
    : e.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
  rust += `  /// ${e.kind === "trap" ? `VTRAP(${e.name}) → Builtin::kThrowWasm${e.name.replace(/^Trap/, "")}` : `V(${e.name})`}\n  ${rustName} = ${i},\n`;
}

rust += `}

/// 跳表条目总数 (21 陷阱 + 134 builtin, 对应 WASM_BUILTINS_WITH_JUMP_TABLE_SLOT 展开长度)
pub const WASM_FAR_JUMP_BUILTIN_COUNT: usize = ${entries.length};

/// 陷阱段条数 (FOREACH_WASM_TRAPREASON)
pub const TRAP_SEGMENT_COUNT: usize = 21;

/// 非跳表 builtin (对应 WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT, ${withoutList.length} 条)
#[rustfmt::skip]
pub static WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT: &[&str] = &[
${withoutList.map((n) => `  "${n}",`).join("\n")}
];

impl WasmFarJumpBuiltin {
  /// builtin 名 (对应 V(NAME) 原文)
  #[must_use]
  pub const fn builtin_name(self) -> &'static str {
    match self {
`;

for (const [i, e] of entries.entries()) {
  const rustName = e.kind === "trap"
    ? "THROW_WASM_" + e.name.replace(/^Trap/, "").replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase()
    : e.name.replace(/([a-z0-9])([A-Z])/g, "$1_$2").toUpperCase();
  rust += `      Self::${rustName} => "${e.kind === "trap" ? `ThrowWasm${e.name.replace(/^Trap/, "")}` : e.name}",\n`;
}

rust += `    }
  }

  /// 是否陷阱段 (VTRAP)
  #[must_use]
  pub const fn is_trap(self) -> bool {
    (self as usize) < TRAP_SEGMENT_COUNT
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(WASM_FAR_JUMP_BUILTIN_COUNT == ${entries.length});
  assert!(TRAP_SEGMENT_COUNT == 21);
  assert!((WasmFarJumpBuiltin::THROW_WASM_UNREACHABLE as usize) == 0);
  assert!((WasmFarJumpBuiltin::WASM_COMPILE_LAZY as usize) == 21);
  assert!((WasmFarJumpBuiltin::WASM_COMPILE_LAZY as usize) == TRAP_SEGMENT_COUNT);
  assert!(WasmFarJumpBuiltin::THROW_WASM_UNREACHABLE.is_trap());
  assert!(!WasmFarJumpBuiltin::WASM_COMPILE_LAZY.is_trap());
  assert!(WASM_BUILTINS_WITHOUT_JUMP_TABLE_SLOT.len() == ${withoutList.length});
};
`;
;

const p = writeSrc("v8-wasm-core", "wasm_builtin_list.rs", rust);
console.log(`written ${p}`);
