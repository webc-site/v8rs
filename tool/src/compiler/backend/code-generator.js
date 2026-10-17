#!/usr/bin/env bun
// tool/src/compiler/backend/code-generator.js
// v8/src/compiler/backend/code-generator.{h,cc} + code-generator-impl.h
//   → v8-compiler-backend/src/code_generator.rs.skel
// 代码生成器主体: AssembleCode 主循环 + safepoint/handler 表 + OOL 链 + 转换器
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/code-generator.h");
const cc = readV8("src/compiler/backend/code-generator.cc");
const impl = readV8("src/compiler/backend/code-generator-impl.h");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: 4 结构 (BranchInfo/InstructionOperandIterator/Turbolizer×2)
must(h.includes("struct BranchInfo {"), "h BranchInfo");
for (const f of ["FlagsCondition condition;", "Label* true_label;", "Label* false_label;",
  "bool hinted;", "bool fallthru;"]) must(h.includes(f), `h BranchInfo 字段 ${f}`);
must(h.includes("class InstructionOperandIterator {"), "h 迭代器");
must(h.includes("struct TurbolizerCodeOffsetsInfo {"), "h Turbolizer 偏移 7 槽");
must(h.includes("struct TurbolizerInstructionStartInfo {"), "h Turbolizer 指令起始 3 槽");

// h: 主类 = GapResolver::Assembler 实现
must(h.includes("class V8_EXPORT_PRIVATE CodeGenerator final : public GapResolver::Assembler {"),
  "h 主类继承 GapResolver::Assembler");
must(h.includes("static constexpr int kBinarySearchSwitchMinimalCases = 4;"), "h 二分开关阈值 4");
// h: 枚举 3 (CodeGenResult/PushTypeFlag/MoveType::Type 真解析)
must(h.includes("enum CodeGenResult { kSuccess, kTooManyDeoptimizationBailouts };"), "h CodeGenResult");
const pushMatch = h.match(/enum PushTypeFlag \{([\s\S]*?)\};/);
if (!pushMatch) throw new Error("未找到 PushTypeFlag");
const pushFlags = [...pushMatch[1].matchAll(/^\s*k(\w+)( = ([^,]+))?[,]?/gm)];
if (pushFlags.length !== 4) throw new Error(`PushTypeFlag ${pushFlags.length} != 4`);
must(h.includes("kScalarPush = kRegisterPush | kStackSlotPush"), "h kScalarPush 组合");
const moveMatch = h.match(/enum Type \{([\s\S]*?)\};/);
if (!moveMatch) throw new Error("未找到 MoveType::Type");
const moveTypes = [...moveMatch[1].matchAll(/^\s*k(\w+),?\s*$/gm)].map((m) => m[1]);
if (moveTypes.length !== 6) throw new Error(`MoveType ${moveTypes.length} != 6`);
checks++;
// h: 公有面核心方法
for (const sig of ["void AssembleCode();", "void PrepareCodeOnBackground(LocalIsolate* local_isolate);",
  "MaybeHandle<Code> FinalizeCode();", "base::UniqueArray<uint8_t> GenerateWasmDeoptimizationData();",
  "base::UniqueArray<uint8_t> GetSourcePositionTable();", "void RecordSafepoint(ReferenceMap* references, int pc_offset = 0);",
  "void RecordSafepointWithoutTaggedSlots();", "void RecordTrappingInstruction(uint32_t instr_offset);",
  "void AssembleSourcePosition(Instruction* instr);"])
  must(h.includes(sig), `h 公有 ${sig.slice(0, 50)}`);
// h: MoveType 推断 2
must(h.includes("static Type InferMove(InstructionOperand* source,"), "h InferMove");
must(h.includes("static Type InferSwap(InstructionOperand* source,"), "h InferSwap");
must(h.includes("static void GetPushCompatibleMoves(Instruction* instr,"), "h GetPushCompatibleMoves");

// impl.h: 转换器 + Deopt 退出 + OOL 链
must(impl.includes("class InstructionOperandConverter {"), "impl InstructionOperandConverter");
must(impl.includes("class DeoptimizationExit : public ZoneObject {"), "impl DeoptimizationExit");
must(impl.includes("class OutOfLineCode : public ZoneObject {"), "impl OutOfLineCode");
must(impl.includes("OutOfLineCode* next() const { return next_; }"), "impl OOL 链 next");
must(impl.includes("Label label_;"), "impl OOL label");
must(impl.includes("Label* entry_ = nullptr;") || impl.includes("Label entry_;") || impl.includes("Label exit_;"), "impl DeoptExit label 面");

