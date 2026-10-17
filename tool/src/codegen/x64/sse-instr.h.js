#!/usr/bin/env bun
// tool/src/codegen/x64/sse-instr.h.js
// 机械展开 v8/src/codegen/x64/sse-instr.h 全部 X-macro 指令表
// → v8-codegen-x64/src/sse_instr.rs

import { readV8, writeSrc, banner } from "../../../lib.js";

const src = readV8("src/codegen/x64/sse-instr.h");
const flat = src.replace(/\\\r?\n/g, " ");

// 提取宏体 (支持组合宏体里的子宏引用行)
function macroBody(name) {
  const re = new RegExp(`#define ${name}\\(V\\) ([\\s\\S]*?)(?=#define |#endif)`);
  const m = flat.match(re);
  if (!m) throw new Error(`未找到宏 ${name}`);
  return m[1];
}

// 展开宏体: V(name, b1, b2, ...) 条目; 组合宏体内嵌套子宏 (X(V) 行) 递归拼接
function expandList(name, depth = 0) {
  if (depth > 3) throw new Error(`${name} 组合过深`);
  const body = macroBody(name);
  const rows = [];
  let expanded = body;
  for (const sm of body.matchAll(/(\w+)\(V\)/g)) {
    const sub = sm[1];
    if (sub === "V") continue;
    const subRows = expandList(sub, depth + 1);
    const wrapped = subRows.map((r) => `V(${r[0]}, ${r[1].join(", ")})`).join(" \\\n  ");
    expanded = expanded.replace(`${sub}(V)`, wrapped);
  }
  for (const em of expanded.matchAll(/V\((\w+)((?:,\s*[\w\d]+)+)\)/g)) {
    const bytes = em[2]
      .split(",")
      .map((s) => s.trim())
      .filter(Boolean);
    rows.push([em[1], bytes]);
  }
  if (rows.length === 0) throw new Error(`${name} 展开为空`);
  return rows;
}

// 每表条数对数 (原版侧独立 grep 计数)
const EXPECT = {
  SSE_UNOP_INSTRUCTION_LIST: 5,
  SSE_BINOP_INSTRUCTION_LIST: 11,
  SSE_INSTRUCTION_LIST_SS: 8,
  SSE2_INSTRUCTION_LIST_PD: 10,
  SSE2_INSTRUCTION_LIST_PI: 48,
  SSE2_INSTRUCTION_LIST_SHIFT: 8,
  SSE2_UNOP_INSTRUCTION_LIST: 5,
  SSE2_INSTRUCTION_LIST_SHIFT_IMM: 8,
  SSE2_INSTRUCTION_LIST_SD: 8,
  SSSE3_INSTRUCTION_LIST: 8,
  SSSE3_UNOP_INSTRUCTION_LIST: 3,
  SSE4_INSTRUCTION_LIST: 12,
  SSE4_UNOP_INSTRUCTION_LIST: 8, // ptest + PMOV 7
  SSE4_EXTRACT_INSTRUCTION_LIST: 4,
  SSE4_2_INSTRUCTION_LIST: 1,
  AVX2_BROADCAST_LIST: 4,
};

const lists = {};
for (const [name, count] of Object.entries(EXPECT)) {
  const rows = expandList(name);
  if (rows.length !== count) throw new Error(`${name} 条数 ${rows.length} != ${count}`);
  lists[name] = rows;
}
// 组合宏对数: SSE2_INSTRUCTION_LIST = PD + PI + SHIFT
const sse2 =
  lists.SSE2_INSTRUCTION_LIST_PD.length +
  lists.SSE2_INSTRUCTION_LIST_PI.length +
  lists.SSE2_INSTRUCTION_LIST_SHIFT.length;
const sse2Composite = expandList("SSE2_INSTRUCTION_LIST");
if (sse2Composite.length !== sse2) throw new Error("SSE2 组合条数不符");
lists.SSE2_INSTRUCTION_LIST = sse2Composite;

// 首尾抽查
if (lists.SSE_UNOP_INSTRUCTION_LIST[0][0] !== "sqrtps") throw new Error("SSE_UNOP 首项不符");
if (lists.SSE2_INSTRUCTION_LIST_PI.at(-1)[0] !== "paddd") throw new Error("SSE2_PI 末项不符");
if (lists.SSE4_INSTRUCTION_LIST[0][0] !== "pmuldq") throw new Error("SSE4 首项不符");
if (lists.AVX2_BROADCAST_LIST[0][0] !== "vpbroadcastb") throw new Error("AVX2 首项不符");

function fmt(name, rows, note) {
  const body = rows
    .map(([n, bytes]) => `  (${JSON.stringify(n)}, &[${bytes.map((b) => JSON.stringify(b)).join(", ")}]),`)
    .join("\n");
  return `/// ${note} (${rows.length} 条)\n#[rustfmt::skip]\npub const ${name}: &[(&str, &[&str])] = &[\n${body}\n];\n`;
}

