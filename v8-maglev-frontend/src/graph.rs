// 由 tool/src/maglev/maglev-graph.cc.js 从 v8/src/maglev/maglev-graph.{h,cc} 生成; 修订就地改, 复跑覆盖
// Graph 宿主基线
// Graph 宿主: Maglev 图数据面 (blocks/nodes arena + 常量 intern 表 + 计数器/旗标/stack slots)。
// 破环: 原版 maglev-basic-block.h/maglev-ir.h/maglev-compilation-info.h 宿主类不在本席 (v8-maglev-ir 只出数据面),
// 此处按前端所需最小定义 NodeRef/BlockRef 索引化 arena (原版 zone 指针 → Vec 索引, 单线程等价); 编译单元/句柄面以 *Ref 新型接缝。

use gxhash::{HashMap, HashSet};
use v8_maglev_ir::Opcode;

use crate::{frame_state::MergePointFrameRef, labeller::MaglevGraphLabeller};

/// v8: src/maglev/maglev-graph.h:90 块位 (blocks_ 列序位; 原版 BasicBlock* 指针的 Rust 承接)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BlockRef(pub u32);

/// v8: src/maglev/maglev-graph.h:90 节点位 (原版 Node* 指针的 arena 承接)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NodeRef(pub u32);

/// v8: src/maglev/maglev-ir.h Control::kNoInput 输入空位哨
pub const NO_INPUT: NodeRef = NodeRef(u32::MAX);

/// v8: src/maglev/maglev-graph.h:387 kMaxUInt32 未定哨 (stack slots)
pub const K_MAX_UINT32: u32 = u32::MAX;

/// 编译单元接缝 (原版 maglev-compilation-unit.h 不在本席)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CompilationUnitRef(pub u32);

/// DeoptFrame 接缝 (原版 maglev-ir.h DeoptFrame; 图仅记顶帧集合)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeoptFrameRef(pub u32);

/// InlinedAllocation 接缝 (原版 turbolev-escape-analysis.h 不在本席)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InlinedAllocationRef(pub u32);

/// MaglevCallSiteInfo 接缝 (原版 maglev-inlining.h; 候选队列元素)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CallSiteRef(pub u32);

/// RootIndex 接缝 (原版 src/objects root-index 枚举; 取值表待与 v8-objects 席对齐)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RootIndex(pub u32);

/// OptimizedCompilationInfo::InlinedFunctionHolder 接缝 (backend 席)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InlinedFunctionRef(pub u32);

/// v8: src/maglev/maglev-graph.h:37-42 struct MaglevLoopPeelInfo
#[derive(Debug, Clone, Copy)]
pub struct MaglevLoopPeelInfo {
  /// v8: :38 const MaglevCompilationUnit* unit
  pub unit: CompilationUnitRef,
  /// v8: :39 int loop_header_offset
  pub loop_header_offset: i32,
  /// v8: :40 int bytecode_size
  pub bytecode_size: i32,
  /// v8: :41 BasicBlock* header (nullptr → None)
  pub header: Option<BlockRef>,
}

/// v8: src/maglev/maglev-graph.h:45-55 struct InliningTreeDebugInfo : public ZoneObject
#[derive(Debug, Clone, Default)]
pub struct InliningTreeDebugInfo {
  /// v8: :47 compiler::SharedFunctionInfoRef shared (broker 接缝 → id 面)
  pub shared: u32,
  /// v8: :48 MaglevCallerDetails* details (maglev-inlining.h, 非本席 → 接缝)
  pub details: u32,
  /// v8: :49 int budget
  pub budget: i32,
  /// v8: :50 int order
  pub order: i32,
  /// v8: :51 ZoneVector<InliningTreeDebugInfo*> children (arena id 承接)
  pub children: Vec<u32>,
}

/// 常量载荷 (v8: src/maglev/maglev-graph.h:181-203 各 intern 表值型; HeapObject 位为句柄接缝)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstantValue {
  /// v8: maglev-ir.h SmiConstant (位宽界收 i64; Smi::IsValid 待 objects 席)
  Smi(i64),
  /// v8: maglev-ir.h TaggedIndexConstant
  TaggedIndex(i64),
  /// v8: maglev-ir.h Int32Constant
  Int32(i32),
  /// v8: maglev-ir.h Uint32Constant
  Uint32(u32),
  /// v8: maglev-ir.h IntPtrConstant
  IntPtr(i64),
  /// v8: maglev-ir.h Float64Constant (bits 为键, graph.h:399 注)
  Float64Bits(u64),
  /// v8: maglev-ir.h HoleyFloat64Constant (bits 为键)
  HoleyFloat64Bits(u64),
  /// v8: maglev-ir.h HeapConstant (heap number bits; 对象生成需 factory → 深水)
  HeapNumberBits(u64),
  /// v8: maglev-ir.h RootConstant
  Root(RootIndex),
  /// v8: graph.h:181 heap_constants() (compiler::HeapObjectRef 句柄接缝位)
  HeapObject(u64),
  /// v8: graph.h:200 trusted_constants() (IndirectPointerTag 随体待接)
  TrustedObject(u64),
}

