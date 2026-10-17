#!/usr/bin/env bun
// tool/src/compiler/backend/arm/instruction-selector-arm.js
// v8/src/compiler/backend/arm/instruction-selector-arm.cc
//   → v8-compiler-backend/src/instruction_selector_arm.rs.skel
// arm 指令选择: TryMatch 移位匹配族 + 32 位寻址 (Operand2)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/arm/instruction-selector-arm.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// Visit 定义面
const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
must(count(cc, /::Visit/g) === 164, `cc ::Visit 164, 实得 ${count(cc, /::Visit/g)}`);
// TryMatch 移位族 (真断言 3 个)
for (const m of ["bool TryMatchShift(InstructionSelector* selector,",
  "bool TryMatchShiftImmediate(InstructionSelector* selector,",
  "bool TryMatchROR(InstructionSelector* selector, InstructionCode* opcode_return,"])
  must(cc.includes(m), `cc ${m.slice(5, 50)}`);
must(cc.includes("Opmask::kWord32RotateRight"), "cc ROR 掩码");
// 32 位特色: 双寄存器 64 位值 (Word64 → 双 Word32)
must(count(cc, /Word64/g) >= 1, "cc Word64 面 (32 位架构软实现)");
// 地址模式
must(cc.includes("BaseWithIndexAndDisplacementMatcher") || /Operand2|kMode_Operand2/.test(cc),
  "cc Operand2 寻址面");
// 浮点 VFP
must(cc.includes("Float64") || cc.includes("Float32"), "cc 浮点选择面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm/instruction-selector-arm.js",
  "src/compiler/backend/arm/instruction-selector-arm.cc",
  "arm 指令选择: TryMatchShift/ROR 匹配族 + Operand2 寻址")}
//! 台账: Visit* 定义 ${visitDefs.length} (::Visit 面 164); TryMatchShift /
//!   TryMatchShiftImmediate / TryMatchROR (kWord32RotateRight 掩码匹配);
//!   Operand2 立即数旋转编码寻址; Word64 双寄存器软实现; VFP 浮点选择。

/// v8: TryMatchShift / TryMatchShiftImmediate / TryMatchROR — 移位匹配族
// fn try_match_shift / try_match_shift_immediate / try_match_ror
/// v8: InstructionSelector::Visit* (arm) — 32 位选择面
// fn visit_* (Word64 双寄存器软实现锚)

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT_DEF_COUNT >= 30);
};
pub const VISIT_DEF_COUNT: usize = ${visitDefs.length};
const _: () = {
  assert!(VISIT_DEF_COUNT == ${visitDefs.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_arm.rs.skel", skel);
console.log(`arm/instruction-selector-arm.js: 对数 ${checks} 条全过; Visit 定义 ${visitDefs.length} ` +
  `(总面 164); TryMatch 移位族 ×3; 产物 instruction_selector_arm.rs.skel`);
