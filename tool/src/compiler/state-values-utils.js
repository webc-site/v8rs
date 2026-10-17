#!/usr/bin/env bun
// tool/src/compiler/state-values-utils.js
// v8/src/compiler/state-values-utils.h + state-values-utils.cc
// → v8-compiler-passes/src/state_values_utils.rs.skel
// StateValues 树工具: 解析两级常量 (kMaxInputCount/kMaxInlineDepth) 与
// StateValuesCache 哈希键结构。
import { readV8, writeSrc, banner } from "../../lib.js";

const h = readV8("src/compiler/state-values-utils.h");
const cc = readV8("src/compiler/state-values-utils.cc");

const need = (cond, msg) => {
  if (!cond) throw new Error(`state-values-utils: ${msg}`);
};

for (const [s, name] of [
  ["class V8_EXPORT_PRIVATE StateValuesCache {", "StateValuesCache"],
  ["class V8_EXPORT_PRIVATE StateValuesAccess {", "StateValuesAccess"],
  ["static const size_t kMaxInputCount = 8;", "kMaxInputCount"],
  ["static const int kMaxInlineDepth = 8;", "kMaxInlineDepth"],
  ["struct StateValuesKey : public NodeKey {", "StateValuesKey"],
  ["Node* StateValuesCache::GetNodeForValues(", "cc GetNodeForValues"],
  ["SparseInputMask::BitMaskType StateValuesCache::FillBufferWithValues(", "cc FillBufferWithValues"],
]) need(h.includes(s) || cc.includes(s), `哨兵缺失 ${name}`);

// 常量真解析
const maxInput = Number(h.match(/kMaxInputCount = (\d+)/)[1]);
const maxDepth = Number(h.match(/kMaxInlineDepth = (\d+)/)[1]);
need(maxInput === 8 && maxDepth === 8, `常量 ${maxInput}/${maxDepth}`);

// StateValuesKey 字段 (真解析)
const keyBody = h.slice(h.indexOf("struct StateValuesKey : public NodeKey {"));
const keyFields = [...keyBody.slice(0, 400).matchAll(/^\s+(size_t|SparseInputMask|Node\*\*) (\w+);/gm)].map((m) => m[2]);
need(keyFields.length === 3, `key 字段 ${keyFields}`);

// cc 定义面
const ccDefs = [
  "StateValuesCache::StateValuesCache(JSGraph* js_graph)",
  "Node* StateValuesCache::GetNodeForValues(",
  "SparseInputMask::BitMaskType StateValuesCache::FillBufferWithValues(",
  "Node* StateValuesCache::BuildTree(",
  "StateValuesCache::WorkingBuffer* StateValuesCache::GetWorkingSpace(",
  "Node* StateValuesCache::GetEmptyStateValues()",
  "Node* StateValuesCache::GetValuesNodeFromCache(",
  "bool StateValuesCache::AreKeysEqual(",
  "bool StateValuesCache::IsKeysEqualToNode(",
  "bool StateValuesCache::AreValueKeysEqual(",
  "StateValuesAccess::iterator& StateValuesAccess::iterator::operator++()",
  "StateValuesAccess::TypedNode StateValuesAccess::iterator::operator*()",
  "size_t StateValuesAccess::size() const",
];
for (const d of ccDefs) need(cc.includes(d), `cc 缺 ${d}`);

