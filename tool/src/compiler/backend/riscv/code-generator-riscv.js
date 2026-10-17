#!/usr/bin/env bun
// tool/src/compiler/backend/riscv/code-generator-riscv.js
// v8/src/compiler/backend/riscv/code-generator-riscv.cc
//   → v8-compiler-backend/src/code_generator_riscv.rs.skel
// riscv 代码生成: ArchInstruction 分发 + RVV 向量 + Zbb 扩展
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/riscv/code-generator-riscv.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kRiscv/g) === 334, `cc case kRiscv 334, 实得 ${count(cc, /case kRiscv/g)}`);
must(count(cc, /case kArch/g) === 41, `cc case kArch 41, 实得 ${count(cc, /case kArch/g)}`);
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 52)}`);
// riscv 特色: RVV 向量
must(count(cc, /Rvv|rvv|vset|vl\b/g) >= 1, "cc RVV 向量面");
must(cc.includes("Word32") || cc.includes("Word64"), "cc 双宽面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/riscv/code-generator-riscv.js",
  "src/compiler/backend/riscv/code-generator-riscv.cc",
  "riscv 代码生成: ArchInstruction 分发 (334 case) + RVV 向量")}
//! 台账: AssembleArchInstruction case kRiscv ×334 (+ 公共 kArch ×41);
//!   ArchBranch/ArchBoolean/TailCallAfterGap; RVV 向量 (RvvLd/St, vset 发射);
//!   32/64 (W/D) 双宽。

/// v8: CodeGenerator::AssembleArchInstruction (riscv)
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 334);
  assert!(COMMON_CASE_COUNT == 41);
};
pub const ARCH_CASE_COUNT: usize = 334;
pub const COMMON_CASE_COUNT: usize = 41;
`;

writeSrc("v8-compiler-backend", "code_generator_riscv.rs.skel", skel);
console.log(`riscv/code-generator-riscv.js: 对数 ${checks} 条全过; case kRiscv 334 + kArch 41 实测; ` +
  `产物 code_generator_riscv.rs.skel`);
