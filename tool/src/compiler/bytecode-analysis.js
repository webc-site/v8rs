#!/usr/bin/env bun
// tool/src/compiler/bytecode-analysis.js
// 从 v8/src/compiler/bytecode-analysis.h + bytecode-analysis.cc
// 提取循环分析结构 (BytecodeLoopAssignments/ResumeJumpTarget/LoopInfo/BytecodeAnalysis)
// 生成 v8-compiler-frontend/src/bytecode_analysis.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/bytecode-analysis.h");
const cc = readV8("src/compiler/bytecode-analysis.cc");

// —— 1. 锚点断言 (防源漂移) ——
const anchors = [
  ["class V8_EXPORT_PRIVATE BytecodeLoopAssignments", "BytecodeLoopAssignments 类"],
  ["class V8_EXPORT_PRIVATE ResumeJumpTarget", "ResumeJumpTarget 类"],
  ["struct V8_EXPORT_PRIVATE LoopInfo", "LoopInfo 结构"],
  ["class V8_EXPORT_PRIVATE BytecodeAnalysis : public ZoneObject", "BytecodeAnalysis 类"],
  ["static ResumeJumpTarget Leaf(int suspend_id, int target_offset);", "Leaf 工厂"],
  ["static ResumeJumpTarget AtLoopHeader(", "AtLoopHeader 工厂"],
  ["bool is_leaf() const { return target_offset_ == final_target_offset_; }", "is_leaf 判据"],
  ["return offset >= loop_start_ && offset < loop_end_;", "LoopInfo::Contains 半开区间"],
  ["int local_count() const { return bit_vector_->length() - parameter_count_; }", "local_count 公式"],
  ["// The offset to the parent loop, or -1 if there is no parent.", "parent_offset=-1 语义"],
  ["LoopInfo(parent_offset, loop_start, loop_end, -1, parameter_count,", "无 JumpLoop 构造 -1"],
  ["class BytecodeAnalysisImpl;", "Impl 前置"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`bytecode-analysis.h 锚点缺失: ${name}`);
}
const ccAnchors = [
  ["void BytecodeAnalysis::BytecodeAnalysisImpl::Analyze() {", "Analyze 主入口"],
  ["bool BytecodeAnalysis::IsLoopHeader(int offset) const {", "IsLoopHeader 实现"],
  ["int BytecodeAnalysis::GetLoopOffsetFor(int offset) const {", "GetLoopOffsetFor 实现"],
  ["int BytecodeAnalysis::GetLoopEndOffsetForInnermost(int header_offset) const {", "GetLoopEndOffsetForInnermost 实现"],
  ["const BytecodeLivenessState* BytecodeAnalysis::GetInLivenessFor(", "GetInLivenessFor 实现"],
  ["const BytecodeLivenessState* BytecodeAnalysis::GetOutLivenessFor(", "GetOutLivenessFor 实现"],
];
for (const [lit, name] of ccAnchors) {
  if (!cc.includes(lit)) throw new Error(`bytecode-analysis.cc 锚点缺失: ${name}`);
}

// —— 2. 真解析: h 中各类的方法签名 ——
const grab = (src, from, to) => {
  const i = src.indexOf(from);
  if (i < 0) throw new Error(`定位失败: ${from}`);
  const j = src.indexOf(to, i);
  if (j < 0) throw new Error(`定位失败: ${to}`);
  return src.slice(i, j);
};
// BytecodeAnalysis 公有 API 签名 (private: 之前)
const publicApi = grab(h, "class V8_EXPORT_PRIVATE BytecodeAnalysis", " private:");
const publicSigs = [...publicApi.matchAll(/(bool|int|const LoopInfo&|const LoopInfo\*|const base::Vector<const LoopInfo>&|const ZoneVector<ResumeJumpTarget>&|const BytecodeLivenessState\*|BytecodeOffset)\s+(\w+)\(([^)]*)\) const\s*[;{]/g)]
  .map((m) => ({ ret: m[1], name: m[2] }));
if (publicSigs.length < 12) throw new Error(`BytecodeAnalysis 公有查询面提取不足: ${publicSigs.length}`);

// —— 3. .cc 定义面计数 (三类分计: LoopAssignments / ResumeJumpTarget / BytecodeAnalysis+Impl) ——
const countDefs = (cls) => [...cc.matchAll(new RegExp(`^[A-Za-z][^;]*${cls}[\\w:]*::\\w+\\(`, "gm"))].length
  + [...cc.matchAll(new RegExp(`^${cls}[\\w:]*::\\w+\\(`, "gm"))].length;
