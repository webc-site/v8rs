#!/usr/bin/env bun
// tool/src/compiler/backend/instruction-selector.js
// v8/src/compiler/backend/instruction-selector.{h,cc} + instruction-selector-impl.h
//   → v8-compiler-backend/src/instruction_selector.rs.skel
// 指令选择器 (turboshaft 后端): SelectInstructions 主流程/VisitNode 分发 97 case/
//   FlagsContinuation/操作数生成器/比较链
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/instruction-selector.h");
const cc = readV8("src/compiler/backend/instruction-selector.cc");
const impl = readV8("src/compiler/backend/instruction-selector-impl.h");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: FlagsContinuation + 模式 5 + 比较链
must(h.includes("class FlagsContinuation final {"), "h FlagsContinuation");
for (const m of ["kFlags_none", "kFlags_branch", "kFlags_deoptimize", "kFlags_trap", "kFlags_select"])
  must(h.includes(m), `h 模式 ${m}`);
must(h.includes("static constexpr size_t kMaxCompareChainSize = 4;"), "h 比较链上限 4");
for (const f of ["InstructionCode code;", "FlagsCondition compare_condition;", "FlagsCondition default_flags;"])
  must(h.includes(f), `h ConditionalCompare 字段 ${f}`);
// h: 配置枚举族 (布尔开关 6)
must(h.includes("enum SourcePositionMode { kCallSourcePositions, kAllSourcePositions };"), "h SourcePositionMode");
for (const e of ["enum EnableScheduling : bool", "enum EnableRootsRelativeAddressing : bool",
  "enum EnableSwitchJumpTable : bool", "enum EnableTraceTurboJson : bool", "enum EnsureDeterministicNan : bool"])
  must(h.includes(e), `h 配置 ${e}`);
must(h.includes("enum CallBufferFlag {"), "h CallBufferFlag");
must(h.includes("enum class FrameStateInputKind { kAny, kStackSlot };"), "h FrameStateInputKind");
must(h.includes("enum class Upper32BitsState : uint8_t {"), "h Upper32BitsState");
must(h.includes("struct PushParameter {"), "h PushParameter");
// h: 主类面
must(h.includes("class V8_EXPORT_PRIVATE InstructionSelector final"), "h 主类 InstructionSelector");
must(h.includes("void MarkAsTrappingInstruction(turboshaft::OpIndex node) {"), "h MarkAsTrappingInstruction");
must(h.includes("bool UseInstructionScheduling() const {"), "h UseInstructionScheduling");
must(h.includes("void set_instruction_selection_failed() {"), "h 失败旗标");
must(h.includes("EmitCcmpCompareChain(compare_chain::CompareSequence& sequence,"), "h EmitCcmpCompareChain");

// cc: 主流程 4 段
must(cc.includes("std::optional<BailoutReason> InstructionSelector::SelectInstructions() {"),
  "cc SelectInstructions 主流程");
must(cc.includes("void InstructionSelector::VisitBlock(const Block* block) {"), "cc VisitBlock");
must(cc.includes("VisitControl(block);"), "cc VisitControl 调用");
must(cc.includes("void InstructionSelector::VisitNode(OpIndex node) {"), "cc VisitNode 分发");
// cc: VisitNode case 总数 (真计数 97, 控制流 case 早返回另计)
must(count(cc, /case Opcode::k/g) === 97, `cc case Opcode 97, 实得 ${count(cc, /case Opcode::k/g)}`);
must(cc.includes("#if V8_ENABLE_WEBASSEMBLY"), "cc wasm 条件 case");
// cc: 调用缓冲 + 参数准备
must(cc.includes("void InstructionSelector::InitializeCallBuffer("), "cc InitializeCallBuffer");
must(cc.includes("EmitPrepareArguments(&buffer.pushed_nodes, call_descriptor, node);"), "cc EmitPrepareArguments");
// cc: switch 表 (jump table/lookup switch)
must(cc.includes("const SwitchInfo& sw, InstructionOperand const& index_operand) {"), "cc 索引 switch 表");
must(cc.includes("SwitchInfo sw(std::move(cases), min_value, max_value, swtch.default_case);"), "cc SwitchInfo 构建");
// cc: 失败传播
must(cc.includes("instruction_selection_failed"), "cc 失败传播面");

