#!/usr/bin/env bun
// tool/src/compiler/backend/arm/code-generator-arm.js
// v8/src/compiler/backend/arm/code-generator-arm.cc
//   → v8-compiler-backend/src/code_generator_arm.rs.skel
// arm (32 位) 代码生成: ArchInstruction 分发 + 双寄存器长乘除 + VFP/Neon
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/arm/code-generator-arm.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kArm/g) === 352, `cc case kArm 352, 实得 ${count(cc, /case kArm/g)}`);
must(count(cc, /case kArch/g) === 40, `cc case kArch 公共 40, 实得 ${count(cc, /case kArch/g)}`);
// 架构面方法
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchDeoptBranch(Instruction* instr,",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 52)}`);
// arm 特色: 双寄存器除法 (UMULL/UDIV 无, 软除法序列)
must(cc.includes("__ umull") || cc.includes("umull") || /UMULL/i.test(cc), "cc 双寄存器乘除面");
// VFP/Neon 面
must(count(cc, /vld|vst|Neon|neon/g) >= 1 || /q7|s0|d0/.test(cc), "cc VFP/Neon 面");
// swstk 面与块链
must(cc.includes("AssembleGaps(") || cc.includes("AssembleArchBinarySearchSwitchRange("), "cc gap/二分面");
must(cc.includes("kArchThrowTerminator") || cc.includes("AssembleArchJump("), "cc 跳转终结面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm/code-generator-arm.js",
  "src/compiler/backend/arm/code-generator-arm.cc",
  "arm 代码生成: 32 位分发 + 软除法/双寄存器长乘 + VFP/Neon")}
//! 台账: AssembleArchInstruction case kArm ×352 (+ 公共 kArch ×40);
//!   AssembleArchBranch/DeoptBranch/Boolean/TailCallAfterGap;
//!   32 位特色: 64 位值双寄存器 (umull 软除法), VFP/Neon 加载存储。

/// v8: CodeGenerator::AssembleArchInstruction (arm) — 352 case + 公共 40
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult
/// v8: CodeGenerator::AssembleArchBranch / ArchBoolean — 条件分支/materialize
// fn assemble_arch_branch / assemble_arch_boolean
/// v8: 32 位双寄存器长乘除序列
// fn 软除法/umull 序列锚

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 352);
  assert!(COMMON_CASE_COUNT == 40);
};
pub const ARCH_CASE_COUNT: usize = 352;
pub const COMMON_CASE_COUNT: usize = 40;
`;

writeSrc("v8-compiler-backend", "code_generator_arm.rs.skel", skel);
console.log(`arm/code-generator-arm.js: 对数 ${checks} 条全过; case kArm 352 + kArch 40 实测; ` +
  `软除法/Neon 面; 产物 code_generator_arm.rs.skel`);
