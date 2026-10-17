#!/usr/bin/env bun
// v8/src/compiler/memory-optimizer.cc + memory-optimizer.h
// → v8-compiler-passes/src/memory_optimizer.rs.skel
//
// 规则面: Token 结构 (node/state/effect_phi)、Visit* 面 12 签名、
// AllocationStates 合并语义锚。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/memory-optimizer.h");
const cc = readV8("src/compiler/memory-optimizer.cc");

// —— 哨兵 ——
const sentinels = [
  [h, /class MemoryOptimizer final \{/, "MemoryOptimizer"],
  [h, /struct Token \{/, "Token 结构"],
  [h, /void VisitNode\(Node\*, AllocationState const\*, NodeId\);/, "VisitNode"],
  [h, /void VisitAllocateRaw\(Node\*, AllocationState const\*, NodeId\);/, "VisitAllocateRaw"],
  [h, /void VisitCall\(Node\*, AllocationState const\*, NodeId\);/, "VisitCall"],
  [cc, /void MemoryOptimizer::Optimize\(\) \{/, "Optimize 实现"],
  [cc, /MemoryOptimizer::AllocationState const\* MemoryOptimizer::MergeStates\(/, "MergeStates"],
  [cc, /void MemoryOptimizer::EnqueueMerge\(Node\* node, int index,/, "EnqueueMerge"],
];
for (const [src, re, name] of sentinels) {
  if (!re.test(src)) throw new Error(`哨兵缺失: ${name}`);
}
// Visit* 方法清单 (h)
const visits = [...h.matchAll(/^  void (Visit\w+)\(Node\*/gm)].map((m) => m[1]);
if (visits.join() !== "VisitTrappingMemOp,VisitNode,VisitAllocateRaw,VisitCall,VisitLoadFromObject,VisitLoadElement,VisitLoadField,VisitTrappingLoad,VisitTrappingStore,VisitStoreToObject,VisitStoreElement,VisitStoreField,VisitStore,VisitOtherEffect") throw new Error(`Visit 面漂移: ${visits}`);

const body = `${banner("src/compiler/memory-optimizer.js", "src/compiler/memory-optimizer.h + src/compiler/memory-optimizer.cc", "MemoryOptimizer — effect 链驱动的内存降级编排器 (Token 穿线)")}
//! MemoryOptimizer: 沿效应链穿 Token(node, state, effect_phi),
//! 逐节点调 MemoryLowering; 合并点做 AllocationStates 归并。
//!
//! 规则面: Token + Visit 面 ${visits.length} + 入口。

#![allow(dead_code, unused_variables)]

/// v8: src/compiler/memory-optimizer.h MemoryOptimizer::Token
/// 效应路径状态穿线: 当前节点 + 分配状态 + 最近 EffectPhi (启发)
pub struct Token {
  /// 原 node
  pub node: crate::node::Node,
  /// 原 state
  pub state: *const crate::memory_lowering::AllocationState,
  /// 原 effect_phi
  pub effect_phi: Option<crate::node::Node>,
}

/// v8: src/compiler/memory-optimizer.h MemoryOptimizer
pub struct MemoryOptimizer {
  /// 原 broker_ / jsgraph_ / zone_
  pub broker: *mut crate::broker::JSHeapBroker,
  pub jsgraph: *mut crate::js_graph::JSGraph,
  pub zone: *mut crate::zone::Zone,
  /// 原 memory_lowering_
  pub memory_lowering: crate::memory_lowering::MemoryLowering,
  /// original_effect_ 等语义面字段推迟
  pub _semantic: (),
}

impl MemoryOptimizer {
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::Optimize
  /// 原文: 从 graph->end() 反向 effect 穿线遍历 (Token 队列)
  pub fn optimize(&mut self) {
    todo!("effect 链 BFS 穿线")
  }

  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::VisitNode
  /// 原文: 按 opcode 分派 Visit*
  pub fn visit_node(&mut self, node: crate::node::Node, state: *const crate::memory_lowering::AllocationState, id: crate::graph_reducer::NodeId) {
    let _ = (node, state, id);
    todo!("分派 Visit*")
  }
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::VisitAllocateRaw
  /// 原文: ReduceAllocateRaw 后按返回的 state 推进; 成组折叠
  pub fn visit_allocate_raw(&mut self, node: crate::node::Node, state: *const crate::memory_lowering::AllocationState, id: crate::graph_reducer::NodeId) {
    let _ = (node, state, id);
    todo!("分配折叠推进")
  }
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::VisitCall
  /// 原文: 调用关闭分配组 (Closed)
  pub fn visit_call(&mut self, node: crate::node::Node, state: *const crate::memory_lowering::AllocationState, id: crate::graph_reducer::NodeId) {
    let _ = (node, state, id);
    todo!("调用切断折叠")
  }
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::VisitEffectPhi
  /// 原文: 多前驱 AllocationStates 归并 (可折叠取 size 交, 否则 Closed)
  pub fn visit_effect_phi(&mut self, node: crate::node::Node, id: crate::graph_reducer::NodeId) {
    let _ = (node, id);
    todo!("合并语义")
  }
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::CanAllocate
  /// 原文: opcode 判定是否可分配节点
  pub fn can_allocate(&self, node: &crate::node::Node) -> bool {
    let _ = node;
    todo!("分配性判定")
  }
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::MergeStates
  /// 原文: 多前驱状态归并 (同组开放取尺寸和, 异组 → Closed)
  pub fn merge_states(&mut self, states: &[*const crate::memory_lowering::AllocationState]) -> *const crate::memory_lowering::AllocationState {
    let _ = states;
    todo!("归并语义")
  }
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::EnqueueMerge
  /// 原文: EffectPhi 各输入到达后统一合并入队
  pub fn enqueue_merge(&mut self, node: crate::node::Node, index: i32, state: *const crate::memory_lowering::AllocationState) {
    let _ = (node, index, state);
    todo!("前驱同步门")
  }
  /// v8: src/compiler/memory-optimizer.cc MemoryOptimizer::AllocationTypeNeedsUpdateToOld
  /// 原文: 外层分配老化传播判定
  pub fn allocation_type_needs_update_to_old(&self, user: &crate::node::Node, edge: crate::node::Edge) -> bool {
    let _ = (user, edge);
    todo!("tenuring 传播")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "memory_optimizer.rs.skel", body);
console.log(`written ${p}\nToken=3字段 Visit面=${visits.length} 方法=9`);
