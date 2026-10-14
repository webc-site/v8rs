#!/usr/bin/env bun
// tool/src/objects/objects-definitions.h.js — 覆盖 v8/src/objects/objects-definitions.h
// X-macro 全量展开: STRING_TYPE_LIST / STRUCT_LIST / STRUCT_MAPS_LIST /
//   ALLOCATION_SITE_LIST / DATA_HANDLER_LIST (INSTANCE_TYPE_LIST 归 instance-type.h.js)
// 对数基准: clang -E 预处理产物 (todo.md 十一节: 原版侧一律取编译器产物)
// → v8-objects-core/src/objects_definitions.rs

import { readV8, writeSrc, banner } from "../../lib.js";
import { execFileSync } from "node:child_process";
import { join } from "node:path";
import { mkdirSync, writeFileSync, rmSync } from "node:fs";
import { tmpdir } from "node:os";

const DEF_SRC = readV8("src/objects/objects-definitions.h");

// —— clang -E 对数基准 (构建配置: INTL 开 / WASM 开 / TEMPORAL 关 / METAGEN 空) ——
const shim = join(tmpdir(), "v8rs-tool-pp-shim");
mkdirSync(join(shim, "absl/numeric"), { recursive: true });
mkdirSync(join(shim, "metagen"), { recursive: true });
writeFileSync(
  join(shim, "absl/numeric/int128.h"),
  "#pragma once\n#include <cstdint>\nnamespace absl { using int128 = __int128; using uint128 = unsigned __int128; }\n",
);
writeFileSync(join(shim, "metagen/instance-types.h"), "#pragma once\n");

function pp(probe) {
  return execFileSync(
    "clang",
    ["-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include", "-I", shim,
      "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-"],
    { input: probe, encoding: "utf8", cwd: join(import.meta.dir, "../../.."), maxBuffer: 256 * 1024 * 1024 },
  );
}

// 行宏探针: 每条目产出一行 ROW(...) 便于逐条对数
const probe = `#include "src/base/macros.h"
#include "src/objects/objects-definitions.h"
#define R4(A, B, C, D) ROW("4|" #A "|" #B "|" #C "|" #D)
#define R5(_, A, B, C, D) ROW("5|" #A "|" #B "|" #C "|" #D)
#define R3(NAME, Name, name) ROW("3|" #NAME "|" #Name "|" #name)
#define R3M(_, name_map, NameMap) ROW("3m|" #name_map "|" #NameMap)
STRING_TYPE_LIST(R4)
STRUCT_LIST(R3)
STRUCT_MAPS_LIST(R3M)
ALLOCATION_SITE_LIST(R5, )
DATA_HANDLER_LIST(R5, )
`;

