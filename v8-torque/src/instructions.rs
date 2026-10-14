// 由 tool/src/torque/cfg.js 从 v8/src/torque/instructions.h 生成; 修订就地改, 复跑覆盖
// 指令集全量 (25 条: 后端无关 3 + 后端相关 22)

use crate::{
  cfg::ControlFlowGraph,
  constants::FieldSynchronization,
  declarable::DeclarableId,
  error::{Error, Result},
  source_positions::SourcePosition,
  types::{BitField, TypeId, TypeVector},
  utils::{BottomOffset, Stack, StackRange, Worklist},
};

// C++ InstructionBase 虚函数 + unique_ptr 克隆 → Rust enum Instruction (判别即 kind);
// InstructionKind 标签 enum 供 kind()/mnemonic() 使用 (变体名去 Instruction 后缀);
// Block* → BlockId; Stack<const Type*> → Stack<TypeId>; 指令在块内序号 InsnRef。
/// 块索引 (对应 C++ Block*)
pub type BlockId = u32;

/// 指令稳定序号 (定义位置引用; 填体时进入块时分配)
pub type InsnRef = u32;

/// 原版 enum InstructionKind 1:1 (变体名去公共后缀 Instruction)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InstructionKind {
  Peek,
  Poke,
  DeleteRange,
  PushUninitialized,
  PushBuiltinPointer,
  LoadReference,
  StoreReference,
  LoadBitField,
  StoreBitField,
  CallCsaMacro,
  CallIntrinsic,
  NamespaceConstant,
  CallCsaMacroAndBranch,
  CallBuiltin,
  CallRuntime,
  CallBuiltinPointer,
  Branch,
  ConstexprBranch,
  Goto,
  GotoExternal,
  MakeLazyNode,
  Return,
  PrintError,
  Abort,
  UnsafeCast,
}

/// DefinitionLocation::Kind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DefinitionLocationKind {
  Invalid,
  Parameter,
  Phi,
  Instruction,
}

/// DefinitionLocation 1:1: (kind, 块/指令定位, 槽位)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefinitionLocation {
  pub kind: DefinitionLocationKind,
  pub block: Option<BlockId>,
  pub instruction: Option<InsnRef>,
  pub index: usize,
}

impl DefinitionLocation {
  /// 原版 Parameter(index)
  #[must_use]
  pub fn parameter(index: usize) -> Self {
    Self {
      kind: DefinitionLocationKind::Parameter,
      block: None,
      instruction: None,
      index,
    }
  }

  /// 原版 Phi(block, index)
  #[must_use]
  pub fn phi(block: BlockId, index: usize) -> Self {
    Self {
      kind: DefinitionLocationKind::Phi,
      block: Some(block),
      instruction: None,
      index,
    }
  }

  /// 原版 Instruction(instruction, index)
  #[must_use]
  pub fn insn(instruction: InsnRef, index: usize) -> Self {
    Self {
      kind: DefinitionLocationKind::Instruction,
      block: None,
      instruction: Some(instruction),
      index,
    }
  }

  #[must_use]
  pub fn is_valid(self) -> bool {
    self.kind != DefinitionLocationKind::Invalid
  }

  #[must_use]
  pub fn is_parameter(self) -> bool {
    self.kind == DefinitionLocationKind::Parameter
  }

  #[must_use]
  pub fn is_phi(self) -> bool {
    self.kind == DefinitionLocationKind::Phi
  }

  #[must_use]
  pub fn is_instruction(self) -> bool {
    self.kind == DefinitionLocationKind::Instruction
  }
}

/// PeekInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeekInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub slot: BottomOffset,
  pub widened_type: Option<TypeId>,
}

/// PokeInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PokeInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub slot: BottomOffset,
  pub widened_type: Option<TypeId>,
}

/// DeleteRangeInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeleteRangeInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub range: StackRange,
}

/// PushUninitializedInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushUninitializedInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub r#type: TypeId,
}

/// PushBuiltinPointerInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PushBuiltinPointerInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub external_name: String,
  pub r#type: TypeId,
}

/// LoadReferenceInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadReferenceInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub r#type: TypeId,
  pub synchronization: FieldSynchronization,
}

/// StoreReferenceInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreReferenceInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub r#type: TypeId,
}

/// LoadBitFieldInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadBitFieldInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub bit_field_struct_type: TypeId,
  pub bit_field: BitField,
}

/// StoreBitFieldInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreBitFieldInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub bit_field_struct_type: TypeId,
  pub bit_field: BitField,
  pub starts_as_zero: bool,
}

/// CallCsaMacroInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallCsaMacroInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub r#macro: DeclarableId,
  pub constexpr_arguments: Vec<String>,
  pub catch_block: Option<BlockId>,
}

/// CallIntrinsicInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallIntrinsicInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub intrinsic: DeclarableId,
  pub specialization_types: TypeVector,
  pub constexpr_arguments: Vec<String>,
}

/// NamespaceConstantInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceConstantInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub constant: DeclarableId,
}

/// CallCsaMacroAndBranchInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallCsaMacroAndBranchInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub r#macro: DeclarableId,
  pub constexpr_arguments: Vec<String>,
  pub return_continuation: Option<BlockId>,
  pub label_blocks: Vec<BlockId>,
  pub catch_block: Option<BlockId>,
}

/// CallBuiltinInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallBuiltinInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub is_tailcall: bool,
  pub builtin: DeclarableId,
  pub argc: usize,
  pub catch_block: Option<BlockId>,
}

/// CallRuntimeInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallRuntimeInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub is_tailcall: bool,
  pub runtime_function: DeclarableId,
  pub argc: usize,
  pub catch_block: Option<BlockId>,
}

/// CallBuiltinPointerInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallBuiltinPointerInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub is_tailcall: bool,
  pub r#type: TypeId,
  pub argc: usize,
}

/// BranchInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub if_true: BlockId,
  pub if_false: BlockId,
}

/// ConstexprBranchInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstexprBranchInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub condition: String,
  pub if_true: BlockId,
  pub if_false: BlockId,
}

/// GotoInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotoInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub destination: BlockId,
}

/// GotoExternalInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotoExternalInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub destination: String,
  pub variable_names: Vec<String>,
}

/// MakeLazyNodeInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MakeLazyNodeInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub r#macro: DeclarableId,
  pub result_type: TypeId,
  pub constexpr_arguments: Vec<String>,
}

/// ReturnInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub count: usize,
}

/// PrintErrorInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrintErrorInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub message: String,
}

/// AbortInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbortInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub kind: AbortKind,
  pub message: String,
}

/// UnsafeCastInstruction 1:1
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafeCastInstruction {
  /// 原版 InstructionBase::pos
  pub pos: SourcePosition,
  pub destination_type: TypeId,
}

/// AbortInstruction::Kind 1:1
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AbortKind {
  DebugBreak,
  Unreachable,
  AssertionFailure,
}

/// 原版 AbortInstruction::KindToString
#[must_use]
pub fn abort_kind_to_string(kind: AbortKind) -> &'static str {
  match kind {
    AbortKind::DebugBreak => "kDebugBreak",
    AbortKind::Unreachable => "kUnreachable",
    AbortKind::AssertionFailure => "kAssertionFailure",
  }
}

/// 全指令合拢: Block 内指令序列元素 (判别 = kind; C++ Clone/Assign 由 derive(Clone) 承担)
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Instruction {
  Peek(PeekInstruction),
  Poke(PokeInstruction),
  DeleteRange(DeleteRangeInstruction),
  PushUninitialized(PushUninitializedInstruction),
  PushBuiltinPointer(PushBuiltinPointerInstruction),
  LoadReference(LoadReferenceInstruction),
  StoreReference(StoreReferenceInstruction),
  LoadBitField(LoadBitFieldInstruction),
  StoreBitField(StoreBitFieldInstruction),
  CallCsaMacro(CallCsaMacroInstruction),
  CallIntrinsic(CallIntrinsicInstruction),
  NamespaceConstant(NamespaceConstantInstruction),
  CallCsaMacroAndBranch(CallCsaMacroAndBranchInstruction),
  CallBuiltin(CallBuiltinInstruction),
  CallRuntime(CallRuntimeInstruction),
  CallBuiltinPointer(CallBuiltinPointerInstruction),
  Branch(BranchInstruction),
  ConstexprBranch(ConstexprBranchInstruction),
  Goto(GotoInstruction),
  GotoExternal(GotoExternalInstruction),
  MakeLazyNode(MakeLazyNodeInstruction),
  Return(ReturnInstruction),
  PrintError(PrintErrorInstruction),
  Abort(AbortInstruction),
  UnsafeCast(UnsafeCastInstruction),
}

