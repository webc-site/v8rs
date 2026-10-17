#!/usr/bin/env bun
// tool/src/compiler/js-inlining-heuristic.js
// 从 v8/src/compiler/js-inlining-heuristic.h + js-inlining-heuristic.cc
// 提取 Mode 枚举、kMaxCallPolymorphism、Candidate 结构与三个字节码尺寸阈值
// 生成 v8-compiler-frontend/src/js_inlining_heuristic.rs.skel
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/js-inlining-heuristic.h");
const cc = readV8("src/compiler/js-inlining-heuristic.cc");

// —— 1. 锚点断言 ——
const anchors = [
  ["class JSInliningHeuristic final : public AdvancedReducer {", "JSInliningHeuristic 类"],
  ["enum Mode {", "Mode 枚举"],
  ["kJSOnly,            // Inline JS calls only.", "kJSOnly"],
  ["kWasmWrappersOnly,  // Inline wasm wrappers only.", "kWasmWrappersOnly"],
  ["kWasmFullInlining,  // Inline wasm wrappers and (if supported) whole wasm", "kWasmFullInlining"],
  ["static const int kMaxCallPolymorphism = 4;", "kMaxCallPolymorphism = 4"],
  ["struct Candidate {", "Candidate 结构"],
  ["OptionalJSFunctionRef functions[kMaxCallPolymorphism];", "functions[4]"],
  ["bool can_inline_function[kMaxCallPolymorphism];", "can_inline_function[4]"],
  ["OptionalBytecodeArrayRef bytecode[kMaxCallPolymorphism];", "bytecode[4]"],
  ["struct CandidateCompare {", "CandidateCompare 比较器"],
  ["using Candidates = ZoneSet<Candidate, CandidateCompare>;", "Candidates 集合型"],
  ["enum StateCloneMode { kCloneState, kChangeInPlace };", "StateCloneMode 枚举"],
  ["max_inlined_bytecode_size_cumulative_(", "累计阈值成员"],
  ["max_inlined_bytecode_size_absolute_(", "绝对阈值成员"],
  ["max_inlined_bytecode_size_small_total_(", "小函数总阈值成员"],
  ["DCHECK_EQ(mode == kWasmWrappersOnly || mode == kWasmFullInlining,", "wasm 模式与 module 绑定检查"],
];
for (const [lit, name] of anchors) {
  if (!h.includes(lit)) throw new Error(`js-inlining-heuristic.h 锚点缺失: ${name}`);
}

// —— 2. 真解析: Mode 枚举值 ——
const modeSrc = h.match(/enum Mode \{([\s\S]*?)\};/);
const modes = [...modeSrc[1].matchAll(/^\s*(k\w+),/gm)].map((m) => m[1]);
if (modes.length !== 3) throw new Error(`Mode 应 3 值: ${modes}`);

// —— 3. 真解析: Candidate 成员清单 ——
const candSrc = h.slice(h.indexOf("struct Candidate {"), h.indexOf("struct CandidateCompare"));
const candMembers = [...candSrc.matchAll(/^\s{4}(?:Optional\w+Ref|int|bool|Node\*|CallFrequency)\s+(\w+)/gm)].map((m) => m[1]);

// —— 4. .cc 定义面 ——
const impls = [...new Set([...cc.matchAll(/^[A-Za-z].*JSInliningHeuristic::(\w+)\(/gm)].map((m) => m[1]))];
const flagReads = (h.match(/v8_flags\.max_inlined_bytecode_size_\w+/g) || []).length;

const body = `${banner("src/compiler/js-inlining-heuristic.js", "src/compiler/js-inlining-heuristic.h + js-inlining-heuristic.cc", "内联启发式: 候选收集 + 尺寸阈值 + 多态分派")}

/// 内联模式
/// v8: src/compiler/js-inlining-heuristic.h JSInliningHeuristic::Mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// 只内联 JS 调用
    JsOnly,
    /// 只内联 wasm 包装
    WasmWrappersOnly,
    /// wasm 包装 + (支持时) 整个 wasm 函数
    WasmFullInlining,
}

/// 多态内联上限
/// v8: src/compiler/js-inlining-heuristic.h JSInliningHeuristic::kMaxCallPolymorphism
pub const K_MAX_CALL_POLYMORPHISM: usize = 4;

/// 内联候选 (${candMembers.length} 成员)
/// v8: src/compiler/js-inlining-heuristic.h JSInliningHeuristic::Candidate
pub struct Candidate {
    /// 多态槽 ×4; num_functions==1 && functions[0].is_null() 表示按 SFI 内联
    pub functions: [Option<JSFunctionRef>; K_MAX_CALL_POLYMORPHISM],
    /// 多态时逐函数能否内联
    pub can_inline_function: [bool; K_MAX_CALL_POLYMORPHISM],
    /// 字节码强引用 — 防选点期间被 SFI 冲刷
    pub bytecode: [Option<BytecodeArrayRef>; K_MAX_CALL_POLYMORPHISM],
    pub shared_info: Option<SharedFunctionInfoRef>,
    pub num_functions: i32,
    /// 内联调用点
    pub node: Option<NodeRef>,
    /// 调用点相对频率
    pub frequency: CallFrequency,
    pub total_size: i32,
    pub own_size: i32,
    pub has_heapnumber_params: bool,
}

/// 帧态复制模式
/// v8: src/compiler/js-inlining-heuristic.h JSInliningHeuristic::StateCloneMode
pub enum StateCloneMode {
    /// 复制一份状态 (多态分派用)
    CloneState,
    /// 原地改 (单态重定向)
    ChangeInPlace,
}

/// JSInliningHeuristic — 收集候选, Finalize 时按重要度挑内联
/// v8: src/compiler/js-inlining-heuristic.h JSInliningHeuristic
pub struct JSInliningHeuristic {
    inliner: JSInliner,
    /// 排序唯一候选集 (CandidateCompare)
    candidates: BTreeSet<Candidate>,
    seen: HashSet<NodeId>,
    /// 三个字节码尺寸阈值 (构造时从 v8_flags 快照, 防中途改旗标):
    /// max_inlined_bytecode_size_cumulative / _absolute / _small_total
    thresholds: InliningThresholds,
    mode: Mode,
}

impl JSInliningHeuristic {
    /// total = 已内联 + 已忽略
    /// v8: src/compiler/js-inlining-heuristic.h JSInliningHeuristic::total_inlined_bytecode_size
    pub fn total_inlined_bytecode_size(&self) -> i32;
    /// 候选收尾: 逐个 InlineCandidate (small_function 判定)
    /// v8: src/compiler/js-inlining-heuristic.cc JSInliningHeuristic::Finalize
    fn finalize(&mut self);
}

// —— 对数基准 ——
// Mode ${modes.length} / Candidate 成员 ${candMembers.length} / 阈值旗标读取 ${flagReads}
// cc 定义面 ${impls.length}: ${impls.slice(0, 6).join(",")}...
`;

const p = writeSrc("v8-compiler-frontend", "js_inlining_heuristic.rs.skel", body);
console.log(`✓ js-inlining-heuristic: Mode ${modes.length} / Candidate 成员 ${candMembers.length} / 阈值旗标 ${flagReads} / cc 定义 ${impls.length} / 锚点 ${anchors.length}`);
