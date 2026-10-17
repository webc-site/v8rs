#!/usr/bin/env bun
// v8/src/compiler/escape-analysis-reducer.cc + escape-analysis-reducer.h
// → v8-compiler-passes/src/escape_analysis_reducer.rs.skel
//
// 规则面: NodeHashCache (Constructor COW 语义 + hash 集自定义相等/哈希)
// 与 EscapeAnalysisReducer 方法面; ReduceDeoptState/Deduplicator 锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/escape-analysis-reducer.h");
const cc = readV8("src/compiler/escape-analysis-reducer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class NodeHashCache \{/, "NodeHashCache"],
  [h, /class Constructor \{/, "Constructor"],
  [h, /if \(!tmp_ && input == NodeProperties::GetValueInput\(from_, i\)\) return;/, "COW 短路"],
  [h, /ZoneUnorderedSet<Node\*, NodeHashCode, NodeEquals> cache_;/, "hash 集"],
  [h, /class V8_EXPORT_PRIVATE EscapeAnalysisReducer final/, "EscapeAnalysisReducer"],
  [h, /void VerifyReplacement\(\) const;/, "VerifyReplacement"],
  [cc, /Node\* EscapeAnalysisReducer::ReduceDeoptState\(/, "ReduceDeoptState"],
  [cc, /void EscapeAnalysisReducer::VerifyReplacement\(\) const \{/, "VerifyReplacement 实现"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// cc 方法面
const ccMethods = [...cc.matchAll(/^(?:Reduction|void|Node\*) EscapeAnalysisReducer::(\w+)\(/gm)].map((m) => m[1]);
const expect = ["ReplaceNode","ObjectIdNode","Reduce","ReduceFrameStateInputs","ReduceDeoptState","VerifyReplacement","Finalize"];
if (ccMethods.length !== expect.length || !expect.every((m) => ccMethods.includes(m))) {
  throw new Error(`cc 方法漂移: ${ccMethods}`);
}

const body = `${banner("src/compiler/escape-analysis-reducer.js", "src/compiler/escape-analysis-reducer.h + src/compiler/escape-analysis-reducer.cc", "EscapeAnalysisReducer + NodeHashCache — 按逃逸分析结果改图 (含去重哈希缓存)")}
//! EscapeAnalysisReducer: 依分析结果替换逃逸对象相关节点;
//! NodeHashCache 对 ObjectState/StateValues/FrameState 做 hash-consing。
//!
//! 规则面: NodeHashCache COW 语义 + 方法面 ${ccMethods.length}。
//! 语义面 (推迟): deopt 状态重建 (ReduceDeoptState 递归)。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/escape-analysis-reducer.h NodeHashCache::Constructor
/// COW: 无修改 (tmp_ 空 且输入未变) 直接复用 from_;
/// 修改走 MutableNode() 临时节点, Get() 命中缓存则回收临时节点。
pub struct NodeHashCacheConstructor<'a> {
  /// 原 node_cache_
  node_cache: &'a mut NodeHashCache,
  /// 原 from_ — 被复制的原节点
  pub from: crate::node::Node,
  /// 原 tmp_ — 可回收临时节点
  tmp: Option<crate::node::Node>,
}

impl NodeHashCacheConstructor<'_> {
  /// v8: src/compiler/escape-analysis-reducer.h Constructor::ReplaceValueInput
  /// 原文: if (!tmp_ && input == GetValueInput(from_, i)) return;
  pub fn replace_value_input(&mut self, input: crate::node::Node, i: i32) {
    if self.tmp.is_none() && input == crate::node_properties::get_value_input(self.from, i) {
      return;
    }
    let node = self.mutable_node();
    crate::node_properties::replace_value_input(node, input, i);
  }
  /// v8: src/compiler/escape-analysis-reducer.h Constructor::ReplaceInput
  /// 原文: if (!tmp_ && input == from_->InputAt(i)) return;
  pub fn replace_input(&mut self, input: crate::node::Node, i: i32) {
    if self.tmp.is_none() && input == self.from.input_at(i) {
      return;
    }
    let node = self.mutable_node();
    node.replace_input(i, input);
  }
  /// v8: src/compiler/escape-analysis-reducer.h Constructor::Get
  /// 取变更节点或缓存副本; 使 Constructor 失效
  pub fn get(&mut self) -> crate::node::Node {
    todo!("Query 命中回收 tmp, 未命中插入缓存")
  }
  /// v8: src/compiler/escape-analysis-reducer.h Constructor::MutableNode
  fn mutable_node(&mut self) -> &mut crate::node::Node {
    todo!("无 tmp 则从 from_ 克隆")
  }
}

/// v8: src/compiler/escape-analysis-reducer.h NodeHashCache
/// cache_ = ZoneUnorderedSet (HashCode=NodeProperties::HashCode,
/// Equals=NodeProperties::Equals); temp_nodes_ 回收池
pub struct NodeHashCache {
  /// 原 graph_
  pub graph: *mut crate::graph::TFGraph,
  /// 原 cache_
  pub cache: std::collections::HashSet<crate::node::Node>,
  /// 原 temp_nodes_ — 可回收内存节点
  pub temp_nodes: Vec<crate::node::Node>,
}

impl NodeHashCache {
  /// v8: src/compiler/escape-analysis-reducer.h NodeHashCache::Query
  fn query(&mut self, node: crate::node::Node) -> Option<crate::node::Node> {
    let _ = node;
    todo!("集合查询")
  }
}

/// v8: src/compiler/escape-analysis-reducer.h EscapeAnalysisReducer
pub struct EscapeAnalysisReducer {
  /// 原 jsgraph_
  pub jsgraph: *mut crate::js_graph::JSGraph,
  /// 原 broker_
  pub broker: *mut crate::broker::JSHeapBroker,
  /// 原 analysis_result_
  pub analysis_result: crate::escape_analysis::EscapeAnalysisResult,
  /// 原 object_id_cache_
  pub object_id_cache: Vec<crate::node::Node>,
  /// 原 node_cache_
  pub node_cache: NodeHashCache,
  /// 原 arguments_elements_
  pub arguments_elements: std::collections::BTreeSet<crate::node::Node>,
}

impl EscapeAnalysisReducer {
  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::reducer_name
  pub const REDUCER_NAME: &'static str = "EscapeAnalysisReducer";

  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::Reduce
  pub fn reduce(&mut self, node: &crate::node::Node) -> crate::graph_reducer::Reduction {
    let _ = node;
    todo!("按 opcode 分派替换")
  }
  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::Finalize
  pub fn finalize(&mut self) {
    todo!("arguments_elements_ 收尾")
  }
  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::VerifyReplacement
  pub fn verify_replacement(&self) {
    todo!("断言所有虚拟分配节点已被处理")
  }
  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::ReduceFrameStateInputs
  pub fn reduce_frame_state_inputs(&mut self, node: &crate::node::Node) {
    let _ = node;
    todo!("FrameState 输入遍历改写")
  }
  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::ReduceDeoptState
  pub fn reduce_deopt_state(&mut self, node: &crate::node::Node, effect: &crate::node::Node, deduplicator: &mut crate::Deduplicator) -> crate::node::Node {
    let _ = (node, effect, deduplicator);
    todo!("ObjectState/StateValues 重建 (hash-consing)")
  }
  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::ObjectIdNode
  pub fn object_id_node(&mut self, vobject: &crate::escape_analysis::VirtualObject) -> crate::node::Node {
    let _ = vobject;
    todo!("object_id_cache_ 缓存查造")
  }
  /// v8: src/compiler/escape-analysis-reducer.cc EscapeAnalysisReducer::ReplaceNode
  pub fn replace_node(&mut self, original: &crate::node::Node, replacement: crate::node::Node) -> crate::graph_reducer::Reduction {
    let _ = (original, replacement);
    todo!("替换或 revisit")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "escape_analysis_reducer.rs.skel", body);
console.log(`written ${p}\n类型=3 (Constructor/NodeHashCache/Reducer) cc方法=${ccMethods.length} COW短路=2`);
