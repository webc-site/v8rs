#!/usr/bin/env bun
// v8/src/compiler/all-nodes.cc + all-nodes.h
// → v8-compiler-passes/src/all_nodes.rs.skel
//
// 全族一个类 3 方法 2 构造。规则面: IsReachable/IsLive 完整语义
// (位向量查含, 逐字核对), Mark/构造签名; 遍历 (inputs vs uses) 留锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/all-nodes.h");
const cc = readV8("src/compiler/all-nodes.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class AllNodes \{/, "AllNodes 类"],
  [h, /bool only_inputs = true\)/, "only_inputs 默认"],
  [h, /return id < is_reachable_\.length\(\) && is_reachable_\.Contains\(id\);/, "IsReachable 体"],
  [cc, /void AllNodes::Mark\(Zone\* local_zone, Node\* end, const TFGraph\* graph\) \{/, "Mark 实现"],
  [cc, /for \(Node\* const input : reachable\[i\]->inputs\(\)\)/, "inputs 遍历"],
  [cc, /for \(Node\* use : reachable\[i\]->uses\(\)\)/, "uses 遍历"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
if (cc.match(/for \(Node\* const input : reachable\[i\]->inputs\(\)\)/g).length !== 1) {
  throw new Error("inputs 遍历应 1 处 (Mark)");
}
if (!/if \(!only_inputs_\) \{/.test(cc)) throw new Error("uses 遍历应在 !only_inputs_ 条件内");

const body = `${banner("src/compiler/all-nodes.js", "src/compiler/all-nodes.h + src/compiler/all-nodes.cc", "AllNodes — 从 End 可达节点集 (inputs 或 uses 方向)")}
//! AllNodes: 从 End 出发标记可达节点。
//!
//! 规则面: IsReachable/IsLive 语义 + 字段布局。
//! 语义面 (推迟): Mark 遍历 (only_inputs ? inputs : uses) — cc:16-50。

#![allow(dead_code)]

/// v8: src/compiler/all-nodes.h AllNodes
pub struct AllNodes {
  /// 原 reachable — 从 end 可达节点 (输出序)
  pub reachable: crate::node::NodeVector,
  /// 原 is_reachable_ — 按 node id 的位向量
  is_reachable: crate::bit_vector::BitVector,
  /// 原 only_inputs_
  only_inputs: bool,
}

impl AllNodes {
  /// v8: src/compiler/all-nodes.cc AllNodes::AllNodes (end 显式版)
  pub fn new(
    local_zone: &mut crate::zone::Zone,
    end: &crate::node::Node,
    graph: &crate::graph::TFGraph,
    only_inputs: bool, // 原默认 true
  ) -> Self {
    let mut s = Self {
      reachable: Default::default(),
      is_reachable: Default::default(),
      only_inputs,
    };
    s.mark(local_zone, end, graph);
    s
  }

  /// v8: src/compiler/all-nodes.cc AllNodes::AllNodes (End 节点版)
  /// 原实现: 取 graph->end() 后委托显式版
  pub fn from_graph_end(
    local_zone: &mut crate::zone::Zone,
    graph: &crate::graph::TFGraph,
    only_inputs: bool, // 原默认 true
  ) -> Self {
    todo!("取 graph.end() 后委托 new")
  }

  /// v8: src/compiler/all-nodes.h AllNodes::IsLive
  /// 原文: CHECK(only_inputs_); return IsReachable(node);
  pub fn is_live(&self, node: &crate::node::Node) -> bool {
    assert!(self.only_inputs);
    self.is_reachable(node)
  }

  /// v8: src/compiler/all-nodes.h AllNodes::IsReachable
  /// 原文: if (!node) return false; id < len && Contains(id)
  pub fn is_reachable(&self, node: Option<&crate::node::Node>) -> bool {
    let Some(node) = node else { return false };
    let id = node.id();
    id < self.is_reachable.length() && self.is_reachable.contains(id)
  }

  /// v8: src/compiler/all-nodes.cc AllNodes::Mark
  /// 语义面: 预留队列遍历; only_inputs=false 时走 uses 边 (End 的 use 特判)
  fn mark(&mut self, local_zone: &mut crate::zone::Zone, end: &crate::node::Node, graph: &crate::graph::TFGraph) {
    todo!("Mark: std::vector 指针队列 + inputs/uses 二选一遍历")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "all_nodes.rs.skel", body);
console.log(`written ${p}\n方法=5 (构造2+is_live+is_reachable+mark) inputs遍历=1 uses遍历=1(条件内)`);
