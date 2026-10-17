//! MaglevGraphLabeller (maglev-graph-labeller.h) — 节点/块标签注册表数据面。
//!
//! 原版 `absl::flat_hash_map<const NodeBase*, NodeInfo>` → `HashMap<NodeRef, NodeInfo>`
//! (NodeBase 指针 → arena 索引, 单线程等价)。
//! 破环: `MaglevCompilationUnit*` / `BytecodeOffset` / `SourcePosition` 均不在本席,
//! 以本地 *Ref / i32 接缝承接; 打印面 (`PrintNode`/`PrintNodeLabel`/`PrintNodeBrief`) 依赖
//! IR 宿主 + regalloc 数据 → 深水, 登记 readme/zh.md 台账。

use core::fmt;

use gxhash::HashMap;

use crate::graph::{BlockRef, CompilationUnitRef, NodeRef};

/// v8: maglev-graph-labeller.h:17-21 struct Provenance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Provenance {
  /// v8: :18 const MaglevCompilationUnit* unit = nullptr
  pub unit: Option<CompilationUnitRef>,
  /// v8: :19 BytecodeOffset bytecode_offset = BytecodeOffset::None() (接缝, i32::MIN = None)
  pub bytecode_offset: i32,
  /// v8: :20 SourcePosition position = SourcePosition::Unknown() (接缝, -1 = Unknown)
  pub position: i32,
}

impl Provenance {
  /// v8: :18-20 nullptr / None / Unknown 默认哨
  #[inline]
  #[must_use]
  pub const fn unknown() -> Self {
    Self {
      unit: None,
      bytecode_offset: i32::MIN,
      position: -1,
    }
  }
}

/// v8: maglev-graph-labeller.h:22-26 struct NodeInfo
#[derive(Debug, Clone, Copy)]
pub struct NodeInfo {
  /// v8: :23 int label = -1
  pub label: i32,
  /// v8: :24 Provenance provenance
  pub provenance: Provenance,
  /// v8: :25 int line_number = -1
  pub line_number: i32,
}

/// v8: maglev-graph-labeller.h:15-104 class MaglevGraphLabeller
#[derive(Debug)]
pub struct MaglevGraphLabeller {
  /// v8: :102 nodes_ (NodeBase* → NodeInfo)
  nodes: HashMap<NodeRef, NodeInfo>,
  /// v8: :103 next_node_label_ = 1
  next_node_label: i32,
  /// v8: maglev-graph-labeller.cc:14-27 GetBlockLabel 块标签表
  blocks: HashMap<BlockRef, i32>,
  /// v8: cc:15 next_block_id_ = 1
  next_block_id: i32,
}

impl Default for MaglevGraphLabeller {
  /// v8: :103/:15 标签起始 1 (非 0); 空 nodes_/blocks_ 表
  fn default() -> Self {
    Self {
      nodes: HashMap::default(),
      next_node_label: 1,
      blocks: HashMap::default(),
      next_block_id: 1,
    }
  }
}

impl MaglevGraphLabeller {
  #[must_use]
  pub fn new() -> Self {
    Self::default()
  }

  /// v8: :28-35 RegisterNode(node, unit, bytecode_offset, position)
  /// emplace .second: 仅首次插入分配 label 并自增, 重复注册不改动既有 label。
  pub fn register_node(
    &mut self,
    node: NodeRef,
    unit: Option<CompilationUnitRef>,
    bytecode_offset: i32,
    position: i32,
  ) {
    if self.nodes.contains_key(&node) {
      return;
    }
    let label = self.next_node_label;
    self.next_node_label += 1;
    self.nodes.insert(
      node,
      NodeInfo {
        label,
        provenance: Provenance {
          unit,
          bytecode_offset,
          position,
        },
        line_number: -1,
      },
    );
  }

  /// v8: :36-38 RegisterNode(node, const Provenance*)
  #[inline]
  pub fn register_node_with_provenance(&mut self, node: NodeRef, provenance: &Provenance) {
    self.register_node(
      node,
      provenance.unit,
      provenance.bytecode_offset,
      provenance.position,
    );
  }

  /// v8: :39-42 RegisterNode(node) 空 provenance 重载
  #[inline]
  pub fn register_node_default(&mut self, node: NodeRef) {
    let p = Provenance::unknown();
    self.register_node(node, p.unit, p.bytecode_offset, p.position);
  }

  /// v8: :44 NodeId
  #[must_use]
  pub fn node_id(&self, node: NodeRef) -> Option<i32> {
    self.nodes.get(&node).map(|n| n.label)
  }

  /// v8: :45-47 GetNodeProvenance
  #[must_use]
  pub fn get_node_provenance(&self, node: NodeRef) -> Option<Provenance> {
    self.nodes.get(&node).map(|n| n.provenance)
  }

  /// v8: :49-51 SetNodeLineNumber
  pub fn set_node_line_number(&mut self, node: NodeRef, line_number: i32) {
    if let Some(info) = self.nodes.get_mut(&node) {
      info.line_number = line_number;
    }
  }

  /// v8: :52-54 GetNodeLineNumber
  #[must_use]
  pub fn get_node_line_number(&self, node: NodeRef) -> Option<i32> {
    self.nodes.get(&node).map(|n| n.line_number)
  }

  /// v8: :56 max_node_id() = next_node_label_ - 1
  #[inline]
  #[must_use]
  pub const fn max_node_id(&self) -> i32 {
    self.next_node_label - 1
  }

  /// v8: :97-99 nodes()
  #[must_use]
  pub const fn nodes(&self) -> &HashMap<NodeRef, NodeInfo> {
    &self.nodes
  }

  /// v8: maglev-graph-labeller.cc:14-27 GetBlockLabel (block 首次注册分配 id, 重复取回原标)
  pub fn get_block_label(&mut self, block: BlockRef) -> i32 {
    if let Some(&id) = self.blocks.get(&block) {
      return id;
    }
    let id = self.next_block_id;
    self.next_block_id += 1;
    self.blocks.insert(block, id);
    id
  }
}

/// 只读概览 (调试面, 承接 labeller 状态导出)
impl fmt::Display for MaglevGraphLabeller {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "labeller nodes={} blocks={}",
      self.nodes.len(),
      self.blocks.len()
    )
  }
}
