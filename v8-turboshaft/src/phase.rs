// 由 tool/src/compiler/turboshaft/phase.h.js 从 v8/src/compiler/turboshaft/phase.{h,cc} 生成; 修订就地改, 复跑覆盖
// Phase 基础: 流水线种类枚举与 phase 命名约定

/// 原版 TurboshaftPipelineKind — 流水线种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum TurboshaftPipelineKind {
  /// kJS
  JS = 0,
  /// kWasm
  Wasm = 1,
  /// kCSA
  CSA = 2,
  /// kTSABuiltin
  TSABuiltin = 3,
  /// kJSToWasm
  JSToWasm = 4,
}

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
/// - BuiltinComponent
/// - GraphComponent
/// - CodegenComponent
/// - InstructionComponent
/// - RegisterComponent
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
