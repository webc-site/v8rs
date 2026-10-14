#!/usr/bin/env bun
// tool/src/compiler/turboshaft/phase.h.js
// 机械提取 v8/src/compiler/turboshaft/phase.{h,cc}
// → v8-turboshaft/src/phase.rs
//
// 提取面: TurboshaftPipelineKind 枚举; DECL_TURBOSHAFT_PHASE_CONSTANTS 宏的
// 命名约定 (kPhaseName = "V8.TF" + CallStatsName); PipelineData 组件面登记。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/phase.h");
const cc = readV8("src/compiler/turboshaft/phase.cc");

// 1. TurboshaftPipelineKind
const pkM = h.match(/enum class TurboshaftPipelineKind\s*\{([^}]+)\}/);
if (!pkM) throw new Error("未找到 TurboshaftPipelineKind");
const pipelineKinds = [...pkM[1].replace(/\/\/[^\n]*/g, "").matchAll(/\bk(\w+)\b/g)].map((m) => m[1]);
if (pipelineKinds.length !== 5) throw new Error(`PipelineKind ${pipelineKinds.length} != 5`);

// 2. phase 命名约定哨兵
for (const [re, name] of [
  [/static constexpr char kPhaseName\[\] = "V8\.TF" #CallStatsName;/, "kPhaseName 命名约定"],
  [/void PrintTurboshaftGraph\(/, "PrintTurboshaftGraph"],
  [/void InitializeRegisterComponent\(/, "InitializeRegisterComponent"],
  [/class V8_EXPORT_PRIVATE PipelineData/, "PipelineData"],
]) {
  if (!(re.test(h) || re.test(cc))) throw new Error(`哨兵缺失: ${name}`);
}

// PipelineData 组件面
const components = [...h.matchAll(/using (\w+Component) = detail::\w+;/g)].map((m) => m[1]);
if (components.length !== 5) throw new Error(`组件 ${components.length} != 5`);

// 3. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/phase.h.js",
  "src/compiler/turboshaft/phase.{h,cc}",
  "Phase 基础: 流水线种类枚举与 phase 命名约定",
);

rust += `
/// 原版 TurboshaftPipelineKind — 流水线种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TurboshaftPipelineKind {
`;
pipelineKinds.forEach((n, i) => {
  rust += `  /// k${n}\n  ${n} = ${i},\n`;
});
rust += `}

/// 原版 DECL_TURBOSHAFT_PHASE_CONSTANTS(Name) 命名约定:
/// 每个 phase 静态 kPhaseName = "V8.TF" + CallStatsName (tracing 名);
/// 带 legacy 名变体另设 kLegacyPhaseName。Rust 侧以 trait 承载:
pub trait Phase {
  /// 原版 kPhaseName ("V8.TF"+CallStatsName)
  const PHASE_NAME: &'static str;
  /// 原版 Run(Phase, Args...) — phase 主入口
  fn run();
}

/// 原版 PipelineData 组件面 (detail 命名空间, 生命周期随编译 job):
${components.map((c) => `/// - ${c}`).join("\n")}
///
/// 方法面登记 (phase.cc):
/// - InitializeRegisterComponent(config, call_descriptor) — 寄存器分配组件
/// - InitializeBrokerAndDependencies(broker, dependencies) — broker 装载
/// - allocator() — isolate 或 wasm engine 的分配器
/// - PrintTurboshaftGraph(data, code_tracer, phase_name) — 可视化
pub struct PipelineData {
  /// pipeline_kind_
  pub pipeline_kind: TurboshaftPipelineKind,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(TurboshaftPipelineKind::JS as u8 == 0);
  assert!(TurboshaftPipelineKind::JSToWasm as u8 == 4);
};
`;

const p = writeSrc("v8-turboshaft", "phase.rs", rust);
console.log(`written ${p}`);
console.log(`PipelineKind=${pipelineKinds.length} 组件=${components.length}`);