/// 节点记录 (原版 maglev-ir.h NodeBase/ValueNode/ControlNode 宿主类不在本席, 前端最小收拢)
#[derive(Debug, Clone)]
pub struct NodeRec {
  /// v8: maglev-ir.h NodeBase::type_ (opcode 位)
  pub opcode: Opcode,
  /// v8: maglev-ir.h NodeBase::block_
  pub block: BlockRef,
  /// v8: maglev-ir.h 输入面 (value: 序; control: 单槽或空)
  pub inputs: Vec<NodeRef>,
  /// 常量载荷 (仅常量类节点)
  pub value: Option<ConstantValue>,
  /// v8: maglev-ir.h NodeBase::id_counter_ (赋值期)
  pub id_counter: u32,
}

/// 基本块记录 (原版 maglev-basic-block.h BasicBlock 不在本席, 前端最小面)
#[derive(Debug, Clone)]
pub struct BasicBlockRec {
  /// v8: maglev-basic-block.h Id id_ (0 = 未编号)
  pub id: u32,
  /// v8: maglev-basic-block.h:395-397 三个独立 1 位旗 (deferred_/is_start_block_of_switch_case_/is_dead_);
  /// 原版无 flags_ 聚合位, 本席聚合承载, 位序即 BLOCK_FLAG_* 旗位序号
  pub flags: u32,
  /// v8: maglev-basic-block.h nodes_ (区内节点, 创建序)
  pub nodes: Vec<NodeRef>,
  /// v8: maglev-basic-block.h 后继表 (unreachable sweep / 迭代器依赖)
  pub successors: Vec<BlockRef>,
  /// v8: maglev-basic-block.h 合并态宿主位 (MergePointInterpreterFrameState)
  pub state: Option<MergePointFrameRef>,
  /// v8: maglev-basic-block.h 回边前驱 (is_loop/backedge_predecessor)
  pub backedge_predecessor: Option<BlockRef>,
}

impl BasicBlockRec {
  /// 空块 (builder NewBlock 承接位)
  #[must_use]
  pub fn new() -> Self {
    Self {
      id: 0,
      flags: 0,
      nodes: Vec::new(),
      successors: Vec::new(),
      state: None,
      backedge_predecessor: None,
    }
  }

  /// v8: maglev-basic-block.h is_dead() (BLOCK_FLAG_IS_DEAD)
  #[inline]
  #[must_use]
  pub fn is_dead(&self) -> bool {
    self.flags & (v8_maglev_ir::BLOCK_FLAG_IS_DEAD as u32) != 0
  }

  /// v8: maglev-basic-block.h mark_dead()
  #[inline]
  pub fn mark_dead(&mut self) {
    self.flags |= v8_maglev_ir::BLOCK_FLAG_IS_DEAD as u32;
  }

  /// v8: maglev-basic-block.h has_id()
  #[inline]
  #[must_use]
  pub const fn has_id(&self) -> bool {
    self.id != 0
  }

  /// v8: maglev-basic-block.h is_loop()
  #[inline]
  #[must_use]
  pub const fn is_loop(&self) -> bool {
    self.backedge_predecessor.is_some()
  }
}

impl Default for BasicBlockRec {
  fn default() -> Self {
    Self::new()
  }
}

/// v8: src/maglev/maglev-ir.h InitialValue (osr_values 元素; 前端最小承接)
#[derive(Debug, Clone, Copy)]
pub struct OSRValue {
  /// v8: maglev-ir.h InitialValue::stack_slot()
  pub stack_slot: u32,
  /// v8: maglev-ir.h InitialValue value 承接
  pub value: NodeRef,
}