impl Instruction {
  /// 原版 kind()
  #[must_use]
  pub fn kind(&self) -> InstructionKind {
    match self {
      Self::Peek(_) => InstructionKind::Peek,
      Self::Poke(_) => InstructionKind::Poke,
      Self::DeleteRange(_) => InstructionKind::DeleteRange,
      Self::PushUninitialized(_) => InstructionKind::PushUninitialized,
      Self::PushBuiltinPointer(_) => InstructionKind::PushBuiltinPointer,
      Self::LoadReference(_) => InstructionKind::LoadReference,
      Self::StoreReference(_) => InstructionKind::StoreReference,
      Self::LoadBitField(_) => InstructionKind::LoadBitField,
      Self::StoreBitField(_) => InstructionKind::StoreBitField,
      Self::CallCsaMacro(_) => InstructionKind::CallCsaMacro,
      Self::CallIntrinsic(_) => InstructionKind::CallIntrinsic,
      Self::NamespaceConstant(_) => InstructionKind::NamespaceConstant,
      Self::CallCsaMacroAndBranch(_) => InstructionKind::CallCsaMacroAndBranch,
      Self::CallBuiltin(_) => InstructionKind::CallBuiltin,
      Self::CallRuntime(_) => InstructionKind::CallRuntime,
      Self::CallBuiltinPointer(_) => InstructionKind::CallBuiltinPointer,
      Self::Branch(_) => InstructionKind::Branch,
      Self::ConstexprBranch(_) => InstructionKind::ConstexprBranch,
      Self::Goto(_) => InstructionKind::Goto,
      Self::GotoExternal(_) => InstructionKind::GotoExternal,
      Self::MakeLazyNode(_) => InstructionKind::MakeLazyNode,
      Self::Return(_) => InstructionKind::Return,
      Self::PrintError(_) => InstructionKind::PrintError,
      Self::Abort(_) => InstructionKind::Abort,
      Self::UnsafeCast(_) => InstructionKind::UnsafeCast,
    }
  }

