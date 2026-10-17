#!/usr/bin/env bun
// v8/src/compiler/loop-peeling.cc + loop-peeling.h
// → v8-compiler-passes/src/loop_peeling.rs.skel
//
// 规则面: kMaxPeeledNodes=1000、CanPeel 直通 HasMarkedExits、
// PeeledIteration::map 语义、EliminateLoopExits 静态双入口。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/loop-peeling.h");
const cc = readV8("src/compiler/loop-peeling.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE PeeledIteration/, "PeeledIteration"],
  [h, /class V8_EXPORT_PRIVATE LoopPeeler \{/, "LoopPeeler"],
  [h, /bool CanPeel\(LoopTree::Loop\* loop\) \{/, "CanPeel"],
  [h, /static const size_t kMaxPeeledNodes = 1000;/, "kMaxPeeledNodes"],
  [cc, /Node\* PeeledIteration::map\(Node\* node\) \{/, "map 实现"],
  [cc, /PeeledIteration\* LoopPeeler::Peel\(LoopTree::Loop\* loop\) \{/, "Peel 实现"],
  [cc, /void LoopPeeler::EliminateLoopExits\(TFGraph\* graph, Zone\* tmp_zone\) \{/, "EliminateLoopExits 实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
if (!/return LoopFinder::HasMarkedExits\(loop_tree_, loop\);/.test(h)) throw new Error("CanPeel 直通缺失");

const body = `${banner("src/compiler/loop-peeling.js", "src/compiler/loop-peeling.h + src/compiler/loop-peeling.cc", "LoopPeeler — 循环剥层 (首迭代外提)")}
//! LoopPeeler: 将循环首迭代复制到循环外, 剩余循环保留。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/loop-peeling.h LoopPeeler::kMaxPeeledNodes
pub const K_MAX_PEELED_NODES: usize = 1000;

/// v8: src/compiler/loop-peeling.h PeeledIteration
/// 剥层结果: 循环体节点 → 剥层副本映射 (NodeCopier 支撑)
pub struct PeeledIteration {
  /// 原 node_copier_ (一副本)
  pub copier: crate::loop_analysis::NodeCopier,
}

impl PeeledIteration {
  /// v8: src/compiler/loop-peeling.cc PeeledIteration::map
  /// 原文: 循环体内的节点 → 对应副本; 否则返回自身
  pub fn map(&self, node: crate::node::Node) -> crate::node::Node {
    self.copier.map(node, 0)
  }
}

/// v8: src/compiler/loop-peeling.h LoopPeeler
pub struct LoopPeeler {
  /// 原 graph_
  pub graph: *mut crate::graph::TFGraph,
  /// 原 common_
  pub common: *mut crate::common_operator::CommonOperatorBuilder,
  /// 原 loop_tree_
  pub loop_tree: *mut crate::loop_analysis::LoopTree,
  /// 原 tmp_zone_
  pub tmp_zone: *mut crate::zone::Zone,
  /// 原 source_positions_ / node_origins_
  pub source_positions: *mut crate::compiler_source_position_table::SourcePositionTable,
  pub node_origins: *mut crate::node_origin_table::NodeOriginTable,
}

impl LoopPeeler {
  /// v8: src/compiler/loop-peeling.h LoopPeeler::CanPeel
  /// 原文: return LoopFinder::HasMarkedExits(loop_tree_, loop);
  pub fn can_peel(&self, loop: &crate::loop_analysis::Loop) -> bool {
    crate::loop_analysis::LoopFinder::has_marked_exits(unsafe { &mut *self.loop_tree }, loop)
  }

  /// v8: src/compiler/loop-peeling.cc LoopPeeler::Peel
  /// 语义: kMaxPeeledNodes 门槛 + 副本接 LoopExit/merge 重构
  pub fn peel(&mut self, loop: &mut crate::loop_analysis::Loop) -> *mut PeeledIteration {
    let _ = loop;
    todo!("NodeCopier 复制 + 控制流重接")
  }

  /// v8: src/compiler/loop-peeling.cc LoopPeeler::PeelInnerLoopsOfTree
  /// 原文: 自树顶向下逐内层循环 Peel (max_peeled 门槛内)
  pub fn peel_inner_loops_of_tree(&mut self) {
    todo!("遍历 loop_tree_.outer_loops_ 递归")
  }

  /// v8: src/compiler/loop-peeling.cc LoopPeeler::EliminateLoopExits (静态)
  /// 原文: AllNodes 收集后逐 LoopExit 调 EliminateLoopExit
  pub fn eliminate_loop_exits(graph: *mut crate::graph::TFGraph, tmp_zone: *mut crate::zone::Zone) {
    let _ = (graph, tmp_zone);
    todo!("收集 + 批量摘除")
  }

  /// v8: src/compiler/loop-peeling.cc LoopPeeler::EliminateLoopExit (静态)
  pub fn eliminate_loop_exit(loop: crate::node::Node) {
    let _ = loop;
    todo!("LoopExit 旁路 (值/效应直通)")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "loop_peeling.rs.skel", body);
console.log(`written ${p}\n常量=1 (kMaxPeeledNodes=1000) 方法=6 CanPeel=直通语义`);
