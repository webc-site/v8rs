#!/usr/bin/env bun
// tool/src/compiler/turboshaft/csa-effects-computation.h.js
// 机械提取 v8/src/compiler/turboshaft/csa-effects-computation.{h,cc}
// → v8-turboshaft/src/csa_effects_computation.rs.skel
//
// 提取面: CsaEffectsComputationPhase (非 CopyingPhase 的分析 phase) +
// cc 侧 CallView 统一视图 / TryGetBuiltin / AnalyzeCall 三件套签名
// + "builtin 可分配" 判定三规则。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/csa-effects-computation.h");
const cc = readV8("src/compiler/turboshaft/csa-effects-computation.cc");

// 1. phase 结构
const structM = h.match(/struct (\w+Phase)\s*\{([\s\S]*?)\n\};/);
if (!structM) throw new Error("未找到 phase struct");
if (structM[1] !== "CsaEffectsComputationPhase") throw new Error(`struct 名 ${structM[1]} != CsaEffectsComputationPhase`);
const declM = structM[2].match(/DECL_TURBOSHAFT_PHASE_CONSTANTS(?:_WITH_LEGACY_NAME)?\((\w+)\)/);
if (!declM) throw new Error("未找到 DECL_TURBOSHAFT_PHASE_CONSTANTS");
if (declM[1] !== "CsaEffectsComputation") throw new Error(`DECL 名 ${declM[1]} != CsaEffectsComputation`);
const phaseName = `V8.TFTurboshaft${declM[1]}`;

// 2. cc 侧三件套 + Run 主体锚点
for (const [re, label] of [
  [/struct CallView\s*\{/, "CallView"],
  [/explicit CallView\(const Operation& op\)/, "CallView 构造 (CallOp/TailCallOp 分派)"],
  [/std::optional<Builtin> TryGetBuiltin\(V<Any> node, Graph& graph\)/, "TryGetBuiltin"],
  [/void AnalyzeCall\(CallView call, std::unordered_set<Builtin>& called_builtins,\s*BuiltinAllocateEffect\* can_allocate, Graph& graph,\s*Builtin current_builtin\)/, "AnalyzeCall"],
  [/void CsaEffectsComputationPhase::Run\(PipelineData\* data, Zone\* temp_zone\)/, "Run 定义"],
  [/data->graph\(\)\.AllOperations\(\)/, "全操作迭代"],
  [/RegisterInitialEffects/, "RegisterInitialEffects 登记"],
  [/Runtime::kCanTriggerGC/, "Runtime::kCanTriggerGC 表引用"],
]) {
  if (!re.test(cc)) throw new Error(`哨兵缺失: ${label}`);
}

// CallView 分派的 opcode 分支 (对数=2: kCall/kTailCall)
const callSwitch = cc.match(/struct CallView\s*\{[\s\S]*?switch \(op\.opcode\)\s*\{([\s\S]*?)default:/);
if (!callSwitch) throw new Error("CallView switch 未找到");
const opcodes = [...callSwitch[1].matchAll(/case Opcode::k(\w+):/g)].map((m) => m[1]);
if (opcodes.length !== 2 || opcodes[0] !== "Call" || opcodes[1] !== "TailCall") {
  throw new Error(`CallView 分派 ${JSON.stringify(opcodes)} != [Call, TailCall]`);
}

// 3. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/csa-effects-computation.h.js",
  "src/compiler/turboshaft/csa-effects-computation.{h,cc}",
  "builtin 分配效应分析 phase: CallView/TryGetBuiltin/AnalyzeCall 骨架",
);
rust += `
/// v8: src/compiler/turboshaft/csa-effects-computation.h CsaEffectsComputationPhase
pub struct CsaEffectsComputationPhase;

impl CsaEffectsComputationPhase {
  /// 原版 kPhaseName = "V8.TF" "Turboshaft" "CsaEffectsComputation"
  pub const PHASE_NAME: &'static str = "${phaseName}";

  /// 原版 void Run(PipelineData* data, Zone* temp_zone)
  /// v8: src/compiler/turboshaft/csa-effects-computation.cc CsaEffectsComputationPhase::Run
  ///
  /// "builtin 可分配" 判定三规则 (原版注释保留):
  /// 1. 图内含 AllocateOp
  /// 2. 调用了可触发 GC 的 runtime 函数 (Runtime::kCanTriggerGC; Abort 除外)
  /// 3. 调用了可分配的其它 builtin (递归, 由 BuiltinsEffectsAnalyzer::Finalize 定点)
  pub fn run(data: &mut crate::phase::PipelineData, temp_zone: &mut crate::phase::Zone) {
    unimplemented!("Developer 修订: 遍历 AllObjects + AnalyzeCall + RegisterInitialEffects")
  }
}

/// v8: src/compiler/turboshaft/csa-effects-computation.cc CallView
/// CallOp / TailCallOp 统一视图 (分派 ${opcodes.length} 分支: ${opcodes.join(" / ")})
pub struct CallView {
  /// target: V<CallTarget>
  pub target: (),
  /// arguments: base::Vector<const OpIndex>
  pub arguments: (),
  /// descriptor: const TSCallDescriptor*
  pub descriptor: (),
}

/// v8: src/compiler/turboshaft/csa-effects-computation.cc TryGetBuiltin
/// HeapConstant<Code> 且 is_builtin → builtin_id
pub fn try_get_builtin(node: crate::index::OpIndex, graph: &crate::graph::Graph) -> Option<u16> {
  unimplemented!("签名骨架, Developer 修订")
}

/// v8: src/compiler/turboshaft/csa-effects-computation.cc AnalyzeCall
/// 三分支: builtin 目标 / ExternalConstant 目标 (不可分配) / 其它 (可分配)
pub fn analyze_call(
  call: &CallView,
  called_builtins: &mut std::collections::HashSet<u16>,
  can_allocate: &mut crate::phase::BuiltinAllocateEffect,
  graph: &crate::graph::Graph,
  current_builtin: u16,
) {
  unimplemented!("签名骨架, Developer 修订")
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(CsaEffectsComputationPhase::PHASE_NAME == "V8.TFTurboshaftCsaEffectsComputation");
};
`;

const p = writeSrc("v8-turboshaft", "csa_effects_computation.rs.skel", rust);
console.log(`written ${p}`);
console.log(`phase=1 decl=${declM[1]} CallView分派=${opcodes.length} (Call/TailCall) 函数面=3`);