const ppOut = pp(probe);
// ROW 由相邻字符串字面量拼接: ROW("3|" "A" "|" "B" ...) → 合并后按 | 切分
const rows = [...ppOut.matchAll(/ROW\(([^)]*)\)/g)].map((m) =>
  [...m[1].matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((x) => x[1]).join("").replaceAll("\\\"", "\"").split("|"),
);
if (rows.length === 0) throw new Error("clang 预处理未产出 ROW 行, 对数基准失效");

const take = (prefix) => rows.filter((r) => r[0] === prefix).map((r) => r.slice(1));
const stringRows = take("4");
const structRows = take("3");
const structMapRows = take("3m");
const siteRows = take("5").filter((r) => r[0].startsWith("ALLOCATION_SITE"));
const handlerRows = take("5").filter((r) => r[0].startsWith("LOAD_HANDLER") || r[0].startsWith("STORE_HANDLER"));

// 锚点对数
if (stringRows.length !== 24) throw new Error(`STRING_TYPE_LIST 基准条数不符: ${stringRows.length}, 期望 24`);
if (stringRows[0][0] !== "SHARED_SEQ_TWO_BYTE_STRING_TYPE") throw new Error("STRING_TYPE_LIST 首项锚点不符");
if (structRows.length !== 40) throw new Error(`STRUCT_LIST 基准条数不符: ${structRows.length}, 期望 40 (39 + IF_WASM)`);
if (structRows[0][1] !== "PromiseFulfillReactionJobTask") throw new Error("STRUCT_LIST 首项锚点不符");
if (structRows[39][1] !== "WasmExceptionTag") throw new Error("STRUCT_LIST 末项锚点不符 (IF_WASM 分支)");
if (structMapRows.length !== structRows.length) throw new Error(`STRUCT_MAPS_LIST 与 STRUCT_LIST 条数不齐: ${structMapRows.length}`);
if (siteRows.length !== 2) throw new Error(`ALLOCATION_SITE_LIST 基准条数不符: ${siteRows.length}`);
if (handlerRows.length !== 9) throw new Error(`DATA_HANDLER_LIST 基准条数不符: ${handlerRows.length}`);
if (handlerRows[0][1] !== "LoadHandler" || handlerRows[8][1] !== "StoreHandler") throw new Error("DATA_HANDLER_LIST 锚点不符");

console.log(`clang 基准: STRING ${stringRows.length}, STRUCT ${structRows.length}, SITE ${siteRows.length}, HANDLER ${handlerRows.length}`);

// —— Rust 生成 ——
let rust = banner(
  "src/objects/objects-definitions.h.js",
  "src/objects/objects-definitions.h",
  "X-macro 全量展开 (clang -E 对数, 配置 INTL+WASM, TEMPORAL/METAGEN 关) — INSTANCE_TYPE_LIST 归 instance_type.rs",
);

const rowsTable = (name, rows, doc) => {
  let s = `/// ${doc}\npub const ${name}: [&str; ${rows.length}] = [\n`;
  for (const r of rows) s += `  "${r.join("|")}",\n`;
  return s + `];\n\n`;
};

rust += "\n" + rowsTable(
  "STRING_TYPE_LIST",
  stringRows,
  "字符串类型表 (原版 STRING_TYPE_LIST, 列: 类型|大小表达式|根名|类名; 顺序关系只读堆布局)",
);
rust += rowsTable(
  "STRUCT_LIST",
  structRows,
  "Struct 族表 (原版 STRUCT_LIST, 列: 实例类型名|类名|字段名)",
);
rust += rowsTable(
  "STRUCT_MAPS_LIST",
  structMapRows.map((r) => [r[1], r[0]]),
  "Struct 族 Map 表 (原版 STRUCT_MAPS_LIST, 列: map 根名|Map 类名)",
);
rust += rowsTable(
  "ALLOCATION_SITE_LIST",
  siteRows,
  "AllocationSite 形态表 (原版 ALLOCATION_SITE_LIST, 列: 实例类型|类名|形态|根名)",
);
rust += rowsTable(
  "DATA_HANDLER_LIST",
  handlerRows,
  "DataHandler 形态表 (原版 DATA_HANDLER_LIST, 列: 实例类型|类名|变体序号|根名)",
);

rust += `// —— 提取不变式断言 ——
const _: () = {
  assert!(STRING_TYPE_LIST.len() == 24);
  assert!(STRING_TYPE_LIST[0].starts_with("SHARED_SEQ_TWO_BYTE_STRING_TYPE|"));
  assert!(STRUCT_LIST.len() == 40);
  assert!(STRUCT_MAPS_LIST.len() == 40);
  assert!(STRUCT_LIST[0].contains("PromiseFulfillReactionJobTask"));
  assert!(STRUCT_LIST[39].contains("WasmExceptionTag"));
  assert!(ALLOCATION_SITE_LIST.len() == 2);
  assert!(DATA_HANDLER_LIST.len() == 9);
};
`;

const p = writeSrc("v8-objects-core", "objects_definitions.rs", rust);
console.log(`written ${p}`);
