#!/usr/bin/env bun
// tool/src/compiler/backend/riscv/instruction-codes-riscv.js
// v8/src/compiler/backend/riscv/instruction-codes-riscv.h
//   → v8-compiler-backend/src/instruction_codes_riscv.rs.skel
// riscv 指令码表: 嵌套宏递归展开 (COMMON+SPECIAL(mem+直列)+ZBA+ZBB+ZBB_32)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/riscv/instruction-codes-riscv.h");

// ── 宏展开 (真解析, 嵌套递归) ────────────────────────────────────────────────
const macroBody = (name) => {
  // riscv 有条件双定义 (RISCV64/RISCV32 分支取 64 侧 = 第一处)
  const defs = [...h.matchAll(new RegExp(`#define ${name}\\(V\\)[ \\t]*\\\\\\n((?:[^\\n]*\\\\\\n)*[^\\n]*)`, "g"))];
  if (defs.length === 0) throw new Error(`未找到宏 ${name}`);
  const withItems = defs.filter((d) => /V\(\w+\)/.test(d[1]) || /_LIST\(V\)/.test(d[1]));
  return (withItems[0] ?? defs[0])[1];
};
const IF_FLAGS = { IF_WASM: true, IF_SIMD128: true, IF_SANDBOX: true };
function expandList(name, depth = 0) {
  if (depth > 6) throw new Error(`宏递归过深: ${name}`);
  const items = [];
  const body = macroBody(name);
  for (const m of body.matchAll(/^\s*V\((\w+)\)/gm)) items.push(m[1]);
  for (const m of body.matchAll(/IF_(\w+)\(V,\s*(\w+)\)/g)) {
    const f = IF_FLAGS[`IF_${m[1]}`];
    if (f === undefined) throw new Error(`未登记 IF_${m[1]}`);
    if (f) items.push(m[2]);
  }
  for (const m of body.matchAll(/^\s*(\w+_LIST\w*)\(V\)/gm)) {
    if (m[1] === name) continue;
    items.push(...expandList(m[1], depth + 1));
  }
  return items;
}
const opcodes = expandList("TARGET_ARCH_OPCODE_LIST");
const memOpcodes = (() => {
  try { return expandList("TARGET_ARCH_OPCODE_WITH_MEMORY_ACCESS_MODE_LIST"); } catch { return []; }
})();
const addressing = (() => {
  const body = macroBody("TARGET_ADDRESSING_MODE_LIST");
  return [...body.matchAll(/^\s*V\((\w+)\)/gm)].map((m) => m[1]);
})();

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
must(opcodes.length === 321, `ArchOpcode ${opcodes.length} != 321 (RISCV64 侧)`);
must(addressing.length === 3, `AddressingMode ${addressing.length} != 3`);
must(addressing.join() === "MRI,MRR,Root", `寻址序 ${addressing.join()}`);
must(opcodes.includes("RiscvAdd32") && opcodes.includes("RiscvAdd64"), "opcode Add32/Add64");
// memList 嵌入主表 (SPECIAL 引用)
if (memOpcodes.length > 0) {
  for (const o of memOpcodes) if (!opcodes.includes(o)) throw new Error(`memList 项 ${o} 未入主表`);
  checks++;
  must(memOpcodes.length >= 20, `memList ${memOpcodes.length} ≥20`);
}
must(opcodes.some((o) => o.startsWith("RiscvRvv")), "RVV 向量 opcode");
// 无重复
must(new Set(opcodes).size === opcodes.length, "主表无重复");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/riscv/instruction-codes-riscv.js",
  "src/compiler/backend/riscv/instruction-codes-riscv.h",
  "riscv 指令码表: 嵌套宏递归展开 (RISCV64 侧: COMMON+SPECIAL+ZBA+ZBB+ZBB_32)")}
//! 台账: TARGET_ARCH_OPCODE_LIST 递归展开 ${opcodes.length} 项 (RISCV64 侧; 32 侧 SPECIAL 另表)
//!   (SPECIAL 含 memory-access 嵌套 + RISCV64/32 双分支取 64 侧;
//!   ZBA/ZBB 位操作扩展; RVV 向量段); 寻址 ${addressing.length} 项 (MRI/MRR/Root);
//!   公共面见 instruction_codes.rs。

/// v8: ArchOpcode (riscv, TARGET 段递归展开, RISCV64 侧)
#[rustfmt::skip]
pub const RISCV_ARCH_OPCODES: &[&str] = &[
${opcodes.map((n) => `  "${n}",`).join("\n")}
];

/// v8: AddressingMode (riscv, TARGET 段)
#[rustfmt::skip]
pub const RISCV_ADDRESSING_MODES: &[&str] = &[
${addressing.map((n) => `  "${n}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(RISCV_ARCH_OPCODES.len() == ${opcodes.length});
  assert!(RISCV_ADDRESSING_MODES.len() == ${addressing.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_codes_riscv.rs.skel", skel);
console.log(`riscv/instruction-codes-riscv.js: 对数 ${checks} 条全过; ArchOpcode ${opcodes.length} (RISCV64) ` +
  `(递归展开, memList ${memOpcodes.length} 嵌入) + AddressingMode ${addressing.length}; 产物 instruction_codes_riscv.rs.skel`);
