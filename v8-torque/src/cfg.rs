// 由 tool/src/torque/cfg.js 从 v8/src/torque/cfg.h 生成; 修订就地改, 复跑覆盖
// CFG: Block/ControlFlowGraph/CfgAssembler

use crate::{
  error::{Error, Result},
  instructions::{BlockId, DefinitionLocation, Instruction},
  types::{TypeId, TypeVector},
  utils::{BottomOffset, Stack, StackRange, Worklist},
};

// C++ Block* 互联 → BlockId (arena 存于 ControlFlowGraph.blocks, 原版 std::list
// 稳定性由 Vec + 下标承担); cfg_ 回指针删除 (登记偏差, 由持有方补齐)。
/// 原版 Block 1:1 (cfg_ 回指针删除)
#[derive(Debug, Clone)]
pub struct Block {
  id: BlockId,
  instructions: Vec<Instruction>,
  input_types: Option<Stack<TypeId>>,
  input_definitions: Option<Stack<DefinitionLocation>>,
  is_deferred: bool,
}

impl Block {
  /// 原版构造
  #[must_use]
  pub fn new(id: BlockId, input_types: Option<Stack<TypeId>>, is_deferred: bool) -> Self {
    Self {
      id,
      instructions: Vec::new(),
      input_types,
      input_definitions: None,
      is_deferred,
    }
  }

  /// 原版 Add
  pub fn add(&mut self, instruction: Instruction) {
    debug_assert!(!self.is_complete());
    self.instructions.push(instruction);
  }

  #[must_use]
  pub fn has_input_types(&self) -> bool {
    self.input_types.is_some()
  }

  /// 原版 InputTypes (前置条件: HasInputTypes())
  #[must_use]
  pub fn input_types(&self) -> Option<&Stack<TypeId>> {
    self.input_types.as_ref()
  }

  /// 原版 SetInputTypes (与已登记类型不匹配则报错)
  pub fn set_input_types(&mut self, input_types: &Stack<TypeId>) -> Result<()> {
    // TODO(填体): 不匹配路径 ReportError
    self.input_types = Some(input_types.clone());
    Ok(())
  }

  /// 原版 Retype: 沿指令重放类型化
  pub fn retype(&mut self) -> Result<()> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  #[must_use]
  pub fn instructions(&self) -> &[Instruction] {
    &self.instructions
  }

  #[must_use]
  pub fn instructions_mut(&mut self) -> &mut Vec<Instruction> {
    &mut self.instructions
  }

  /// 原版 IsComplete: 末条指令为块终结
  #[must_use]
  pub fn is_complete(&self) -> bool {
    self
      .instructions
      .last()
      .is_some_and(Instruction::is_block_terminator)
  }

  #[must_use]
  pub const fn id(&self) -> BlockId {
    self.id
  }

  #[must_use]
  pub const fn is_deferred(&self) -> bool {
    self.is_deferred
  }

  /// 原版 MergeInputDefinitions (不一致槽位折为 Phi)
  pub fn merge_input_definitions(
    &mut self,
    input_definitions: &Stack<DefinitionLocation>,
    worklist: &mut Worklist<BlockId>,
  ) -> Result<()> {
    // TODO(填体): Phi 合并语义
    let _ = (input_definitions, worklist);
    Err(Error::Unimplemented)
  }

  #[must_use]
  pub fn has_input_definitions(&self) -> bool {
    self.input_definitions.is_some()
  }

  /// 原版 InputDefinitions (前置条件: HasInputDefinitions())
  #[must_use]
  pub fn input_definitions(&self) -> Option<&Stack<DefinitionLocation>> {
    self.input_definitions.as_ref()
  }

  /// 原版 IsDead
  #[must_use]
  pub fn is_dead(&self) -> bool {
    !self.has_input_definitions()
  }
}

/// 原版 ControlFlowGraph 1:1
#[derive(Debug)]
pub struct ControlFlowGraph {
  blocks: Vec<Block>,
  start: BlockId,
  placed_blocks: Vec<BlockId>,
  end: Option<BlockId>,
  return_type: Option<TypeVector>,
  next_block_id: usize,
}

impl ControlFlowGraph {
  /// 原版构造: 建入口块并放置
  #[must_use]
  pub fn new(input_types: Stack<TypeId>) -> Self {
    let mut g = Self {
      blocks: Vec::new(),
      start: 0,
      placed_blocks: Vec::new(),
      end: None,
      return_type: None,
      next_block_id: 0,
    };
    let b = g.new_block(Some(input_types), false);
    g.place_block(b);
    g
  }

