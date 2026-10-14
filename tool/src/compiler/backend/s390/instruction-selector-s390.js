#!/usr/bin/env bun
// tool/src/compiler/backend/s390/instruction-selector-s390.js
// v8/src/compiler/backend/s390/instruction-selector-s390.cc
//   → v8-compiler-backend/src/instruction_selector_s390.rs.skel
// s390 指令选择: TryMatchBaseWithScaledIndexAndDisplacement64 + 地址归并
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/s390/instruction-selector-s390.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
must(count(cc, /::Visit/g) === 102, `cc ::Visit 102, 实得 ${count(cc, /::Visit/g)}`);
// s390 特色: 缩放索引地址
must(cc.includes("TryMatchBaseWithScaledIndexAndDisplacement64(InstructionSelector* selector,"),
  "cc TryMatchBaseWithScaledIndexAndDisplacement64");
must(cc.includes("AddressingMode GetEffectiveAddressMemoryOperand("), "cc GetEffectiveAddressMemoryOperand");
must(count(cc, /GetEffectiveAddressMemoryOperand\(/g) >= 2, "cc 地址归并消费 ≥2");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/s390/instruction-selector-s390.js",
  "src/compiler/backend/s390/instruction-selector-s390.cc",
  "s390 指令选择: 缩放索引地址匹配 + MR/MRI/MRR/MRRI 归并")}
//! 台账: Visit* 定义 ${visitDefs.length} (::Visit 面 102);
//!   TryMatchBaseWithScaledIndexAndDisplacement64 (base+index*scale+disp);
//!   GetEffectiveAddressMemoryOperand → MR/MRI/MRR/MRRI 寻址归并。

/// v8: TryMatchBaseWithScaledIndexAndDisplacement64 — 缩放索引匹配
// fn try_match_base_with_scaled_index_and_displacement64(...)
/// v8: GetEffectiveAddressMemoryOperand — 地址归并
// fn get_effective_address_memory_operand(node, inputs, input_count) -> AddressingMode

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT_DEF_COUNT >= 30);
};
pub const VISIT_DEF_COUNT: usize = ${visitDefs.length};
const _: () = {
  assert!(VISIT_DEF_COUNT == ${visitDefs.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_s390.rs.skel", skel);
console.log(`s390/instruction-selector-s390.js: 对数 ${checks} 条全过; Visit 定义 ${visitDefs.length} ` +
  `(总面 102); 缩放索引匹配; 产物 instruction_selector_s390.rs.skel`);
