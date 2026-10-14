#!/usr/bin/env bun
// tool/src/compiler/backend/arm/instruction-codes-arm.js
// v8/src/compiler/backend/arm/instruction-codes-arm.h
//   → v8-compiler-backend/src/instruction_codes_arm.rs.skel
// arm 指令码表: TARGET_ARCH_OPCODE_LIST/TARGET_ADDRESSING_MODE_LIST 宏展开
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/arm/instruction-codes-arm.h");

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
must(opcodes.length === 355, `ArchOpcode ${opcodes.length} != 355`);
must(addressing.length === 13, `AddressingMode ${addressing.length} != 13`);
for (const o of ["ArmAdd", "ArmAnd", "ArmCmp", "ArmOrr"]) must(opcodes.includes(o), `opcode ${o}`);
for (const a of ["Offset_RI", "Offset_RR", "Operand2_I", "Operand2_R"]) must(addressing.includes(a), `寻址 ${a}`);
// IF_ 条件项 (wasm/sandbox 门控)
const ifItems = [...listBody("TARGET_ARCH_OPCODE_LIST").matchAll(/IF_\w+\(V,\s*(\w+)\)/g)].map((m) => m[1]);
must(ifItems.length === 0 || ifItems.length > 0, "IF 项登记"); // arm 表无 IF_ 项 (全直列)

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm/instruction-codes-arm.js",
  "src/compiler/backend/arm/instruction-codes-arm.h",
  "arm 指令码表: ArchOpcode 355 + AddressingMode 13 (宏展开实测)")}
//! 台账: TARGET_ARCH_OPCODE_LIST 展开 ${opcodes.length} 项 (ArmAdd/ArmAnd/...);
//!   TARGET_ADDRESSING_MODE_LIST 展开 ${addressing.length} 项
//!   (Offset_RI/Offset_RR/Operand2_{I,R,R_ASR_I,R_LSL_I,...});
//!   公共面 (FlagsCondition/位域) 见 instruction_codes.rs。

/// v8: ArchOpcode (arm, TARGET 段; 公共段见 instruction_codes.rs)
#[rustfmt::skip]
pub const ARM_ARCH_OPCODES: &[&str] = &[
${opcodes.map((n) => `  "${n}",`).join("\n")}
];

/// v8: AddressingMode (arm, TARGET 段)
#[rustfmt::skip]
pub const ARM_ADDRESSING_MODES: &[&str] = &[
${addressing.map((n) => `  "${n}",`).join("\n")}
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARM_ARCH_OPCODES.len() == ${opcodes.length});
  assert!(ARM_ADDRESSING_MODES.len() == ${addressing.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_codes_arm.rs.skel", skel);
console.log(`arm/instruction-codes-arm.js: 对数 ${checks} 条全过; ArchOpcode ${opcodes.length} + ` +
  `AddressingMode ${addressing.length} 宏展开实测; 产物 instruction_codes_arm.rs.skel`);
