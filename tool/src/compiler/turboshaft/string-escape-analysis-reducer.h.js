#!/usr/bin/env bun
// tool/src/compiler/turboshaft/string-escape-analysis-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/string-escape-analysis-reducer.{h,cc}
// → v8-turboshaft/src/string_escape_analysis_reducer.rs.skel
//
// 提取面: StringEscapeAnalyzer 方法面; ElidedStringPart::Kind;
// reducer 的 StringConcat 消除与 FrameState 重建锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/string-escape-analysis-reducer.h");
const cc = readV8("src/compiler/turboshaft/string-escape-analysis-reducer.cc");

// 1. 分析器方法面 (.h 声明)
const analyzerMethods = [...h.matchAll(/^\s{2}void (\w+)\(/gm)].map((m) => m[1]).filter((n) => !n.startsWith("String"));
const analyzerQueries = [...h.matchAll(/^\s{2}bool (Is\w+|Should\w+)\(/gm)].map((m) => m[1]);
if (analyzerQueries.length !== 2) throw new Error(`查询 ${analyzerQueries.length} != 2`);

// 2. ElidedStringPart::Kind
const kindM = h.match(/struct ElidedStringPart \{[\s\S]*?enum class Kind : uint8_t \{([^}]+)\}/);
if (!kindM) throw new Error("未找到 ElidedStringPart::Kind");
const elidedKinds = [...kindM[1].matchAll(/k(\w+)/g)].map((m) => m[1]);
if (elidedKinds.length !== 2) throw new Error(`ElidedKind ${elidedKinds.length} != 2`);

// 3. 哨兵
for (const [re, name] of [
  [/class StringEscapeAnalyzer \{/, "分析器"],
  [/class StringEscapeAnalysisReducer : public Next/, "reducer"],
  [/TURBOSHAFT_REDUCER_BOILERPLATE\(StringEscapeAnalysis\)/, "boilerplate"],
  [/FixedOpIndexSidetable<bool>\s*\n\s*escaping_operations_and_frame_states_to_reconstruct_;/, "逃逸侧表"],
  [/maybe_non_escaping_string_concats_/, "待复查 concat"],
  [/FrameStateData::Instr/, "FrameState 指令"],
  [/void StringEscapeAnalyzer::Run\(\)/, "cc Run"],
  [/ReprocessStringConcats/, "二扫"],
  [/ComputeFrameStatesToReconstruct/, "FrameState 重建集"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/string-escape-analysis-reducer.h.js",
  "src/compiler/turboshaft/string-escape-analysis-reducer.{h,cc}",
  "字符串逃逸分析: 消除不逃逸的 StringConcat",
);

rust += `
/// 原版 StringEscapeAnalyzer — 输入图逃逸分析
/// v8: string-escape-analysis-reducer.{h,cc} StringEscapeAnalyzer
pub struct StringEscapeAnalyzer<'g> {
  /// 逃逸/需重建侧表 (OpIndex -> bool, 双语义复用)
  pub escaping_operations_and_frame_states_to_reconstruct: Vec<bool>,
  /// 首扫不能定论的 concat (循环 phi), 二扫 ReprocessStringConcats
  pub maybe_non_escaping_string_concats: Vec<V<String>>,
  pub maybe_to_reconstruct_frame_states: Vec<V<AnyFrameState>>,
}

impl<'g> StringEscapeAnalyzer<'g> {
  /// v8: cc Run — ProcessBlock 全图扫描后 ReprocessStringConcats +
  /// ComputeFrameStatesToReconstruct
  pub fn run(&mut self) { /* ... */ }
  /// 内部方法面:
${analyzerMethods.map((m) => `  /// v8: ${m}`).join("\n")}
  /// 查询面: ${analyzerQueries.join(" / ")}
  pub fn is_escaping(&self, idx: OpIndex) -> bool { todo!() }
  pub fn should_reconstruct_frame_state(&self, idx: V<AnyFrameState>) -> bool { todo!() }
}

/// 原版 ElidedStringPart::Kind — 消除后部件两态
/// v8: string-escape-analysis-reducer.h ElidedStringPart::Kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ElidedStringPartKind {
  /// kNotElided: 输出图已有 String
  NotElided,
  /// kElided: 另一个被消除的 StringConcat
  Elided,
}

/// 原版 StringEscapeAnalysisReducer — 消除仅用于拼接/FrameState 的
/// StringConcat, 重建相应 FrameState (DuplicatedId 去重)。
/// v8: string-escape-analysis-reducer.h StringEscapeAnalysisReducer
pub struct StringEscapeAnalysisReducer;
`;

const p = writeSrc("v8-turboshaft", "string_escape_analysis_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`分析器私有方法=${analyzerMethods.length} 查询=${analyzerQueries.length} ElidedKind=${elidedKinds.length}`);
