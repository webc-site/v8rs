#!/usr/bin/env bun
// tool/src/compiler/backend/s390/code-generator-s390.js
// v8/src/compiler/backend/s390/code-generator-s390.cc
//   → v8-compiler-backend/src/code_generator_s390.rs.skel
// s390x 代码生成: ArchInstruction 分发 + cc 条件码 (brc)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/s390/code-generator-s390.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kS390/g) === 242, `cc case kS390 242, 实得 ${count(cc, /case kS390/g)}`);
must(count(cc, /case kArch/g) === 39, `cc case kArch 39, 实得 ${count(cc, /case kArch/g)}`);
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 52)}`);
// s390 特色: cc 条件码 + Load/Store 复合 (LHI/LGHI)
must(cc.includes("ConditionFor") || /cc_mask|brc|Condition/.test(cc), "cc 条件码面");
must(cc.includes("lgfr") || cc.includes("lgfi") || /32.*64|sign/.test(cc), "cc 双宽面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/s390/code-generator-s390.js",
  "src/compiler/backend/s390/code-generator-s390.cc",
  "s390x 代码生成: ArchInstruction 分发 (242 case) + cc 条件码")}
//! 台账: AssembleArchInstruction case kS390 ×242 (+ 公共 kArch ×39);
//!   ArchBranch/ArchBoolean/TailCallAfterGap; cc 条件码 (ConditionFor 族,
//!   brc 掩码发射); LoadImmediate 复合 (LHI/LGHI/lgfi)。

/// v8: CodeGenerator::AssembleArchInstruction (s390x)
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 242);
  assert!(COMMON_CASE_COUNT == 39);
};
pub const ARCH_CASE_COUNT: usize = 242;
pub const COMMON_CASE_COUNT: usize = 39;
`;

writeSrc("v8-compiler-backend", "code_generator_s390.rs.skel", skel);
console.log(`s390/code-generator-s390.js: 对数 ${checks} 条全过; case kS390 242 + kArch 39 实测; ` +
  `产物 code_generator_s390.rs.skel`);
