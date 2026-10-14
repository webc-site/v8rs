#!/usr/bin/env bun
// tool/src/compiler/backend/register-allocator-verifier.js
// v8/src/compiler/backend/register-allocator-verifier.{h,cc}
//   → v8-compiler-backend/src/register_allocator_verifier.rs.skel
// RegisterAllocatorVerifier: 分配前后操作数替换正确性验证 (Assessment 模型)
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/register-allocator-verifier.h");
const cc = readV8("src/compiler/backend/register-allocator-verifier.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;

// h: AssessmentKind 2 值
must(h.includes("enum AssessmentKind { Final, Pending };"), "h AssessmentKind");
// h: 三类 Assessment (基 + Pending + Final) + BlockAssessments + DelayedAssessments
must(h.includes("class Assessment : public ZoneObject {"), "h Assessment 基类");
must(h.includes("class PendingAssessment final : public Assessment {"), "h PendingAssessment");
must(h.includes("class FinalAssessment final : public Assessment {"), "h FinalAssessment");
must(h.includes("class BlockAssessments : public ZoneObject {"), "h BlockAssessments");
must(h.includes("class DelayedAssessments : public ZoneObject {"), "h DelayedAssessments");
must(h.includes("struct OperandAsKeyLess {"), "h OperandAsKeyLess (规范化序)");
must(h.includes("using OperandMap = ZoneMap<InstructionOperand, Assessment*, OperandAsKeyLess>;"), "h OperandMap");

// h: 真解析主类 ConstraintType 枚举 (12 值)
const ctMatch = h.match(/enum ConstraintType \{([\s\S]*?)\};/);
if (!ctMatch) throw new Error("未找到 ConstraintType");
const constraintTypes = [...ctMatch[1].matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (constraintTypes.length !== 13) throw new Error(`ConstraintType ${constraintTypes.length} != 13`);
for (const t of ["Constant", "Immediate", "Register", "FixedRegister", "FPRegister", "FixedFPRegister",
  "Slot", "FixedSlot", "RegisterOrSlot", "RegisterOrSlotFP", "RegisterOrSlotOrConstant", "SameAsInput",
  "RegisterAndSlot"])
  must(constraintTypes.includes(t), `ConstraintType ${t}`);

// h: 主类公有 2 + 私有结构 + 字段 6
must(h.includes("class RegisterAllocatorVerifier final : public ZoneObject {"), "h 主类");
must(h.includes("void VerifyAssignment(const char* caller_info);"), "h VerifyAssignment");
must(h.includes("void VerifyGapMoves();"), "h VerifyGapMoves");
must(h.includes("struct OperandConstraint {"), "h OperandConstraint (3 字段)");
must(h.includes("struct InstructionConstraint {"), "h InstructionConstraint");
for (const f of ["Zone* const zone_;", "const InstructionSequence* const sequence_;",
  "Constraints constraints_;", "ZoneMap<RpoNumber, BlockAssessments*> assessments_;",
  "ZoneMap<RpoNumber, DelayedAssessments*> outstanding_assessments_;", "int spill_slot_delta_;"])
  must(h.includes(f), `h 字段 ${f}`);

// cc: 匿名 3 自由函数 + 成员实现 12
for (const fn of ["size_t OperandCount(", "void VerifyEmptyGaps(", "void VerifyAllocatedGaps("])
  must(cc.includes(fn), `cc 匿名 ${fn}`);
must(count(cc, /^[\w:* ]+RegisterAllocatorVerifier::\w+\(/gm) === 10, "cc 成员实现 10");
// cc: 静态 Verify* ×3 (CHECK 虚拟寄存器有效)
must(count(cc, /CHECK_NE\(InstructionOperand::kInvalidVirtualRegister,/g) === 2, "cc CHECK_NE ×2");
// cc: 循环头延迟评估 (回边)
must(cc.includes("outstanding_assessments_"), "cc 回边延迟评估消费");

// ── 产物 ────────────────────────────────────────────────────────────────────
const variants = constraintTypes.map((t) => `  /// k${t}\n  ${t},`).join("\n");
const skel = `${banner("src/compiler/backend/register-allocator-verifier.js",
  "src/compiler/backend/register-allocator-verifier.h + register-allocator-verifier.cc",
  "RegisterAllocatorVerifier: 分配正确性验证 (Assessment 约束/位图模型)")}
//! 台账: VerifyAssignment (预分配约束表比对) + VerifyGapMoves (块间 Assessment
//!   传播验证); AssessmentKind Final/Pending; BlockAssessments: OperandMap +
//!   map_for_moves_ + stale_ref_stack_slots_ (引用图陈旧槽检测) + CopyFrom/DropRegisters;
//!   DelayedAssessments 环回头延迟验证; ConstraintType 12 值实测。

/// v8: AssessmentKind — Final(已定 vreg)/Pending(多前驱待定)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AssessmentKind { Final = 0, Pending = 1 }

/// v8: RegisterAllocatorVerifier::ConstraintType — 操作数约束类 (实测 13)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
#[rustfmt::skip]
pub enum ConstraintType {
${variants}
}

/// v8: RegisterAllocatorVerifier::VerifyAssignment — 预分配约束 vs 实分配
// pub fn verify_assignment(&mut self, caller_info: &str)
/// v8: RegisterAllocatorVerifier::VerifyGapMoves — 块间移动 Assessment 验证
// pub fn verify_gap_moves(&mut self)
/// v8: RegisterAllocatorVerifier::ValidatePendingAssessment — 别名证明
// fn validate_pending_assessment(&mut self, block: RpoNumber, assessment: &PendingAssessment, vreg: i32)
/// v8: RegisterAllocatorVerifier::ValidateUse — 使用点验证
// fn validate_use(&mut self, block: RpoNumber, current: &mut BlockAssessments, op: InstructionOperand, vreg: i32)
/// v8: RegisterAllocatorVerifier::CreateForBlock — 块初始 Assessment (前驱合并)
// fn create_for_block(&mut self, block: &InstructionBlock) -> BlockAssessments
/// v8: BlockAssessments::PerformParallelMoves — 并行移动按 GapResolver 序应用
// BlockAssessments::perform_parallel_moves / perform_moves / check_reference_map

pub const CONSTRAINT_TYPE_COUNT: usize = ${constraintTypes.length};

// —— 提取不变式断言 ——
const _: () = {
  assert!(AssessmentKind::Pending as u8 == 1);
  assert!(CONSTRAINT_TYPE_COUNT == 13);
};
`;

writeSrc("v8-compiler-backend", "register_allocator_verifier.rs.skel", skel);
console.log(`register-allocator-verifier.js: 对数 ${checks} 条全过; ConstraintType 13 值实测; ` +
  `Assessment 3 类 + Block/Delayed; 成员实现 10; 产物 register_allocator_verifier.rs.skel`);