let rust = banner(
  "src/codegen/x64/sse-instr.h.js",
  "src/codegen/x64/sse-instr.h",
  "SSE/AVX 指令编码表 (X-macro 机械展开)",
);

rust += `
// 二元组: (指令助记名, 操作码字节序列, 前 1-2 字节为前缀)。
// 操作码大小写保持原版 (hex 字面)。

${fmt("SSE_UNOP_INSTRUCTION_LIST", lists.SSE_UNOP_INSTRUCTION_LIST, "AVX 双操作数版 SSE 单目指令")}
${fmt("SSE_BINOP_INSTRUCTION_LIST", lists.SSE_BINOP_INSTRUCTION_LIST, "AVX 三操作数版 SSE 双目指令")}
${fmt("SSE_INSTRUCTION_LIST_SS", lists.SSE_INSTRUCTION_LIST_SS, "标量单精度指令")}
${fmt("SSE2_INSTRUCTION_LIST_PD", lists.SSE2_INSTRUCTION_LIST_PD, "SSE2 打包双精度")}
${fmt("SSE2_INSTRUCTION_LIST_PI", lists.SSE2_INSTRUCTION_LIST_PI, "SSE2 打包整数")}
${fmt("SSE2_INSTRUCTION_LIST_SHIFT", lists.SSE2_INSTRUCTION_LIST_SHIFT, "SSE2 移位 (XMM/m128 操作数)")}
${fmt("SSE2_INSTRUCTION_LIST", lists.SSE2_INSTRUCTION_LIST, "SSE2 组合表 = PD + PI + SHIFT")}
${fmt("SSE2_UNOP_INSTRUCTION_LIST", lists.SSE2_UNOP_INSTRUCTION_LIST, "AVX 双操作数版 SSE2 单目")}
${fmt("SSE2_INSTRUCTION_LIST_SHIFT_IMM", lists.SSE2_INSTRUCTION_LIST_SHIFT_IMM, "SSE2 立即数移位 (末位为 opcode 扩展)")}
${fmt("SSE2_INSTRUCTION_LIST_SD", lists.SSE2_INSTRUCTION_LIST_SD, "标量双精度指令")}
${fmt("SSSE3_INSTRUCTION_LIST", lists.SSSE3_INSTRUCTION_LIST, "SSSE3 指令")}
${fmt("SSSE3_UNOP_INSTRUCTION_LIST", lists.SSSE3_UNOP_INSTRUCTION_LIST, "AVX 双操作数版 SSSE3 单目")}
${fmt("SSE4_INSTRUCTION_LIST", lists.SSE4_INSTRUCTION_LIST, "SSE4 指令")}
${fmt("SSE4_UNOP_INSTRUCTION_LIST", lists.SSE4_UNOP_INSTRUCTION_LIST, "AVX 双操作数版 SSE4 单目 (ptest + PMOV 族)")}
${fmt("SSE4_EXTRACT_INSTRUCTION_LIST", lists.SSE4_EXTRACT_INSTRUCTION_LIST, "SSE4 抽取指令")}
${fmt("SSE4_2_INSTRUCTION_LIST", lists.SSE4_2_INSTRUCTION_LIST, "SSE4.2 指令")}
${fmt("AVX2_BROADCAST_LIST", lists.AVX2_BROADCAST_LIST, "AVX2 广播指令 (需 AVX2)")}

// —— 提取不变式断言 ——
const _: () = {
  assert!(SSE_UNOP_INSTRUCTION_LIST.len() == ${lists.SSE_UNOP_INSTRUCTION_LIST.length});
  assert!(SSE_BINOP_INSTRUCTION_LIST.len() == ${lists.SSE_BINOP_INSTRUCTION_LIST.length});
  assert!(SSE2_INSTRUCTION_LIST_PI.len() == ${lists.SSE2_INSTRUCTION_LIST_PI.length});
  assert!(SSE2_INSTRUCTION_LIST.len() == ${sse2Composite.length});
  assert!(SSE4_INSTRUCTION_LIST.len() == ${lists.SSE4_INSTRUCTION_LIST.length});
  assert!(SSE4_2_INSTRUCTION_LIST.len() == 1);
  assert!(SSE_UNOP_INSTRUCTION_LIST[0].0 == "sqrtps");
  assert!(SSE2_INSTRUCTION_LIST_PI.last().unwrap().0 == "paddd");
};
`;

const p = writeSrc("v8-codegen-x64", "sse_instr.rs", rust);
console.log(
  `sse-instr: ${Object.keys(lists).length} 表 / 共 ${Object.values(lists).reduce((a, b) => a + b.length, 0)} 条指令编码 → ${p}`,
);
