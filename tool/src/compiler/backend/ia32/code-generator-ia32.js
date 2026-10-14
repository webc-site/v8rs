#!/usr/bin/env bun
// tool/src/compiler/backend/ia32/code-generator-ia32.js
// v8/src/compiler/backend/ia32/code-generator-ia32.cc
//   → v8-compiler-backend/src/code_generator_ia32.rs.skel
// ia32 代码生成: ArchInstruction 分发 + 双寄存器 64 位值 (edx:eax)
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/ia32/code-generator-ia32.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("),
  "cc AssembleArchInstruction");
must(count(cc, /case kIA32/g) === 304, `cc case kIA32 304, 实得 ${count(cc, /case kIA32/g)}`);
must(count(cc, /case kArch/g) === 40, `cc case kArch 公共 40, 实得 ${count(cc, /case kArch/g)}`);
// 架构面方法
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchDeoptBranch(Instruction* instr,",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 52)}`);
// ia32 特色: 64 位双寄存器
must(cc.includes("edx") || cc.includes("kEdx") || /edx.*eax|HighPair/i.test(cc), "cc 64 位双寄存器面");
// x87/SSE 双路
must(count(cc, /SSE|xmm|Xmm/g) >= 1, "cc SSE 面");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/ia32/code-generator-ia32.js",
  "src/compiler/backend/ia32/code-generator-ia32.cc",
  "ia32 代码生成: 32 位 x86 分发 + edx:eax 双寄存器 64 位 + SSE")}
//! 台账: AssembleArchInstruction case kIA32 ×304 (+ 公共 kArch ×40 + kMode ×21 地址特化);
//!   ArchBranch/DeoptBranch/Boolean/TailCallAfterGap; 64 位值 edx:eax 双寄存器。

/// v8: CodeGenerator::AssembleArchInstruction (ia32) — 304 case + 公共 40
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult
/// v8: 64 位双寄存器序列 (edx:eax)
// fn 双寄存器乘除/移位锚

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 304);
  assert!(COMMON_CASE_COUNT == 40);
};
pub const ARCH_CASE_COUNT: usize = 304;
pub const COMMON_CASE_COUNT: usize = 40;
`;

writeSrc("v8-compiler-backend", "code_generator_ia32.rs.skel", skel);
console.log(`ia32/code-generator-ia32.js: 对数 ${checks} 条全过; case kIA32 304 + kArch 40 实测; ` +
  `产物 code_generator_ia32.rs.skel`);