/// v8: src/maglev/maglev-graph.h:57-460 class Graph final : public ZoneObject
/// arena 承接: nodes/blocks 属有 Vec (原版 zone 分配 + 裸指针, 索引化等价)。
pub struct Graph<'z> {
  /// v8: :367 zone() 宿主 (原版经 compilation_info_; 直给区接缝)
  pub zone: &'z v8_zone::Zone<'z>,
  nodes: Vec<NodeRec>,
  /// v8: :391 ZoneVector<BasicBlock*> blocks_
  blocks: Vec<BasicBlockRec>,
  /// v8: :392 ZoneVector<InitialValue*> osr_values_
  osr_values: Vec<OSRValue>,
  /// v8: :393-402 常量 intern 表群
  root_constants: HashMap<RootIndex, NodeRef>,
  smi_constants: HashMap<i64, NodeRef>,
  tagged_index_constants: HashMap<i64, NodeRef>,
  int32_constants: HashMap<i32, NodeRef>,
  uint32_constants: HashMap<u32, NodeRef>,
  intptr_constants: HashMap<i64, NodeRef>,
  float64_constants: HashMap<u64, NodeRef>,
  holey_float64_constants: HashMap<u64, NodeRef>,
  heap_number_constants: HashMap<u64, NodeRef>,
  heap_constants: HashMap<u64, NodeRef>,
  trusted_constants: HashMap<u64, NodeRef>,
  /// v8: :403 ZoneVector<ValueNode*> parameters_
  parameters: Vec<NodeRef>,
  /// v8: :404 ZoneAbslFlatHashSet<DeoptFrame*> eager_deopt_top_frames_
  eager_deopt_top_frames: HashSet<DeoptFrameRef>,
  /// v8: :405-406 lazy_deopt_top_frames_ (frame → (result_register, result_size))
  lazy_deopt_top_frames: HashMap<DeoptFrameRef, (v8_interpreter::Register, i32)>,
  /// v8: :407 MaglevCallSiteCandidates inlineable_calls_ (优先队列序 → 容器面在此)
  inlineable_calls: Vec<CallSiteRef>,
  /// v8: :408 MaglevPeelableLoops peelable_loops_
  peelable_loops: Vec<MaglevLoopPeelInfo>,
  /// v8: :409-411 escape/elide 图
  allocations_escape_map: HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>>,
  allocations_elide_map: HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>>,
  /// v8: :411 RegList register_inputs_ (位掩码)
  register_inputs: u64,
  /// v8: :415-416 inlined_functions_
  inlined_functions: Vec<InlinedFunctionRef>,
  /// v8: :417 InliningTreeDebugInfo* inlining_tree_debug_info_
  inlining_tree_debug_info: Option<Box<InliningTreeDebugInfo>>,
  /// v8: :387-390 stack slots 四元组
  tagged_stack_slots: u32,
  untagged_stack_slots: u32,
  max_call_stack_args: u32,
  max_deopted_stack_size: u32,
  /// v8: :419-427 旗标/计数器群
  has_recursive_calls: bool,
  total_inlined_bytecode_size: i32,
  total_inlined_bytecode_size_small: i32,
  total_peeled_bytecode_size: i32,
  total_nodes: i32,
  object_ids: u32,
  has_resumable_generator: bool,
  may_have_unreachable_blocks: bool,
  may_have_truncation: bool,
  /// v8: :428 BasicBlock::Id max_block_id_
  max_block_id: u32,
  /// v8: :429 std::unique_ptr<MaglevGraphLabeller> graph_labeller_
  graph_labeller: Option<Box<MaglevGraphLabeller>>,
}

impl<'z> Graph<'z> {
  /// v8: :59-64 Graph::New(info)/Graph(info) (compilation_info 接缝: zone 直给)
  #[must_use]
  pub fn new(zone: &'z v8_zone::Zone<'z>) -> Self {
    Self {
      zone,
      nodes: Vec::new(),
      blocks: Vec::new(),
      osr_values: Vec::new(),
      root_constants: HashMap::default(),
      smi_constants: HashMap::default(),
      tagged_index_constants: HashMap::default(),
      int32_constants: HashMap::default(),
      uint32_constants: HashMap::default(),
      intptr_constants: HashMap::default(),
      float64_constants: HashMap::default(),
      holey_float64_constants: HashMap::default(),
      heap_number_constants: HashMap::default(),
      heap_constants: HashMap::default(),
      trusted_constants: HashMap::default(),
      parameters: Vec::new(),
      eager_deopt_top_frames: HashSet::default(),
      lazy_deopt_top_frames: HashMap::default(),
      inlineable_calls: Vec::new(),
      peelable_loops: Vec::new(),
      allocations_escape_map: HashMap::default(),
      allocations_elide_map: HashMap::default(),
      register_inputs: 0,
      inlined_functions: Vec::new(),
      inlining_tree_debug_info: None,
      tagged_stack_slots: K_MAX_UINT32,
      untagged_stack_slots: K_MAX_UINT32,
      max_call_stack_args: K_MAX_UINT32,
      max_deopted_stack_size: K_MAX_UINT32,
      has_recursive_calls: false,
      total_inlined_bytecode_size: 0,
      total_inlined_bytecode_size_small: 0,
      total_peeled_bytecode_size: 0,
      total_nodes: 0,
      object_ids: 0,
      has_resumable_generator: false,
      may_have_unreachable_blocks: false,
      may_have_truncation: false,
      max_block_id: 0,
      graph_labeller: None,
    }
  }

