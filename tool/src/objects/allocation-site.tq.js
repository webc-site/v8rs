#!/usr/bin/env bun
// tool/src/objects/allocation-site.tq.js — 覆盖 v8/src/objects/allocation-site.tq
// 组→文件映射 (本脚本管辖): 席位表差集 (.h 常量/位域归既有 tool/src/objects/allocation-site.h.js)
// → v8-objects-js-async/src/allocation_site_tq.rs
// 机械提取: AllocationSite 外部声明 + AllocationMemento 布局 (1 字段)

import { readV8, writeSrc, banner } from "../../lib.js";

const tq = readV8("src/objects/allocation-site.tq");

let checks = 0;
const must = (c, m) => { checks++; if (!c) throw new Error(`[allocation-site.tq.js] ${m}`); };

// ── 1. AllocationSite 外部声明 (布局定义在 .h, tq 侧零字段) ──────────────
must(tq.includes("extern class AllocationSite extends HeapObject;"), "tq 缺 AllocationSite 外部声明 (零字段)");

// ── 2. AllocationMemento (Struct, 1 字段) ────────────────────────────────
const cls = tq.match(/extern class AllocationMemento extends Struct \{([\s\S]*?)\}/);
must(cls, "tq 缺 AllocationMemento");
const fields = [...cls[1].matchAll(/^  (\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
must(fields.length === 1, `AllocationMemento 字段数 ${fields.length} != 1`);
must(fields[0][0] === "allocation_site" && fields[0][1] === "AllocationSite", "AllocationMemento 布局漂移");
must(tq.includes("@cppObjectLayoutDefinition"), "tq 缺 @cppObjectLayoutDefinition");

let rust = banner("src/objects/allocation-site.tq.js", "src/objects/allocation-site.tq",
  "席位表差集: tq 声明面 (.h 常量/位域归既有 allocation-site.h.js)");
rust += `//! AllocationSite/AllocationMemento tq 差集。
//!
//! AllocationSite 布局与嵌套链 (nested_site) 在 .h 定义 (V8_OBJECT 非 tq 生成);
//! AllocationMemento 为字面量分配后追加的 memento 槽 (指向所属 site)。

/// AllocationSite: tq 侧仅外部声明, 布局归 .h (既有 allocation-site.h.js 产物)。
pub const ALLOCATION_SITE_TQ_NEW_FIELDS: usize = 0;

// ── AllocationMemento 布局 (tq extern class extends Struct) ──
pub const ALLOCATION_MEMENTO_FIELDS: [(&str, &str); ${fields.length}] = [
`;
for (const f of fields) rust += `  ("${f[0]}", "${f[1]}"),\n`;
rust += `];

// ── 提取不变式断言 ──
const _: () = {
  assert!(ALLOCATION_SITE_TQ_NEW_FIELDS == 0);
  assert!(ALLOCATION_MEMENTO_FIELDS.len() == 1);
  assert!(ALLOCATION_MEMENTO_FIELDS[0].0 == "allocation_site");
};
`;

const p = writeSrc("v8-objects-js-async", "allocation_site_tq.rs", rust);
console.log(`allocation-site.tq.js: ${p} — AllocationSite 0 字段声明 + AllocationMemento ${fields.length} 字段, 断言 ${checks} 项全过`);
