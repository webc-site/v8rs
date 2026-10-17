#!/usr/bin/env bun
// tool/src/compiler/turboshaft/tracing.h.js
// 机械提取 v8/src/compiler/turboshaft/tracing.h
// → v8-turboshaft/src/tracing.rs.skel
//
// 提取面: Tracing contextual 类; 每操作/每块自定义 JSON 数据打印面。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/tracing.h");

// 1. 哨兵
for (const [re, name] of [
  [/class Tracing : public base::ContextualClass<Tracing> \{/, "Contextual 类"],
  [/using OperationDataPrinter =\s*\n\s*std::function<bool\(std::ostream&, const Graph&, OpIndex\)>;/, "OperationDataPrinter"],
  [/using BlockDataPrinter =\s*\n\s*std::function<bool\(std::ostream&, const Graph&, BlockIndex\)>;/, "BlockDataPrinter"],
  [/inline bool is_enabled\(\) const \{ return info_->trace_turbo_json\(\); \}/, "is_enabled"],
  [/void PrintPerOperationData\(const char\* data_name/, "PrintPerOperationData"],
  [/void PrintPerBlockData\(const char\* data_name/, "PrintPerBlockData"],
  [/PrintTurboshaftCustomDataPerOperation\(json_of, data_name, graph, printer\)/, "per-op 落盘"],
  [/PrintTurboshaftCustomDataPerBlock\(json_of, data_name, graph, printer\)/, "per-block 落盘"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

let rust = banner(
  "src/compiler/turboshaft/tracing.h.js",
  "src/compiler/turboshaft/tracing.h",
  "追踪: --trace-turbo-json 自定义数据打印",
);

rust += `
/// 原版 Tracing — ContextualClass 单例, 持 OptimizedCompilationInfo;
/// is_enabled = info->trace_turbo_json()。
/// v8: src/compiler/turboshaft/tracing.h Tracing
pub struct Tracing<'i> {
  pub info: &'i OptimizedCompilationInfo,
}

/// 原版 OperationDataPrinter — (os, graph, OpIndex) -> bool
/// v8: tracing.h Tracing::OperationDataPrinter
pub type OperationDataPrinter<'g> = Box<dyn FnMut(&mut dyn Write, &'g Graph, OpIndex) -> bool + '_>;

/// 原版 BlockDataPrinter — (os, graph, BlockIndex) -> bool
/// v8: tracing.h Tracing::BlockDataPrinter
pub type BlockDataPrinter<'g> = Box<dyn FnMut(&mut dyn Write, &'g Graph, BlockIndex) -> bool + '_>;

impl<'i> Tracing<'i> {
  /// v8: tracing.h is_enabled
  pub fn is_enabled(&self) -> bool { self.info.trace_turbo_json() }

  /// v8: tracing.h PrintPerOperationData — TurboJsonFile append 模式,
  /// PrintTurboshaftCustomDataPerOperation 逐操作落 JSON
  pub fn print_per_operation_data(&self, data_name: &str, graph: &Graph, printer: OperationDataPrinter) { /* ... */ }

  /// v8: tracing.h PrintPerBlockData — PrintTurboshaftCustomDataPerBlock
  pub fn print_per_block_data(&self, data_name: &str, graph: &Graph, printer: BlockDataPrinter) { /* ... */ }
}
`;

const p = writeSrc("v8-turboshaft", "tracing.rs.skel", rust);
console.log(`written ${p}`);
console.log(`printer面=2 方法=3`);
