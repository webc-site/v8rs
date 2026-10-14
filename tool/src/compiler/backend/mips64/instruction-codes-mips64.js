#!/usr/bin/env bun
// tool/src/compiler/backend/mips64/instruction-codes-mips64.js
// v8/src/compiler/backend/mips64/instruction-codes-mips64.h
//   → v8-compiler-backend/src/instruction_codes_mips64.rs.skel
// mips64 指令码表: TARGET 宏展开 (主表 + memory-access 段独立)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/mips64/instruction-codes-mips64.h");

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
const memOpcodes = []; // mips64 无独立 memory-access 宏 (全在主表)
const addressing = expand("TARGET_ADDRESSING_MODE_LIST");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
must(opcodes.length + memOpcodes.length === 392,
  `ArchOpcode 总 ${opcodes.length}+${memOpcodes.length} != 392`);
must(addressing.length === 3, `AddressingMode ${addressing.length} != 3`);
must(opcodes.includes("Mips64Add"), "opcode Mips64Add");
must(addressing.join() === "MRI,MRR,Root", `寻址序 ${addressing.join()}`);
// 两表独立无重叠
const all = [...new Set([...opcodes, ...memOpcodes])];
must(all.length === opcodes.length, "主表无重复");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/mips64/instruction-codes-mips64.js",
  "src/compiler/backend/mips64/instruction-codes-mips64.h",
  "mips64 指令码表: ArchOpcode 392 (主+memAccess 两段) + AddressingMode 3 (宏展开实测)")}
//! 台账: TARGET_ARCH_OPCODE_LIST ${opcodes.length} 项 (无独立 memory-access 段);
//!   TARGET_ADDRESSING_MODE_LIST
//!   ${addressing.length} 项 (MRI/MRR/Root); 公共面见 instruction_codes.rs。

/// v8: ArchOpcode (mips64, TARGET 段两表并集)
#[rustfmt::skip]
pub const MIPS64_ARCH_OPCODES: &[&str] = &[
${all.map((n) => `  "${n}",`).join("\n")}
];

/// v8: AddressingMode (mips64, TARGET 段)
#[rustfmt::skip]
pub const MIPS64_ADDRESSING_MODES: &[&str] = &[
${addressing.map((n) => `  "${n}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(MIPS64_ARCH_OPCODES.len() == ${all.length});
  assert!(MIPS64_ADDRESSING_MODES.len() == ${addressing.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_codes_mips64.rs.skel", skel);
console.log(`mips64/instruction-codes-mips64.js: 对数 ${checks} 条全过; ArchOpcode 主 ${opcodes.length} + ` +
  `memAccess ${memOpcodes.length} (总 392) + AddressingMode ${addressing.length}; 产物 instruction_codes_mips64.rs.skel`);