const body = `${banner("src/compiler/state-values-utils.js", "src/compiler/state-values-utils.h + state-values-utils.cc", "StateValues 树构建缓存与稀疏遍历")}
//! state-values-utils — 帧状态值树的共享子树缓存 + 深度遍历
//!
//! 树节点输入数上限 ${maxInput} (kMaxInputCount), 遍历栈深上限 ${maxDepth}
//! (kMaxInlineDepth); 稀疏编码由 SparseInputMask 位掩码承载。

/// 原版 kMaxInputCount — 单个 StateValues 节点输入上限
pub const K_MAX_INPUT_COUNT: usize = ${maxInput};
/// 原版 kMaxInlineDepth — 遍历内联栈深上限
pub const K_MAX_INLINE_DEPTH: i32 = ${maxDepth};

/// 原版 StateValuesCache::StateValuesKey — 子树缓存键 (${keyFields.join("/")})
pub struct StateValuesKey<'n> {
  /// count
  pub count: usize,
  /// mask
  pub mask: SparseInputMask,
  /// values
  pub values: &'n [*mut Node],
}

/// 原版 StateValuesCache — 共享子树缓存 (CustomMatcherZoneHashMap)
pub struct StateValuesCache<'z, 'g> {
  /// hash_map_
  pub hash_map: CustomMatcherZoneHashMap<'z>,
  /// working_space_ — 每层一个工作缓冲
  pub working_space: Vec<[NodeSlot; K_MAX_INPUT_COUNT]>,
  /// empty_state_values_
  pub empty_state_values: *mut Node,
  /// js_graph_
  pub js_graph: &'g JSGraph<'z>,
}

impl StateValuesCache<'_, '_> {
  /// 原版 GetNodeForValues — 值数组 → (缓存共享的) StateValues 树
  /// v8: src/compiler/state-values-utils.cc StateValuesCache::GetNodeForValues
  pub fn get_node_for_values(&mut self, values: &[*mut Node], liveness: Option<&BytecodeLivenessState>) -> *mut Node {
    todo!("v8: src/compiler/state-values-utils.cc StateValuesCache::GetNodeForValues")
  }

  /// 原版 FillBufferWithValues — 稀疏填充一层缓冲并返回位掩码
  /// v8: src/compiler/state-values-utils.cc StateValuesCache::FillBufferWithValues
  fn fill_buffer_with_values(&mut self, node_buffer: &mut [NodeSlot; K_MAX_INPUT_COUNT], node_count: &mut usize, values_idx: &mut usize, values: &[*mut Node], count: usize, liveness: Option<&BytecodeLivenessState>) -> SparseInputMask {
    todo!("v8: src/compiler/state-values-utils.cc StateValuesCache::FillBufferWithValues")
  }

  /// 原版 BuildTree — 逐层递归建树
  /// v8: src/compiler/state-values-utils.cc StateValuesCache::BuildTree
  fn build_tree(&mut self, values_idx: &mut usize, values: &[*mut Node], count: usize, liveness: Option<&BytecodeLivenessState>, level: usize) -> *mut Node {
    todo!("v8: src/compiler/state-values-utils.cc StateValuesCache::BuildTree")
  }
}

/// 原版 StateValuesAccess::TypedNode — (node, machine type) 对
pub struct TypedNode<'n> {
  /// node
  pub node: &'n Node,
  /// type
  pub type_: MachineType,
}

/// 原版 StateValuesAccess::iterator — 深度优先扁平遍历 (栈深 ≤ ${maxDepth})
pub struct StateValuesIterator<'n> {
  /// stack_ — SparseInputMask::InputIterator 栈
  pub stack: [SparseInputIterator<'n>; K_MAX_INLINE_DEPTH as usize],
  /// current_depth_
  pub current_depth: i32,
}

impl StateValuesIterator<'_> {
  /// 原版 operator++ / Advance — 步进 (跳过空槽由 AdvanceTillNotEmpty 承担)
  /// v8: src/compiler/state-values-utils.cc StateValuesAccess::iterator::operator++
  pub fn advance(&mut self) {
    todo!("v8: src/compiler/state-values-utils.cc StateValuesAccess::iterator::operator++")
  }

  /// 原版 operator* — 当前 TypedNode
  /// v8: src/compiler/state-values-utils.cc StateValuesAccess::iterator::operator*
  pub fn current(&mut self) -> TypedNode<'_> {
    todo!("v8: src/compiler/state-values-utils.cc StateValuesAccess::iterator::operator*")
  }

  /// 原版 done — current_depth_ < 0
  /// v8: src/compiler/state-values-utils.h StateValuesAccess::iterator::done
  pub fn done(&self) -> bool {
    self.current_depth < 0
  }

  /// 原版 AdvanceTillNotEmpty — 跳过空节点并返回跳过数
  /// v8: src/compiler/state-values-utils.cc StateValuesAccess::iterator::AdvanceTillNotEmpty
  pub fn advance_till_not_empty(&mut self) -> usize {
    todo!("v8: src/compiler/state-values-utils.cc StateValuesAccess::iterator::AdvanceTillNotEmpty")
  }
}

/// 原版 StateValuesAccess — StateValues 树遍历门面
pub struct StateValuesAccess<'n> {
  /// node_
  pub node: &'n Node,
}

impl StateValuesAccess<'_> {
  /// 原版 size — 叶子计数
  /// v8: src/compiler/state-values-utils.cc StateValuesAccess::size
  pub fn size(&self) -> usize {
    todo!("v8: src/compiler/state-values-utils.cc StateValuesAccess::size")
  }

  /// 原版 begin_without_receiver — 跳过 receiver
  /// v8: src/compiler/state-values-utils.h StateValuesAccess::begin_without_receiver
  pub fn begin_without_receiver(&self) -> StateValuesIterator<'_> {
    todo!("v8: src/compiler/state-values-utils.h StateValuesAccess::begin_without_receiver")
  }
}
`;

const p = writeSrc("v8-compiler-passes", "state_values_utils.rs.skel", body);
console.log(`written ${p}\nkMaxInputCount=${maxInput} kMaxInlineDepth=${maxDepth} key_fields=${keyFields.length} cc_defs=${ccDefs.length}`);
