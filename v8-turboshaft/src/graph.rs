// 由 tool/src/compiler/turboshaft/graph.h.js 从 v8/src/compiler/turboshaft/graph.{h,cc} 生成; 修订就地改, 复跑覆盖
// Turboshaft 图结构: OperationBuffer/Block/Graph 枚举、常量与签名骨架

use crate::index::{BlockIndex, OpIndex, SLOTS_PER_ID};

/// 原版 Block::Kind — 块种类
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BlockKind {
  /// kMerge
  Merge = 0,
  /// kLoopHeader
  LoopHeader = 1,
  /// kBranchTarget
  BranchTarget = 2,
}

impl BlockKind {
  /// 原版 operator<< 名 (graph.cc)
  #[must_use]
  pub const fn as_str(self) -> &'static str {
    match self {
      BlockKind::Merge => "MERGE",
      BlockKind::LoopHeader => "LOOP",
      BlockKind::BranchTarget => "BLOCK",
    }
  }
}

/// 原版 Block::CustomDataKind — 块级临时数据种类 (算法间约定, 跨图不保留)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CustomDataKind {
  /// kUnset — 未设置
  Unset = 0,
  /// kPhiInputIndex
  PhiInputIndex = 1,
  /// kDeferredInSchedule
  DeferredInSchedule = 2,
}

/// 原版 Graph::Origin — 图来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Origin {
  /// kInvalid
  Invalid = 0,
  /// kCreatedFromTurbofan
  CreatedFromTurbofan = 1,
  /// kCreatedFromMaglev
  CreatedFromMaglev = 2,
  /// kPureTurboshaft
  PureTurboshaft = 3,
}

/// 原版 kInvalidPredecessorIndex — 非直接前驱时返回的哨兵
pub const INVALID_PREDECESSOR_INDEX: i32 = -1;

/// 原版 Block 扩容下限 (AllocateNewBlocks 内 kMinCapacity)
pub const MIN_BLOCK_CAPACITY: usize = 32;

/// 原版 Graph 默认初始容量 (操作槽数; 跨 phase 内存复用的关键)
pub const DEFAULT_INITIAL_CAPACITY: usize = 2048;

/// 原版 Block — 基本块 (RandomAccessStackDominatorNode 支配树节点)
///
/// 字段面登记 (原版 Block 私有域):
/// - kind: BlockKind
/// - begin/end: OpIndex (块内操作区间, Invalid 未绑定)
/// - index: BlockIndex (Invalid 未绑定)
/// - last_predecessor / neighboring_predecessor: 前驱链表 (逆序挂链)
/// - single_loop_predecessor: 环回前驱专用槽
/// - predecessor_count: u32
/// - origin: 优化前块来源 (DEBUG 追溯)
/// - custom_data: u32 (算法临时数据)
#[derive(Debug, Clone, Copy)]
pub struct Block {
  /// kind_
  pub kind: BlockKind,
  /// begin_
  pub begin: OpIndex,
  /// end_
  pub end: OpIndex,
  /// index_
  pub index: BlockIndex,
  /// predecessor_count_
  pub predecessor_count: u32,
  /// custom_data_
  pub custom_data: u32,
}

impl Block {
  /// 原版 Block::Block(Kind) — begin/end/index 置 Invalid 哨兵
  /// v8: src/compiler/turboshaft/graph.h Block::Block
  #[must_use]
  pub const fn new(kind: BlockKind, index: BlockIndex) -> Self {
    Self {
      kind,
      begin: OpIndex::INVALID,
      end: OpIndex::INVALID,
      index,
      predecessor_count: 0,
      custom_data: 0,
    }
  }

  /// 原版 IsMerge()
  #[inline]
  #[must_use]
  pub const fn is_merge(self) -> bool {
    matches!(self.kind, BlockKind::Merge)
  }

  /// 原版 IsBranchTarget()
  #[inline]
  #[must_use]
  pub const fn is_branch_target(self) -> bool {
    matches!(self.kind, BlockKind::BranchTarget)
  }

  /// 原版 IsLoopHeader()
  #[inline]
  #[must_use]
  pub const fn is_loop_header(self) -> bool {
    matches!(self.kind, BlockKind::LoopHeader)
  }
}

// 推迟面 (依赖 Zone 与 Operation 载荷, 见 readme/zh.md 台账):
// - begin/end/end_cap 槽指针与 operation_sizes 双写 (首末 id)
// - Allocate/RemoveLast/Index/Get/Next/Previous/BeginIndex/EndIndex/SlotCount/Reset/Grow
// 当前仅登记占用/分配槽计数, 支撑 op_id_count/op_id_capacity。
/// 原版 OperationBuffer — 操作顺序存储 (双向迭代: 尺寸记于首末两个 id)
pub struct OperationBuffer {
  /// 当前已占用操作槽数 (原版 operations_.size())
  pub slot_count: usize,
  /// 已分配操作槽容量 (原版 operations_.capacity())
  pub slot_capacity: usize,
}

impl OperationBuffer {
  /// 原版空缓冲构造
  #[must_use]
  pub const fn new() -> Self {
    Self {
      slot_count: 0,
      slot_capacity: 0,
    }
  }

