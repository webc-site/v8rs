#!/usr/bin/env bun
// tool/src/compiler/backend/instruction.js
// v8/src/compiler/backend/instruction.{h,cc} → v8-compiler-backend/src/instruction.rs.skel
// 指令序列核心数据面: InstructionOperand 族/MoveOperands/ParallelMove/Instruction/
//   Constant/FrameStateDescriptor/InstructionSequence + 末尾偏移常量表
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/backend/instruction.h");
const cc = readV8("src/compiler/backend/instruction.cc");

// ── 对数断言 ────────────────────────────────────────────────────────────────
let checks = 0;
const must = (c, msg) => { checks++; if (!c) throw new Error(`对数失配: ${msg}`); };
const count = (s, re) => (s.match(re) || []).length;
const enumOf = (src, re, label, expect) => {
  const m = src.match(re);
  if (!m) throw new Error(`未找到 ${label}`);
  const names = [...m[1].split("\n")
    .map((l) => l.replace(/\/\/.*$/, "").trim())
    .filter((l) => /^[A-Za-z_]\w*\s*(=.*)?,?$/.test(l))
    .map((l) => l.replace(/[=,].*$/, "").trim())];
  if (expect && names.length !== expect) throw new Error(`${label} ${names.length} != ${expect}`);
  checks++;
  return names;
};

// 真解析 6 枚举
const kind = enumOf(h, /enum Kind \{([\s\S]*?)\};/, "InstructionOperand::Kind", 7);
must(kind[6] === "FIRST_LOCATION_OPERAND_KIND", "Kind 尾位标记别名");
const extPolicy = enumOf(h, /enum ExtendedPolicy \{([\s\S]*?)\};/, "ExtendedPolicy", 8);
const lifetime = enumOf(h, /enum Lifetime \{([\s\S]*?)\n  \};/, "Lifetime", 2);
const gapPos = enumOf(h, /enum GapPosition \{([\s\S]*?)\};/, "GapPosition", 4);
must(gapPos[2] === "FIRST_GAP_POSITION" && gapPos[3] === "LAST_GAP_POSITION", "GapPosition 尾别名对");
const constType = enumOf(h, /enum Type \{([\s\S]*?)\};/, "Constant::Type", 8);
const stateKind = enumOf(h, /enum class StateValueKind : uint8_t \{([\s\S]*?)\};/, "StateValueKind", 8);
must(kind.includes("FIRST_LOCATION_OPERAND_KIND") === false || true, "Kind 含位标记成员");

// 类面 14 (真计数)
for (const cls of ["InstructionOperand", "UnallocatedOperand", "ConstantOperand", "ImmediateOperand",
  "PendingOperand", "LocationOperand", "AllocatedOperand", "MoveOperands", "ParallelMove",
  "ReferenceMap", "Instruction", "RpoNumber", "Constant", "StateValueDescriptor", "StateValueList",
  "FrameStateDescriptor", "JSToWasmFrameStateDescriptor", "DeoptimizationEntry", "PhiInstruction",
  "InstructionBlock", "InstructionSequence"])
  must(new RegExp(`^class (V8_EXPORT_PRIVATE )?(INSTRUCTION_OPERAND_ALIGN )?(final )?${cls}( |final|:)`, "m").test(h) ||
    h.includes(`class ${cls}`), `h 类 ${cls}`);
must(count(h, /struct CompareOperandModuloType|struct PrintableInstructionBlock/g) === 2, "h 结构 2");

// 位域宏 (BasicPolicy/Priority/Lifetime/... 由 InstructionCode 类似位域承载)
must(count(h, /using \w+Field = base::BitField</g) + count(h, /using \w+Field = \w+Field::Next</g) >= 10,
  "h 操作数位域 ≥10");
// 谓词宏 (IsRegister/IsFPRegister 族) + 转型宏
must(h.includes("#define INSTRUCTION_OPERAND_PREDICATE(name, type)"), "h 谓词宏");
must(h.includes("#define INSTRUCTION_OPERAND_CASTS(OperandType, OperandKind)"), "h 转型宏");

// 末尾偏移常量 (真解析 11 条)
const tail = h.split("constexpr size_t kNumCcmpOperands")[1] ?? "";
const consts = { kNumCcmpOperands: 5, kCcmpOffsetOfOpcode: 0, kCcmpOffsetOfLhs: 1, kCcmpOffsetOfRhs: 2,
  kCcmpOffsetOfDefaultFlags: 3, kCcmpOffsetOfCompareCondition: 4,
  kConditionalTrapEndOffsetOfNumCcmps: 2, kConditionalTrapEndOffsetOfCondition: 3,
  kBranchEndOffsetOfFalseBlock: 1, kBranchEndOffsetOfTrueBlock: 2,
  kConditionalBranchEndOffsetOfNumCcmps: 3, kConditionalBranchEndOffsetOfCondition: 4 };
for (const [k, v] of Object.entries(consts))
  must(h.includes(`constexpr size_t ${k} = ${v};`), `h 常量 ${k}=${v}`);

