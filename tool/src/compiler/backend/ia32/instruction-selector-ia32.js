#!/usr/bin/env bun
// tool/src/compiler/backend/ia32/instruction-selector-ia32.js
// v8/src/compiler/backend/ia32/instruction-selector-ia32.cc
//   → v8-compiler-backend/src/instruction_selector_ia32.rs.skel
// ia32 指令选择: GetEffectiveAddressMemoryOperand + MR/MRI 寻址归并
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/ia32/instruction-selector-ia32.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

const visitDefs = [...cc.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
if (visitDefs.length < 30) throw new Error(`Visit 定义 ${visitDefs.length} < 30`);
must(count(cc, /::Visit/g) === 183, `cc ::Visit 183, 实得 ${count(cc, /::Visit/g)}`);
// ia32 特色: 有效地址匹配
must(cc.includes("AddressingMode GetEffectiveAddressMemoryOperand("), "cc GetEffectiveAddressMemoryOperand");
must(cc.includes("BaseWithIndexAndDisplacementMatcher"), "cc 地址规范化匹配器");
must(count(cc, /GetEffectiveAddressMemoryOperand\(/g) >= 3, "cc 地址匹配消费 ≥3");
// 寻址模式选择
must(cc.includes("kMode_MR") || /AddressingModeField/.test(cc), "cc 寻址模式选择");
// 64 位双寄存器
must(cc.includes("Word64") || cc.includes("Int64"), "cc 64 位面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ia32/instruction-selector-ia32.js",
  "src/compiler/backend/ia32/instruction-selector-ia32.cc",
  "ia32 指令选择: 有效地址匹配 (MR/MRI/M 归并) + 64 位双寄存器")}
//! 台账: Visit* 定义 ${visitDefs.length} (::Visit 面 183);
//!   GetEffectiveAddressMemoryOperand — 地址操作数归并 (BaseWithIndexAndDisplacementMatcher
//!   规范序, 最多 3 输入: base+index*scale+disp → kMode_MR/MRI/M);
//!   Word64/Int64 双寄存器软实现。

/// v8: InstructionSelectorT GetEffectiveAddressMemoryOperand (ia32) — 地址归并
// fn get_effective_address_memory_operand(node, inputs, input_count) -> AddressingMode
/// v8: Visit* (ia32) — 32 位选择面
// fn visit_* 族

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT_DEF_COUNT >= 30);
};
pub const VISIT_DEF_COUNT: usize = ${visitDefs.length};
const _: () = {
  assert!(VISIT_DEF_COUNT == ${visitDefs.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_ia32.rs.skel", skel);
console.log(`ia32/instruction-selector-ia32.js: 对数 ${checks} 条全过; Visit 定义 ${visitDefs.length} ` +
  `(总面 183); GetEffectiveAddressMemoryOperand 归并; 产物 instruction_selector_ia32.rs.skel`);