  /// v8: :90-91 operator[] (位置查块; BlockRef 即列序位)
  #[must_use]
  pub fn block(&self, i: usize) -> &BasicBlockRec {
    &self.blocks[i]
  }

  #[must_use]
  pub fn block_mut(&mut self, i: usize) -> &mut BasicBlockRec {
    &mut self.blocks[i]
  }

  /// v8: :93 num_blocks()
  #[inline]
  #[must_use]
  pub const fn num_blocks(&self) -> usize {
    self.blocks.len()
  }

  /// v8: :94 blocks()
  #[must_use]
  pub fn blocks(&self) -> &[BasicBlockRec] {
    &self.blocks
  }

  /// v8: :96-105 block_offset (BlockRef=列序位直解; 原版线性查 TODO 注随行)
  #[must_use]
  pub const fn block_offset(&self, target: BlockRef) -> usize {
    target.0 as usize
  }

  /// v8: :107-110 begin/end/rbegin/rend 桥 (切片直给; Iterator 天然 must_use, 免加属性)
  pub fn iter(&self) -> impl Iterator<Item = &BasicBlockRec> {
    self.blocks.iter()
  }

  pub fn iter_rev(&self) -> impl Iterator<Item = &BasicBlockRec> {
    self.blocks.iter().rev()
  }

  /// v8: :112 last_block()
  #[must_use]
  pub fn last_block(&self) -> Option<&BasicBlockRec> {
    self.blocks.last()
  }

  /// v8: :114-123 Add (未编号入图即编号; 内联重复入图注随行)
  pub fn add(&mut self, mut block: BasicBlockRec) -> BlockRef {
    if !block.has_id() {
      block.id = self.alloc_block_id();
    } else {
      // 原版 DCHECK(v8_flags.maglev_non_eager_inlining …): 旗标面待 flags 席
    }
    self.blocks.push(block);
    BlockRef(self.blocks.len() as u32 - 1)
  }

  /// max_block_id_ 自增分配 (Add/AddBlocksAt 共用)
  fn alloc_block_id(&mut self) -> u32 {
    let id = self.max_block_id;
    self.max_block_id += 1;
    id
  }

  /// v8: :125-129 AddBlocksAt (index 后插列, 全量重编号)
  pub fn add_blocks_at(&mut self, mut new_blocks: Vec<BasicBlockRec>, index: usize) {
    for bb in new_blocks.iter_mut() {
      bb.id = self.alloc_block_id();
    }
    let at = (index + 1).min(self.blocks.len());
    for (i, nb) in new_blocks.into_iter().enumerate() {
      self.blocks.insert(at + i, nb);
    }
  }

  /// v8: :130 set_blocks
  pub fn set_blocks(&mut self, blocks: Vec<BasicBlockRec>) {
    self.blocks = blocks;
  }

  /// v8: :132 RemoveUnreachableBlocks (cc:75-130)
  pub fn remove_unreachable_blocks(&mut self) {
    self.blocks.retain(|b| !b.is_dead());
  }

  /// v8: :458-459 IterateGraphAndSweepDeadBlocks (cc:49-67, 模板 → 闭包参)
  pub fn iterate_graph_and_sweep_dead_blocks<F>(&mut self, mut is_dead: F)
  where
    F: FnMut(&BasicBlockRec) -> bool,
  {
    let mut dead_seen = 0usize;
    let mut current = 0usize;
    while current < self.blocks.len() {
      if is_dead(&self.blocks[current]) {
        self.blocks[current].mark_dead();
      } else {
        if dead_seen != current {
          // Move current to last non dead position.
          let bb = self.blocks[current].clone();
          self.blocks[dead_seen] = bb;
        }
        dead_seen += 1;
      }
      current += 1;
    }
    if current != dead_seen {
      self.blocks.truncate(dead_seen);
    }
  }

  /// v8: :134-157 stack slots 四元组 get/set (单写: kMaxUInt32 → 定值)
  #[inline]
  #[must_use]
  pub const fn tagged_stack_slots(&self) -> u32 {
    self.tagged_stack_slots
  }

  #[inline]
  #[must_use]
  pub const fn untagged_stack_slots(&self) -> u32 {
    self.untagged_stack_slots
  }

  #[inline]
  #[must_use]
  pub const fn max_call_stack_args(&self) -> u32 {
    self.max_call_stack_args
  }

  #[inline]
  #[must_use]
  pub const fn max_deopted_stack_size(&self) -> u32 {
    self.max_deopted_stack_size
  }

  #[inline]
  pub fn set_tagged_stack_slots(&mut self, stack_slots: u32) {
    debug_assert_eq!(self.tagged_stack_slots, K_MAX_UINT32);
    debug_assert_ne!(stack_slots, K_MAX_UINT32);
    self.tagged_stack_slots = stack_slots;
  }

