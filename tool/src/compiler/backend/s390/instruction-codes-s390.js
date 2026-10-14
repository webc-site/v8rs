#!/usr/bin/env bun
// tool/src/compiler/backend/s390/instruction-codes-s390.js
// v8/src/compiler/backend/s390/instruction-codes-s390.h
//   → v8-compiler-backend/src/instruction_codes_s390.rs.skel
// s390 指令码表: TARGET 宏展开 (5 寻址含 MRRI)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/s390/instruction-codes-s390.h");

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
must(opcodes.length === 390, `ArchOpcode ${opcodes.length} != 390`);
must(addressing.length === 5, `AddressingMode ${addressing.length} != 5`);
must(opcodes.includes("S390_Add32") || opcodes.some((o) => o.startsWith("S390")), "opcode S390_ 前缀");
must(addressing.join() === "MR,MRI,MRR,MRRI,Root", `寻址序 ${addressing.join()}`);

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/s390/instruction-codes-s390.js",
  "src/compiler/backend/s390/instruction-codes-s390.h",
  "s390 指令码表: ArchOpcode 390 + AddressingMode 5 (含 MRRI, 宏展开实测)")}
//! 台账: TARGET_ARCH_OPCODE_LIST 展开 ${opcodes.length} 项;
//!   TARGET_ADDRESSING_MODE_LIST ${addressing.length} 项 (MR/MRI/MRR/MRRI/Root);
//!   公共面见 instruction_codes.rs。

/// v8: ArchOpcode (s390, TARGET 段)
#[rustfmt::skip]
pub const S390_ARCH_OPCODES: &[&str] = &[
${opcodes.map((n) => `  "${n}",`).join("\n")}
];

/// v8: AddressingMode (s390, TARGET 段 — 含独有 MRRI)
#[rustfmt::skip]
pub const S390_ADDRESSING_MODES: &[&str] = &[
${addressing.map((n) => `  "${n}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(S390_ARCH_OPCODES.len() == ${opcodes.length});
  assert!(S390_ADDRESSING_MODES.len() == ${addressing.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_codes_s390.rs.skel", skel);
console.log(`s390/instruction-codes-s390.js: 对数 ${checks} 条全过; ArchOpcode ${opcodes.length} + ` +
  `AddressingMode ${addressing.length} (含 MRRI); 产物 instruction_codes_s390.rs.skel`);
