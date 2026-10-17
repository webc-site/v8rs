#!/usr/bin/env bun
// tool/src/compiler/backend/riscv/instruction-selector-riscv.js
// v8/src/compiler/backend/riscv/instruction-selector-riscv.{h,riscv32.cc,riscv64.cc}
//   → v8-compiler-backend/src/instruction_selector_riscv.rs.skel
// riscv 指令选择: RiscvOperandGenerator (CanBeImmediate) + 32/64 双变体族
import { readV8, writeSrc, banner } from "../../../../lib.js";

const h = readV8("src/compiler/backend/riscv/instruction-selector-riscv.h");
const cc32 = readV8("src/compiler/backend/riscv/instruction-selector-riscv32.cc");
const cc64 = readV8("src/compiler/backend/riscv/instruction-selector-riscv64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 生成器 + 公共面
must(h.includes("class RiscvOperandGenerator final : public OperandGenerator {"), "h RiscvOperandGenerator");
must(h.includes("InstructionOperand UseOperand(OpIndex node, InstructionCode opcode) {"), "h UseOperand");
must(h.includes("bool CanBeImmediate(OpIndex node, InstructionCode mode) {"), "h CanBeImmediate(node)");
must(h.includes("bool CanBeImmediate(int64_t value, InstructionCode opcode);"), "h CanBeImmediate(value) 声明");
must(h.includes("ReadOnlyRootPtr"), "h 只读根指针特化");
must(count(h, /::Visit/g) === 122, `h ::Visit 122, 实得 ${count(h, /::Visit/g)}`);
must(cc64.includes("bool InstructionSelector::ZeroExtendsWord32ToWord64NoPhis(OpIndex node) {"),
  "riscv64 ZeroExtends 传播");

// 32/64 双变体 (真计数)
const defs32 = [...cc32.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
const defs64 = [...cc64.matchAll(/^void InstructionSelector::Visit(\w+)\(/gm)].map((m) => m[1]);
must(defs32.length === 89, `riscv32 Visit 定义 ${defs32.length} != 89`);
must(defs64.length === 133, `riscv64 Visit 定义 ${defs64.length} != 133`);
// 64 侧独有 Word64 族
must(defs64.some((v) => v.startsWith("Word64")), "riscv64 Word64 族");
// 32 侧 Word64 软实现
must(defs32.includes("Word64ReverseBytes"), "riscv32 Word64 软实现锚");
must(cc64.includes("void InstructionSelector::VisitWord64And(OpIndex node) {"), "riscv64 VisitWord64And");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/riscv/instruction-selector-riscv.js",
  "src/compiler/backend/riscv/instruction-selector-riscv.h + instruction-selector-riscv32.cc + instruction-selector-riscv64.cc",
  "riscv 指令选择: OperandGenerator (CanBeImmediate) + riscv32/64 双变体")}
//! 台账: RiscvOperandGenerator (UseOperand → CanBeImmediate: 12 位立即数/
//!   只读根指针/RelocInfo 特化 + int64 重载); 公共 Visit 122 (h);
//!   riscv32 变体 89 定义 (Word64 软实现, ReverseBytes 锚); riscv64 变体
//!   133 定义 (Word64 And/Or/Xor/Shl 原生族); ZeroExtendsWord32ToWord64 传播。

/// v8: RiscvOperandGenerator::CanBeImmediate — 12 位/根指针/重定位特化
// fn can_be_immediate(value: i64, opcode: InstructionCode) -> bool
/// v8: riscv32 变体 — 32 位软实现族 (89 Visit)
// mod riscv32 { visit_* }
/// v8: riscv64 变体 — 原生 64 位族 (133 Visit)
// mod riscv64 { visit_word64_* }

// —— 提取不变式断言 ——
const _: () = {
  assert!(VISIT32_COUNT == 89);
  assert!(VISIT64_COUNT == 133);
};
pub const VISIT32_COUNT: usize = ${defs32.length};
pub const VISIT64_COUNT: usize = ${defs64.length};
const _: () = {
  assert!(VISIT32_COUNT == ${defs32.length});
  assert!(VISIT64_COUNT == ${defs64.length});
};
`;

writeSrc("v8-compiler-backend", "instruction_selector_riscv.rs.skel", skel);
console.log(`riscv/instruction-selector-riscv.js: 对数 ${checks} 条全过; 公共 122 + riscv32 89 + ` +
  `riscv64 133 双变体实测; CanBeImmediate 特化; 产物 instruction_selector_riscv.rs.skel`);