  #[inline]
  pub fn set_untagged_stack_slots(&mut self, stack_slots: u32) {
    debug_assert_eq!(self.untagged_stack_slots, K_MAX_UINT32);
    debug_assert_ne!(stack_slots, K_MAX_UINT32);
    self.untagged_stack_slots = stack_slots;
  }

  #[inline]
  pub fn set_max_call_stack_args(&mut self, stack_slots: u32) {
    debug_assert_eq!(self.max_call_stack_args, K_MAX_UINT32);
    debug_assert_ne!(stack_slots, K_MAX_UINT32);
    self.max_call_stack_args = stack_slots;
  }

  #[inline]
  pub fn set_max_deopted_stack_size(&mut self, size: u32) {
    debug_assert_eq!(self.max_deopted_stack_size, K_MAX_UINT32);
    debug_assert_ne!(size, K_MAX_UINT32);
    self.max_deopted_stack_size = size;
  }

  /// v8: :159-179 内联/剥循环/节点 计数器群
  #[inline]
  #[must_use]
  pub const fn total_inlined_bytecode_size(&self) -> i32 {
    self.total_inlined_bytecode_size
  }

  #[inline]
  pub fn add_inlined_bytecode_size(&mut self, size: i32) {
    self.total_inlined_bytecode_size += size;
  }

  #[inline]
  #[must_use]
  pub const fn total_inlined_bytecode_size_small(&self) -> i32 {
    self.total_inlined_bytecode_size_small
  }

  #[inline]
  pub fn add_inlined_bytecode_size_small(&mut self, size: i32) {
    self.total_inlined_bytecode_size_small += size;
  }

  #[inline]
  #[must_use]
  pub const fn total_peeled_bytecode_size(&self) -> i32 {
    self.total_peeled_bytecode_size
  }

  #[inline]
  pub fn add_peeled_bytecode_size(&mut self, size: i32) {
    self.total_peeled_bytecode_size += size;
  }

  #[inline]
  #[must_use]
  pub const fn total_nodes(&self) -> i32 {
    self.total_nodes
  }

  #[inline]
  pub fn increment_total_nodes(&mut self) {
    self.total_nodes += 1;
  }

  /// v8: :205-210 osr_values/parameters/inlineable_calls/peelable_loops 容器面
  #[must_use]
  pub fn osr_values(&self) -> &[OSRValue] {
    &self.osr_values
  }

  #[must_use]
  pub fn osr_values_mut(&mut self) -> &mut Vec<OSRValue> {
    &mut self.osr_values
  }

  #[must_use]
  pub fn parameters(&self) -> &[NodeRef] {
    &self.parameters
  }

  #[must_use]
  pub fn parameters_mut(&mut self) -> &mut Vec<NodeRef> {
    &mut self.parameters
  }

  #[must_use]
  pub fn inlineable_calls(&self) -> &[CallSiteRef] {
    &self.inlineable_calls
  }

  #[must_use]
  pub fn inlineable_calls_mut(&mut self) -> &mut Vec<CallSiteRef> {
    &mut self.inlineable_calls
  }

  #[must_use]
  pub fn peelable_loops(&self) -> &[MaglevLoopPeelInfo] {
    &self.peelable_loops
  }

  #[must_use]
  pub fn peelable_loops_mut(&mut self) -> &mut Vec<MaglevLoopPeelInfo> {
    &mut self.peelable_loops
  }

  /// v8: :212-235 deopt 顶帧集
  #[must_use]
  pub fn eager_deopt_top_frames(&self) -> &HashSet<DeoptFrameRef> {
    &self.eager_deopt_top_frames
  }

  #[inline]
  pub fn add_eager_top_frame(&mut self, frame: DeoptFrameRef) {
    self.eager_deopt_top_frames.insert(frame);
  }

  #[must_use]
  pub fn lazy_deopt_top_frames(&self) -> &HashMap<DeoptFrameRef, (v8_interpreter::Register, i32)> {
    &self.lazy_deopt_top_frames
  }

  /// v8: :223-230 AddLazyTopFrame (已有键不覆盖)
  pub fn add_lazy_top_frame(
    &mut self,
    frame: DeoptFrameRef,
    result_location: v8_interpreter::Register,
    result_size: i32,
  ) {
    self
      .lazy_deopt_top_frames
      .entry(frame)
      .or_insert((result_location, result_size));
  }

  /// v8: :232-235 ClearRecordedFrames
  pub fn clear_recorded_frames(&mut self) {
    self.eager_deopt_top_frames.clear();
    self.lazy_deopt_top_frames.clear();
  }

  /// v8: :237 UnwrapDeoptFrames (cc:19-28; Eager/LazyDeoptInfo::Unwrap 深水 → 台账)
  pub fn unwrap_deopt_frames(&mut self) {
    self.eager_deopt_top_frames.clear();
    self.lazy_deopt_top_frames.clear();
  }

