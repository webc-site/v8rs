#!/usr/bin/env bun
// tool/src/compiler/backend/ia32/instruction-codes-ia32.js
// v8/src/compiler/backend/ia32/instruction-codes-ia32.h
//   → v8-compiler-backend/src/instruction_codes_ia32.rs.skel
// ia32 指令码表: TARGET 宏展开 (ArchOpcode + AddressingMode)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/ia32/instruction-codes-ia32.h");

// ── 宏展开 (真解析) ─────────────────────────────────────────────────────────
const listBody = (name) => {
  const m = h.match(new RegExp(`#define ${name}\\(V\\)[ \\t]*\\\\\\n((?:[^\\n]*\\\\\\n)*[^\\n]*)`));
  if (!m) throw new Error(`未找到宏 ${name}`);
  return m[1];
};
const expand = (name) => {
  const items = [];
  for (const m of listBody(name).matchAll(/^\s*V\((\w+)\)/gm)) items.push(m[1]);
  if (items.length === 0) throw new Error(`${name} 展开为空`);
  return items;
};
const opcodes = expand("TARGET_ARCH_OPCODE_LIST");
const addressing = expand("TARGET_ADDRESSING_MODE_LIST");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
must(opcodes.length === 361, `ArchOpcode ${opcodes.length} != 361`);
must(addressing.length === 20, `AddressingMode ${addressing.length} != 20`);
for (const o of ["IA32Add", "IA32And", "IA32Cmp"]) must(opcodes.includes(o), `opcode ${o}`);
for (const a of ["MR", "MRI", "M1", "M8", "Root"]) must(addressing.includes(a), `寻址 ${a}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ia32/instruction-codes-ia32.js",
  "src/compiler/backend/ia32/instruction-codes-ia32.h",
  "ia32 指令码表: ArchOpcode 361 + AddressingMode 20 (宏展开实测)")}
//! 台账: TARGET_ARCH_OPCODE_LIST 展开 ${opcodes.length} 项 (IA32Add/...);
//!   TARGET_ADDRESSING_MODE_LIST 展开 ${addressing.length} 项 (MR/MR1/MRI/...);
//!   公共面见 instruction_codes.rs。

/// v8: ArchOpcode (ia32, TARGET 段)
#[rustfmt::skip]
pub const IA32_ARCH_OPCODES: &[&str] = &[
${opcodes.map((n) => `  "${n}",`).join("\n")}
];

/// v8: AddressingMode (ia32, TARGET 段)
#[rustfmt::skip]
pub const IA32_ADDRESSING_MODES: &[&str] = &[
${addressing.map((n) => `  "${n}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(IA32_ARCH_OPCODES.len() == ${opcodes.length});
  assert!(IA32_ADDRESSING_MODES.len() == ${addressing.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_codes_ia32.rs.skel", skel);
console.log(`ia32/instruction-codes-ia32.js: 对数 ${checks} 条全过; ArchOpcode ${opcodes.length} + ` +
  `AddressingMode ${addressing.length} 宏展开实测; 产物 instruction_codes_ia32.rs.skel`);