  /// 原版 Mnemonic()
  #[must_use]
  pub fn mnemonic(&self) -> &'static str {
    match self {
      Self::Peek(_) => "PeekInstruction",
      Self::Poke(_) => "PokeInstruction",
      Self::DeleteRange(_) => "DeleteRangeInstruction",
      Self::PushUninitialized(_) => "PushUninitializedInstruction",
      Self::PushBuiltinPointer(_) => "PushBuiltinPointerInstruction",
      Self::LoadReference(_) => "LoadReferenceInstruction",
      Self::StoreReference(_) => "StoreReferenceInstruction",
      Self::LoadBitField(_) => "LoadBitFieldInstruction",
      Self::StoreBitField(_) => "StoreBitFieldInstruction",
      Self::CallCsaMacro(_) => "CallCsaMacroInstruction",
      Self::CallIntrinsic(_) => "CallIntrinsicInstruction",
      Self::NamespaceConstant(_) => "NamespaceConstantInstruction",
      Self::CallCsaMacroAndBranch(_) => "CallCsaMacroAndBranchInstruction",
      Self::CallBuiltin(_) => "CallBuiltinInstruction",
      Self::CallRuntime(_) => "CallRuntimeInstruction",
      Self::CallBuiltinPointer(_) => "CallBuiltinPointerInstruction",
      Self::Branch(_) => "BranchInstruction",
      Self::ConstexprBranch(_) => "ConstexprBranchInstruction",
      Self::Goto(_) => "GotoInstruction",
      Self::GotoExternal(_) => "GotoExternalInstruction",
      Self::MakeLazyNode(_) => "MakeLazyNodeInstruction",
      Self::Return(_) => "ReturnInstruction",
      Self::PrintError(_) => "PrintErrorInstruction",
      Self::Abort(_) => "AbortInstruction",
      Self::UnsafeCast(_) => "UnsafeCastInstruction",
    }
  }

  /// 原版 IsBlockTerminator (逐指令常量语义)
  #[must_use]
  pub fn is_block_terminator(&self) -> bool {
    match self {
      Self::CallCsaMacroAndBranch(_) => true,
      Self::Branch(_)
      | Self::ConstexprBranch(_)
      | Self::Goto(_)
      | Self::GotoExternal(_)
      | Self::Return(_) => true,
      Self::CallBuiltin(i) => i.is_tailcall,
      Self::CallBuiltinPointer(i) => i.is_tailcall,
      Self::CallRuntime(i) => i.is_tailcall,
      Self::Abort(i) => i.kind != AbortKind::DebugBreak,
      _ => false,
    }
  }

  /// 原版 AppendSuccessorBlocks
  pub fn append_successor_blocks(&self, block_list: &mut Vec<BlockId>) {
    match self {
      Self::CallCsaMacro(i) => {
        if let Some(b) = i.catch_block {
          block_list.push(b);
        }
      }
      Self::CallCsaMacroAndBranch(i) => {
        if let Some(b) = i.catch_block {
          block_list.push(b);
        }
        if let Some(b) = i.return_continuation {
          block_list.push(b);
        }
        block_list.extend(i.label_blocks.iter().copied());
      }
      Self::CallBuiltin(i) => {
        if let Some(b) = i.catch_block {
          block_list.push(b);
        }
      }
      Self::CallRuntime(i) => {
        if let Some(b) = i.catch_block {
          block_list.push(b);
        }
      }
      Self::Branch(i) => {
        block_list.push(i.if_true);
        block_list.push(i.if_false);
      }
      Self::ConstexprBranch(i) => {
        block_list.push(i.if_true);
        block_list.push(i.if_false);
      }
      Self::Goto(i) => block_list.push(i.destination),
      _ => {}
    }
  }

  /// 原版 TypeInstruction (栈类型化, 语义填体)
  pub fn type_instruction(&self, stack: &mut Stack<TypeId>, cfg: &ControlFlowGraph) -> Result<()> {
    // TODO(填体): 25 条指令逐一类型化
    let _ = (stack, cfg);
    Err(Error::Unimplemented)
  }

  /// 原版 RecomputeDefinitionLocations (定义位置重算, 语义填体)
  pub fn recompute_definition_locations(
    &self,
    locations: &mut Stack<DefinitionLocation>,
    worklist: &mut Worklist<BlockId>,
  ) -> Result<()> {
    // TODO(填体)
    let _ = (locations, worklist);
    Err(Error::Unimplemented)
  }

  /// 原版 InvalidateTransientTypes (自由函数于 InstructionBase)
  pub fn invalidate_transient_types(&self, stack: &mut Stack<TypeId>) -> Result<()> {
    // TODO(填体)
    let _ = stack;
    Err(Error::Unimplemented)
  }
}