// impl.h: 5 组件
must(impl.includes("struct CaseInfo {"), "impl CaseInfo");
must(impl.includes("class SwitchInfo {"), "impl SwitchInfo");
must(impl.includes("class OperandGenerator : public turboshaft::OperationMatcher {"), "impl OperandGenerator");
must(impl.includes("enum class RegisterUseKind { kUseRegister, kUseUniqueRegister };"), "impl RegisterUseKind");
must(impl.includes("enum RegisterMode {"), "impl RegisterMode");
must(impl.includes("class CompareChainNode final : public ZoneObject {"), "impl CompareChainNode");
must(impl.includes("enum class NodeKind : uint8_t { kFlagSetting, kLogicalCombine };"), "impl NodeKind");
must(impl.includes("class CompareSequence {"), "impl CompareSequence");

// ── 产物 ────────────────────────────────────────────────────────────────────
const skel = `${banner("src/compiler/backend/instruction-selector.js",
  "src/compiler/backend/instruction-selector.h + instruction-selector.cc + instruction-selector-impl.h",
  "指令选择器 (turboshaft): 主流程/VisitNode 分发/FlagsContinuation/操作数生成")}
//! 台账: SelectInstructions → 逐块 VisitBlock (VisitNode 值节点 97 case 分发 +
//!   VisitControl 控制节点) + 整合 Emit; 调用面 InitializeCallBuffer (压参
//!   EmitPrepareArguments); switch 面 SwitchInfo (CaseInfo/跳转表/二分);
//!   FlagsContinuation 5 模式 (none/branch/deoptimize/trap/select) +
//!   ConditionalCompare 5 字段 + 比较链上限 4 (EmitCcmpCompareChain);
//!   配置 6 布尔 + SourcePositionMode + CallBufferFlag; impl.h: OperandGenerator
//!   (RegisterUseKind/RegisterMode) + CompareChainNode (kFlagSetting/kLogicalCombine)
//!   + CompareSequence; 失败通道 set_instruction_selection_failed → BailoutReason。

/// v8: FlagsContinuation 模式 — kFlags_none/branch/deoptimize/trap/select
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum FlagsModeKind { None = 0, Branch = 1, Deoptimize = 2, Trap = 3, Select = 4 }

/// v8: kMaxCompareChainSize — ccmp 比较链上限 (实测 4)
pub const MAX_COMPARE_CHAIN_SIZE: usize = 4;

/// v8: SourcePositionMode — kCallSourcePositions/kAllSourcePositions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SourcePositionMode { CallSourcePositions = 0, AllSourcePositions = 1 }

/// v8: OperandGenerator::RegisterUseKind — kUseRegister/kUseUniqueRegister
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RegisterUseKind { UseRegister = 0, UseUniqueRegister = 1 }

/// v8: CompareChainNode::NodeKind — kFlagSetting/kLogicalCombine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CompareChainNodeKind { FlagSetting = 0, LogicalCombine = 1 }

/// v8: InstructionSelector::SelectInstructions — 主流程 (Option<BailoutReason>)
// pub fn select_instructions(&mut self) -> Option<BailoutReason>
/// v8: InstructionSelector::VisitNode — 值节点 97 case 分发 (架构镜像扩展)
// fn visit_node(&mut self, node: OpIndex)
/// v8: InstructionSelector::VisitBlock / VisitControl — 块与控制终结
// fn visit_block(&mut self, block) / visit_control
/// v8: InstructionSelector::InitializeCallBuffer — 调用实参缓冲 (CallBufferFlag)
// fn initialize_call_buffer(...)
/// v8: SwitchInfo + CaseInfo (impl.h) — switch 表构建 (跳转表/查找表/二分)
// struct SwitchInfo { cases: Vec<CaseInfo>, min_value, max_value, default_case }
/// v8: OperandGenerator (impl.h) — 操作数构造 (Use/RegisterMode 族)
// struct OperandGenerator

// —— 提取不变式断言 ——
const _: () = {
  assert!(MAX_COMPARE_CHAIN_SIZE == 4);
  assert!(FlagsModeKind::Select as u8 == 4);
};
/// v8: VisitNode 分发 case 总数 (实测 97, wasm 条件 case 在内)
pub const VISIT_NODE_CASE_COUNT: usize = 97;
const _: () = {
  assert!(VISIT_NODE_CASE_COUNT == 97);
};
`;

writeSrc("v8-compiler-backend", "instruction_selector.rs.skel", skel);
console.log(`instruction-selector.js: 对数 ${checks} 条全过; VisitNode case 97 实测; ` +
  `FlagsContinuation 5 模式; 比较链上限 4; impl 5 组件; 产物 instruction_selector.rs.skel`);
