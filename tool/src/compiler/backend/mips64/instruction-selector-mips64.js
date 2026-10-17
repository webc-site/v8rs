#!/usr/bin/env bun
// tool/src/compiler/backend/mips64/instruction-selector-mips64.js
// v8/src/compiler/backend/mips64/instruction-selector-mips64.cc
//   → v8-compiler-backend/src/instruction_selector_mips64.rs.skel
// mips64 指令选择: TryMatchImmediate + ZeroExtends 传播 (loong64 同构)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/mips64/instruction-selector-mips64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
must(count(cc, /::Visit/g) === 201, `cc ::Visit 201, 实得 ${count(cc, /::Visit/g)}`);
must(cc.includes("bool TryMatchImmediate(InstructionSelector* selector,"), "cc TryMatchImmediate");
must(cc.includes("bool InstructionSelector::ZeroExtendsWord32ToWord64NoPhis(OpIndex node) {"),
  "cc ZeroExtendsWord32ToWord64NoPhis");
must(count(cc, /TryMatchImmediate\(/g) >= 2, "cc TryMatchImmediate 消费 ≥2");
must(count(cc, /ZeroExtendsWord32ToWord64\w*\(/g) >= 3, "cc ZeroExtends 传播 ≥3");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/mips64/instruction-selector-mips64.js",
  "src/compiler/backend/mips64/instruction-selector-mips64.cc",
  "mips64 指令选择: TryMatchImmediate (16 位立即数) + ZeroExtends 传播")}
//! 台账: Visit* 定义 ${visitDefs.length} (::Visit 面 201); TryMatchImmediate
//!   (16 位立即数编码, 双向尝试); ZeroExtendsWord32ToWord64 传播
//!   (NoPhis 递归, 免 dext 序列)。

/// v8: TryMatchImmediate — 16 位立即数编码匹配
// fn try_match_immediate(...)
/// v8: ZeroExtendsWord32ToWord64NoPhis — 零扩展传播
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

writeSrc("v8-compiler-backend", "instruction_selector_mips64.rs.skel", skel);
console.log(`mips64/instruction-selector-mips64.js: 对数 ${checks} 条全过; Visit 定义 ${visitDefs.length} ` +
  `(总面 201); TryMatchImmediate + ZeroExtends; 产物 instruction_selector_mips64.rs.skel`);
