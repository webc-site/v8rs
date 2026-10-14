#!/usr/bin/env bun
// tool/src/codegen/x64/fma-instr.h.js
// 机械展开 v8/src/codegen/x64/fma-instr.h FMA 指令表
// → v8-codegen-x64/src/fma_instr.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/x64/fma-instr.h");
const flat = src.replace(/\\\r?\n/g, " ");

function expandList(name, depth = 0) {
  if (depth > 3) throw new Error(`${name} 组合过深`);
  const re = new RegExp(`#define ${name}\\(V\\) ([\\s\\S]*?)(?=#define |#endif)`);
  const m = flat.match(re);
  if (!m) throw new Error(`未找到宏 ${name}`);
  let expanded = m[1];
  // 组合宏: 子宏 X(V) 行递归展开
  for (const sm of m[1].matchAll(/(\w+)\(V\)/g)) {
    if (sm[1] === "V") continue;
    const sub = expandList(sm[1], depth + 1);
    expanded = expanded.replace(`${sm[1]}(V)`, sub.map((r) => `V(${r[0]}, ${r[1].join(", ")})`).join(" \\\n  "));
  }
  const rows = [];
  for (const em of expanded.matchAll(/V\((\w+)((?:,\s*[\w\d]+)+)\)/g)) {
    rows.push([
      em[1],
      em[2]
        .split(",")
        .map((s) => s.trim())
        .filter(Boolean),
    ]);
  }
  return rows;
}

// 对数 (原版侧逐项): SD 12 / SS 12 / PS 6 / PD 6, 组合 = 36
const fmaSd = expandList("FMA_SD_INSTRUCTION_LIST");
const fmaSs = expandList("FMA_SS_INSTRUCTION_LIST");
const fmaPs = expandList("FMA_PS_INSTRUCTION_LIST");
const fmaPd = expandList("FMA_PD_INSTRUCTION_LIST");
for (const [rows, want, nm] of [[fmaSd, 12, "SD"], [fmaSs, 12, "SS"], [fmaPs, 6, "PS"], [fmaPd, 6, "PD"]]) {
  if (rows.length !== want) throw new Error(`FMA_${nm} ${rows.length} != ${want}`);
}
const fmaComposite = expandList("FMA_INSTRUCTION_LIST");
if (fmaComposite.length !== 36) throw new Error(`FMA 组合 ${fmaComposite.length} != 36`);

function fmt(name, rows, note) {
  const body = rows
    .map(([n, bytes]) => `  (${JSON.stringify(n)}, &[${bytes.map((b) => JSON.stringify(b)).join(", ")}]),`)
    .join("\n");
  return `/// ${note} (${rows.length} 条)\n#[rustfmt::skip]\npub const ${name}: &[(&str, &[&str])] = &[\n${body}\n];\n`;
}

let rust = banner(
  "src/codegen/x64/fma-instr.h.js",
  "src/codegen/x64/fma-instr.h",
  "FMA 指令编码表 (X-macro 机械展开)",
);

rust += `
// 二元组: (指令助记名, 编码序列: 66,0F,38,W标,opcode)。
// W0/W1 为 VEX.W 位标。

${fmt("FMA_SD_INSTRUCTION_LIST", fmaSd, "FMA 标量双精度")}
${fmt("FMA_SS_INSTRUCTION_LIST", fmaSs, "FMA 标量单精度")}
${fmt("FMA_PS_INSTRUCTION_LIST", fmaPs, "FMA 打包单精度")}
${fmt("FMA_PD_INSTRUCTION_LIST", fmaPd, "FMA 打包双精度")}
${fmt("FMA_INSTRUCTION_LIST", fmaComposite, "FMA 组合表 = SD + SS + PS + PD")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(FMA_SD_INSTRUCTION_LIST.len() == 12);
  assert!(FMA_SS_INSTRUCTION_LIST.len() == 12);
  assert!(FMA_PS_INSTRUCTION_LIST.len() == 6);
  assert!(FMA_PD_INSTRUCTION_LIST.len() == 6);
  assert!(FMA_INSTRUCTION_LIST.len() == 36);
  assert!(FMA_SD_INSTRUCTION_LIST[0].0 == "vfmadd132sd");
  assert!(FMA_INSTRUCTION_LIST.last().unwrap().0 == "vfnmsub231pd");
};
`;

const p = writeSrc("v8-codegen-x64", "fma_instr.rs", rust);
console.log(`fma-instr: 4 子表 + 1 组合 / 36 条 → ${p}`);
