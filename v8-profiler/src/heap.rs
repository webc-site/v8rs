// 由 tool/src/profiler/heap.js 从 v8/src/profiler/heap-profiler.h, src/profiler/heap-profiler.cc, src/profiler/heap-snapshot-generator.h, src/profiler/heap-snapshot-generator.cc, src/profiler/heap-snapshot-common.h, src/profiler/allocation-tracker.h, src/profiler/allocation-tracker.cc, src/profiler/sampling-heap-profiler.h, src/profiler/sampling-heap-profiler.cc, src/profiler/output-stream-writer.h, src/profiler/output-stream-writer.cc 生成; 修订就地改, 复跑覆盖
// 堆快照面; 边类型与 v8::HeapGraphEdge 公开 ABI 对齐 (见 include/v8-profiler.h)

//! 堆快照与内存分析 — 原版 namespace v8::internal (heap-profiler)
//!
//! 语义: 生成堆对象关系有向图, 追踪内存分配路径与保留树。

use strum::{Display, EnumString};

/// 堆图边类型 (原版 HeapGraphEdge::Type, 7 变体)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, EnumString)]
pub enum HeapGraphEdgeType {
  /// ContextVariable
  ContextVariable,
  /// Element
  Element,
  /// Property
  Property,
  /// Internal
  Internal,
  /// Hidden
  Hidden,
  /// Shortcut
  Shortcut,
  /// Weak
  Weak,
}

impl HeapGraphEdgeType {
  /// 谓词: 是否为弱引用边
  #[inline]
  pub const fn is_weak(&self) -> bool {
    matches!(self, Self::Weak)
  }

  /// 谓词: 是否为内部/隐藏关联
  #[inline]
  pub const fn is_internal_or_hidden(&self) -> bool {
    matches!(self, Self::Internal | Self::Hidden)
  }
}

/// 标记节点条目是否被访问 (原版 enum class MarkEntryAccessed 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkEntryAccessed {
  /// 未访问
  No = 0,
  /// 已访问
  Yes = 1,
}

/// 是否为原生对象 (原版 enum class IsNativeObject 1:1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IsNativeObject {
  /// 否
  No = 0,
  /// 是
  Yes = 1,
}

/// 堆对象之间的引用边 (原版 class HeapGraphEdge 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapGraphEdge {
  /// 边类型
  pub edge_type: HeapGraphEdgeType,
  /// 边名称或属性键
  pub name: String,
  /// 目标节点唯一标识
  pub to_node_id: u64,
}

impl HeapGraphEdge {
  /// 创建新的堆图边
  pub fn new(edge_type: HeapGraphEdgeType, name: impl Into<String>, to_node_id: u64) -> Self {
    Self {
      edge_type,
      name: name.into(),
      to_node_id,
    }
  }
}

/// 堆图节点 (原版 class HeapEntry 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapGraphNode {
  /// 节点 ID
  pub id: u64,
  /// 节点类型名称 (如 "Object", "Array", "Closure")
  pub name: String,
  /// 自身占用浅层大小 (shallow size)
  pub self_size: usize,
  /// 出边列表
  pub edges: Vec<HeapGraphEdge>,
}

impl HeapGraphNode {
  /// 构造新的堆图节点
  pub fn new(id: u64, name: impl Into<String>, self_size: usize) -> Self {
    Self {
      id,
      name: name.into(),
      self_size,
      edges: Vec::new(),
    }
  }

  /// 添加指向其它节点的出边
  pub fn add_edge(&mut self, edge: HeapGraphEdge) {
    self.edges.push(edge);
  }
}

/// 堆快照完整有向图容器 (原版 class HeapSnapshot 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeapSnapshot {
  /// 快照标题
  pub title: String,
  /// 节点集合
  pub nodes: Vec<HeapGraphNode>,
}

impl HeapSnapshot {
  /// 创建新的堆快照
  pub fn new(title: impl Into<String>) -> Self {
    Self {
      title: title.into(),
      nodes: Vec::new(),
    }
  }

  /// 添加节点
  pub fn add_node(&mut self, node: HeapGraphNode) {
    self.nodes.push(node);
  }

  /// 计算所有节点浅层大小总和
  pub fn total_shallow_size(&self) -> usize {
    self.nodes.iter().map(|n| n.self_size).sum()
  }

  /// 节点总数
  #[inline]
  pub fn node_count(&self) -> usize {
    self.nodes.len()
  }

  /// 边总数
  pub fn total_edge_count(&self) -> usize {
    self.nodes.iter().map(|n| n.edges.len()).sum()
  }
}

/// 分配追踪调用节点 (原版 class AllocationTraceNode 1:1)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AllocationTraceNode {
  /// 节点 ID
  pub node_id: u32,
  /// 函数名
  pub name: String,
  /// 分配次数
  pub allocation_count: u32,
  /// 分配总字节数
  pub allocation_size: usize,
  /// 子节点
  pub children: Vec<AllocationTraceNode>,
}

impl AllocationTraceNode {
  /// 构造新的分配追踪节点
  pub fn new(node_id: u32, name: impl Into<String>) -> Self {
    Self {
      node_id,
      name: name.into(),
      allocation_count: 0,
      allocation_size: 0,
      children: Vec::new(),
    }
  }

  /// 记录一次分配
  pub fn record_allocation(&mut self, size: usize) {
    self.allocation_count += 1;
    self.allocation_size += size;
  }
}