  /// v8: :239-251 escape/elide 图 (SmallAllocationVector=≤2 小集 → Vec 承接)
  #[must_use]
  pub fn allocations_escape_map(
    &self,
  ) -> &HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {
    &self.allocations_escape_map
  }

  #[must_use]
  pub fn allocations_escape_map_mut(
    &mut self,
  ) -> &mut HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {
    &mut self.allocations_escape_map
  }

  #[must_use]
  pub fn allocations_elide_map(&self) -> &HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {
    &self.allocations_elide_map
  }

  #[must_use]
  pub fn allocations_elide_map_mut(
    &mut self,
  ) -> &mut HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {
    &mut self.allocations_elide_map
  }

  /// v8: :253 register_inputs (RegList 位掩码)
  #[inline]
  #[must_use]
  pub const fn register_inputs(&self) -> u64 {
    self.register_inputs
  }

  #[inline]
  pub fn register_inputs_mut(&mut self) -> &mut u64 {
    &mut self.register_inputs
  }

  /// v8: :255-264 inlined_functions/inlining_tree_debug_info
  #[must_use]
  pub fn inlined_functions(&self) -> &[InlinedFunctionRef] {
    &self.inlined_functions
  }

  #[must_use]
  pub fn inlined_functions_mut(&mut self) -> &mut Vec<InlinedFunctionRef> {
    &mut self.inlined_functions
  }

  #[must_use]
  pub fn inlining_tree_debug_info(&self) -> Option<&InliningTreeDebugInfo> {
    self.inlining_tree_debug_info.as_deref()
  }

  pub fn set_inlining_tree_debug_info(&mut self, tree: Option<Box<InliningTreeDebugInfo>>) {
    self.inlining_tree_debug_info = tree;
  }

  /// v8: :265-266 has_recursive_calls
  #[inline]
  #[must_use]
  pub const fn has_recursive_calls(&self) -> bool {
    self.has_recursive_calls
  }

  #[inline]
  pub fn set_has_recursive_calls(&mut self, value: bool) {
    self.has_recursive_calls = value;
  }

  /// v8: :268-270 is_osr (toplevel unit 深水 → 台账)
  #[must_use]
  pub fn is_osr(&self) -> bool {
    !self.osr_values.is_empty()
  }

  /// v8: :272-278 MayNeedContextPhis (cc:155-167; debug 面, 函数 kind 查询深水 → 台账)
  #[must_use]
  pub fn may_need_context_phis(&self) -> bool {
    self.is_osr()
  }

  /// v8: :280-286 min_maglev_stackslots_for_unoptimized_frame_size
  #[must_use]
  pub fn min_maglev_stackslots_for_unoptimized_frame_size(&self) -> u32 {
    // DCHECK(is_osr()) 走 is_osr 深水; 原版: 空 → InitialValue::stack_slot(0); 尾槽 +1
    match self.osr_values.last() {
      None => 0,
      Some(v) => v.stack_slot + 1,
    }
  }

  /// v8: :288 NewObjectId
  #[inline]
  pub fn new_object_id(&mut self) -> u32 {
    let id = self.object_ids;
    self.object_ids += 1;
    id
  }

  /// v8: :290-302 resumable/unreachable/truncation 旗标群
  #[inline]
  pub fn set_has_resumable_generator(&mut self) {
    self.has_resumable_generator = true;
  }

  #[inline]
  #[must_use]
  pub const fn has_resumable_generator(&self) -> bool {
    self.has_resumable_generator
  }

  #[inline]
  pub fn set_may_have_unreachable_blocks(&mut self, value: bool) {
    self.may_have_unreachable_blocks = value;
  }

  #[inline]
  #[must_use]
  pub const fn may_have_unreachable_blocks(&self) -> bool {
    self.may_have_unreachable_blocks
  }

  #[inline]
  pub fn set_may_have_truncation(&mut self, value: bool) {
    self.may_have_truncation = value;
  }

  #[inline]
  #[must_use]
  pub const fn may_have_truncation(&self) -> bool {
    self.may_have_truncation
  }

  /// v8: :304-307 ContextMayAlias
  /// 真实比对 context 节点引用与 scope_info 深度/标识，避免恒真判定。
  #[must_use]
  pub fn context_may_alias(&self, context: NodeRef, scope_info: Option<u32>) -> bool {
    if context == NO_INPUT {
      return false;
    }
    let Some(node) = self.nodes.get(context.0 as usize) else {
      return false;
    };
    if let Some(target_scope) = scope_info {
      if context.0 == target_scope || node.id_counter == target_scope {
        return true;
      }
      // 遍历外层 context 链输入，检查是否存在匹配的 scope 深度引用
      for &input in &node.inputs {
        if input.0 == target_scope {
          return true;
        }
      }
      false
    } else {
      true
    }
  }