const loopAssignDefs = countDefs("BytecodeLoopAssignments");     // 构造+Add+AddList+Union+Contains×2
const resumeDefs = countDefs("ResumeJumpTarget");                // 构造+Leaf+AtLoopHeader
const analysisDefs = countDefs("BytecodeAnalysis");              // 顶层查询 + 内嵌 Impl (含跨行/模板变体)
if (loopAssignDefs < 6 || resumeDefs < 3 || analysisDefs < 10) {
  throw new Error(`cc 定义面提取不足: loop=${loopAssignDefs} resume=${resumeDefs} analysis=${analysisDefs}`);
}
const implFns = [];
for (const cls of ["BytecodeLoopAssignments", "ResumeJumpTarget", "BytecodeAnalysis"]) {
  for (const m of cc.matchAll(new RegExp(`^[A-Za-z][^;]*${cls}[\\w:]*::(\\w+)\\(`, "gm"))) implFns.push(m[1]);
  for (const m of cc.matchAll(new RegExp(`^${cls}[\\w:]*::\\w+\\(`, "gm"))) implFns.push(m[0].match(/::(\w+)\(/)[1]);
}
const implMethodNames = new Set(implFns);

// —— 4. LoopInfo 字段 ——
const loopInfoFields = [
  "parent_offset_", "loop_start_", "loop_end_", "jump_loop_offset_",
  "resumable_", "innermost_", "assignments_", "resume_jump_targets_",
];

const body = `${banner("src/compiler/bytecode-analysis.js", "src/compiler/bytecode-analysis.h + bytecode-analysis.cc", "字节码循环分析: 循环范围/嵌套/赋值集/活性")}
//! BytecodeAnalysis — 分析字节码找循环范围、循环嵌套、循环赋值与活性。
//! OSR 分析 (osr_bailout_id 非 None) 子sumes 非 OSR 分析 (broker/serializer 依赖)。

/// 循环变量赋值集 — 位向量: [0, parameter_count) 参数 / 其余局部
/// v8: src/compiler/bytecode-analysis.h BytecodeLoopAssignments
pub struct BytecodeLoopAssignments {
    parameter_count: i32,
    /// local_count = bit_vector.length - parameter_count
    /// v8: src/compiler/bytecode-analysis.h BytecodeLoopAssignments::local_count
    bit_vector: BitVector,
}

impl BytecodeLoopAssignments {
    /// v8: src/compiler/bytecode-analysis.cc BytecodeLoopAssignments::Add (Register)
    pub fn add(&mut self, r: Register);
    /// v8: src/compiler/bytecode-analysis.cc BytecodeLoopAssignments::AddList
    pub fn add_list(&mut self, r: Register, count: u32);
    /// v8: src/compiler/bytecode-analysis.cc BytecodeLoopAssignments::Union
    pub fn union_(&mut self, other: &BytecodeLoopAssignments);
    /// v8: src/compiler/bytecode-analysis.cc BytecodeLoopAssignments::ContainsParameter
    pub fn contains_parameter(&self, index: i32) -> bool;
    /// v8: src/compiler/bytecode-analysis.cc BytecodeLoopAssignments::ContainsLocal
    pub fn contains_local(&self, index: i32) -> bool;
}

/// 挂起生成器的恢复跳转目标
/// v8: src/compiler/bytecode-analysis.h ResumeJumpTarget
pub struct ResumeJumpTarget {
    /// 恢复的 suspend id
    suspend_id: i32,
    /// 本恢复跳转的目标偏移
    target_offset: i32,
    /// 跨多次跳转后的最终偏移; is_leaf = target_offset == final_target_offset
    /// v8: src/compiler/bytecode-analysis.h ResumeJumpTarget::is_leaf
    final_target_offset: i32,
}

impl ResumeJumpTarget {
    /// 实际恢复点
    /// v8: src/compiler/bytecode-analysis.cc ResumeJumpTarget::Leaf
    pub fn leaf(suspend_id: i32, target_offset: i32) -> Self;
    /// 循环头处恢复 — 跨循环头后还有一次恢复跳转
    /// v8: src/compiler/bytecode-analysis.cc ResumeJumpTarget::AtLoopHeader
    pub fn at_loop_header(loop_header_offset: i32, next: &ResumeJumpTarget) -> Self;
}

/// 单个循环信息
/// v8: src/compiler/bytecode-analysis.h LoopInfo
pub struct LoopInfo {
    /// 父循环偏移, 无父为 -1
    parent_offset: i32,
    /// 半开区间 [loop_start, loop_end)
    /// v8: src/compiler/bytecode-analysis.h LoopInfo::Contains
    loop_start: i32,
    loop_end: i32,
    /// 无 JumpLoop 时为 -1 (双构造函数协议)
    jump_loop_offset: i32,
    resumable: bool,   // = false
    innermost: bool,   // = true
    assignments: BytecodeLoopAssignments,
    resume_jump_targets: Vec<ResumeJumpTarget>,
}

/// 字节码分析主类
/// v8: src/compiler/bytecode-analysis.h BytecodeAnalysis
pub struct BytecodeAnalysis {
    osr_bailout_id: BytecodeOffset,
    analyze_liveness: bool,
    resume_jump_targets: Vec<ResumeJumpTarget>,
    /// loop_end → loop_header 映射
    end_to_header: ZoneMap<i32, i32>,
    /// 按 loop header 偏移升序
    loop_infos: Vec<LoopInfo>,
    osr_entry_point: i32,
    liveness_map: Option<BytecodeLivenessMap>,
    /// 字节码操作数, 初值 -1
    bytecode_count: i32,
}

impl BytecodeAnalysis {
${publicSigs.map((s) => `    /// v8: src/compiler/bytecode-analysis.h BytecodeAnalysis::${s.name}\n    pub fn ${s.name}(&self${s.ret === "bool" ? "" : ""});`).join("\n")}
}

// —— 对数基准 (原版 .cc 定义面, 脚本计数) ——
// LoopAssignments 定义 ${loopAssignDefs} / ResumeJumpTarget 定义 ${resumeDefs}
// BytecodeAnalysis(+Impl) 定义 ${analysisDefs} / 去重方法 ${implMethodNames.size}
// BytecodeAnalysis 公有查询: ${publicSigs.length} 个
// LoopInfo 字段: ${loopInfoFields.length} 个
`;

const p = writeSrc("v8-compiler-frontend", "bytecode_analysis.rs.skel", body);
console.log(`✓ bytecode-analysis: LoopAssignments ${loopAssignDefs} / Resume ${resumeDefs} / Analysis ${analysisDefs} / 公有查询 ${publicSigs.length} / LoopInfo 字段 ${loopInfoFields.length} / 锚点 ${anchors.length + ccAnchors.length}`);
