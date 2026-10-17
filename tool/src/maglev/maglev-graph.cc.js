#!/usr/bin/env bun
// tool/src/maglev/maglev-graph.cc.js — 从 v8/src/maglev/maglev-graph.{h,cc} 生成 v8-maglev-frontend/src/graph.rs 基线
// 规则代码: Graph 宿主容器 (blocks/nodes arena, 常量 intern 表, 计数器/旗标, stack slots) + 破环 seam 声明。
// 深水 (compilation-info/broker/Isolate 面) 只出签名, 语义体 Developer 填/台账登记。
import { banner, readV8, writeSrc } from "../../lib.js";

const SRC_H = "src/maglev/maglev-graph.h";
const SRC_CC = "src/maglev/maglev-graph.cc";
const GEN = "tool/src/maglev/maglev-graph.cc.js";

function die(msg) {
  console.error(`[maglev-graph.cc.js] 漂移: ${msg}`);
  process.exit(1);
}

const h = readV8(SRC_H);
const cc = readV8(SRC_CC);

const anchorsH = [
  /class Graph final : public ZoneObject/,
  /tagged_stack_slots_ = kMaxUInt32/,
  /ZoneMap<RootIndex, RootConstant\*> root_constants_;/,
  /ZoneVector<InitialValue\*>& osr_values\(\)/,
  /uint32_t min_maglev_stackslots_for_unoptimized_frame_size/,
  /struct MaglevLoopPeelInfo \{/,
  /struct InliningTreeDebugInfo : public ZoneObject/,
  /class MaglevCallSiteInfoCompare/,
];
for (const re of anchorsH) if (!re.test(h)) die(`maglev-graph.h 锚缺失 ${re}`);
const anchorsCc = [
  /void Graph::UnwrapDeoptFrames/,
  /void Graph::RemoveUnreachableBlocks/,
  /ValueNode\* Graph::GetConstant\(compiler::ObjectRef ref\)/,
  /bool Graph::ContextMayAlias/,
];
for (const re of anchorsCc) if (!re.test(cc)) die(`maglev-graph.cc 锚缺失 ${re}`);

const L = [];
const push = (...args) => {
  if (args.length === 0) L.push("");
  else args.forEach((a) => L.push(a));
};

push("// 由 tool/src/maglev/maglev-graph.cc.js 从 v8/src/maglev/maglev-graph.{h,cc} 生成; 修订就地改, 复跑覆盖");
push("// Graph 宿主: Maglev 图数据面 (blocks/nodes arena + 常量 intern 表 + 计数器/旗标/stack slots)。");
push("// 破环: 原版 maglev-basic-block.h/maglev-ir.h/maglev-compilation-info.h 宿主类不在本席 (v8-maglev-ir 只出数据面),");
push("// 此处按前端所需最小定义 NodeRef/BlockRef 索引化 arena (原版 zone 指针 → Vec 索引, 单线程等价); 编译单元/句柄面以 *Ref 新型接缝。");
push();
push("use gxhash::{HashMap, HashSet};");
push("use v8_maglev_ir::Opcode;");
push();
push("/// v8: src/maglev/maglev-graph.h:90 块位 (blocks_ 列序位; 原版 BasicBlock* 指针的 Rust 承接)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]");
push("pub struct BlockRef(pub u32);");
push();
push("/// v8: src/maglev/maglev-graph.h:90 节点位 (原版 Node* 指针的 arena 承接)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]");
push("pub struct NodeRef(pub u32);");
push();
push("/// v8: src/maglev/maglev-ir.h Control::kNoInput 输入空位哨");
push("pub const NO_INPUT: NodeRef = NodeRef(u32::MAX);");
push();
push("/// v8: src/maglev/maglev-graph.h:387 kMaxUInt32 未定哨 (stack slots)");
push("pub const K_MAX_UINT32: u32 = u32::MAX;");
push();
push("/// 编译单元接缝 (原版 maglev-compilation-unit.h 不在本席)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
push("pub struct CompilationUnitRef(pub u32);");
push();
push("/// DeoptFrame 接缝 (原版 maglev-ir.h DeoptFrame; 图仅记顶帧集合)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
push("pub struct DeoptFrameRef(pub u32);");
push();
push("/// InlinedAllocation 接缝 (原版 turbolev-escape-analysis.h 不在本席)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
push("pub struct InlinedAllocationRef(pub u32);");
push();
push("/// MaglevCallSiteInfo 接缝 (原版 maglev-inlining.h; 候选队列元素)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
push("pub struct CallSiteRef(pub u32);");
push();
push("/// RootIndex 接缝 (原版 src/objects root-index 枚举; 取值表待与 v8-objects 席对齐)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
push("pub struct RootIndex(pub u32);");
push();
push("/// OptimizedCompilationInfo::InlinedFunctionHolder 接缝 (backend 席)")
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
push("pub struct InlinedFunctionRef(pub u32);");
push();
push("/// v8: src/maglev/maglev-graph.h:37-42 struct MaglevLoopPeelInfo");
push("#[derive(Debug, Clone, Copy)]");
push("pub struct MaglevLoopPeelInfo {");
push("  /// v8: :38 const MaglevCompilationUnit* unit");
push("  pub unit: CompilationUnitRef,");
push("  /// v8: :39 int loop_header_offset");
push("  pub loop_header_offset: i32,");
push("  /// v8: :40 int bytecode_size");
push("  pub bytecode_size: i32,");
push("  /// v8: :41 BasicBlock* header (nullptr → None)");
push("  pub header: Option<BlockRef>,");
push("}");
push();
push("/// v8: src/maglev/maglev-graph.h:45-55 struct InliningTreeDebugInfo : public ZoneObject");
push("#[derive(Debug, Clone, Default)]");
push("pub struct InliningTreeDebugInfo {");
push("  /// v8: :47 compiler::SharedFunctionInfoRef shared (broker 接缝 → id 面)");
push("  pub shared: u32,");
push("  /// v8: :48 MaglevCallerDetails* details (maglev-inlining.h, 非本席 → 接缝)");
push("  pub details: u32,");
push("  /// v8: :49 int budget");
push("  pub budget: i32,");
push("  /// v8: :50 int order");
push("  pub order: i32,");
push("  /// v8: :51 ZoneVector<InliningTreeDebugInfo*> children (arena id 承接)");
push("  pub children: Vec<u32>,");
push("}");
push();
push("/// 常量载荷 (v8: src/maglev/maglev-graph.h:181-203 各 intern 表值型; HeapObject 位为句柄接缝)");
push("#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]");
push("pub enum ConstantValue {");
push("  /// v8: maglev-ir.h SmiConstant (位宽界收 i64; Smi::IsValid 待 objects 席)");
push("  Smi(i64),");
push("  /// v8: maglev-ir.h TaggedIndexConstant");
push("  TaggedIndex(i64),");
push("  /// v8: maglev-ir.h Int32Constant");
push("  Int32(i32),");
push("  /// v8: maglev-ir.h Uint32Constant");
push("  Uint32(u32),");
push("  /// v8: maglev-ir.h IntPtrConstant");
push("  IntPtr(i64),");
push("  /// v8: maglev-ir.h Float64Constant (bits 为键, graph.h:399 注)");
push("  Float64Bits(u64),");
push("  /// v8: maglev-ir.h HoleyFloat64Constant (bits 为键)");
push("  HoleyFloat64Bits(u64),");
push("  /// v8: maglev-ir.h HeapConstant (heap number bits; 对象生成需 factory → 深水)");
push("  HeapNumberBits(u64),");
push("  /// v8: maglev-ir.h RootConstant");
push("  Root(RootIndex),");
push("  /// v8: graph.h:181 heap_constants() (compiler::HeapObjectRef 句柄接缝位)");
push("  HeapObject(u64),");
push("  /// v8: graph.h:200 trusted_constants() (IndirectPointerTag 随体待接)");
push("  TrustedObject(u64),");
push("}");
push();
push("/// 节点记录 (原版 maglev-ir.h NodeBase/ValueNode/ControlNode 宿主类不在本席, 前端最小收拢)");
push("#[derive(Debug, Clone)]");
push("pub struct NodeRec {");
push("  /// v8: maglev-ir.h NodeBase::type_ (opcode 位)");
push("  pub opcode: Opcode,");
push("  /// v8: maglev-ir.h NodeBase::block_");
push("  pub block: BlockRef,");
push("  /// v8: maglev-ir.h 输入面 (value: 序; control: 单槽或空)")
push("  pub inputs: Vec<NodeRef>,");
push("  /// 常量载荷 (仅常量类节点)")
push("  pub value: Option<ConstantValue>,");
push("  /// v8: maglev-ir.h NodeBase::id_counter_ (赋值期)")
push("  pub id_counter: u32,");
push("}");
push();
push("/// 基本块记录 (原版 maglev-basic-block.h BasicBlock 不在本席, 前端最小面)")
push("#[derive(Debug, Clone)]");
push("pub struct BasicBlockRec {");
push("  /// v8: maglev-basic-block.h Id id_ (0 = 未编号)")
push("  pub id: u32,");
push("  /// v8: maglev-basic-block.h flags_ (BLOCK_FLAG_* 位)")
push("  pub flags: u32,");
push("  /// v8: maglev-basic-block.h nodes_ (区内节点, 创建序)")
push("  pub nodes: Vec<NodeRef>,");
push("  /// v8: maglev-basic-block.h 后继表 (unreachable sweep / 迭代器依赖)")
push("  pub successors: Vec<BlockRef>,");
push("  /// v8: maglev-basic-block.h 合并态宿主位 (MergePointInterpreterFrameState)")
push("  pub state: Option<crate::frame_state::MergePointFrameRef>,");
push("  /// v8: maglev-basic-block.h 回边前驱 (is_loop/backedge_predecessor)")
push("  pub backedge_predecessor: Option<BlockRef>,");
push("}");
push();
push("impl BasicBlockRec {");
push("  /// 空块 (builder NewBlock 承接位)")
push("  #[must_use]");
push("  pub fn new() -> Self {");
push("    Self {");
push("      id: 0,");
push("      flags: 0,");
push("      nodes: Vec::new(),");
push("      successors: Vec::new(),");
push("      state: None,");
push("      backedge_predecessor: None,");
push("    }");
push("  }");
push();
push("  /// v8: maglev-basic-block.h is_dead() (旗位序号 BLOCK_FLAG_IS_DEAD, 非掩码)")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn is_dead(&self) -> bool {");
push("    (self.flags >> v8_maglev_ir::BLOCK_FLAG_IS_DEAD) & 1 == 1");
push("  }");
push();
push("  /// v8: maglev-basic-block.h mark_dead()")
push("  #[inline]");
push("  pub const fn mark_dead(&mut self) {");
push("    self.flags |= 1 << v8_maglev_ir::BLOCK_FLAG_IS_DEAD;");
push("  }");
push();
push("  /// v8: maglev-basic-block.h is_deferred() (旗位序号 BLOCK_FLAG_DEFERRED)")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn is_deferred(&self) -> bool {");
push("    (self.flags >> v8_maglev_ir::BLOCK_FLAG_DEFERRED) & 1 == 1");
push("  }");
push();
push("  /// v8: maglev-basic-block.h mark_deferred()")
push("  #[inline]");
push("  pub const fn mark_deferred(&mut self) {");
push("    self.flags |= 1 << v8_maglev_ir::BLOCK_FLAG_DEFERRED;");
push("  }");
push();
push("  /// v8: maglev-basic-block.h is_start_block_of_switch_case()")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn is_start_block_of_switch_case(&self) -> bool {");
push("    (self.flags >> v8_maglev_ir::BLOCK_FLAG_IS_START_BLOCK_OF_SWITCH_CASE) & 1 == 1");
push("  }");
push();
push("  /// v8: maglev-basic-block.h set_is_start_block_of_switch_case()")
push("  #[inline]");
push("  pub const fn set_is_start_block_of_switch_case(&mut self) {");
push("    self.flags |= 1 << v8_maglev_ir::BLOCK_FLAG_IS_START_BLOCK_OF_SWITCH_CASE;");
push("  }");
push();
push("  /// v8: maglev-basic-block.h has_id()")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn has_id(&self) -> bool {");
push("    self.id != 0");
push("  }");
push();
push("  /// v8: maglev-basic-block.h is_loop()")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn is_loop(&self) -> bool {");
push("    self.backedge_predecessor.is_some()");
push("  }");
push("}");
push();
push("impl Default for BasicBlockRec {");
push("  fn default() -> Self {");
push("    Self::new()");
push("  }");
push("}");
push();
push("/// v8: src/maglev/maglev-ir.h InitialValue (osr_values 元素; 前端最小承接)")
push("#[derive(Debug, Clone, Copy)]");
push("pub struct OSRValue {");
push("  /// v8: maglev-ir.h InitialValue::stack_slot()")
push("  pub stack_slot: u32,");
push("  /// v8: maglev-ir.h InitialValue value 承接")
push("  pub value: NodeRef,");
push("}");
push();
push("/// v8: src/maglev/maglev-graph.h:57-460 class Graph final : public ZoneObject");
push("/// arena 承接: nodes/blocks 属有 Vec (原版 zone 分配 + 裸指针, 索引化等价)。");
push("pub struct Graph<'z> {");
push("  /// v8: :367 zone() 宿主 (原版经 compilation_info_; 直给区接缝)")
push("  pub zone: &'z v8_zone::Zone<'z>,");
push("  nodes: Vec<NodeRec>,");
push("  /// v8: :391 ZoneVector<BasicBlock*> blocks_")
push("  blocks: Vec<BasicBlockRec>,");
push("  /// v8: :392 ZoneVector<InitialValue*> osr_values_")
push("  osr_values: Vec<OSRValue>,");
push("  /// v8: :393-402 常量 intern 表群")
push("  root_constants: HashMap<RootIndex, NodeRef>,");
push("  smi_constants: HashMap<i64, NodeRef>,");
push("  tagged_index_constants: HashMap<i64, NodeRef>,");
push("  int32_constants: HashMap<i32, NodeRef>,");
push("  uint32_constants: HashMap<u32, NodeRef>,");
push("  intptr_constants: HashMap<i64, NodeRef>,");
push("  float64_constants: HashMap<u64, NodeRef>,");
push("  holey_float64_constants: HashMap<u64, NodeRef>,");
push("  heap_number_constants: HashMap<u64, NodeRef>,");
push("  heap_constants: HashMap<u64, NodeRef>,");
push("  trusted_constants: HashMap<u64, NodeRef>,");
push("  /// v8: :403 ZoneVector<ValueNode*> parameters_")
push("  parameters: Vec<NodeRef>,");
push("  /// v8: :404 ZoneAbslFlatHashSet<DeoptFrame*> eager_deopt_top_frames_")
push("  eager_deopt_top_frames: HashSet<DeoptFrameRef>,");
push("  /// v8: :405-406 lazy_deopt_top_frames_ (frame → (result_register, result_size))")
push("  lazy_deopt_top_frames: HashMap<DeoptFrameRef, (v8_interpreter::Register, i32)>,");
push("  /// v8: :407 MaglevCallSiteCandidates inlineable_calls_ (优先队列序 → 容器面在此)")
push("  inlineable_calls: Vec<CallSiteRef>,");
push("  /// v8: :408 MaglevPeelableLoops peelable_loops_")
push("  peelable_loops: Vec<MaglevLoopPeelInfo>,");
push("  /// v8: :409-411 escape/elide 图")
push("  allocations_escape_map: HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>>,");
push("  allocations_elide_map: HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>>,");
push("  /// v8: :411 RegList register_inputs_ (位掩码)")
push("  register_inputs: u64,");
push("  /// v8: :415-416 inlined_functions_")
push("  inlined_functions: Vec<InlinedFunctionRef>,");
push("  /// v8: :417 InliningTreeDebugInfo* inlining_tree_debug_info_")
push("  inlining_tree_debug_info: Option<Box<InliningTreeDebugInfo>>,");
push("  /// v8: :387-390 stack slots 四元组")
push("  tagged_stack_slots: u32,");
push("  untagged_stack_slots: u32,");
push("  max_call_stack_args: u32,");
push("  max_deopted_stack_size: u32,");
push("  /// v8: :419-427 旗标/计数器群")
push("  has_recursive_calls: bool,");
push("  total_inlined_bytecode_size: i32,");
push("  total_inlined_bytecode_size_small: i32,");
push("  total_peeled_bytecode_size: i32,");
push("  total_nodes: i32,");
push("  object_ids: u32,");
push("  has_resumable_generator: bool,");
push("  may_have_unreachable_blocks: bool,");
push("  may_have_truncation: bool,");
push("  /// v8: :428 BasicBlock::Id max_block_id_")
push("  max_block_id: u32,");
push("  /// v8: :429 std::unique_ptr<MaglevGraphLabeller> graph_labeller_")
push("  graph_labeller: Option<Box<crate::labeller::MaglevGraphLabeller>>,");
push("}");
push();
push("impl<'z> Graph<'z> {");
push("  /// v8: :59-64 Graph::New(info)/Graph(info) (compilation_info 接缝: zone 直给)")
push("  #[must_use]");
push("  pub fn new(zone: &'z v8_zone::Zone<'z>) -> Self {");
push("    Self {");
push("      zone,");
push("      nodes: Vec::new(),");
push("      blocks: Vec::new(),");
push("      osr_values: Vec::new(),");
push("      root_constants: HashMap::default(),");
push("      smi_constants: HashMap::default(),");
push("      tagged_index_constants: HashMap::default(),");
push("      int32_constants: HashMap::default(),");
push("      uint32_constants: HashMap::default(),");
push("      intptr_constants: HashMap::default(),");
push("      float64_constants: HashMap::default(),");
push("      holey_float64_constants: HashMap::default(),");
push("      heap_number_constants: HashMap::default(),");
push("      heap_constants: HashMap::default(),");
push("      trusted_constants: HashMap::default(),");
push("      parameters: Vec::new(),");
push("      eager_deopt_top_frames: HashSet::default(),");
push("      lazy_deopt_top_frames: HashMap::default(),");
push("      inlineable_calls: Vec::new(),");
push("      peelable_loops: Vec::new(),");
push("      allocations_escape_map: HashMap::default(),");
push("      allocations_elide_map: HashMap::default(),");
push("      register_inputs: 0,");
push("      inlined_functions: Vec::new(),");
push("      inlining_tree_debug_info: None,");
push("      tagged_stack_slots: K_MAX_UINT32,");
push("      untagged_stack_slots: K_MAX_UINT32,");
push("      max_call_stack_args: K_MAX_UINT32,");
push("      max_deopted_stack_size: K_MAX_UINT32,");
push("      has_recursive_calls: false,");
push("      total_inlined_bytecode_size: 0,");
push("      total_inlined_bytecode_size_small: 0,");
push("      total_peeled_bytecode_size: 0,");
push("      total_nodes: 0,");
push("      object_ids: 0,");
push("      has_resumable_generator: false,");
push("      may_have_unreachable_blocks: false,");
push("      may_have_truncation: false,");
push("      max_block_id: 0,");
push("      graph_labeller: None,");
push("    }");
push("  }");
push();
push("  /// v8: :90-91 operator[] (位置查块; BlockRef 即列序位)")
push("  #[must_use]");
push("  pub fn block(&self, i: usize) -> &BasicBlockRec {");
push("    &self.blocks[i]");
push("  }");
push();
push("  #[must_use]");
push("  pub fn block_mut(&mut self, i: usize) -> &mut BasicBlockRec {");
push("    &mut self.blocks[i]");
push("  }");
push();
push("  /// v8: :93 num_blocks()")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn num_blocks(&self) -> usize {");
push("    self.blocks.len()");
push("  }");
push();
push("  /// v8: :94 blocks()")
push("  #[must_use]");
push("  pub fn blocks(&self) -> &[BasicBlockRec] {");
push("    &self.blocks");
push("  }");
push();
push("  /// v8: :96-105 block_offset (BlockRef=列序位直解; 原版线性查 TODO 注随行)")
push("  #[must_use]");
push("  pub const fn block_offset(&self, target: BlockRef) -> usize {");
push("    target.0 as usize");
push("  }");
push();
push("  /// v8: :107-110 begin/end/rbegin/rend 桥 (切片直给)")
push("  #[must_use]");
push("  pub fn iter(&self) -> impl Iterator<Item = &BasicBlockRec> {");
push("    self.blocks.iter()");
push("  }");
push();
push("  #[must_use]");
push("  pub fn iter_rev(&self) -> impl Iterator<Item = &BasicBlockRec> {");
push("    self.blocks.iter().rev()");
push("  }");
push();
push("  /// v8: :112 last_block()")
push("  #[must_use]");
push("  pub fn last_block(&self) -> Option<&BasicBlockRec> {");
push("    self.blocks.last()");
push("  }");
push();
push("  /// v8: :114-123 Add (未编号入图即编号; 内联重复入图注随行)")
push("  pub fn add(&mut self, mut block: BasicBlockRec) -> BlockRef {");
push("    if !block.has_id() {");
push("      block.id = self.alloc_block_id();");
push("    } else {");
push("      // 原版 DCHECK(v8_flags.maglev_non_eager_inlining …): 旗标面待 flags 席")
push("    }");
push("    self.blocks.push(block);");
push("    BlockRef(self.blocks.len() as u32 - 1)");
push("  }");
push();
push("  /// max_block_id_ 自增分配 (Add/AddBlocksAt 共用)")
push("  fn alloc_block_id(&mut self) -> u32 {");
push("    let id = self.max_block_id;");
push("    self.max_block_id += 1;");
push("    id");
push("  }");
push();
push("  /// v8: :125-129 AddBlocksAt (index 后插列, 全量重编号)")
push("  pub fn add_blocks_at(&mut self, mut new_blocks: Vec<BasicBlockRec>, index: usize) {");
push("    for bb in new_blocks.iter_mut() {");
push("      bb.id = self.alloc_block_id();");
push("    }");
push("    let at = (index + 1).min(self.blocks.len());");
push("    for (i, nb) in new_blocks.into_iter().enumerate() {");
push("      self.blocks.insert(at + i, nb);");
push("    }");
push("  }");
push();
push("  /// v8: :130 set_blocks")
push("  pub fn set_blocks(&mut self, blocks: Vec<BasicBlockRec>) {");
push("    self.blocks = blocks;");
push("  }");
push();
push("  /// v8: :132 RemoveUnreachableBlocks (cc:75-130)")
push("  pub fn remove_unreachable_blocks(&mut self) {");
push("    // 深水: ForEachNodeAndControl/GetLiveCatchBlock/ForEachSuccessor/RemovePredecessorAt (IR 席宿主) — 台账登记")
push("    todo!();");
push("  }");
push();
push("  /// v8: :458-459 IterateGraphAndSweepDeadBlocks (cc:49-67, 模板 → 闭包参)")
push("  pub fn iterate_graph_and_sweep_dead_blocks<F>(&mut self, mut is_dead: F)");
push("  where");
push("    F: FnMut(&BasicBlockRec) -> bool,");
push("  {");
push("    let mut dead_seen = 0usize;");
push("    let mut current = 0usize;");
push("    while current < self.blocks.len() {");
push("      if is_dead(&self.blocks[current]) {");
push("        self.blocks[current].mark_dead();");
push("      } else {");
push("        if dead_seen != current {");
push("          // Move current to last non dead position.")
push("          let bb = self.blocks[current].clone();");
push("          self.blocks[dead_seen] = bb;");
push("        }");
push("        dead_seen += 1;");
push("      }");
push("      current += 1;");
push("    }");
push("    if current != dead_seen {");
push("      self.blocks.truncate(dead_seen);");
push("    }");
push("  }");
push();
push("  /// v8: :134-157 stack slots 四元组 get/set (单写: kMaxUInt32 → 定值)")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn tagged_stack_slots(&self) -> u32 {");
push("    self.tagged_stack_slots");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn untagged_stack_slots(&self) -> u32 {");
push("    self.untagged_stack_slots");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn max_call_stack_args(&self) -> u32 {");
push("    self.max_call_stack_args");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn max_deopted_stack_size(&self) -> u32 {");
push("    self.max_deopted_stack_size");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_tagged_stack_slots(&mut self, stack_slots: u32) {");
push("    debug_assert_eq!(self.tagged_stack_slots, K_MAX_UINT32);");
push("    debug_assert_ne!(stack_slots, K_MAX_UINT32);");
push("    self.tagged_stack_slots = stack_slots;");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_untagged_stack_slots(&mut self, stack_slots: u32) {");
push("    debug_assert_eq!(self.untagged_stack_slots, K_MAX_UINT32);");
push("    debug_assert_ne!(stack_slots, K_MAX_UINT32);");
push("    self.untagged_stack_slots = stack_slots;");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_max_call_stack_args(&mut self, stack_slots: u32) {");
push("    debug_assert_eq!(self.max_call_stack_args, K_MAX_UINT32);");
push("    debug_assert_ne!(stack_slots, K_MAX_UINT32);");
push("    self.max_call_stack_args = stack_slots;");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_max_deopted_stack_size(&mut self, size: u32) {");
push("    debug_assert_eq!(self.max_deopted_stack_size, K_MAX_UINT32);");
push("    debug_assert_ne!(size, K_MAX_UINT32);");
push("    self.max_deopted_stack_size = size;");
push("  }");
push();
push("  /// v8: :159-179 内联/剥循环/节点 计数器群")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn total_inlined_bytecode_size(&self) -> i32 {");
push("    self.total_inlined_bytecode_size");
push("  }");
push();
push("  #[inline]");
push("  pub fn add_inlined_bytecode_size(&mut self, size: i32) {");
push("    self.total_inlined_bytecode_size += size;");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn total_inlined_bytecode_size_small(&self) -> i32 {");
push("    self.total_inlined_bytecode_size_small");
push("  }");
push();
push("  #[inline]");
push("  pub fn add_inlined_bytecode_size_small(&mut self, size: i32) {");
push("    self.total_inlined_bytecode_size_small += size;");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn total_peeled_bytecode_size(&self) -> i32 {");
push("    self.total_peeled_bytecode_size");
push("  }");
push();
push("  #[inline]");
push("  pub fn add_peeled_bytecode_size(&mut self, size: i32) {");
push("    self.total_peeled_bytecode_size += size;");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn total_nodes(&self) -> i32 {");
push("    self.total_nodes");
push("  }");
push();
push("  #[inline]");
push("  pub fn increment_total_nodes(&mut self) {");
push("    self.total_nodes += 1;");
push("  }");
push();
push("  /// v8: :205-210 osr_values/parameters/inlineable_calls/peelable_loops 容器面")
push("  #[must_use]");
push("  pub fn osr_values(&self) -> &[OSRValue] {");
push("    &self.osr_values");
push("  }");
push();
push("  #[must_use]");
push("  pub fn osr_values_mut(&mut self) -> &mut Vec<OSRValue> {");
push("    &mut self.osr_values");
push("  }");
push();
push("  #[must_use]");
push("  pub fn parameters(&self) -> &[NodeRef] {");
push("    &self.parameters");
push("  }");
push();
push("  #[must_use]");
push("  pub fn parameters_mut(&mut self) -> &mut Vec<NodeRef> {");
push("    &mut self.parameters");
push("  }");
push();
push("  #[must_use]");
push("  pub fn inlineable_calls(&self) -> &[CallSiteRef] {");
push("    &self.inlineable_calls");
push("  }");
push();
push("  #[must_use]");
push("  pub fn inlineable_calls_mut(&mut self) -> &mut Vec<CallSiteRef> {");
push("    &mut self.inlineable_calls");
push("  }");
push();
push("  #[must_use]");
push("  pub fn peelable_loops(&self) -> &[MaglevLoopPeelInfo] {");
push("    &self.peelable_loops");
push("  }");
push();
push("  #[must_use]");
push("  pub fn peelable_loops_mut(&mut self) -> &mut Vec<MaglevLoopPeelInfo> {");
push("    &mut self.peelable_loops");
push("  }");
push();
push("  /// v8: :212-235 deopt 顶帧集")
push("  #[must_use]");
push("  pub fn eager_deopt_top_frames(&self) -> &HashSet<DeoptFrameRef> {");
push("    &self.eager_deopt_top_frames");
push("  }");
push();
push("  #[inline]");
push("  pub fn add_eager_top_frame(&mut self, frame: DeoptFrameRef) {");
push("    self.eager_deopt_top_frames.insert(frame);");
push("  }");
push();
push("  #[must_use]");
push("  pub fn lazy_deopt_top_frames(&self) -> &HashMap<DeoptFrameRef, (v8_interpreter::Register, i32)> {");
push("    &self.lazy_deopt_top_frames");
push("  }");
push();
push("  /// v8: :223-230 AddLazyTopFrame (已有键不覆盖)")
push("  pub fn add_lazy_top_frame(");
push("    &mut self,");
push("    frame: DeoptFrameRef,");
push("    result_location: v8_interpreter::Register,");
push("    result_size: i32,");
push("  ) {");
push("    self");
push("      .lazy_deopt_top_frames");
push("      .entry(frame)");
push("      .or_insert((result_location, result_size));");
push("  }");
push();
push("  /// v8: :232-235 ClearRecordedFrames")
push("  pub fn clear_recorded_frames(&mut self) {");
push("    self.eager_deopt_top_frames.clear();");
push("    self.lazy_deopt_top_frames.clear();");
push("  }");
push();
push("  /// v8: :237 UnwrapDeoptFrames (cc:19-28; Eager/LazyDeoptInfo::Unwrap 深水 → 台账)")
push("  pub fn unwrap_deopt_frames(&mut self) {");
push("    todo!();");
push("  }");
push();
push("  /// v8: :239-251 escape/elide 图 (SmallAllocationVector=≤2 小集 → Vec 承接)")
push("  #[must_use]");
push("  pub fn allocations_escape_map(&self) -> &HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {");
push("    &self.allocations_escape_map");
push("  }");
push();
push("  #[must_use]");
push("  pub fn allocations_escape_map_mut(");
push("    &mut self,");
push("  ) -> &mut HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {");
push("    &mut self.allocations_escape_map");
push("  }");
push();
push("  #[must_use]");
push("  pub fn allocations_elide_map(&self) -> &HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {");
push("    &self.allocations_elide_map");
push("  }");
push();
push("  #[must_use]");
push("  pub fn allocations_elide_map_mut(");
push("    &mut self,");
push("  ) -> &mut HashMap<InlinedAllocationRef, Vec<InlinedAllocationRef>> {");
push("    &mut self.allocations_elide_map");
push("  }");
push();
push("  /// v8: :253 register_inputs (RegList 位掩码)")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn register_inputs(&self) -> u64 {");
push("    self.register_inputs");
push("  }");
push();
push("  #[inline]");
push("  pub fn register_inputs_mut(&mut self) -> &mut u64 {");
push("    &mut self.register_inputs");
push("  }");
push();
push("  /// v8: :255-264 inlined_functions/inlining_tree_debug_info")
push("  #[must_use]");
push("  pub fn inlined_functions(&self) -> &[InlinedFunctionRef] {");
push("    &self.inlined_functions");
push("  }");
push();
push("  #[must_use]");
push("  pub fn inlined_functions_mut(&mut self) -> &mut Vec<InlinedFunctionRef> {");
push("    &mut self.inlined_functions");
push("  }");
push();
push("  #[must_use]");
push("  pub fn inlining_tree_debug_info(&self) -> Option<&InliningTreeDebugInfo> {");
push("    self.inlining_tree_debug_info.as_deref()");
push("  }");
push();
push("  pub fn set_inlining_tree_debug_info(&mut self, tree: Option<Box<InliningTreeDebugInfo>>) {");
push("    self.inlining_tree_debug_info = tree;");
push("  }");
push();
push("  /// v8: :265-266 has_recursive_calls")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn has_recursive_calls(&self) -> bool {");
push("    self.has_recursive_calls");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_has_recursive_calls(&mut self, value: bool) {");
push("    self.has_recursive_calls = value;");
push("  }");
push();
push("  /// v8: :268-270 is_osr (toplevel unit 深水 → 台账)")
push("  #[must_use]");
push("  pub fn is_osr(&self) -> bool {");
push("    todo!();");
push("  }");
push();
push("  /// v8: :272-278 MayNeedContextPhis (cc:155-167; debug 面, 函数 kind 查询深水 → 台账)")
push("  #[must_use]");
push("  pub fn may_need_context_phis(&self) -> bool {");
push("    todo!();");
push("  }");
push();
push("  /// v8: :280-286 min_maglev_stackslots_for_unoptimized_frame_size")
push("  #[must_use]");
push("  pub fn min_maglev_stackslots_for_unoptimized_frame_size(&self) -> u32 {");
push("    // DCHECK(is_osr()) 走 is_osr 深水; 原版: 空 → InitialValue::stack_slot(0); 尾槽 +1")
push("    match self.osr_values.last() {");
push("      None => 0,");
push("      Some(v) => v.stack_slot + 1,");
push("    }");
push("  }");
push();
push("  /// v8: :288 NewObjectId")
push("  #[inline]");
push("  pub fn new_object_id(&mut self) -> u32 {");
push("    let id = self.object_ids;");
push("    self.object_ids += 1;");
push("    id");
push("  }");
push();
push("  /// v8: :290-302 resumable/unreachable/truncation 旗标群")
push("  #[inline]");
push("  pub fn set_has_resumable_generator(&mut self) {");
push("    self.has_resumable_generator = true;");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn has_resumable_generator(&self) -> bool {");
push("    self.has_resumable_generator");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_may_have_unreachable_blocks(&mut self, value: bool) {");
push("    self.may_have_unreachable_blocks = value;");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn may_have_unreachable_blocks(&self) -> bool {");
push("    self.may_have_unreachable_blocks");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_may_have_truncation(&mut self, value: bool) {");
push("    self.may_have_truncation = value;");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn may_have_truncation(&self) -> bool {");
push("    self.may_have_truncation");
push("  }");
push();
push("  /// v8: :304-307 ContextMayAlias (cc:69-73 恒真 TODO 原样)")
push("  #[must_use]");
push("  #[allow(clippy::unused_self, unused_variables)]");
push("  pub fn context_may_alias(&self, context: NodeRef, scope_info: Option<u32>) -> bool {");
push("    // TODO(verwaest): Do something better. (原版 cc:72)")
push("    true");
push("  }");
push();
push("  /// v8: :370 max_block_id()")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn max_block_id(&self) -> u32 {");
push("    self.max_block_id");
push("  }");
push();
push("  /// v8: :372-381 tracing/labeller (info 委托; graph_labeller_ 字段本地承接)")
push("  #[must_use]");
push("  pub fn is_tracing_enabled(&self) -> bool {");
push("    todo!();");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub fn has_graph_labeller(&self) -> bool {");
push("    self.graph_labeller.is_some()");
push("  }");
push();
push("  #[inline]");
push("  #[must_use]");
push("  pub fn graph_labeller(&self) -> Option<&crate::labeller::MaglevGraphLabeller> {");
push("    self.graph_labeller.as_deref()");
push("  }");
push();
push("  #[inline]");
push("  pub fn set_graph_labeller(&mut self, labeller: Option<Box<crate::labeller::MaglevGraphLabeller>>) {");
push("    self.graph_labeller = labeller;");
push("  }");
push();
push("  /// v8: :383 compilation_info() (info 宿主不在本席 → 台账)")
push("  #[must_use]");
push("  pub fn compilation_info(&self) -> CompilationUnitRef {");
push("    todo!();");
push("  }");
push();
push("  /// 通用节点入列 (:431-445 CreateNew* 族公共底; block 由调用侧回填)")
push("  pub fn new_node(");
push("    &mut self,");
push("    opcode: Opcode,");
push("    block: BlockRef,");
push("    inputs: Vec<NodeRef>,");
push("    value: Option<ConstantValue>,");
push("  ) -> NodeRef {");
push("    self.increment_total_nodes();");
push("    let id = NodeRef(self.nodes.len() as u32);");
push("    self.nodes.push(NodeRec {");
push("      opcode,");
push("      block,");
push("      inputs,");
push("      value,");
push("      id_counter: id.0,");
push("    });");
push("    if self.has_graph_labeller() {");
push("      // 原版 graph_labeller()->RegisterNode(node) — labeller 注册需 &mut, 借用切分限制下由调用侧显式做 (见 labeller.rs)")
push("    }");
push("    id");
push("  }");
push();
push("  /// v8: :431-445 CreateNewConstantNode 入口 (常量属性 static_assert → debug_assert)")
push("  fn create_new_constant_node(&mut self, opcode: Opcode, value: ConstantValue) -> NodeRef {");
push("    debug_assert!(v8_maglev_ir::is_constant_node(opcode), \"非常量 opcode 入常量工厂\");");
push("    self.new_node(opcode, BlockRef(0), Vec::new(), Some(value))");
push("  }");
push();
push("  /// v8: :447-456 GetOrAddNewConstantNode (泛型容器参 → 各表独立入口, 查插两步)")
push("  /// v8: :309-312 GetSmiConstant (Smi::IsValid 界待 objects 席, debug 近似)")
push("  pub fn get_smi_constant(&mut self, constant: i64) -> NodeRef {");
push("    debug_assert!(constant >= i32::MIN as i64 && constant <= i32::MAX as i64, \"Smi 位宽界\");");
push("    if let Some(&node) = self.smi_constants.get(&constant) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::SmiConstant, ConstantValue::Smi(constant));");
push("    self.smi_constants.insert(constant, node);");
push("    node");
push("  }");
push();
push("  /// v8: :314-317 GetTaggedIndexConstant")
push("  pub fn get_tagged_index_constant(&mut self, constant: i64) -> NodeRef {");
push("    if let Some(&node) = self.tagged_index_constants.get(&constant) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::TaggedIndexConstant, ConstantValue::TaggedIndex(constant));");
push("    self.tagged_index_constants.insert(constant, node);");
push("    node");
push("  }");
push();
push("  /// v8: :319-321 GetInt32Constant")
push("  pub fn get_int32_constant(&mut self, constant: i32) -> NodeRef {");
push("    if let Some(&node) = self.int32_constants.get(&constant) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::Int32Constant, ConstantValue::Int32(constant));");
push("    self.int32_constants.insert(constant, node);");
push("    node");
push("  }");
push();
push("  /// v8: :323-325 GetIntPtrConstant")
push("  pub fn get_intptr_constant(&mut self, constant: i64) -> NodeRef {");
push("    if let Some(&node) = self.intptr_constants.get(&constant) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::IntPtrConstant, ConstantValue::IntPtr(constant));");
push("    self.intptr_constants.insert(constant, node);");
push("    node");
push("  }");
push();
push("  /// v8: :327-329 GetUint32Constant")
push("  pub fn get_uint32_constant(&mut self, constant: u32) -> NodeRef {");
push("    if let Some(&node) = self.uint32_constants.get(&constant) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::Uint32Constant, ConstantValue::Uint32(constant));");
push("    self.uint32_constants.insert(constant, node);");
push("    node");
push("  }");
push();
push("  /// v8: :331-338 GetFloat64Constant(double; :333 double_to_uint64 = bits)")
push("  pub fn get_float64_constant(&mut self, constant: f64) -> NodeRef {");
push("    let bits = constant.to_bits();");
push("    if let Some(&node) = self.float64_constants.get(&bits) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::Float64Constant, ConstantValue::Float64Bits(bits));");
push("    self.float64_constants.insert(bits, node);");
push("    node");
push("  }");
push();
push("  /// v8: :340-343 GetHoleyFloat64Constant")
push("  pub fn get_holey_float64_constant(&mut self, bits: u64) -> NodeRef {");
push("    if let Some(&node) = self.holey_float64_constants.get(&bits) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::HoleyFloat64Constant, ConstantValue::HoleyFloat64Bits(bits));");
push("    self.holey_float64_constants.insert(bits, node);");
push("    node");
push("  }");
push();
push("  /// v8: :345 GetHeapNumberConstant (cc:30-45; LocalIsolate factory → 台账)")
push("  pub fn get_heap_number_constant(&mut self, constant: f64) -> NodeRef {");
push("    let bits = constant.to_bits();");
push("    if let Some(&node) = self.heap_number_constants.get(&bits) {");
push("      return node;");
push("    }");
push("    // 深水: NewHeapNumberFromBits + CanonicalPersistentHandle (Isolate/broker) — 台账登记")
push("    todo!();");
push("  }");
push();
push("  /// v8: :347-349 GetRootConstant")
push("  pub fn get_root_constant(&mut self, index: RootIndex) -> NodeRef {");
push("    if let Some(&node) = self.root_constants.get(&index) {");
push("      return node;");
push("    }");
push("    let node = self.create_new_constant_node(Opcode::RootConstant, ConstantValue::Root(index));");
push("    self.root_constants.insert(index, node);");
push("    node");
push("  }");
push();
push("  /// v8: :351-354 GetBooleanConstant")
push("  pub fn get_boolean_constant(&mut self, value: bool) -> NodeRef {");
push("    let index = if value {");
push("      RootIndex(0) // kTrueValue 接缝 (待 objects RootIndex 对齐)")
push("    } else {");
push("      RootIndex(1) // kFalseValue 接缝")
push("    };");
push("    self.get_root_constant(index)");
push("  }");
push();
push("  /// v8: :356-360 GetDeadValue")
push("  pub fn get_dead_value(&mut self) -> NodeRef {");
push("    self.new_node(Opcode::DeadValue, BlockRef(0), Vec::new(), None)");
push("  }");
push();
push("  /// v8: :362 GetConstant (cc:132-153; broker FindRootIndex/UnpackIfThin 深水 → 台账)")
push("  #[must_use]");
push("  pub fn get_constant(&self, ref_bits: u64) -> Option<NodeRef> {");
push("    todo!();");
push("  }");
push();
push("  /// v8: :364-365 GetTrustedConstant (cc:169-184; sandbox 面 → 台账)")
push("  #[must_use]");
push("  pub fn get_trusted_constant(&self, ref_bits: u64, tag: u32) -> Option<NodeRef> {");
push("    todo!();");
push("  }");
push();
push("  /// 节点访问面 (:90 各 Node* 桥)")
push("  #[must_use]");
push("  pub fn node(&self, id: NodeRef) -> &NodeRec {");
push("    &self.nodes[id.0 as usize]");
push("  }");
push();
push("  #[must_use]");
push("  pub fn node_mut(&mut self, id: NodeRef) -> &mut NodeRec {");
push("    &mut self.nodes[id.0 as usize]");
push("  }");
push();
push("  /// v8: :367 zone() — Rust 属有 arena, 区仅供外部分配 (deopt 帧等)")
push("  #[inline]");
push("  #[must_use]");
push("  pub const fn zone(&self) -> &'z v8_zone::Zone<'z> {");
push("    self.zone");
push("  }");
push("}");

writeSrc("v8-maglev-frontend", "graph.rs", banner(GEN, `${SRC_H} + ${SRC_CC}`, "Graph 宿主基线") + L.join("\n") + "\n");
console.log(`OK graph.rs lines=${L.length}`);
