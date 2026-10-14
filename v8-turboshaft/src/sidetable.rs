// 由 tool/src/compiler/turboshaft/sidetable.h.js 从 v8/src/compiler/turboshaft/sidetable.{h,cc} 生成; 已收编 (W11TS2), 复跑产物冲突以本体现态为准
// 侧表: OpIndex/BlockIndex 到值的映射族

use core::mem::swap;

use crate::{graph::Graph, index::OpIndex};

/// 原版 detail::GrowingSidetable<T, Key> — 概念无限映射, 越界自动按
/// out_of_bounds_index + out_of_bounds_index / 2 + 32 增长并默认初始化;
/// Reset() 以 T{} 填充保留内存 (留给后续 phase 复用)。
/// Rust 侧 Zone 推迟 (见 readme/zh.md 台账), 载体取 Vec。
/// v8: src/compiler/turboshaft/sidetable.h detail::GrowingSidetable
#[derive(Debug)]
pub struct GrowingSidetable<T> {
  /// 原版 mutable ZoneVector<T> table_
  pub table: Vec<T>,
}

impl<T> GrowingSidetable<T> {
  /// 原版 explicit GrowingSidetable(Zone* zone) — 空表
  #[must_use]
  pub const fn new() -> Self {
    Self { table: Vec::new() }
  }

  /// 只读探查 (对应原版 const operator[] 的读面, 不触发增长)
  #[inline]
  #[must_use]
  pub fn get(&self, id: u32) -> Option<&T> {
    self.table.get(id as usize)
  }

  /// v8: sidetable.h NextSize — out_of_bounds + out_of_bounds/2 + 32
  const fn next_size(out_of_bounds_index: usize) -> usize {
    out_of_bounds_index + out_of_bounds_index / 2 + 32
  }

  /// 原版 empty — 从未含值 (含 Reset 前)
  /// v8: sidetable.h GrowingSidetable::empty
  #[inline]
  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.table.is_empty()
  }
}

impl<T: Default + Clone> GrowingSidetable<T> {
  /// 原版 GrowingSidetable(size, initial_value, zone)
  #[must_use]
  pub fn with_size(size: usize, initial_value: T) -> Self {
    Self {
      table: vec![initial_value; size],
    }
  }

  /// 原版 operator[](Key) — 越界先 NextSize 增长, 再吞并 resize 过分配
  /// (原版 const/非 const 双版本; Rust 侧增长需 &mut, 只读面走 get)
  /// v8: sidetable.h GrowingSidetable::operator[]
  pub fn get_or_grow(&mut self, id: u32) -> &mut T {
    let i = id as usize;
    if i >= self.table.len() {
      let next = Self::next_size(i);
      self.table.resize(next, T::default());
      // 原版: Make sure we also get access to potential over-allocation by resize()
      let cap = self.table.capacity();
      if cap > self.table.len() {
        self.table.resize(cap, T::default());
      }
    }
    &mut self.table[i]
  }

  /// 原版 Reset — fill(T{}) 不缩容
  /// v8: sidetable.h GrowingSidetable::Reset
  pub fn reset(&mut self) {
    self.table.fill(T::default());
  }
}

impl<T: Default> Default for GrowingSidetable<T> {
  fn default() -> Self {
    Self::new()
  }
}

/// 原版 detail::FixedSidetable<T, Key> — 定长, 越界 DCHECK
/// (Rust 侧禁 panic: 越界以 None 表达 DCHECK 失败的未定义面)
/// v8: src/compiler/turboshaft/sidetable.h detail::FixedSidetable
#[derive(Debug)]
pub struct FixedSidetable<T> {
  /// 原版 ZoneVector<T> table_
  pub table: Vec<T>,
}

impl<T: Default + Clone> FixedSidetable<T> {
  /// 原版 FixedSidetable(size, zone) — 默认初始化
  #[must_use]
  pub fn new(size: usize) -> Self {
    Self {
      table: vec![T::default(); size],
    }
  }

  /// 原版 FixedSidetable(size, default_value, zone)
  #[must_use]
  pub fn with_init(size: usize, default_value: T) -> Self {
    Self {
      table: vec![default_value; size],
    }
  }

  /// 原版 operator[](Key) — DCHECK_LT(op.id(), table_.size())
  #[inline]
  #[must_use]
  pub fn get(&self, id: u32) -> Option<&T> {
    self.table.get(id as usize)
  }

  /// 原版 operator[](Key) 非 const 面
  #[inline]
  pub fn get_mut(&mut self, id: u32) -> Option<&mut T> {
    self.table.get_mut(id as usize)
  }
}

/// 原版 GrowingBlockSidetable<T> — Key = BlockIndex
/// v8: src/compiler/turboshaft/sidetable.h GrowingBlockSidetable
pub type GrowingBlockSidetable<T> = GrowingSidetable<T>;

