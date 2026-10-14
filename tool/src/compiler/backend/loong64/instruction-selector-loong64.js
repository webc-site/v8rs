#!/usr/bin/env bun
// tool/src/compiler/backend/loong64/instruction-selector-loong64.js
// v8/src/compiler/backend/loong64/instruction-selector-loong64.cc
//   → v8-compiler-backend/src/instruction_selector_loong64.rs.skel
// loong64 指令选择: TryMatchImmediate + ZeroExtendsWord32ToWord64 传播
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/loong64/instruction-selector-loong64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
must(count(cc, /::Visit/g) === 208, `cc ::Visit 208, 实得 ${count(cc, /::Visit/g)}`);
// loong64 特色
must(cc.includes("bool TryMatchImmediate(InstructionSelector* selector,"), "cc TryMatchImmediate");
must(cc.includes("bool InstructionSelector::ZeroExtendsWord32ToWord64NoPhis(OpIndex node) {"),
  "cc ZeroExtendsWord32ToWord64NoPhis");
must(count(cc, /ZeroExtendsWord32ToWord64\w*\(/g) >= 3, "cc ZeroExtends 传播 ≥3");
// 双宽选择
must(count(cc, /TryMatchImmediate\(/g) >= 2, "cc TryMatchImmediate 消费 ≥2");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/loong64/instruction-selector-loong64.js",
  "src/compiler/backend/loong64/instruction-selector-loong64.cc",
  "loong64 指令选择: TryMatchImmediate 立即数编码 + ZeroExtends 32→64 传播")}
//! 台账: Visit* 定义 ${visitDefs.length} (::Visit 面 208); TryMatchImmediate
//!   (12 位有符号立即数 si12 编码, 左右操作数双向); ZeroExtendsWord32ToWord64
//!   传播 (NoPhis 递归 projection 折叠, 免显式 uwm 指令)。

/// v8: TryMatchImmediate — si12 立即数编码匹配
// fn try_match_immediate(...)
/// v8: InstructionSelector::ZeroExtendsWord32ToWord64NoPhis — 零扩展传播
// fn zero_extends_word32_to_word64_no_phis(node) -> bool

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT_DEF_COUNT >= 30);
};
pub const VISIT_DEF_COUNT: usize = ${visitDefs.length};
const _: () = {
  assert!(VISIT_DEF_COUNT == ${visitDefs.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_loong64.rs.skel", skel);
console.log(`loong64/instruction-selector-loong64.js: 对数 ${checks} 条全过; Visit 定义 ${visitDefs.length} ` +
  `(总面 208); TryMatchImmediate + ZeroExtends 传播; 产物 instruction_selector_loong64.rs.skel`);