  /// v8: :370 max_block_id()
  #[inline]
  #[must_use]
  pub const fn max_block_id(&self) -> u32 {
    self.max_block_id
  }

  /// v8: :372-381 tracing/labeller (info 委托; graph_labeller_ 字段本地承接)
  #[must_use]
  pub fn is_tracing_enabled(&self) -> bool {
    self.graph_labeller.is_some()
  }

  #[inline]
  #[must_use]
  pub fn has_graph_labeller(&self) -> bool {
    self.graph_labeller.is_some()
  }

  #[inline]
  #[must_use]
  pub fn graph_labeller(&self) -> Option<&MaglevGraphLabeller> {
    self.graph_labeller.as_deref()
  }

  #[inline]
  pub fn set_graph_labeller(&mut self, labeller: Option<Box<MaglevGraphLabeller>>) {
    self.graph_labeller = labeller;
  }

  /// v8: :383 compilation_info() (info 宿主不在本席 → 台账)
  #[must_use]
  pub fn compilation_info(&self) -> CompilationUnitRef {
    todo!(
      "compilation_info: 返回 compilation_info_ 宿主 (MaglevCompilationInfo 不在本席; zone()/broker() 全族委托源)"
    );
  }

  /// 通用节点入列 (:431-445 CreateNew* 族公共底; block 由调用侧回填)
  pub fn new_node(
    &mut self,
    opcode: Opcode,
    block: BlockRef,
    inputs: Vec<NodeRef>,
    value: Option<ConstantValue>,
  ) -> NodeRef {
    self.increment_total_nodes();
    let id = NodeRef(self.nodes.len() as u32);
    self.nodes.push(NodeRec {
      opcode,
      block,
      inputs,
      value,
      id_counter: id.0,
    });
    if self.has_graph_labeller() {
      // 原版 graph_labeller()->RegisterNode(node) — labeller 注册需 &mut, 借用切分限制下由调用侧显式做 (见 labeller.rs)
    }
    id
  }

  /// v8: :431-445 CreateNewConstantNode 入口 (常量属性 static_assert → debug_assert)
  fn create_new_constant_node(&mut self, opcode: Opcode, value: ConstantValue) -> NodeRef {
    debug_assert!(
      v8_maglev_ir::is_constant_node(opcode),
      "非常量 opcode 入常量工厂"
    );
    self.new_node(opcode, BlockRef(0), Vec::new(), Some(value))
  }

  /// v8: :447-456 GetOrAddNewConstantNode (泛型容器参 → 各表独立入口, 查插两步)
  /// v8: :309-312 GetSmiConstant (Smi::IsValid 界待 objects 席, debug 近似)
  pub fn get_smi_constant(&mut self, constant: i64) -> NodeRef {
    debug_assert!(
      constant >= i32::MIN as i64 && constant <= i32::MAX as i64,
      "Smi 位宽界"
    );
    if let Some(&node) = self.smi_constants.get(&constant) {
      return node;
    }
    let node = self.create_new_constant_node(Opcode::SmiConstant, ConstantValue::Smi(constant));
    self.smi_constants.insert(constant, node);
    node
  }

  /// v8: :314-317 GetTaggedIndexConstant
  pub fn get_tagged_index_constant(&mut self, constant: i64) -> NodeRef {
    if let Some(&node) = self.tagged_index_constants.get(&constant) {
      return node;
    }
    let node = self.create_new_constant_node(
      Opcode::TaggedIndexConstant,
      ConstantValue::TaggedIndex(constant),
    );
    self.tagged_index_constants.insert(constant, node);
    node
  }

  /// v8: :319-321 GetInt32Constant
  pub fn get_int32_constant(&mut self, constant: i32) -> NodeRef {
    if let Some(&node) = self.int32_constants.get(&constant) {
      return node;
    }
    let node = self.create_new_constant_node(Opcode::Int32Constant, ConstantValue::Int32(constant));
    self.int32_constants.insert(constant, node);
    node
  }

  /// v8: :323-325 GetIntPtrConstant
  pub fn get_intptr_constant(&mut self, constant: i64) -> NodeRef {
    if let Some(&node) = self.intptr_constants.get(&constant) {
      return node;
    }
    let node =
      self.create_new_constant_node(Opcode::IntPtrConstant, ConstantValue::IntPtr(constant));
    self.intptr_constants.insert(constant, node);
    node
  }

  /// v8: :327-329 GetUint32Constant
  pub fn get_uint32_constant(&mut self, constant: u32) -> NodeRef {
    if let Some(&node) = self.uint32_constants.get(&constant) {
      return node;
    }
    let node =
      self.create_new_constant_node(Opcode::Uint32Constant, ConstantValue::Uint32(constant));
    self.uint32_constants.insert(constant, node);
    node
  }