// 原版逐指令 GetValueDefinition/GetExceptionObjectDefinition (定义位置查询, 填体):
impl PushUninitializedInstruction {
  /// 原版 PushUninitializedInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl PushBuiltinPointerInstruction {
  /// 原版 PushBuiltinPointerInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl LoadReferenceInstruction {
  /// 原版 LoadReferenceInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl LoadBitFieldInstruction {
  /// 原版 LoadBitFieldInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl StoreBitFieldInstruction {
  /// 原版 StoreBitFieldInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl MakeLazyNodeInstruction {
  /// 原版 MakeLazyNodeInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl UnsafeCastInstruction {
  /// 原版 UnsafeCastInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallIntrinsicInstruction {
  /// 原版 CallIntrinsicInstruction::get_value_definition_count
  pub fn get_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallIntrinsicInstruction {
  /// 原版 CallIntrinsicInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl NamespaceConstantInstruction {
  /// 原版 NamespaceConstantInstruction::get_value_definition_count
  pub fn get_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl NamespaceConstantInstruction {
  /// 原版 NamespaceConstantInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroInstruction {
  /// 原版 CallCsaMacroInstruction::get_value_definition_count
  pub fn get_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroInstruction {
  /// 原版 CallCsaMacroInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroInstruction {
  /// 原版 CallCsaMacroInstruction::get_exception_object_definition
  pub fn get_exception_object_definition(&self) -> Result<Option<DefinitionLocation>> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroAndBranchInstruction {
  /// 原版 CallCsaMacroAndBranchInstruction::get_label_count
  pub fn get_label_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroAndBranchInstruction {
  /// 原版 CallCsaMacroAndBranchInstruction::get_label_value_definition_count
  pub fn get_label_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroAndBranchInstruction {
  /// 原版 CallCsaMacroAndBranchInstruction::get_label_value_definition
  pub fn get_label_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroAndBranchInstruction {
  /// 原版 CallCsaMacroAndBranchInstruction::get_value_definition_count
  pub fn get_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroAndBranchInstruction {
  /// 原版 CallCsaMacroAndBranchInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallCsaMacroAndBranchInstruction {
  /// 原版 CallCsaMacroAndBranchInstruction::get_exception_object_definition
  pub fn get_exception_object_definition(&self) -> Result<Option<DefinitionLocation>> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallBuiltinInstruction {
  /// 原版 CallBuiltinInstruction::get_value_definition_count
  pub fn get_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallBuiltinInstruction {
  /// 原版 CallBuiltinInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallBuiltinInstruction {
  /// 原版 CallBuiltinInstruction::get_exception_object_definition
  pub fn get_exception_object_definition(&self) -> Result<Option<DefinitionLocation>> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallBuiltinPointerInstruction {
  /// 原版 CallBuiltinPointerInstruction::get_value_definition_count
  pub fn get_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallBuiltinPointerInstruction {
  /// 原版 CallBuiltinPointerInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallRuntimeInstruction {
  /// 原版 CallRuntimeInstruction::get_value_definition_count
  pub fn get_value_definition_count(&self) -> Result<usize> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallRuntimeInstruction {
  /// 原版 CallRuntimeInstruction::get_value_definition
  pub fn get_value_definition(&self) -> Result<DefinitionLocation> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

impl CallRuntimeInstruction {
  /// 原版 CallRuntimeInstruction::get_exception_object_definition
  pub fn get_exception_object_definition(&self) -> Result<Option<DefinitionLocation>> {
    // TODO(填体)
    Err(Error::Unimplemented)
  }
}

// CallCsaMacroAndBranchInstruction::GetLabelValueDefinition(label, index) 原双参 →
// 填体时并入 get_label_value_definition (参数登记于此)。
// ── 原版 src/torque/instructions.cc ──
// L15: const InstructionKind Name::kKind = InstructionKind::k##Name; \ std::unique_ptr<InstructionBase> Name::Clone() const
// L19: void Name::Assign(const InstructionBase& other)
// L26: void ExpectType(const Type* expected, const Type* actual)
// L31: void ExpectSubtype(const Type* subtype, const Type* supertype)
// L38: void PeekInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L52: void PeekInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L57: void PokeInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L68: void PokeInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L73: void DeleteRangeInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L78: void DeleteRangeInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L83: void PushUninitializedInstruction::TypeInstruction( Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L88: void PushUninitializedInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L93: DefinitionLocation PushUninitializedInstruction::GetValueDefinition() const
// L97: void PushBuiltinPointerInstruction::TypeInstruction( Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L102: void PushBuiltinPointerInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L107: DefinitionLocation PushBuiltinPointerInstruction::GetValueDefinition() const
// L111: void NamespaceConstantInstruction::TypeInstruction( Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L116: void NamespaceConstantInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L123: std::size_t NamespaceConstantInstruction::GetValueDefinitionCount() const
// L127: DefinitionLocation NamespaceConstantInstruction::GetValueDefinition( std::size_t index) const
// L133: std::ostream& operator<<(std::ostream& os, const NamespaceConstantInstruction& instruction)
// L138: void InstructionBase::InvalidateTransientTypes( Stack<const Type*>* stack) const
// L153: void CallIntrinsicInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L172: void CallIntrinsicInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L182: std::size_t CallIntrinsicInstruction::GetValueDefinitionCount() const
// L186: DefinitionLocation CallIntrinsicInstruction::GetValueDefinition( std::size_t index) const
// L192: std::ostream& operator<<(std::ostream& os, const CallIntrinsicInstruction& instruction)
// L208: void CallCsaMacroInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L235: void CallCsaMacroInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L252: std::optional<DefinitionLocation>CallCsaMacroInstruction::GetExceptionObjectDefinition() const
// L258: std::size_t CallCsaMacroInstruction::GetValueDefinitionCount() const
// L262: DefinitionLocation CallCsaMacroInstruction::GetValueDefinition( std::size_t index) const
// L268: std::ostream& operator<<(std::ostream& os, const CallCsaMacroInstruction& instruction)
// L280: void CallCsaMacroAndBranchInstruction::TypeInstruction( Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L328: void CallCsaMacroAndBranchInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L362: std::size_t CallCsaMacroAndBranchInstruction::GetLabelCount() const
// L366: std::size_t CallCsaMacroAndBranchInstruction::GetLabelValueDefinitionCount( std::size_t label) const
// L372: DefinitionLocation CallCsaMacroAndBranchInstruction::GetLabelValueDefinition( std::size_t label, std::size_t index) const
// L382: std::size_t CallCsaMacroAndBranchInstruction::GetValueDefinitionCount() const
// L388: DefinitionLocation CallCsaMacroAndBranchInstruction::GetValueDefinition( std::size_t index) const
// L394: std::optional<DefinitionLocation>CallCsaMacroAndBranchInstruction::GetExceptionObjectDefinition() const
// L400: std::ostream& operator<<(std::ostream& os, const CallCsaMacroAndBranchInstruction& instruction)
// L420: void CallBuiltinInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L440: void CallBuiltinInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L455: std::size_t CallBuiltinInstruction::GetValueDefinitionCount() const
// L459: DefinitionLocation CallBuiltinInstruction::GetValueDefinition( std::size_t index) const
// L465: std::optional<DefinitionLocation>CallBuiltinInstruction::GetExceptionObjectDefinition() const
// L471: void CallBuiltinPointerInstruction::TypeInstruction( Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L486: void CallBuiltinPointerInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L494: std::size_t CallBuiltinPointerInstruction::GetValueDefinitionCount() const
// L498: DefinitionLocation CallBuiltinPointerInstruction::GetValueDefinition( std::size_t index) const
// L504: std::ostream& operator<<(std::ostream& os, const CallBuiltinInstruction& instruction)
// L517: void CallRuntimeInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L541: void CallRuntimeInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L559: std::size_t CallRuntimeInstruction::GetValueDefinitionCount() const
// L565: DefinitionLocation CallRuntimeInstruction::GetValueDefinition( std::size_t index) const
// L571: std::optional<DefinitionLocation>CallRuntimeInstruction::GetExceptionObjectDefinition() const
// L577: std::ostream& operator<<(std::ostream& os, const CallRuntimeInstruction& instruction)
// L590: void BranchInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L600: void BranchInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L607: std::ostream& operator<<(std::ostream& os, const BranchInstruction& instruction)
// L613: void ConstexprBranchInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L619: void ConstexprBranchInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L625: std::ostream& operator<<(std::ostream& os, const ConstexprBranchInstruction& instruction)
// L632: void GotoInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L637: void GotoInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L642: std::ostream& operator<<(std::ostream& os, const GotoInstruction& instruction)
// L646: void GotoExternalInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L653: void GotoExternalInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L656: void ReturnInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L661: void ReturnInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L666: void PrintErrorInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L669: void PrintErrorInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L672: void AbortInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L675: void AbortInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L678: void UnsafeCastInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L683: void UnsafeCastInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L688: DefinitionLocation UnsafeCastInstruction::GetValueDefinition() const
// L692: void LoadReferenceInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L702: void LoadReferenceInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L709: DefinitionLocation LoadReferenceInstruction::GetValueDefinition() const
// L713: void StoreReferenceInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L722: void StoreReferenceInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L729: void LoadBitFieldInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L735: void LoadBitFieldInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L741: DefinitionLocation LoadBitFieldInstruction::GetValueDefinition() const
// L745: void StoreBitFieldInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L752: void StoreBitFieldInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L759: DefinitionLocation StoreBitFieldInstruction::GetValueDefinition() const
// L763: void MakeLazyNodeInstruction::TypeInstruction(Stack<const Type*>* stack, ControlFlowGraph* cfg) const
// L780: void MakeLazyNodeInstruction::RecomputeDefinitionLocations( Stack<DefinitionLocation>* locations, Worklist<Block*>* worklist) const
// L789: DefinitionLocation MakeLazyNodeInstruction::GetValueDefinition() const
// L793: std::ostream& operator<<(std::ostream& os, const MakeLazyNodeInstruction& instruction)
// L803: bool CallRuntimeInstruction::IsBlockTerminator() const