/// 原版 FixedBlockSidetable<T> — Key = BlockIndex
/// v8: src/compiler/turboshaft/sidetable.h FixedBlockSidetable
pub type FixedBlockSidetable<T> = FixedSidetable<T>;

/// 原版 GrowingOpIndexSidetable<T> — Key = OpIndex;
/// DEBUG 下持 graph_ 做归属校验 (Rust 侧 Option<&Graph>, None = release 语义)
/// v8: src/compiler/turboshaft/sidetable.h GrowingOpIndexSidetable
pub struct GrowingOpIndexSidetable<'g, T> {
  pub base: GrowingSidetable<T>,
  /// 原版 DEBUG 字段 graph_
  pub graph: Option<&'g Graph>,
}

impl<'g, T: Default + Clone> GrowingOpIndexSidetable<'g, T> {
  /// 原版 ctor (zone, graph) — graph 仅 DEBUG DCHECK 用
  #[must_use]
  pub fn new(graph: Option<&'g Graph>) -> Self {
    Self {
      base: GrowingSidetable::new(),
      graph,
    }
  }

  /// 原版 ctor (size, initial_value, zone, graph)
  #[must_use]
  pub fn with_size(size: usize, initial_value: T, graph: Option<&'g Graph>) -> Self
  where
    T: Clone,
  {
    Self {
      base: GrowingSidetable::with_size(size, initial_value),
      graph,
    }
  }

  /// 原版 operator[](OpIndex) — DCHECK(OpIndexBelongsToTableGraph(graph_, index))
  pub fn get_or_grow(&mut self, index: OpIndex) -> &mut T {
    debug_assert!(
      self
        .graph
        .is_none_or(|g| op_index_belongs_to_table_graph(g, index)),
      "OpIndex 不属于本侧表图"
    );
    self.base.get_or_grow(index.id())
  }

  /// v8: sidetable.h SwapData — 交换底层 table_
  pub fn swap_data(&mut self, other: &mut Self) {
    swap(&mut self.base.table, &mut other.base.table);
  }
}

/// 原版 FixedOpIndexSidetable<T> — Key = OpIndex, 定长 + DEBUG 归属校验
/// v8: src/compiler/turboshaft/sidetable.h FixedOpIndexSidetable
pub struct FixedOpIndexSidetable<'g, T> {
  pub base: FixedSidetable<T>,
  /// 原版 DEBUG 字段 graph_
  pub graph: Option<&'g Graph>,
}

impl<'g, T: Default + Clone> FixedOpIndexSidetable<'g, T> {
  /// 原版 ctor (size, zone, graph)
  #[must_use]
  pub fn new(size: usize, graph: Option<&'g Graph>) -> Self {
    Self {
      base: FixedSidetable::new(size),
      graph,
    }
  }

  /// 原版 ctor (size, default_value, zone, graph)
  #[must_use]
  pub fn with_init(size: usize, default_value: T, graph: Option<&'g Graph>) -> Self {
    Self {
      base: FixedSidetable::with_init(size, default_value),
      graph,
    }
  }

  /// 原版 operator[](OpIndex) — DCHECK 归属 + DCHECK_LT 定长
  #[inline]
  #[must_use]
  pub fn get(&self, index: OpIndex) -> Option<&T> {
    debug_assert!(
      self
        .graph
        .is_none_or(|g| op_index_belongs_to_table_graph(g, index)),
      "OpIndex 不属于本侧表图"
    );
    self.base.get(index.id())
  }

  /// 原版 operator[](OpIndex) 非 const 面
  #[inline]
  pub fn get_mut(&mut self, index: OpIndex) -> Option<&mut T> {
    debug_assert!(
      self
        .graph
        .is_none_or(|g| op_index_belongs_to_table_graph(g, index)),
      "OpIndex 不属于本侧表图"
    );
    self.base.get_mut(index.id())
  }

  /// v8: sidetable.h SwapData — 交换底层 table_
  pub fn swap_data(&mut self, other: &mut Self) {
    swap(&mut self.base.table, &mut other.base.table);
  }
}

/// 原版 OpIndexBelongsToTableGraph — graph->BelongsToThisGraph(index)
/// v8: src/compiler/turboshaft/sidetable.cc OpIndexBelongsToTableGraph
#[inline]
#[must_use]
pub fn op_index_belongs_to_table_graph(graph: &Graph, index: OpIndex) -> bool {
  graph.belongs_to_this_graph(index)
}

// 推迟面 (见 readme/zh.md 台账):
// - SparseOpIndexSideTable (依赖 ZoneAbslFlatHashMap + Graph 归属, 消费方在寄存器分配轮)
// - BlockIndex 键的 valid DCHECK 面 (BlockIndex::valid 已在 index.rs, 消费方按 id 传值)
