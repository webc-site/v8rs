#!/usr/bin/env bun
// tool/src/compiler/backend/ppc/instruction-codes-ppc.js
// v8/src/compiler/backend/ppc/instruction-codes-ppc.h
//   → v8-compiler-backend/src/instruction_codes_ppc.rs.skel
// ppc 指令码表: TARGET 宏展开 (PPC_ 前缀 + 3 寻址)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/ppc/instruction-codes-ppc.h");

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
must(opcodes.length === 383, `ArchOpcode ${opcodes.length} != 383`);
must(addressing.length === 3, `AddressingMode ${addressing.length} != 3`);
must(opcodes.includes("PPC_Add32") && opcodes.includes("PPC_AbsDouble"), "opcode 抽查 PPC_ 前缀");
must(addressing.join() === "MRI,MRR,Root", `寻址序 ${addressing.join()}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ppc/instruction-codes-ppc.js",
  "src/compiler/backend/ppc/instruction-codes-ppc.h",
  "ppc 指令码表: ArchOpcode 383 (PPC_ 前缀) + AddressingMode 3 (宏展开实测)")}
//! 台账: TARGET_ARCH_OPCODE_LIST 展开 ${opcodes.length} 项 (PPC_Add32/PPC_AbsDouble/...);
//!   TARGET_ADDRESSING_MODE_LIST ${addressing.length} 项 (MRI/MRR/Root);
//!   公共面见 instruction_codes.rs。

/// v8: ArchOpcode (ppc, TARGET 段)
#[rustfmt::skip]
pub const PPC_ARCH_OPCODES: &[&str] = &[
${opcodes.map((n) => `  "${n}",`).join("\n")}
];

/// v8: AddressingMode (ppc, TARGET 段)
#[rustfmt::skip]
pub const PPC_ADDRESSING_MODES: &[&str] = &[
${addressing.map((n) => `  "${n}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(PPC_ARCH_OPCODES.len() == ${opcodes.length});
  assert!(PPC_ADDRESSING_MODES.len() == ${addressing.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_codes_ppc.rs.skel", skel);
console.log(`ppc/instruction-codes-ppc.js: 对数 ${checks} 条全过; ArchOpcode ${opcodes.length} + ` +
  `AddressingMode ${addressing.length} 宏展开实测; 产物 instruction_codes_ppc.rs.skel`);
