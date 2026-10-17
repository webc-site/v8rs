#!/usr/bin/env bun
// tool/src/compiler/turboshaft/turbolev-graph-builder.h.js
// 机械提取 v8/src/compiler/turboshaft/turbolev-graph-builder.{h,cc}
// → v8-turboshaft/src/turbolev_graph_builder.rs.skel
//
// 提取面: TurbolevGraphBuildingPhase 签名; GraphBuildingNodeProcessor /
// NodeProcessorBase 结构; maglev Process 覆盖面计数; bailout 通道。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/turbolev-graph-builder.h");
const cc = readV8("src/compiler/turboshaft/turbolev-graph-builder.cc");

// 1. 哨兵
for (const [re, name] of [
  [/struct TurbolevGraphBuildingPhase \{/, "phase struct"],
  [/std::optional<BailoutReason> Run\(PipelineData\* data, Zone\* temp_zone,\s*\n\s*maglev::Graph\* maglev_graph\);/, "Run 签名"],
  [/class NodeProcessorBase : public GraphBuildingNodeProcessor/, "NodeProcessorBase"],
  [/std::optional<BailoutReason>\* bailout_;/, "bailout 通道"],
  [/maglev::GraphProcessor<NodeProcessorBase> builder\(/, "GraphProcessor 驱动"],
  [/builder.ProcessGraph\(maglev_graph\);/, "ProcessGraph"],
  [/data->graph\(\).Reset\(\);/, "bailout 后重置"],
  [/inlined_functions\(\)\.push_back\(holder\)/, "内联函数拷贝"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${name}`);
}

// 2. maglev Process 覆盖面
const processes = [...cc.matchAll(/maglev::ProcessResult Process\(maglev::(\w+)\* node,/g)].map((m) => m[1]);
const uniqueNodes = [...new Set(processes)];
if (processes.length !== 289) throw new Error(`Process 覆盖 ${processes.length} != 289`);

// 3. BailoutReason 面
const bailouts = [...new Set([...cc.matchAll(/BailoutReason::k(\w+)/g)].map((m) => m[1]))];

let rust = banner(
  "src/compiler/turboshaft/turbolev-graph-builder.h.js",
  "src/compiler/turboshaft/turbolev-graph-builder.{h,cc}",
  "Turbolev 图构建: maglev 图 -> Turboshaft 图",
);

rust += `
/// 原版 TurbolevGraphBuildingPhase — maglev 图转 Turboshaft 图的总入口
/// v8: src/compiler/turboshaft/turbolev-graph-builder.h
pub struct TurbolevGraphBuildingPhase;

impl TurbolevGraphBuildingPhase {
  /// v8: cc Run —
  /// - maglev::GraphProcessor<NodeProcessorBase> 驱动 ProcessGraph
  /// - 拷贝 maglev inlined_functions 到 compilation info
  /// - node_origins 存在时解码 maglev 外部 id 记录来源
  /// - bailout 有值且 trace_turbo 时 Reset 图 (图可能处于非法态)
  /// 返回 Some(BailoutReason) 表示放弃
  pub fn run(&mut self, data: &mut PipelineData, temp_zone: &mut Zone,
             maglev_graph: &mut MaglevGraph) -> Option<BailoutReason> {
    todo!()
  }
}

/// 原版 NodeProcessorBase — GraphBuildingNodeProcessor 的包装:
/// 跳过 Unreachable 代码内节点 + 记录 source positions;
/// 持 std::optional<BailoutReason>* bailout_ 外发通道。
/// v8: turbolev-graph-builder.cc NodeProcessorBase
pub struct NodeProcessorBase {
  pub bailout: Option<BailoutReason>,
}

/// maglev 节点覆盖面: ${processes.length} 个 Process 重载
/// (${uniqueNodes.length} 个唯一 maglev 节点类型)。
/// GraphBuildingNodeProcessor 本体还含:
/// - 常量折叠面 (Constant/int/float 系)
/// - 各 bytecodewise 的 state (interpreter register state)
/// v8: turbolev-graph-builder.cc 各 maglev::ProcessResult Process(...)
///
/// 触发的 BailoutReason 面 (${bailouts.length} 种):
${bailouts.map((b) => `/// - k${b}`).join("\n")}
`;

const p = writeSrc("v8-turboshaft", "turbolev_graph_builder.rs.skel", rust);
console.log(`written ${p}`);
console.log(`Process重载=${processes.length} 唯一节点=${uniqueNodes.length} BailoutReason=${bailouts.length}`);
