#!/usr/bin/env bun
// tool/src/compiler/turboshaft/graph-builder.h.js
// 机械提取 v8/src/compiler/turboshaft/graph-builder.{h,cc}
// → v8-turboshaft/src/graph_builder.rs.skel
//
// 提取面: BuildGraph 入口签名 + cc 内 GraphBuilder 结构体 (2956 行文件,
// 提取类面与关键方法锚) + IsValidSmi + bailout 协议。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/graph-builder.h");
const cc = readV8("src/compiler/turboshaft/graph-builder.cc");

// 1. h: 入口声明
if (!/std::optional<BailoutReason> BuildGraph\(PipelineData\* data, Schedule\* schedule,\s*Zone\* phase_zone, Linkage\* linkage\);/.test(h)) {
  throw new Error("h BuildGraph 声明不匹配");
}
for (const [re, label] of [
  [/class Schedule;/, "Schedule 前向声明"],
  [/class SourcePositionTable;/, "SourcePositionTable 前向声明"],
  [/class TFPipelineData;/, "TFPipelineData 前向声明"],
  [/class PipelineData;/, "PipelineData 前向声明"],
]) {
  if (!re.test(h)) throw new Error(`h 哨兵缺失: ${label}`);
}

// 2. cc: 匿名命名空间三件套
const structM = cc.match(/struct GraphBuilder\s*\{/);
if (!structM) throw new Error("cc 未找到 struct GraphBuilder");
if (!/^bool IsValidSmi\(intptr_t c\)/m.test(cc)) throw new Error("cc 未找到 IsValidSmi");
const tailM = cc.match(/std::optional<BailoutReason> BuildGraph\(PipelineData\* data, Schedule\* schedule,\s*Zone\* phase_zone, Linkage\* linkage\)\s*\{/);
if (!tailM) throw new Error("cc 未找到 BuildGraph 定义");

// 3. GraphBuilder 方法面 (public 方法抽样锚, 逆序遍历调度块)
for (const [re, label] of [
  [/Schedule& schedule;/, "schedule 字段 (Schedule&)"],
  [/struct BlockData/, "BlockData 结构"],
  [/struct ThrowingScope/, "ThrowingScope 结构"],
  [/BasicBlock\* block;/, "BasicBlock 字段"],
  [/BailoutReason|kBailout/, "bailout 协议"],
  [/source_positions/i, "source positions 传递"],
  [/void BuildFrameStateData\(FrameStateData::Builder\* builder,/, "BuildFrameStateData 方法族"],
  [/void ProcessStateValues\(/, "ProcessStateValues"],
  [/void ProcessDeoptInput\(/, "ProcessDeoptInput"],
]) {
  if (!re.test(cc)) throw new Error(`cc 结构锚缺失: ${label}`);
}

// 方法数量对数: struct GraphBuilder 内 void/bool/V<> 方法
const gbBody = cc.slice(structM.index, structM.index + 200000);
const methodCount = [...gbBody.matchAll(/^\s{2}(?:void|bool|OpIndex|V<\w+>|std::optional<BailoutReason>|MaybeConstOrV<\w+>) (\w+)\(/gm)].map((m) => m[1]);
const uniq = [...new Set(methodCount)];
if (JSON.stringify(uniq) !== JSON.stringify(["Run", "Map", "FixLoopPhis", "ProcessDeoptInput", "ProcessStateValues", "BuildFrameStateData", "Process"])) {
  throw new Error(`GraphBuilder 方法面 ${JSON.stringify(uniq)} != 预期 7 项`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/graph-builder.h.js",
  "src/compiler/turboshaft/graph-builder.{h,cc}",
  "Turbofan Schedule → Turboshaft Graph 构建器骨架",
);
rust += `
/// v8: src/compiler/turboshaft/graph-builder.h BuildGraph
///
/// 原版 std::optional<BailoutReason> BuildGraph(PipelineData*, Schedule*,
/// Zone* phase_zone, Linkage* linkage)
/// 实现: cc 尾部 — 构造匿名 ns 的 GraphBuilder 后逐块构建。
pub fn build_graph(
  data: &mut crate::phase::PipelineData,
  schedule: &mut crate::phase::Schedule,
  phase_zone: *mut crate::phase::Zone,
  linkage: &mut crate::phase::Linkage,
) -> Option<crate::phase::BailoutReason> {
  unimplemented!("Developer 修订")
}

/// v8: src/compiler/turboshaft/graph-builder.cc IsValidSmi
pub fn is_valid_smi(c: isize) -> bool {
  unimplemented!("Developer 修订")
}

/// v8: src/compiler/turboshaft/graph-builder.cc GraphBuilder
///
/// 匿名命名空间结构体; 方法面 (顶层全部 ${uniq.length} 项):
${uniq.map((m) => `/// - ${m}`).join("\n")}
/// (完整清单见原版; 语义面 Developer 修订)
pub struct GraphBuilder {
  pub placeholder: (),
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(true); // 方法面 =7 由脚本侧断言
};
`;

const p = writeSrc("v8-turboshaft", "graph_builder.rs.skel", rust);
console.log(`written ${p}`);
console.log(`入口=1 IsValidSmi=1 GraphBuilder方法抽样=${uniq.length} 哨兵=11`);
