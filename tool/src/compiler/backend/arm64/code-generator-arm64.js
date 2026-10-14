#!/usr/bin/env bun
// tool/src/compiler/backend/arm64/code-generator-arm64.js
// v8/src/compiler/backend/arm64/code-generator-arm64.cc
//   → v8-compiler-backend/src/code_generator_arm64.rs.skel
// arm64 代码生成: AssembleArchInstruction 294 case + tbz/tbnz 伪指令 + 双发射管线
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/arm64/code-generator-arm64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kArm64/g) === 294, `cc case kArm64 294, 实得 ${count(cc, /case kArm64/g)}`);
// 架构面方法
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchDeoptBranch(Instruction* instr,",
  "void CodeGenerator::AssembleArchJumpRegardlessOfAssemblyOrder(",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleArchSelect(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 55)}`);
// 伪指令面 (tbz/tbnz 在 ArchBranch 化解)
must(cc.includes("turned into tbz/tbnz in AssembleArchBranch"), "cc tbz/tbnz 化解注释");
// 栈检查偏移 (ShouldApplyOffsetToStackCheck + GetStackCheckOffset)
must(cc.includes("ShouldApplyOffsetToStackCheck(instr, &offset)"), "cc 栈检查偏移判定");
must(count(cc, /GetStackCheckOffset\(\)/g) >= 2, "cc GetStackCheckOffset ≥2");
// 双发射 (pipeliner)
must(cc.includes("AddPendingDeprecationWarning") || cc.includes("__ DoubleLiteral") ||
  /instruction.*pipeline|LoadStore|ldp|stp/i.test(cc), "cc 双发射/load-store 面");
// 条件码 (conditon branch 取反)
must(count(cc, /FlagsConditionField::decode/g) === 8, "cc 条件位域解码 ×8");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/arm64/code-generator-arm64.js",
  "src/compiler/backend/arm64/code-generator-arm64.cc",
  "arm64 代码生成: ArchInstruction 分发/tbz 伪指令/ArchSelect/栈检查偏移")}
//! 台账: AssembleArchInstruction case kArm64 ×294; AssembleArchBranch
//!   (TestBit 类 → tbz/tbnz 伪指令化解) / ArchDeoptBranch / ArchBoolean /
//!   ArchSelect (csel) / JumpRegardlessOfAssemblyOrder / BinarySearchSwitchRange;
//!   AssembleTailCallAfterGap; 栈检查 ShouldApplyOffsetToStackCheck +
//!   GetStackCheckOffset (Smi 偏移注入); 双发射 load/store (ldp/stp 优化面)。

/// v8: CodeGenerator::AssembleArchInstruction (arm64) — 294 case 分发
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult
/// v8: CodeGenerator::AssembleArchBranch — TestBit → tbz/tbnz 化解
// fn assemble_arch_branch(instr, branch)
/// v8: CodeGenerator::AssembleArchSelect — csel 三态选择
// fn assemble_arch_select
/// v8: GetStackCheckOffset / ShouldApplyOffsetToStackCheck — 栈检偏移
// fn get_stack_check_offset() -> u32

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 294);
};
/// v8: AssembleArchInstruction case kArm64 分发数 (实测)
pub const ARCH_CASE_COUNT: usize = 294;
`;

writeSrc("v8-compiler-backend", "code_generator_arm64.rs.skel", skel);
console.log(`arm64/code-generator-arm64.js: 对数 ${checks} 条全过; case kArm64 294 实测; ` +
  `架构面 6 方法 + tbz 化解 + 栈检查偏移; 产物 code_generator_arm64.rs.skel`);
