#!/usr/bin/env bun
// tool/src/compiler/value-numbering-reducer.js
// v8/src/compiler/value-numbering-reducer.h + value-numbering-reducer.cc
// → v8-compiler-passes/src/value_numbering_reducer.rs.skel
// GVN: 解析 kInitialCapacity / 哈希混合公式与 Grow 扩容语义。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/value-numbering-reducer.h");
const cc = readV8("src/compiler/value-numbering-reducer.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`value-numbering-reducer: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE ValueNumberingReducer final", "类定义"],
  ["enum { kInitialCapacity = 256u };", "kInitialCapacity"],
  ['return "ValueNumberingReducer";', "reducer_name"],
  ["Reduction ValueNumberingReducer::Reduce(Node* node)", "cc Reduce"],
  ["void ValueNumberingReducer::Grow()", "cc Grow"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// kInitialCapacity 真解析
const cap = h.match(/kInitialCapacity = (\d+)u/);
need(cap && cap[1] === "256", `容量 ${cap && cap[1]}`);

// cc: 扩容形态 (真解析 — capacity_ *= 2)
const growFactor = cc.match(/capacity_ \*= 2;/);
need(growFactor !== null, "Grow 扩容形态未匹配");

const body = `${banner("src/compiler/value-numbering-reducer.js", "src/compiler/value-numbering-reducer.h + value-numbering-reducer.cc", "全局部值编号 (纯节点去重)")}
//! value-numbering-reducer — open-addressing 哈希表上的 GVN
//!
//! 键 = (opcode, 参数); 表初始 ${cap[1]} 槽, 装填因子过半 Grow 翻倍重插;
//! 替换候选需类型匹配 (ReplaceIfTypesMatch)。

/// 原版 kInitialCapacity — 初始桶数
pub const K_INITIAL_CAPACITY: usize = ${cap[1]};

/// 原版 ValueNumberingReducer — Reducer (temp_zone 建表, graph_zone 分配节点)
pub struct ValueNumberingReducer<'zt, 'zg> {
  /// entries_ — open-addressing 槽
  pub entries: Vec<*mut Node>,
  /// capacity_
  pub capacity: usize,
  /// size_
  pub size: usize,
  /// temp_zone_
  pub temp_zone: &'zt Zone<'zt>,
  /// graph_zone_
  pub graph_zone: &'zg Zone<'zg>,
}

impl ValueNumberingReducer<'_, '_> {
  /// 原版 reducer_name
  pub const REDUCER_NAME: &'static str = "ValueNumberingReducer";

  /// 原版 Reduce — 查表命中且类型匹配 → Replace
  /// v8: src/compiler/value-numbering-reducer.cc ValueNumberingReducer::Reduce
  pub fn reduce(&mut self, node: &Node) -> Reduction {
    todo!("v8: src/compiler/value-numbering-reducer.cc ValueNumberingReducer::Reduce")
  }

  /// 原版 ReplaceIfTypesMatch — 类型相容才替换
  /// v8: src/compiler/value-numbering-reducer.cc ValueNumberingReducer::ReplaceIfTypesMatch
  fn replace_if_types_match(&self, node: &Node, replacement: &Node) -> Reduction {
    todo!("v8: src/compiler/value-numbering-reducer.cc ValueNumberingReducer::ReplaceIfTypesMatch")
  }

  /// 原版 Grow — 翻倍重插
  /// v8: src/compiler/value-numbering-reducer.cc ValueNumberingReducer::Grow
  fn grow(&mut self) {
    todo!("v8: src/compiler/value-numbering-reducer.cc ValueNumberingReducer::Grow")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "value_numbering_reducer.rs.skel", body);
console.log(`written ${p}\ninitial_capacity=${cap[1]} grow=capacity_*=2`);
