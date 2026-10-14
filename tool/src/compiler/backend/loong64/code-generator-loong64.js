#!/usr/bin/env bun
// tool/src/compiler/backend/loong64/code-generator-loong64.js
// v8/src/compiler/backend/loong64/code-generator-loong64.cc
//   → v8-compiler-backend/src/code_generator_loong64.rs.skel
// loong64 代码生成: ArchInstruction 分发 + 32/64 双宽混合
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/loong64/code-generator-loong64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kLoong64/g) === 387, `cc case kLoong64 387, 实得 ${count(cc, /case kLoong64/g)}`);
must(count(cc, /case kArch/g) === 41, `cc case kArch 41, 实得 ${count(cc, /case kArch/g)}`);
must(count(cc, /case kMode_MR[IR]/g) === 2, `cc case kMode_MRI/MRR ×2, 实得 ${count(cc, /case kMode_MR[IR]/g)}`);
// 架构面方法
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 52)}`);
// loong64 特色: 双宽 (W/D 寄存器)
must(cc.includes("Word32") || cc.includes("Word64"), "cc 双宽面");
must(cc.includes("bstr") || cc.includes("alsl") || /Loong64Or|Loong64Xor/.test(cc), "cc loong64 特有指令面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/loong64/code-generator-loong64.js",
  "src/compiler/backend/loong64/code-generator-loong64.cc",
  "loong64 代码生成: ArchInstruction 分发 (387 case) + 32/64 双宽")}
//! 台账: AssembleArchInstruction case kLoong64 ×387 (+ 公共 kArch ×41 +
//!   kMode_MRI/MRR 寻址 ×2); ArchBranch/ArchBoolean/TailCallAfterGap; W/D 双宽。

/// v8: CodeGenerator::AssembleArchInstruction (loong64)
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult
/// v8: 32/64 双宽 (W/D 寄存器) 面锚
// fn 双宽发射锚

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 387);
  assert!(COMMON_CASE_COUNT == 41);
};
pub const ARCH_CASE_COUNT: usize = 387;
pub const COMMON_CASE_COUNT: usize = 41;
`;

writeSrc("v8-compiler-backend", "code_generator_loong64.rs.skel", skel);
console.log(`loong64/code-generator-loong64.js: 对数 ${checks} 条全过; case kLoong64 387 + kArch 41 实测; ` +
  `产物 code_generator_loong64.rs.skel`);
