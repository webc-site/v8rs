#!/usr/bin/env bun
// v8/src/compiler/graph-trimmer.cc + graph-trimmer.h
// → v8-compiler-passes/src/graph_trimmer.rs.skel
//
// 规则面: IsLive/MarkAsLive 完整语义 (node-marker 位 + 活表追加) +
// TrimGraph 双根遍历; live_ 字段布局。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/graph-trimmer.h");
const cc = readV8("src/compiler/graph-trimmer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class V8_EXPORT_PRIVATE GraphTrimmer final \{/, "GraphTrimmer"],
  [h, /V8_INLINE bool IsLive\(Node\* const node\) \{ return is_live_\.Get\(node\); \}/, "IsLive"],
  [h, /is_live_\.Set\(node, true\);/, "MarkAsLive 置位"],
  [h, /live_\.push_back\(node\);/, "活表追加"],
  [cc, /void GraphTrimmer::TrimGraph\(\) \{/, "TrimGraph 实现"],
  [cc, /for \(Node\* const input : live->inputs\(\)\) MarkAsLive\(input\);/, "inputs 反向遍历"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
if (!/for \(size_t i = 0; i < live_\.size\(\); \+\+i\)/.test(cc)) throw new Error("队列式遍历缺失");
if (!/for \(Node\* const live : live_\)/.test(cc)) throw new Error("死->活边清理遍历缺失");

const body = `${banner("src/compiler/graph-trimmer.js", "src/compiler/graph-trimmer.h + src/compiler/graph-trimmer.cc", "GraphTrimmer — 死节点修剪 (end 可达性标记)")}
//! GraphTrimmer: 从 end (可加自定义根) 沿 inputs 标活, 其余节点 Kill。

#![allow(dead_code)]

/// v8: src/compiler/graph-trimmer.h GraphTrimmer
pub struct GraphTrimmer {
  /// 原 graph_
  pub graph: *mut crate::graph::TFGraph,
  /// 原 is_live_ — NodeMarker 标记
  pub is_live: crate::node_marker::NodeMarker<bool>,
  /// 原 live_ — 活节点表 (兼作遍历队列)
  pub live: Vec<crate::node::Node>,
}

impl GraphTrimmer {
  /// v8: src/compiler/graph-trimmer.cc GraphTrimmer::GraphTrimmer
  pub fn new(zone: &mut crate::zone::Zone, graph: *mut crate::graph::TFGraph) -> Self {
    let _ = zone;
    todo!("NodeMarker 以 graph->NodeCount() 初始化")
  }

  /// v8: src/compiler/graph-trimmer.h GraphTrimmer::IsLive
  pub fn is_live(&self, node: &crate::node::Node) -> bool {
    self.is_live.get(node)
  }

  /// v8: src/compiler/graph-trimmer.h GraphTrimmer::MarkAsLive
  /// 原文: DCHECK(!node->IsDead()); 未标则 { is_live_.Set(node,true);
  /// live_.push_back(node); }
  pub fn mark_as_live(&mut self, node: crate::node::Node) {
    debug_assert!(!node.is_dead());
    if !self.is_live(&node) {
      self.is_live.set(node, true);
      self.live.push(node);
    }
  }

  /// v8: src/compiler/graph-trimmer.cc GraphTrimmer::TrimGraph (无参版)
  /// 原文: MarkAsLive(graph->end()) → 队列遍历 inputs 标活 →
  /// graph->KillNodes([&](Node* node){ return !IsLive(node); })
  pub fn trim_graph(&mut self) {
    self.mark_as_live(unsafe { (*self.graph).end() });
    let mut i = 0;
    while i < self.live.len() {
      let node = self.live[i];
      for input in node.inputs() {
        if let Some(input) = input {
          self.mark_as_live(input);
        }
      }
      i += 1;
    }
    todo!("KillNodes(非活)")
  }

  /// v8: src/compiler/graph-trimmer.h GraphTrimmer::TrimGraph (迭代器版)
  /// 原文: 先标活 [begin,end) 中非死节点, 再委托无参版
  pub fn trim_graph_with_roots(&mut self, roots: impl Iterator<Item = crate::node::Node>) {
    for node in roots {
      if !node.is_dead() {
        self.mark_as_live(node);
      }
    }
    self.trim_graph();
  }
}
`;

const p = writeSrc("v8-compiler-passes", "graph_trimmer.rs.skel", body);
console.log(`written ${p}\n方法=5 (TrimGraph 重载 2) MarkAsLive语义=完整`);
