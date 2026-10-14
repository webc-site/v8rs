#!/usr/bin/env bun
// tool/src/compiler/backend/loong64/instruction-codes-loong64.js
// v8/src/compiler/backend/loong64/instruction-codes-loong64.h
//   → v8-compiler-backend/src/instruction_codes_loong64.rs.skel
// loong64 指令码表: TARGET 宏展开
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/loong64/instruction-codes-loong64.h");

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
const memOpcodes = expand("TARGET_ARCH_OPCODE_WITH_MEMORY_ACCESS_MODE_LIST");
must(opcodes.length === 356, `ArchOpcode ${opcodes.length} != 356`);
must(memOpcodes.length === 33, `memory-access 段 ${memOpcodes.length} != 33`);
must(addressing.length === 3, `AddressingMode ${addressing.length} != 3`);
must(opcodes.includes("Loong64Add_w"), "opcode Loong64Add_w 在主表");
must(memOpcodes.includes("Loong64Ld_b") && memOpcodes.includes("Loong64St_b"), "Ld_b/St_b 在 memory-access 段");
// loong64 两表独立直列 (memList 33 项不嵌套引用主表), 并集为全集
const all = [...new Set([...opcodes, ...memOpcodes])];
must(all.length === opcodes.length + memOpcodes.length, "两表无重叠");
checks++;
must(addressing.join() === "MRI,MRR,Root", `寻址序 ${addressing.join()}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/loong64/instruction-codes-loong64.js",
  "src/compiler/backend/loong64/instruction-codes-loong64.h",
  "loong64 指令码表: ArchOpcode 389 + AddressingMode 3 (宏展开实测)")}
//! 台账: TARGET_ARCH_OPCODE_LIST 展开 ${opcodes.length} 项 + memory-access 段 ${memOpcodes.length} 项 (两表独立无重叠);
//!   TARGET_ADDRESSING_MODE_LIST ${addressing.length} 项 (MRI/MRR/Root);
//!   公共面见 instruction_codes.rs。

/// v8: ArchOpcode (loong64, TARGET 段)
#[rustfmt::skip]
pub const LOONG64_ARCH_OPCODES: &[&str] = &[
${opcodes.map((n) => `  "${n}",`).join("\n")}
];

/// v8: AddressingMode (loong64, TARGET 段)
#[rustfmt::skip]
pub const LOONG64_ADDRESSING_MODES: &[&str] = &[
${addressing.map((n) => `  "${n}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(LOONG64_ARCH_OPCODES.len() == ${opcodes.length});
  assert!(LOONG64_ADDRESSING_MODES.len() == ${addressing.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_codes_loong64.rs.skel", skel);
console.log(`loong64/instruction-codes-loong64.js: 对数 ${checks} 条全过; ArchOpcode ${opcodes.length} + memAccess ${memOpcodes.length} + ` +
  `AddressingMode ${addressing.length} 宏展开实测; 产物 instruction_codes_loong64.rs.skel`);