  /// 原版 size() — 占用槽数
  #[inline]
  #[must_use]
  pub const fn size(&self) -> usize {
    self.slot_count
  }

  /// 原版 capacity() — 分配槽容量
  #[inline]
  #[must_use]
  pub const fn capacity(&self) -> usize {
    self.slot_capacity
  }
}

impl Default for OperationBuffer {
  fn default() -> Self {
    Self::new()
  }
}

/// 原版 Graph — Turboshaft 图属主 (跨 phase 复用内存)
///
/// 就位面: origin 记录、已绑定块表 bound_blocks 与块/操作计数。
/// 推迟面 (依赖 Sidetable/Zone/Operation 载荷, 见 readme/zh.md 台账):
/// - operations_ 实际分配 (见 OperationBuffer)
/// - op_to_block_ / source_positions_ / operation_origins_ / operation_types_ (Sidetable)
/// - graph_zone_ (Zone) / block_permutation_ / stack_checks_to_remove_
/// - Get/KillOperation/Index/BlockOf/SetBlockOf/BlockIndexOf/NextIndex/
///   PreviousIndex/NextOperationIndex/Reset/AllocateNewBlocks/GrowAndNewBlock/
///   ComputeDominator/PrintDominatorTree
pub struct Graph {
  /// origin_ — 图来源
  pub origin: Origin,
  /// operations_ — 操作缓冲
  pub operations: OperationBuffer,
  /// bound_blocks_ — 已绑定块表
  pub bound_blocks: Vec<Block>,
}

impl Graph {
  /// 原版 Graph(graph_zone, origin, initial_capacity = 2048)
  /// Rust 侧不引 Zone (推迟), 仅登记 origin 与空缓冲/空块表。
  #[must_use]
  pub fn new(origin: Origin) -> Self {
    Self {
      origin,
      operations: OperationBuffer::new(),
      bound_blocks: Vec::new(),
    }
  }

  /// 原版 next_block_index() -> BlockIndex — bound_blocks_.size()
  #[inline]
  #[must_use]
  pub fn next_block_index(&self) -> BlockIndex {
    BlockIndex::new(self.bound_blocks.len() as u32)
  }

  /// 原版 block_count() -> u32 — 已绑定块数
  #[inline]
  #[must_use]
  pub fn block_count(&self) -> u32 {
    self.bound_blocks.len() as u32
  }

  /// 原版 op_id_count() -> u32 — (size + kSlotsPerId-1) / kSlotsPerId
  #[inline]
  #[must_use]
  pub fn op_id_count(&self) -> u32 {
    self.operations.size().div_ceil(SLOTS_PER_ID) as u32
  }

  /// 原版 op_id_capacity() -> u32 — capacity / kSlotsPerId
  #[inline]
  #[must_use]
  pub fn op_id_capacity(&self) -> u32 {
    (self.operations.capacity() / SLOTS_PER_ID) as u32
  }

  /// 原版 NewBlockUnchecked 语义面 — 追加一个已绑定块, 返回其 BlockIndex。
  /// 推迟 AllocateNewBlocks 的 Zone 预留与 next_block_ 空闲槽复用 (见台账)。
  pub fn new_block(&mut self, kind: BlockKind) -> BlockIndex {
    let index = self.next_block_index();
    self.bound_blocks.push(Block::new(kind, index));
    index
  }

  /// 原版 Get(BlockIndex) -> &Block — 按 id 取已绑定块 (越界返回 None)。
  #[inline]
  #[must_use]
  pub fn get_block(&self, index: BlockIndex) -> Option<&Block> {
    self.bound_blocks.get(index.id() as usize)
  }

  /// 原版 start_block() -> &Block — 块 0 (未绑定任何块时返回 None)。
  #[inline]
  #[must_use]
  pub fn start_block(&self) -> Option<&Block> {
    self.bound_blocks.first()
  }

  /// 原版 blocks() -> &[Block] — 块切片与迭代
  #[inline]
  #[must_use]
  pub fn blocks(&self) -> &[Block] {
    &self.bound_blocks
  }

  /// 原版 blocks_mut() -> &mut [Block] — 可变块切片
  #[inline]
  pub fn blocks_mut(&mut self) -> &mut [Block] {
    &mut self.bound_blocks
  }

  /// 原版 BelongsToThisGraph(idx) (graph.h) — DEBUG 布局按世代位奇偶判定;
  /// Rust 侧按 release 布局 (无世代位), 归属判定取语义等价面:
  /// 有效 id、非跨图外部 id、且 id 落在本图已分配操作 id 范围内。
  /// v8: src/compiler/turboshaft/graph.h Graph::BelongsToThisGraph
  #[inline]
  #[must_use]
  pub fn belongs_to_this_graph(&self, index: OpIndex) -> bool {
    index.valid() && !index.is_external_id() && index.id() < self.op_id_count()
  }
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(INVALID_PREDECESSOR_INDEX == -1);
  assert!(MIN_BLOCK_CAPACITY == 32);
  assert!(DEFAULT_INITIAL_CAPACITY == 2048);
  assert!(Origin::PureTurboshaft as u8 == 3);
  assert!(CustomDataKind::Unset as u8 == 0);
  assert!(SLOTS_PER_ID == 2);
};