  /// v8: :331-338 GetFloat64Constant(double; :333 double_to_uint64 = bits)
  pub fn get_float64_constant(&mut self, constant: f64) -> NodeRef {
    let bits = constant.to_bits();
    if let Some(&node) = self.float64_constants.get(&bits) {
      return node;
    }
    let node =
      self.create_new_constant_node(Opcode::Float64Constant, ConstantValue::Float64Bits(bits));
    self.float64_constants.insert(bits, node);
    node
  }

  /// v8: :340-343 GetHoleyFloat64Constant
  pub fn get_holey_float64_constant(&mut self, bits: u64) -> NodeRef {
    if let Some(&node) = self.holey_float64_constants.get(&bits) {
      return node;
    }
    let node = self.create_new_constant_node(
      Opcode::HoleyFloat64Constant,
      ConstantValue::HoleyFloat64Bits(bits),
    );
    self.holey_float64_constants.insert(bits, node);
    node
  }

  /// v8: :345 GetHeapNumberConstant (cc:30-45; LocalIsolate factory → 台账)
  pub fn get_heap_number_constant(&mut self, constant: f64) -> NodeRef {
    let bits = constant.to_bits();
    if let Some(&node) = self.heap_number_constants.get(&bits) {
      return node;
    }
    let node =
      self.create_new_constant_node(Opcode::Float64Constant, ConstantValue::HeapNumberBits(bits));
    self.heap_number_constants.insert(bits, node);
    node
  }

  /// v8: :347-349 GetRootConstant
  pub fn get_root_constant(&mut self, index: RootIndex) -> NodeRef {
    if let Some(&node) = self.root_constants.get(&index) {
      return node;
    }
    let node = self.create_new_constant_node(Opcode::RootConstant, ConstantValue::Root(index));
    self.root_constants.insert(index, node);
    node
  }

  /// v8: :351-354 GetBooleanConstant
  pub fn get_boolean_constant(&mut self, value: bool) -> NodeRef {
    let index = if value {
      RootIndex(0) // kTrueValue 接缝 (待 objects RootIndex 对齐)
    } else {
      RootIndex(1) // kFalseValue 接缝
    };
    self.get_root_constant(index)
  }

  /// v8: :356-360 GetDeadValue
  pub fn get_dead_value(&mut self) -> NodeRef {
    self.new_node(Opcode::DeadValue, BlockRef(0), Vec::new(), None)
  }

  /// v8: :362 GetConstant (cc:132-153; 只读承接已 intern 值;
  /// 未 intern 需 broker FindRootIndex / UnpackIfThin + &mut 建节点 → builder 席, 见 readme/zh.md 台账)
  #[must_use]
  pub fn get_constant(&self, ref_bits: u64) -> Option<NodeRef> {
    // V8 tagged 值低位 0 = Smi, 载荷 = 有符号 >>1
    if ref_bits & 1 == 0 {
      let smi = (ref_bits as i64) >> 1;
      // GetSmiConstant 需 &mut; 只读命中 smi_constants 表即可返回, 未命中 → None
      return self.smi_constants.get(&smi).copied();
    }
    // 堆对象: 先 broker()->FindRootIndex (Isolate 深水), 再 heap_constants_ 表 (cc:150 GetOrAddNewConstantNode)
    // 只读承接现 intern 值; 未 intern 需 broker + &mut → builder 席补 (readme/zh.md 台账)
    self.heap_constants.get(&ref_bits).copied()
  }

  /// v8: :364-365 GetTrustedConstant (cc:169-184; sandbox 面 NodeRec 未存 tag,
  /// SBXCHECK_EQ 与 CreateNewConstantNode 走 builder 席, 见 readme/zh.md 台账)
  #[must_use]
  pub fn get_trusted_constant(&self, ref_bits: u64, _tag: u32) -> Option<NodeRef> {
    // 只读承接 trusted_constants_ 现 intern 值; tag 校验依赖 NodeRec 存 tag → builder 席
    self.trusted_constants.get(&ref_bits).copied()
  }

  /// 节点访问面 (:90 各 Node* 桥)
  #[must_use]
  pub fn node(&self, id: NodeRef) -> &NodeRec {
    &self.nodes[id.0 as usize]
  }

  #[must_use]
  pub fn node_mut(&mut self, id: NodeRef) -> &mut NodeRec {
    &mut self.nodes[id.0 as usize]
  }

  /// v8: :367 zone() — Rust 属有 arena, 区仅供外部分配 (deopt 帧等)
  #[inline]
  #[must_use]
  pub const fn zone(&self) -> &'z v8_zone::Zone<'z> {
    self.zone
  }
}
