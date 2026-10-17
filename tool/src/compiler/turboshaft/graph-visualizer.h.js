#!/usr/bin/env bun
// tool/src/compiler/turboshaft/graph-visualizer.h.js
// 机械提取 v8/src/compiler/turboshaft/graph-visualizer.{h,cc}
// → v8-turboshaft/src/graph_visualizer.rs.skel
//
// 提取面: TurboshaftGraphAsJSON + JSONTurboshaftGraphWriter 方法面
// (Print/PrintNodes/PrintEdges/PrintBlocks) + 自定义数据打印双入口
// (PerOperation/PerBlock) + operator<<。语义实现禁编造。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/graph-visualizer.h");
const cc = readV8("src/compiler/turboshaft/graph-visualizer.cc");

// 1. h 结构面
for (const [re, label] of [
  [/struct TurboshaftGraphAsJSON\s*\{/, "TurboshaftGraphAsJSON"],
  [/class JSONTurboshaftGraphWriter\s*\{/, "JSONTurboshaftGraphWriter"],
  [/V8_EXPORT_PRIVATE std::ostream& operator<<\(std::ostream& os,\s*const TurboshaftGraphAsJSON&/, "operator<< 声明"],
  [/void PrintTurboshaftCustomDataPerOperation\(/, "PrintTurboshaftCustomDataPerOperation 声明"],
  [/void PrintTurboshaftCustomDataPerBlock\(/, "PrintTurboshaftCustomDataPerBlock 声明"],
]) {
  if (!re.test(h)) throw new Error(`h 哨兵缺失: ${label}`);
}

// 2. TurboshaftGraphAsJSON 字段面
const asJsonM = h.match(/struct TurboshaftGraphAsJSON\s*\{([\s\S]*?)\n\};/);
if (!asJsonM) throw new Error("TurboshaftGraphAsJSON 体缺失");
for (const f of ["turboshaft_graph", "origins", "temp_zone"]) {
  if (!asJsonM[1].includes(f)) throw new Error(`TurboshaftGraphAsJSON 字段缺失: ${f}`);
}

// 3. cc 方法面 (定义核对, 保序)
const methods = [...cc.matchAll(/void JSONTurboshaftGraphWriter::(\w+)\(/g)].map((m) => m[1]);
if (JSON.stringify(methods) !== JSON.stringify(["Print", "PrintNodes", "PrintEdges", "PrintBlocks"])) {
  throw new Error(`writer 方法 ${JSON.stringify(methods)} != [Print, PrintNodes, PrintEdges, PrintBlocks]`);
}
// 顶层双入口 + operator<< 定义
for (const [re, label] of [
  [/std::ostream& operator<<\(std::ostream& os, const TurboshaftGraphAsJSON& ad\)/, "operator<< 定义"],
  [/void PrintTurboshaftCustomDataPerOperation\(/, "PerOperation 定义"],
  [/void PrintTurboshaftCustomDataPerBlock\(/, "PerBlock 定义"],
]) {
  if (!re.test(cc)) throw new Error(`cc 哨兵缺失: ${label}`);
}

// 4. 生成 Rust
let rust = banner(
  "src/compiler/turboshaft/graph-visualizer.h.js",
  "src/compiler/turboshaft/graph-visualizer.{h,cc}",
  "JSON 图可视化: 节点/边/块三段打印骨架",
);
rust += `
/// v8: src/compiler/turboshaft/graph-visualizer.h TurboshaftGraphAsJSON
/// 字段面: turboshaft_graph / origins: NodeOriginTable* / temp_zone
/// 构造: AsJSON(graph, origins, temp_zone) 辅助函数
pub struct TurboshaftGraphAsJson<'a> {
  pub turboshaft_graph: &'a crate::graph::Graph,
  pub origins: Option<&'a mut ()>,
  pub temp_zone: *mut crate::phase::Zone,
}

/// v8: src/compiler/turboshaft/graph-visualizer.h JSONTurboshaftGraphWriter
/// 方法面 (cc 定义 ${methods.length} 项): ${methods.join(" / ")}
pub struct JsonTurboshaftGraphWriter<'a> {
  pub graph: &'a crate::graph::Graph,
}

impl<'a> JsonTurboshaftGraphWriter<'a> {
  /// 原版 void Print() — 总入口
  pub fn print(&mut self) {
    unimplemented!("Developer 修订: Print → Nodes/Edges/Blocks 序列")
  }
  /// 原版 void PrintNodes() — 节点段
  pub fn print_nodes(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void PrintEdges() — 边段
  pub fn print_edges(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
  /// 原版 void PrintBlocks() — 块段
  pub fn print_blocks(&mut self) {
    unimplemented!("签名骨架, Developer 修订")
  }
}

/// v8: src/compiler/turboshaft/graph-visualizer.cc PrintTurboshaftCustomDataPerOperation
pub fn print_turboshaft_custom_data_per_operation() {
  unimplemented!("签名骨架, Developer 修订")
}

/// v8: src/compiler/turboshaft/graph-visualizer.cc PrintTurboshaftCustomDataPerBlock
pub fn print_turboshaft_custom_data_per_block() {
  unimplemented!("签名骨架, Developer 修订")
}
`;

const p = writeSrc("v8-turboshaft", "graph_visualizer.rs.skel", rust);
console.log(`written ${p}`);
console.log(`writer方法=${methods.length} 顶层入口=2 字段=3 哨兵=8`);
