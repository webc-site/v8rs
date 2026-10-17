#!/usr/bin/env bun
// tool/src/compiler/backend/mips64/code-generator-mips64.js
// v8/src/compiler/backend/mips64/code-generator-mips64.cc
//   → v8-compiler-backend/src/code_generator_mips64.rs.skel
// mips64 代码生成: ArchInstruction 分发 + 32/64 双宽 + dsra/dinsr 位操作
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/mips64/code-generator-mips64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kMips64/g) === 388, `cc case kMips64 388, 实得 ${count(cc, /case kMips64/g)}`);
must(count(cc, /case kArch/g) === 40, `cc case kArch 40, 实得 ${count(cc, /case kArch/g)}`);
must(count(cc, /case kMode_MR[IR]/g) === 2, `cc case kMode_MRI/MRR ×2, 实得 ${count(cc, /case kMode_MR[IR]/g)}`);
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 52)}`);
// mips64 特色: 双宽 + 乘除
must(cc.includes("__ dmul") || cc.includes("dmul") || /Dmul/i.test(cc), "cc 双宽乘面");
must(cc.includes("Word32") || cc.includes("Word64"), "cc 双宽面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/mips64/code-generator-mips64.js",
  "src/compiler/backend/mips64/code-generator-mips64.cc",
  "mips64 代码生成: ArchInstruction 分发 (388 case) + 32/64 双宽")}
//! 台账: AssembleArchInstruction case kMips64 ×388 (+ 公共 kArch ×40 +
//!   kMode_MRI/MRR 寻址 ×2); ArchBranch/ArchBoolean/TailCallAfterGap; dmul/ddiv
//!   64 位乘除; 32/64 双宽发射。

/// v8: CodeGenerator::AssembleArchInstruction (mips64)
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 388);
  assert!(COMMON_CASE_COUNT == 40);
};
pub const ARCH_CASE_COUNT: usize = 388;
pub const COMMON_CASE_COUNT: usize = 40;
`;

writeSrc("v8-compiler-backend", "code_generator_mips64.rs.skel", skel);
console.log(`mips64/code-generator-mips64.js: 对数 ${checks} 条全过; case kMips64 388 + kArch 40 实测; ` +
  `产物 code_generator_mips64.rs.skel`);
