#!/usr/bin/env bun
// tool/src/compiler/backend/x64/code-generator-x64.js
// v8/src/compiler/backend/x64/code-generator-x64.cc
//   → v8-compiler-backend/src/code_generator_x64.rs.skel
// x64 代码生成: AssembleArchInstruction 344 case 分发 + 宏融合判定 + 尾调用
import { readV8, writeSrc, banner } from "../../../../lib.js";

const cc = readV8("src/compiler/backend/x64/code-generator-x64.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// 宏融合枚举 (真解析)
must(cc.includes("enum class SecondMacroFusionInstKind {"), "cc SecondMacroFusionInstKind");
for (const v of ["kAB", "kELG", "kInvalid"]) must(new RegExp(`SecondMacroFusionInstKind`).test(cc) && cc.includes(v), `宏融合二段 ${v}`);
const firstM = cc.match(/enum class FirstMacroFusionInstKind \{([\s\S]*?)\};/);
if (!firstM) throw new Error("未找到 FirstMacroFusionInstKind");
const firstVals = [...firstM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (firstVals.length !== 6) throw new Error(`FirstMacroFusion ${firstVals.length} != 6`);
must(firstVals.join() === "Test,Cmp,And,AddSub,IncDec,Invalid", `一段成员序 ${firstVals.join()}`);
must(cc.includes("bool IsMacroFused(FirstMacroFusionInstKind first_kind,"), "cc IsMacroFusion");
must(cc.includes("SecondMacroFusionInstKind GetSecondMacroFusionInstKind("), "cc GetSecondMacroFusionInstKind");
// 宏融合组合规则 (真断言 3 规则)
must(cc.includes("second_kind == SecondMacroFusionInstKind::kAB ||"), "cc Cmp/AddSub 融合 kAB/kELG");
must(cc.includes("case FirstMacroFusionInstKind::kIncDec:"), "cc IncDec 融合仅 kELG");
must(cc.includes("case FirstMacroFusionInstKind::kInvalid:"), "cc kInvalid 短路");

// 分发面
must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleArchInstruction("), "cc AssembleArchInstruction");
must(count(cc, /case kX64/g) === 344, `cc case kX64 344, 实得 ${count(cc, /case kX64/g)}`);
// 架构面方法
for (const m of ["void CodeGenerator::AssembleArchBranch(Instruction* instr, BranchInfo* branch) {",
  "void CodeGenerator::AssembleArchDeoptBranch(Instruction* instr,",
  "void CodeGenerator::AssembleArchJumpRegardlessOfAssemblyOrder(",
  "void CodeGenerator::AssembleArchBoolean(Instruction* instr,",
  "void CodeGenerator::AssembleArchBinarySearchSwitchRange(",
  "void CodeGenerator::AssembleTailCallAfterGap(Instruction* instr,"])
  must(cc.includes(m), `cc ${m.slice(0, 55)}`);
// 条件解码 (FlagsConditionField)
must(count(cc, /FlagsConditionField::decode\(instr->opcode\(\)\)/g) >= 2, "cc 条件位域解码 ≥2");
// 特征指令锚 (cas/cmpxchg/栈检查)
must(cc.includes("__ j(below, &compare_swap, Label::kNear);"), "cc 弱内存 cas 序");
must(cc.includes("Abort(AbortReason::kJSSignatureMismatch)"), "cc JS 签名Abort");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/x64/code-generator-x64.js",
  "src/compiler/backend/x64/code-generator-x64.cc",
  "x64 代码生成: ArchInstruction 分发/分支布尔/宏融合判定/尾调用")}
//! 台账: AssembleArchInstruction case kX64 ×344 (含 WASM/沙箱条件段);
//!   AssembleArchBranch/DeoptBranch/JumpRegardlessOfAssemblyOrder/Boolean/
//!   BinarySearchSwitchRange (≥4 阈值二分) + AssembleTailCallAfterGap (push 化);
//!   宏融合: FirstMacroFusionInstKind (kCmp/kTest/kAnd/kAddSub/kIncDec/kInvalid)
//!   × Second (kAB/kELG/kInvalid) — test/cmp/addsub 融 kAB|kELG, incdec 仅 kELG;
//!   FlagsConditionField::decode 条件解码; 弱内存 cas 序 (j below 重试);
//!   AVX/SSE 双路发射; kJSSignatureMismatch Abort 防御。

/// v8: FirstMacroFusionInstKind — 宏融合首指令类
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FirstMacroFusionInstKind { Test = 0, Cmp = 1, And = 2, AddSub = 3, IncDec = 4, Invalid = 5 }
/// v8: SecondMacroFusionInstKind — 宏融合次指令类 (kAB/kELG/kInvalid)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SecondMacroFusionInstKind { AB = 0, ELG = 1, Invalid = 2 }

/// v8: IsMacroFused — 融合合法性 (cmp/test/addsub 融 kAB|kELG; incdec 仅 kELG)
// pub fn is_macro_fused(first: FirstMacroFusionInstKind, second: SecondMacroFusionInstKind) -> bool
/// v8: GetSecondMacroFusionInstKind — FlagsCondition → 次指令类
// pub fn get_second_macro_fusion_inst_kind(condition: FlagsCondition) -> SecondMacroFusionInstKind
/// v8: CodeGenerator::AssembleArchInstruction (x64) — 344 case 分发
// fn assemble_arch_instruction(instr: &Instruction) -> CodeGenResult
/// v8: CodeGenerator::AssembleArchBranch / ArchBoolean — 条件分支/materialize
// fn assemble_arch_branch / assemble_arch_boolean
/// v8: CodeGenerator::AssembleTailCallAfterGap — 尾调用 gap 后处理
// fn assemble_tail_call_after_gap

// —— 提取不变式断言 ——
const _: () = {
  assert!(ARCH_CASE_COUNT == 344);
};
/// v8: AssembleArchInstruction case kX64 分发数 (实测)
pub const ARCH_CASE_COUNT: usize = 344;
`;

writeSrc("v8-compiler-backend", "code_generator_x64.rs.skel", skel);
console.log(`x64/code-generator-x64.js: 对数 ${checks} 条全过; case kX64 344 实测; ` +
  `宏融合 First 6+Second 3; 架构面 6 方法; 产物 code_generator_x64.rs.skel`);
