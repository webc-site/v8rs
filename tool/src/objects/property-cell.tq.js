#!/usr/bin/env bun
// tool/src/objects/property-cell.tq.js — 覆盖 v8/src/objects/property-cell.tq (单文件族)
// → v8-objects-maps/src/property_cell_tq.rs
// 机械提取: PropertyCell tq 布局字段表 + 压缩配置偏移
// 对数 oracle: clang -E 预处理 v8/src/objects/property-cell.h
//   (C++ TaggedMember 声明序 1:1; 压缩指针 K_TAGGED_SIZE=4)

import { execFileSync } from "node:child_process";
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { tmpdir } from "node:os";
import { readV8, writeSrc, banner, root } from "../../lib.js";

const tq = readV8("src/objects/property-cell.tq");

let checks = 0;
const must = (c, m) => {
  checks++;
  if (!c) throw new Error(`锚点失配: ${m}`);
};

// ── clang -E 对数基准 (原版侧 = 编译器预处理产物; 缺失头自动空 stub) ──────
const ppShim = join(tmpdir(), "v8rs-r3o4-pp-shim");
function clangE(rel) {
  const args = [
    "-E", "-P", "-x", "c++", "-std=c++20", "-I", "v8", "-I", "v8/include",
    "-I", ppShim, "-DV8_INTL_SUPPORT=1", "-DV8_ENABLE_WEBASSEMBLY=1", "-",
  ];
  for (let round = 0; round < 48; round++) {
    try {
      return execFileSync("clang", args, {
        input: `#include "${rel}"\n`,
        encoding: "utf8",
        cwd: root,
        maxBuffer: 512 * 1024 * 1024,
      });
    } catch (e) {
      const err = String(e.stderr ?? "");
      const m = err.match(/fatal error: '([^']+)' file not found/);
      if (!m) throw new Error(`clang -E ${rel} 失败: ${err.slice(0, 300)}`);
      const stub = join(ppShim, m[1]);
      mkdirSync(dirname(stub), { recursive: true });
      writeFileSync(stub, "#pragma once\n");
    }
  }
  throw new Error(`clang -E ${rel}: stub 轮数超限`);
}

// ── 产物 fmt 归一 (与 cargo +nightly fmt 同配置; 幂等) ────────────────────
function fmtNormalize(p) {
  const rustfmt = execFileSync("rustup", ["which", "--toolchain", "nightly", "rustfmt"],
    { encoding: "utf8", cwd: root }).trim();
  execFileSync(rustfmt, ["--edition", "2024", "--config-path", join(root, "rustfmt.toml"), p],
    { cwd: root });
}


const hpp = clangE("src/objects/property-cell.h");

// ── tq 布局字段 ───────────────────────────────────────────────────────────
const cls = tq.match(/extern class PropertyCell extends HeapObject \{([\s\S]*?)\n\}/);
if (!cls) throw new Error("未找到 tq PropertyCell");
const fields = [...cls[1].matchAll(/^ {2}(\w+): ([^;]+);$/gm)].map((m) => [m[1], m[2].trim()]);
must(JSON.stringify(fields) === JSON.stringify([
  ["name", "Name"],
  ["property_details_raw", "Smi"],
  ["value", "Object"],
  ["dependent_code", "DependentCode"],
]), `tq 字段失配: ${JSON.stringify(fields)}`);

// ── pp 侧 TaggedMember 序对数 ────────────────────────────────────────────
const cellSeg = hpp.slice(hpp.indexOf("class PropertyCell : public HeapObject"));
const cellBody = cellSeg.slice(0, cellSeg.indexOf("V8_OBJECT_END"));
const members = [...cellBody.matchAll(/TaggedMember<([\w<>, ]+)> (\w+);/g)]
  .map((m) => [m[2], m[1].replace(/\s+/g, " ")]);
must(members.length === 4, `pp 成员数 ${members.length} != 4`);
const memberNames = members.map((x) => x[0]);
// tq name ↔ C++ name_ (去尾下划线)
for (let i = 0; i < 4; i++) {
  must(fields[i][0] + "_" === memberNames[i], `tq/pp 名失配 @${i}: ${fields[i][0]} vs ${memberNames[i]}`);
}
must(members[0][1] === "Name" && members[1][1] === "Smi" && members[2][1] === "Object" &&
  members[3][1] === "DependentCode", `pp 成员类型失配: ${JSON.stringify(members)}`);

// ── 压缩配置偏移 (K_TAGGED_SIZE=4; HeapObject 头 = map word) ─────────────
const T = 4;
const offsets = fields.map((_, i) => T + i * T); // 4/8/12/16
const K_HEADER_SIZE = T + fields.length * T; // 20
must(K_HEADER_SIZE === 20, `头尺寸推导 ${K_HEADER_SIZE} != 20`);
// PropertyCellType / kInvalidType 等语义面归 property-cell.h (v8-objects-rest
// 的 property-cell.h.js 已覆盖); 本脚本只落 tq 布局。

let rust = banner("src/objects/property-cell.tq.js", "src/objects/property-cell.tq",
  "PropertyCell tq 布局 (clang -E property-cell.h 对数, 压缩配置)");
rust += `//! PropertyCell 的 Torque 侧布局镜像: 4 字段表 + 偏移 (压缩 K_TAGGED_SIZE=4)。
//! 语义面 (PropertyCellType 状态机/依赖代码失效) 归 property-cell.h 侧脚本。

/// tq extern class PropertyCell 字段表 (顺序 1:1)。
pub const PROPERTY_CELL_FIELDS: [(&str, &str); ${fields.length}] = [
${fields.map(([n, t]) => `  ("${n}", "${t}"),`).join("\n")}
];

/// name 偏移 (tagged)。
pub const K_NAME_OFFSET: usize = ${offsets[0]};
/// property_details_raw 偏移 (tagged Smi)。
pub const K_PROPERTY_DETAILS_RAW_OFFSET: usize = ${offsets[1]};
/// value 偏移 (tagged)。
pub const K_VALUE_OFFSET: usize = ${offsets[2]};
/// dependent_code 偏移 (tagged)。
pub const K_DEPENDENT_CODE_OFFSET: usize = ${offsets[3]};
/// 对象头尺寸 (字段后第一个字节)。
pub const K_HEADER_SIZE: usize = ${K_HEADER_SIZE};

// —— 提取不变式断言 ——
const _: () = {
  assert!(PROPERTY_CELL_FIELDS.len() == 4);
  assert!((K_NAME_OFFSET == 4) && (K_VALUE_OFFSET == 12) && (K_DEPENDENT_CODE_OFFSET == 16));
  assert!(K_HEADER_SIZE == 20);
};
`;

const p = writeSrc("v8-objects-maps", "property_cell_tq.rs", rust);
fmtNormalize(p);
console.log(`property_cell_tq.rs: ${fields.length} 字段 (pp TaggedMember 序 1:1), 头尺寸 ${K_HEADER_SIZE}, 断言 ${checks} 项通过 → ${p}`);