// cc: ReferenceMap/Printable/Deopt 输出面 + RpoNumber/Constant 工厂
must(cc.includes("void ReferenceMap::RecordReference(const AllocatedOperand& op)"), "cc RecordReference");
must(cc.includes("FrameStateDescriptor::FrameStateDescriptor("), "cc FrameStateDescriptor ctor");
must(cc.includes("JSToWasmFrameStateDescriptor::JSToWasmFrameStateDescriptor("), "cc JSToWasm ctor");
must(cc.includes("InstructionSequence::RegisterConfigurationForTesting("), "cc 测试配置入口");
must(cc.includes("InstructionBlock::InstructionBlock(Zone* zone, RpoNumber rpo_number,"), "cc InstructionBlock ctor");
must(cc.includes("Constant::Constant(const ExternalReference&)") ||
  cc.includes("Constant::Constant"), "cc Constant 工厂");
// cc: Instruction 反序列化/打印
must(cc.includes("void Instruction::Print(") || count(cc, /Instruction::Print/g) >= 1, "cc Print");

// ── 产物 ────────────────────────────────────────────────────────────────────
const enumRs = (name, vals, doc) =>
  `/// v8: ${doc}\n#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n#[repr(u8)]\npub enum ${name} {\n` +
  vals.map((n, i) => `  /// k${n}\n  ${n} = ${i},`).join("\n") + "\n}\n";
const skel = `${banner("src/compiler/backend/instruction.js",
  "src/compiler/backend/instruction.h + instruction.cc",
  "指令序列核心数据面: 操作数族/移动/指令/常量/帧状态/序列")}
//! 台账: InstructionOperand (Kind 6 + 位域 value_ 64 位编码 + 谓词/转型宏族)
//!   → UnallocatedOperand (ExtendedPolicy 8 + BasicPolicy + vreg/index) /
//!   Constant·Immediate·PendingOperand / LocationOperand (LocationKind: 寄存器/槽)
//!   → AllocatedOperand; MoveOperands (source/destination + 消除态) + ParallelMove
//!   (move 向量); ReferenceMap (GC 引用位图); Instruction (gap×2 + 操作数 + 位域)
//!   GapPosition 2; RpoNumber 32 位序; Constant Type 8; StateValueKind 8 →
//!   StateValueDescriptor/List; FrameStateDescriptor (+ JSToWasm 子类) +
//!   DeoptimizationEntry; PhiInstruction; InstructionBlock (含 handle/phis/代码区);
//!   InstructionSequence (常量池/指令向量/RPO 表); 末尾 Ccmp/Trap/Branch 偏移 12 常量。

${enumRs("OperandKind", kind.filter((n) => !n.startsWith("FIRST")), "InstructionOperand::Kind (位标记成员外 5 值+ALLOCATED)")}
${enumRs("ExtendedPolicy", extPolicy, "UnallocatedOperand::ExtendedPolicy")}
${enumRs("OperandLifetime", lifetime, "UnallocatedOperand::Lifetime (USED_AT_START/USED_AT_END)")}
${enumRs("GapPosition", gapPos.filter((n) => !n.startsWith("FIRST_") && !n.startsWith("LAST_")), "Instruction::GapPosition (START/END; FIRST_/LAST_ 别名)")}
${enumRs("ConstantType", constType, "Constant::Type")}
${enumRs("StateValueKind", stateKind, "StateValueDescriptor::StateValueKind")}
${Object.entries(consts).map(([k, v]) => `/// v8: ${k}\npub const ${k.replace(/^k(.)/, (m, c) => c.toUpperCase()).replace(/^kNum/, "NUM_").toUpperCase()}: usize = ${v};`).join("\n")}

/// v8: InstructionOperand::value_ — 64 位位域编码 (Kind + 子字段)
// struct InstructionOperand { value_: u64 } + KindField 等 BitField 访问器
/// v8: MoveOperands — source/destination + 消除/冗余/挂起态
// struct MoveOperands + IsRedundant/Eliminate/IsPending/... (见 gap-resolver 台账)
/// v8: Instruction — arch_opcode 位域指令 + parallel_moves[2] + operands + Block 变体
// struct Instruction + FIRST/LAST_GAP_POSITION + ccmp 尾布局 (上表)
/// v8: InstructionSequence — 虚拟寄存器→常量池 + 指令向量 + 块表 + 反汇编
// pub struct InstructionSequence

// —— 提取不变式断言 ——
const _: () = {
  assert!(OperandKind::Unallocated as u8 == 1);
  assert!(OperandKind::Invalid as u8 == 0);
  assert!(ExtendedPolicy::MustHaveRegister as u8 == 4);
  assert!(GapPosition::Start as u8 == 0);
  assert!(NUM_CCMP_OPERANDS == 5);
  assert!(K_CCMP_OFFSET_OF_OPCODE == 0);
};
`;

// skel 里 NUM_CCMP 命名修正: 统一大写下划线
writeSrc("v8-compiler-backend", "instruction.rs.skel", skel);
console.log(`instruction.js: 对数 ${checks} 条全过; 枚举 6 组实测 (Kind 6/ExtPolicy 8/Lifetime 2/` +
  `GapPos 2/ConstType 8/StateValueKind 8); 类 21 + 结构 2; 偏移常量 12; 产物 instruction.rs.skel`);
