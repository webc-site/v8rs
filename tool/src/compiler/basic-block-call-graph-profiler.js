#!/usr/bin/env bun
// v8/src/compiler/basic-block-call-graph-profiler.cc + .h
// → v8-compiler-passes/src/basic_block_call_graph_profiler.rs.skel
//
// 规则面: IsBuiltinCall 的分派链 (CallOp/TailCallOp → kCallCodeObject →
// MatchHeapConstant → Is<Code> → is_builtin, 逐条核对) + StoreCallGraph
// 的双层遍历签名; 语义面 (turboshaft 图遍历) 留锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/basic-block-call-graph-profiler.h");
const cc = readV8("src/compiler/basic-block-call-graph-profiler.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class BasicBlockCallGraphProfiler : public AllStatic \{/, "AllStatic 类"],
  [h, /static void StoreCallGraph\(OptimizedCompilationInfo\* info,/, "StoreCallGraph 声明"],
  [cc, /bool IsBuiltinCall\(const turboshaft::Operation& op,/, "IsBuiltinCall"],
  [cc, /ts_descriptor->descriptor->kind\(\) != CallDescriptor::kCallCodeObject/, "kCallCodeObject 门槛"],
  [cc, /matcher\.MatchHeapConstant\(callee_index, &heap_constant\)/, "MatchHeapConstant"],
  [cc, /if \(!Is<Code>\(heap_constant\)\) return false;/, "Is<Code>"],
  [cc, /if \(!code->is_builtin\(\)\) return false;/, "is_builtin"],
  [cc, /profiler->AddBuiltinCall\(info->builtin\(\), called_builtin, block_id\);/, "AddBuiltinCall"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// 分派链顺序: TryCast<CallOp> 在 TryCast<TailCallOp> 之前
if (cc.indexOf("op.TryCast<CallOp>()") > cc.indexOf("op.TryCast<TailCallOp>()")) {
  throw new Error("CallOp/TailCallOp 分派顺序漂移");
}

const body = `${banner("src/compiler/basic-block-call-graph-profiler.js", "src/compiler/basic-block-call-graph-profiler.h + src/compiler/basic-block-call-graph-profiler.cc", "BasicBlockCallGraphProfiler — builtin 间调用图采集 (reorder_builtins 配套)")}
//! 遍历 turboshaft 图, 记录 builtin → builtin 的调用边 (带 block id)。
//!
//! 规则面: IsBuiltinCall 五重门槛链。
//! 语义面 (推迟): turboshaft OperationMatcher / BuiltinsCallGraph 落点。

#![allow(dead_code)]

/// v8: src/compiler/basic-block-call-graph-profiler.cc IsBuiltinCall
/// 门槛链 (原文顺序): CallOp|TailCallOp → kCallCodeObject →
/// MatchHeapConstant → Is<Code> → is_builtin; 通过则写出 builtin_id
fn is_builtin_call(op: &crate::turboshaft::Operation, graph: &crate::turboshaft::Graph) -> Option<crate::builtins::Builtin> {
  let (descriptor, callee) = if let Some(call) = op.try_cast_call() {
    (call.descriptor, call.callee())
  } else if let Some(tail) = op.try_cast_tail_call() {
    (tail.descriptor, tail.callee())
  } else {
    return None;
  };
  if descriptor.descriptor.kind() != crate::CallDescriptorKind::CallCodeObject {
    return None;
  }
  let matcher = crate::turboshaft::OperationMatcher::new(graph);
  let heap_constant = matcher.match_heap_constant(callee)?;
  let code = heap_constant.as_code()?;
  if !code.is_builtin() {
    return None;
  }
  Some(code.builtin_id())
}

/// v8: src/compiler/basic-block-call-graph-profiler.h BasicBlockCallGraphProfiler::StoreCallGraph
pub fn store_call_graph(info: &mut crate::OptimizedCompilationInfo, graph: &crate::turboshaft::Graph) {
  assert!(crate::builtins::is_builtin_id(info.builtin()));
  let profiler = crate::diagnostics::BuiltinsCallGraph::get();
  for block in graph.blocks_vector() {
    let block_id = block.index().id();
    for op in graph.operations(block) {
      if let Some(called) = is_builtin_call(op, graph) {
        profiler.add_builtin_call(info.builtin(), called, block_id);
      }
    }
  }
}
`;

const p = writeSrc("v8-compiler-passes", "basic_block_call_graph_profiler.rs.skel", body);
console.log(`written ${p}\n函数=2 (is_builtin_call + store_call_graph) 门槛链=5`);