  /// 原版 NewBlock
  pub fn new_block(&mut self, input_types: Option<Stack<TypeId>>, is_deferred: bool) -> BlockId {
    let id = self.next_block_id as BlockId;
    self.next_block_id += 1;
    self.blocks.push(Block::new(id, input_types, is_deferred));
    id
  }

  /// 原版 PlaceBlock
  pub fn place_block(&mut self, block: BlockId) {
    self.placed_blocks.push(block);
  }

  /// 原版 UnplaceBlockIf
  pub fn unplace_block_if(&mut self, predicate: impl Fn(BlockId) -> bool) {
    self.placed_blocks.retain(|b| !predicate(*b));
  }

  #[must_use]
  pub const fn start(&self) -> BlockId {
    self.start
  }

  #[must_use]
  pub const fn end(&self) -> Option<BlockId> {
    self.end
  }

  pub fn set_end(&mut self, end: BlockId) {
    self.end = Some(end);
  }

  /// 原版 SetReturnType (不一致即编译错误)
  pub fn set_return_type(&mut self, t: TypeVector) -> Result<()> {
    if self.return_type.is_none() {
      self.return_type = Some(t);
      return Ok(());
    }
    // TODO(填体): 与既有返回型比对, 不一致 → Error::Message
    if self.return_type.as_ref() != Some(&t) {
      return Err(Error::Unimplemented);
    }
    Ok(())
  }

  /// 原版 blocks(): 已放置块序
  #[must_use]
  pub fn blocks(&self) -> &[BlockId] {
    &self.placed_blocks
  }

  #[must_use]
  pub fn block(&self, id: BlockId) -> &Block {
    &self.blocks[id as usize]
  }

  #[must_use]
  pub fn block_mut(&mut self, id: BlockId) -> &mut Block {
    &mut self.blocks[id as usize]
  }

  /// 原版 NumberOfBlockIds
  #[must_use]
  pub fn number_of_block_ids(&self) -> usize {
    self.next_block_id
  }

  /// 原版 ParameterCount
  #[must_use]
  pub fn parameter_count(&self) -> usize {
    self.block(self.start).input_types().map_or(0, |s| s.size())
  }
}

/// 原版 CfgAssembler 1:1
#[derive(Debug)]
pub struct CfgAssembler {
  current_stack: Stack<TypeId>,
  cfg: ControlFlowGraph,
  current_block: BlockId,
}

impl CfgAssembler {
  /// 原版构造
  #[must_use]
  pub fn new(input_types: Stack<TypeId>) -> Self {
    let current_stack = input_types.clone();
    let cfg = ControlFlowGraph::new(input_types);
    let current_block = cfg.start();
    Self {
      current_stack,
      cfg,
      current_block,
    }
  }

  /// 原版 Result() (收尾 + 优化 + 输入定义计算)
  pub fn result(&mut self) -> Result<ControlFlowGraph> {
    // TODO(填体): OptimizeCfg + ComputeInputDefinitions
    if !self.current_block_is_complete() {
      self.cfg.set_end(self.current_block);
    }
    Err(Error::Unimplemented)
  }

  /// 原版 NewBlock
  pub fn new_block(&mut self, input_types: Option<Stack<TypeId>>, is_deferred: bool) -> BlockId {
    self.cfg.new_block(input_types, is_deferred)
  }

  #[must_use]
  pub fn current_block_is_complete(&self) -> bool {
    self.cfg.block(self.current_block).is_complete()
  }

  /// 原版 CfgIsComplete
  #[must_use]
  pub fn cfg_is_complete(&self) -> bool {
    self
      .cfg
      .blocks()
      .iter()
      .all(|b| self.cfg.end() == Some(*b) || self.cfg.block(*b).is_complete())
  }

  /// 原版 Emit: 先类型化再入块
  pub fn emit(&mut self, instruction: Instruction) -> Result<()> {
    // TODO(填体): type_instruction 消费栈
    self.cfg.block_mut(self.current_block).add(instruction);
    Ok(())
  }

  #[must_use]
  pub fn current_stack(&self) -> &Stack<TypeId> {
    &self.current_stack
  }

  #[must_use]
  pub fn top_range(&self, slot_count: usize) -> StackRange {
    self.current_stack.top_range(slot_count)
  }

  /// 原版 Bind
  pub fn bind(&mut self, block: BlockId) -> Result<()> {
    // TODO(填体)
    let _ = block;
    Err(Error::Unimplemented)
  }

