#!/usr/bin/env bun
// tool/src/compiler/turboshaft/pretenuring-propagation-reducer.h.js
// 机械提取 v8/src/compiler/turboshaft/pretenuring-propagation-reducer.{h,cc}
// → v8-turboshaft/src/pretenuring_propagation_reducer.rs.skel
//
// 提取面: 分析器数据面 (old_allocs/store_graph/old_phis/queue);
// Process{Store,Phi,Allocate} 分派; 老生代传播规则锚。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/pretenuring-propagation-reducer.h");
const cc = readV8("src/compiler/turboshaft/pretenuring-propagation-reducer.cc");

// 1. 分析器面哨兵
for (const [re, name] of [
  [/class PretenuringPropagationAnalyzer/, "分析器类"],
  [/ZoneVector<OpIndex> old_allocs_;/, "old_allocs_"],
  [/ZoneAbslFlatHashMap<OpIndex, ZoneVector<OpIndex>\*> store_graph_;/, "store_graph_"],
  [/ZoneAbslFlatHashSet<OpIndex> old_phis_;/, "old_phis_"],
  [/class PretenuringPropagationReducer : public Next/, "reducer 类"],
]) {
  if (!re.test(h)) throw new Error(`哨兵缺失: ${name}`);
}

// 2. cc 方法面
const ccMethods = [...cc.matchAll(/void PretenuringPropagationAnalyzer::(\w+)\(/g)].map((m) => m[1]);
if (ccMethods.length !== 7) throw new Error(`cc 方法 ${ccMethods.length} != 7`);
for (const must of ["Run", "ProcessStore", "ProcessPhi", "ProcessAllocate", "OldifySubgraph", "PropagateAllocationTypes", "BuildStoreInputGraph"]) {
  if (!ccMethods.includes(must)) throw new Error(`cc 缺 ${must}`);
}

// 3. cc 规则锚: 老->新传播
for (const [re, name] of [
  [/AllocationType::kOld/, "kOld 判定"],
  [/alloc->type = AllocationType::kOld;/, "改写分配类型"],
  [/PropagateAllocationTypes/, "传播入口"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${name}`);
}

// 4. 数据面字段
const dataFields = [
  "old_allocs_", "store_graph_", "old_phis_", "queue_", "zone_",
];

let rust = banner(
  "src/compiler/turboshaft/pretenuring-propagation-reducer.h.js",
  "src/compiler/turboshaft/pretenuring-propagation-reducer.{h,cc}",
  "预分配传播: 老/新生代分配类型传播分析",
);

rust += `
/// 原版 PretenuringPropagationAnalyzer — 就地改写输入图的分配类型分析
/// v8: src/compiler/turboshaft/pretenuring-propagation-reducer.{h,cc}
pub struct PretenuringPropagationAnalyzer {
  /// 数据面 (原版字段):
${dataFields.map((f) => `  /// - ${f}`).join("\n")}
  /// - old_allocs_: 已判老生代的分配
  /// - store_graph_: base -> 存入其中的 value 集合 (找 Allocate)
  /// - old_phis_: 输入全为老分配的 phi
  /// - queue_: 传播工作队列
  pub zone: Zone,
}

impl PretenuringPropagationAnalyzer {
  /// v8: cc PretenuringPropagationAnalyzer::Run — 入口
  pub fn run(&mut self) { /* ... */ }
  /// v8: cc ProcessStore — value 为 Allocate 且 kOld 时记 store_graph 边
  pub fn process_store(&mut self, store: &StoreOp) { /* ... */ }
  /// v8: cc ProcessPhi — 全 kOld 输入的 phi 记入 old_phis_
  pub fn process_phi(&mut self, phi: &PhiOp) { /* ... */ }
  /// v8: cc ProcessAllocate — kOld 分配入 old_allocs_ 队列
  pub fn process_allocate(&mut self, alloc: &AllocateOp) { /* ... */ }
  /// v8: cc OldifySubgraph — 从老分配 BFS 老 store 子图
  pub fn oldify_subgraph(&mut self, old_alloc: OpIndex) { /* ... */ }
  /// v8: cc PropagateAllocationTypes — 队列固定点, 传播后统一
  /// alloc->type = AllocationType::kOld
  pub fn propagate_allocation_types(&mut self) { /* ... */ }
  /// v8: cc BuildStoreInputGraph — 建 store_graph_ 边
  pub fn build_store_input_graph(&mut self) { /* ... */ }
}

/// 原版 PretenuringPropagationReducer — 包装分析器先跑再复制
/// v8: src/compiler/turboshaft/pretenuring-propagation-reducer.h PretenuringPropagationReducer
pub struct PretenuringPropagationReducer;
`;

const p = writeSrc("v8-turboshaft", "pretenuring_propagation_reducer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`cc方法=${ccMethods.length} 数据字段=${dataFields.length}`);
