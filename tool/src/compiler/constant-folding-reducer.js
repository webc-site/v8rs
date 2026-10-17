#!/usr/bin/env bun
// v8/src/compiler/constant-folding-reducer.cc + constant-folding-reducer.h
// → v8-compiler-passes/src/constant_folding_reducer.rs.skel
//
// 规则面: TryGetConstant 的 7 分支类型判读链 (None/Null/Undefined/
// MinusZero/NaN/HeapConstant/PlainNumber 单值, 逐字核对) + 2 条 DCHECK
// 不变式; Reduce 为薄封装。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/constant-folding-reducer.h");
const cc = readV8("src/compiler/constant-folding-reducer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE ConstantFoldingReducer final/, "ConstantFoldingReducer"],
  [cc, /Node\* TryGetConstant\(JSGraph\* jsgraph, Node\* node, JSHeapBroker\* broker\)/, "TryGetConstant"],
  [cc, /type\.Is\(Type::Null\(\)\)/, "Null 分支"],
  [cc, /type\.Is\(Type::MinusZero\(\)\)/, "MinusZero 分支"],
  [cc, /type\.Is\(Type::NaN\(\)\)/, "NaN 分支"],
  [cc, /type\.IsHeapConstant\(\)/, "HeapConstant 分支"],
  [cc, /type\.Is\(Type::PlainNumber\(\)\) && type\.Min\(\) == type\.Max\(\)/, "PlainNumber 单值分支"],
  [cc, /DCHECK_EQ\(result != nullptr, type\.IsSingleton\(\)\);/, "不变式 1"],
  [cc, /DCHECK_IMPLIES\(result != nullptr,\n                 type\.Equals\(NodeProperties::GetType\(result\)\)\);/, "不变式 2"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// 分支顺序: None → Null → Undefined → MinusZero → NaN → HeapConstant → PlainNumber
const order = ["IsNone", "Is(Type::Null())", "Is(Type::Undefined())", "Is(Type::MinusZero())", "Is(Type::NaN())", "IsHeapConstant()", "Is(Type::PlainNumber()) && type.Min() == type.Max()"];
let prev = -1;
for (const frag of order.map((f) => `type.${f}`)) {
  const i = cc.indexOf(frag);
  if (i < 0 || i <= prev) throw new Error(`分支序漂移: ${frag}`);
  prev = i;
}

const body = `${banner("src/compiler/constant-folding-reducer.js", "src/compiler/constant-folding-reducer.h + src/compiler/constant-folding-reducer.cc", "ConstantFoldingReducer — 单例类型常量折叠")}
//! ConstantFoldingReducer: 节点类型为 singleton 时以常量节点替换。
//!
//! 规则面: TryGetConstant 7 分支判读链 + 2 不变式。

#![allow(dead_code)]

/// v8: src/compiler/constant-folding-reducer.cc TryGetConstant
/// 分支链 (原文顺序):
/// None→null; Null→NullConstant; Undefined→UndefinedConstant;
/// MinusZero→MinusZeroConstant; NaN→NaNConstant;
/// HeapConstant→ConstantNoHole(Ref); PlainNumber 且 Min==Max→数值常量;
/// 其余 → null。
/// 不变式: (result!=null) == type.IsSingleton();
///         result!=null ⇒ type.Equals(GetType(result))。
fn try_get_constant(jsgraph: &mut crate::js_graph::JSGraph, node: &crate::node::Node, broker: &mut crate::broker::JSHeapBroker) -> Option<crate::node::Node> {
  let ty = crate::node_properties::get_type(node);
  let result = if ty.is_none() {
    None
  } else if ty.is(crate::typer::Type::null()) {
    Some(jsgraph.null_constant())
  } else if ty.is(crate::typer::Type::undefined()) {
    Some(jsgraph.undefined_constant())
  } else if ty.is(crate::typer::Type::minus_zero()) {
    Some(jsgraph.minus_zero_constant())
  } else if ty.is(crate::typer::Type::nan()) {
    Some(jsgraph.nan_constant())
  } else if ty.is_heap_constant() {
    Some(jsgraph.constant_no_hole(ty.as_heap_constant().reference(), broker))
  } else if ty.is(crate::typer::Type::plain_number()) && ty.min() == ty.max() {
    Some(jsgraph.constant_no_hole_double(ty.min()))
  } else {
    None
  };
  debug_assert_eq!(result.is_some(), ty.is_singleton(broker));
  if let Some(r) = &result {
    debug_assert!(ty.equals(&crate::node_properties::get_type(r)));
  }
  result
}

/// v8: src/compiler/constant-folding-reducer.h ConstantFoldingReducer
pub struct ConstantFoldingReducer {
  /// 原 js_graph_
  pub js_graph: *mut crate::js_graph::JSGraph,
  /// 原 broker_
  pub broker: *mut crate::broker::JSHeapBroker,
}

impl ConstantFoldingReducer {
  /// v8: src/compiler/constant-folding-reducer.cc ConstantFoldingReducer::reducer_name
  pub const REDUCER_NAME: &'static str = "ConstantFoldingReducer";

  /// v8: src/compiler/constant-folding-reducer.cc ConstantFoldingReducer::Reduce
  /// 原文: !IsTyped → NoChange; TryGetConstant 命中 → Replace(常量)
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    if !crate::node_properties::is_typed(node) {
      return crate::graph_reducer::Reduction::NoChange;
    }
    match try_get_constant(unsafe { &mut *self.js_graph }, node, unsafe { &mut *self.broker }) {
      Some(c) => crate::graph_reducer::Reduction::Replace(c),
      None => crate::graph_reducer::Reduction::NoChange,
    }
  }
}
`;

const p = writeSrc("v8-compiler-passes", "constant_folding_reducer.rs.skel", body);
console.log(`written ${p}\n函数=2 (try_get_constant + reduce) 分支链=7 不变式=2`);