  /// 原版 Goto
  pub fn goto(&mut self, block: BlockId) -> Result<()> {
    // TODO(填体)
    let _ = block;
    Err(Error::Unimplemented)
  }

  /// 原版 Goto(block, preserved_slots): 保留顶部槽位版
  pub fn goto_preserved(&mut self, block: BlockId, preserved_slots: usize) -> Result<StackRange> {
    // TODO(填体)
    let _ = (block, preserved_slots);
    Err(Error::Unimplemented)
  }

  /// 原版 Branch (栈顶 bool 弹出后分支)
  pub fn branch(&mut self, if_true: BlockId, if_false: BlockId) -> Result<()> {
    // TODO(填体)
    let _ = (if_true, if_false);
    Err(Error::Unimplemented)
  }

  /// 原版 DeleteRange
  pub fn delete_range(&mut self, range: StackRange) {
    self.current_stack.delete_range(range);
  }

  /// 原版 DropTo
  pub fn drop_to(&mut self, new_level: BottomOffset) {
    let n = self.current_stack.above_top().offset - new_level.offset;
    self.current_stack.pop_many(n);
  }

  /// 原版 Peek(range, type)
  pub fn peek(&mut self, range: StackRange, type_: Option<TypeId>) -> Result<StackRange> {
    // TODO(填体)
    let _ = (range, type_);
    Err(Error::Unimplemented)
  }

  /// 原版 Poke(destination, origin, type)
  pub fn poke(
    &mut self,
    destination: StackRange,
    origin: StackRange,
    type_: Option<TypeId>,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (destination, origin, type_);
    Err(Error::Unimplemented)
  }

  /// 原版 Print
  pub fn print(&mut self, s: String) -> Result<()> {
    // TODO(填体): PrintErrorInstruction 发射
    let _ = s;
    Err(Error::Unimplemented)
  }

  /// 原版 AssertionFailure
  pub fn assertion_failure(&mut self, message: String) -> Result<()> {
    // TODO(填体)
    let _ = message;
    Err(Error::Unimplemented)
  }

  /// 原版 Unreachable
  pub fn unreachable(&mut self) -> Result<()> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  /// 原版 DebugBreak
  pub fn debug_break(&mut self) -> Result<()> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }

  /// 原版 OptimizeCfg
  pub fn optimize_cfg(&mut self) {
    // TODO(填体)
  }

  /// 原版 ComputeInputDefinitions
  pub fn compute_input_definitions(&mut self) {
    // TODO(填体)
  }
}

/// 原版 CfgAssemblerScopedTemporaryBlock — Rust 借用模型下改显式 enter/exit 对
/// (原版 RAII swap current_block/current_stack; Drop 守卫留填体设计)
#[derive(Debug)]
pub struct CfgAssemblerScopedTemporaryBlock {
  pub saved_stack: Stack<TypeId>,
  pub saved_block: BlockId,
}

impl CfgAssemblerScopedTemporaryBlock {
  /// 原版构造 (swap 进临时块)
  pub fn enter(_assembler: &mut CfgAssembler, block: BlockId) -> Result<Self> {
    // TODO(填体): swap 语义
    let _ = block;
    Err(Error::Unimplemented)
  }

  /// 原版析构 (swap 回)
  pub fn exit(self, _assembler: &mut CfgAssembler) {
    // TODO(填体)
    let _ = self;
  }
}

// ── 原版 src/torque/cfg.cc ──
// L13: void Block::SetInputTypes(const Stack<const Type*>& input_types)
// L72: void CfgAssembler::Bind(Block* block)
// L81: void CfgAssembler::Goto(Block* block)
// L88: StackRange CfgAssembler::Goto(Block* block, size_t preserved_slots)
// L99: void CfgAssembler::Branch(Block* if_true, Block* if_false)
// L104: void CfgAssembler::DeleteRange(StackRange range)
// L110: void CfgAssembler::DropTo(BottomOffset new_level)
// L114: StackRange CfgAssembler::Peek(StackRange range, std::optional<const Type*> type)
// L129: void CfgAssembler::Poke(StackRange destination, StackRange origin, std::optional<const Type*> type)
// L146: void CfgAssembler::Print(std::string s)
// L150: void CfgAssembler::AssertionFailure(std::string message)
// L155: void CfgAssembler::Unreachable()
// L159: void CfgAssembler::DebugBreak()
// L163: std::vector<std::size_t> CountBlockPredecessors(const ControlFlowGraph& cfg)
// L181: void CfgAssembler::OptimizeCfg()
// L212: void CfgAssembler::ComputeInputDefinitions()
