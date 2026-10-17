#!/usr/bin/env bun
// tool/src/compiler/backend/ppc/code-generator-ppc.js
// v8/src/compiler/backend/ppc/code-generator-ppc.cc
//   → v8-compiler-backend/src/code_generator_ppc.rs.skel
// ppc64 代码生成: ArchInstruction 分发 + SetCR/SetBWZ 条件码
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/ppc/code-generator-ppc.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kPPC/g) === 205, `cc case kPPC 205, 实得 ${count(cc, /case kPPC/g)}`);
must(count(cc, /case kArch/g) === 39, `cc case kArch 39, 实得 ${count(cc, /case kArch/g)}`);
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 52)}`);
// ppc 特色: CR 条件寄存器 + 记录位 (SetCR)
must(cc.includes("__ cmpw") || cc.includes("__ cmpd") || /cmpw|cmpd/.test(cc), "cc CR 比较面");
must(cc.includes("extsb") || cc.includes("extsw") || /SignExt/i.test(cc), "cc 符号扩展面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ppc/code-generator-ppc.js",
  "src/compiler/backend/ppc/code-generator-ppc.cc",
  "ppc64 代码生成: ArchInstruction 分发 (205 case) + CR 条件寄存器")}
//! 台账: AssembleArchInstruction case kPPC ×205 (+ 公共 kArch ×39);
//!   ArchBranch/ArchBoolean/TailCallAfterGap; CR 条件寄存器记录位 (SetCR 族)。

/// v8: CodeGenerator::AssembleArchInstruction (ppc64)
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 205);
  assert!(COMMON_CASE_COUNT == 39);
};
pub const ARCH_CASE_COUNT: usize = 205;
pub const COMMON_CASE_COUNT: usize = 39;
`;

writeSrc("v8-compiler-backend", "code_generator_ppc.rs.skel", skel);
console.log(`ppc/code-generator-ppc.js: 对数 ${checks} 条全过; case kPPC 205 + kArch 39 实测; ` +
  `产物 code_generator_ppc.rs.skel`);