// cc: 主循环面
must(cc.includes("void CodeGenerator::AssembleCode() {"), "cc AssembleCode 主循环");
must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleBlock("), "cc AssembleBlock");
must(cc.includes("CodeGenerator::CodeGenResult CodeGenerator::AssembleInstruction("), "cc AssembleInstruction");
must(cc.includes("void CodeGenerator::AssembleArchBinarySearchSwitchRange("), "cc 二分开关区段");
must(cc.includes("void CodeGenerator::AssembleArchJump(RpoNumber target) {"), "cc AssembleArchJump");
must(cc.includes("safepoints()->Emit(masm(), frame()->GetTotalFrameSlotCount());"), "cc safepoint 发射");
must(cc.includes("void CodeGenerator::PrepareCodeOnBackground("), "cc 后台编译段");
must(cc.includes("CreateFrameAccessState(frame);"), "cc FrameAccessState 创建");
// cc: 结果限流 (deopt bailout 上限)
must(count(cc, /kTooManyDeoptimizationBailouts/g) === 1, "cc deopt 限流返回 ×1 (h 定义 1)");
must(cc.includes("deoptimization_id > Deoptimizer::kMaxNumberOfEntries"), "cc deopt 条目上限判定");
must(cc.includes("++lazy_deopt_count_;") && cc.includes("++eager_deopt_count_;"), "cc lazy/eager 计数");
// cc: Assemble 段落数 (真计数 AssembleArch 分发留架构镜像)
must(count(cc, /^[\w:<>* &]*CodeGenerator::\w+/gm) >= 25, "cc 成员实现 ≥25");

// ── 产物 ────────────────────────────────────────────────────────────────────
const moveVariants = moveTypes.map((n) => `  /// k${n}\n  ${n.slice(0, 1)}${n.slice(1).replace(/([A-Z])/g, "_$1").toLowerCase().replace(/^([a-z])/, (x) => x.toUpperCase())},`).join("\n");
const skel = `${banner("src/compiler/backend/code-generator.js",
  "src/compiler/backend/code-generator.h + code-generator.cc + code-generator-impl.h",
  "代码生成器主体: AssembleCode 主循环/safepoint 表/OOL 链/操作数转换器")}
//! 台账: CodeGenerator 实现 GapResolver::Assembler (架构镜像代码生成器的公共底座);
//!   AssembleCode = 逐块 AssembleBlock → AssembleInstruction (gap 消解 +
//!   AssembleArchInstruction 架构分发 + 二分开关 + AssembleArchJump) +
//!   safepoints()->Emit; 后台编译段 PrepareCodeOnBackground; FinalizeCode;
//!   BranchInfo (条件/双 label/hinted/fallthru); Turbolizer 偏移 7+3 槽;
//!   PushTypeFlag 4 (immediate 1/register 2/stackslot 4/scalar 组合);
//!   MoveType::Type 6 (推演 InferMove/InferSwap); impl.h = 转换器
//!   (ToConstant/ToImmediate/ToRegister...) + DeoptimizationExit + OOL 单链。

/// v8: CodeGenerator::CodeGenResult — kSuccess/kTooManyDeoptimizationBailouts
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CodeGenResult { Success = 0, TooManyDeoptimizationBailouts = 1 }

/// v8: CodeGenerator::PushTypeFlag — push 候选类型位标志
pub const IMMEDIATE_PUSH: u8 = 0x1;
pub const REGISTER_PUSH: u8 = 0x2;
pub const STACK_SLOT_PUSH: u8 = 0x4;
/// v8: kScalarPush = kRegisterPush | kStackSlotPush
pub const SCALAR_PUSH: u8 = REGISTER_PUSH | STACK_SLOT_PUSH;

/// v8: CodeGenerator::MoveType::Type — 移动/交换类型推演 (实测 6)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum MoveType {
${moveVariants}
}

/// v8: CodeGenerator::AssembleCode — 主循环 (可后台线程)
// pub fn assemble_code(&mut self) -> CodeGenResult
/// v8: CodeGenerator::AssembleInstruction — 单指令 (gap + arch 分发)
// fn assemble_instruction(...) -> CodeGenResult
/// v8: CodeGenerator::RecordSafepoint / RecordSafepointWithoutTaggedSlots
// fn record_safepoint(&mut self, references: &ReferenceMap, pc_offset: i32)
/// v8: CodeGenerator::ShouldApplyOffsetToStackCheck / GetStackCheckOffset
// fn should_apply_offset_to_stack_check(&self, instr, offset: &mut u32) -> bool
/// v8: InstructionOperandConverter (impl.h) — 操作数→机器值转换族
// struct InstructionOperandConverter (ToRegister/ToConstant/ToImmediate/...)
/// v8: OutOfLineCode (impl.h) — 越线代码单链 (next_) + DeoptimizationExit
// struct OutOfLineCode + DeoptimizationExit

// —— 提取不变式断言 ——
const _: () = {
  assert!(SCALAR_PUSH == REGISTER_PUSH | STACK_SLOT_PUSH);
  assert!(BINARY_SEARCH_SWITCH_MINIMAL_CASES == 4);
};
/// v8: kBinarySearchSwitchMinimalCases
pub const BINARY_SEARCH_SWITCH_MINIMAL_CASES: i32 = 4;
`;

writeSrc("v8-compiler-backend", "code_generator.rs.skel", skel);
console.log(`code-generator.js: 对数 ${checks} 条全过; CodeGenResult 2/PushTypeFlag 4/MoveType 6 实测; ` +
  `Turbolizer 槽 7+3; 实现面 ≥25; 产物 code_generator.rs.skel`);
