#!/usr/bin/env bun
// tool/src/compiler/turboshaft/sidetable.h.js
// 机械提取 v8/src/compiler/turboshaft/sidetable.{h,cc}
// → v8-turboshaft/src/sidetable.rs.skel
//
// 提取面: Growing/Fixed × Block/OpIndex 四类侧表; NextSize 增长公式;
// OpIndexBelongsToTableGraph (DEBUG, cc)。
import { readV8, writeSrc, banner } from "../../../lib.js";

const h = readV8("src/compiler/turboshaft/sidetable.h");
const cc = readV8("src/compiler/turboshaft/sidetable.cc");

// 1. 四个公开侧表类
for (const c of ["GrowingBlockSidetable", "FixedBlockSidetable", "GrowingOpIndexSidetable", "FixedOpIndexSidetable"]) {
  if (!new RegExp(`class ${c}`).test(h)) throw new Error(`缺 ${c}`);
}

// 2. NextSize 公式
const nsM = h.match(/size_t NextSize\(size_t out_of_bounds_index\) const \{\s*\n\s*DCHECK_GE\([^;]+;\s*\n\s*return ([^;]+);/);
if (!nsM) throw new Error("未找到 NextSize 公式");
const formula = nsM[1].replace(/\s+/g, " ");

// 3. 哨兵
for (const [re, name] of [
  [/conceptually infinite mapping/, "无限映射注释"],
  [/mutable ZoneVector<T> table_;/, "table_ 字段"],
  [/void SwapData\(GrowingOpIndexSidetable<T>& other\)/, "SwapData"],
  [/bool OpIndexBelongsToTableGraph\(const Graph\* graph, OpIndex index\)/, "cc DEBUG 函数"],
  [/graph->BelongsToThisGraph\(index\)/, "cc 实现"],
]) {
  if (!re.test(h) && !re.test(cc)) throw new Error(`哨兵缺失: ${name}`);
}

// 4. 方法面
const methods = ["operator[]", "Reset", "empty", "SwapData"];

let rust = banner(
  "src/compiler/turboshaft/sidetable.h.js",
  "src/compiler/turboshaft/sidetable.{h,cc}",
  "侧表: OpIndex/BlockIndex 到值的映射族",
);

rust += `
/// 原版 GrowingSidetable — 概念无限映射, 越界自动按
/// ${formula}
/// 增长并默认初始化; Reset() 以 T{} 填充保留内存。
/// v8: src/compiler/turboshaft/sidetable.h detail::GrowingSidetable
pub struct GrowingSidetable<T> {
  pub table: Vec<T>,
}

impl<T: Default> GrowingSidetable<T> {
  /// v8: sidetable.h GrowingSidetable::operator[] — 双倍余量增长
  pub fn get_or_grow(&mut self, id: u32) -> &mut T { todo!() }
  /// v8: sidetable.h Reset — fill(T{}) 不缩容
  pub fn reset(&mut self) { todo!() }
  /// v8: sidetable.h empty — 从未含值 (含 Reset 前)
  pub fn is_empty(&self) -> bool { todo!() }
}

/// 原版 FixedSidetable — 定长, 越界 DCHECK
/// v8: src/compiler/turboshaft/sidetable.h detail::FixedSidetable
pub struct FixedSidetable<T> {
  pub table: Vec<T>,
}

/// 四个公开别名 (Key 限定 BlockIndex / OpIndex):
/// - GrowingBlockSidetable<T>: 构造 (zone) 或 (size, init, zone)
/// - FixedBlockSidetable<T>:   构造 (size, zone) 或 (size, init, zone)
/// - GrowingOpIndexSidetable<T>: DEBUG 下持 graph 校验归属
///     OpIndexBelongsToTableGraph(graph, index) (cc, 仅 DEBUG 编译)
/// - FixedOpIndexSidetable<T>
/// v8: src/compiler/turboshaft/sidetable.h 同名类
pub type GrowingBlockSidetable<T> = GrowingSidetable<T>;
pub type FixedBlockSidetable<T> = FixedSidetable<T>;

pub struct GrowingOpIndexSidetable<T> {
  pub base: GrowingSidetable<T>,
  /// v8: DEBUG 字段 graph_ — Rust 侧 debug_assert 归属
  pub graph: Option<*const Graph>,
}
impl<T> GrowingOpIndexSidetable<T> {
  /// v8: sidetable.h SwapData — 交换底层 table_
  pub fn swap_data(&mut self, other: &mut Self) { todo!() }
}

/// v8: src/compiler/turboshaft/sidetable.cc OpIndexBelongsToTableGraph
/// (仅 DEBUG): graph->BelongsToThisGraph(index)
pub fn op_index_belongs_to_table_graph(graph: &Graph, index: OpIndex) -> bool {
  graph.belongs_to_this_graph(index)
}
`;

const p = writeSrc("v8-turboshaft", "sidetable.rs.skel", rust);
console.log(`written ${p}`);
console.log(`侧表类=4 方法=${methods.length} cc仅DEBUG=1`);
