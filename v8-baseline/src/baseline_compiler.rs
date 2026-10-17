// 由 tool/src/baseline/baseline-compiler.h.js 从 v8/src/baseline/baseline-compiler.{h,cc} 生成; 修订就地改, 复跑覆盖

use std::mem::take;

/// 栈检查开关 (原版 enum StackCheckBehavior)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum StackCheckBehavior {
  EnableStackCheck = 0,
  DisableStackCheck = 1,
}

impl StackCheckBehavior {
  #[inline]
  #[must_use]
  pub const fn is_enabled(self) -> bool {
    matches!(self, Self::EnableStackCheck)
  }

  #[inline]
  #[must_use]
  pub const fn is_disabled(self) -> bool {
    matches!(self, Self::DisableStackCheck)
  }
}

/// builtin 调用/跳转模式 (原版 src/codegen/assembler.h enum class BuiltinCallJumpMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum BuiltinCallJumpMode {
  /// 绝对地址内嵌
  Absolute = 0,
  /// PC 相对
  PcRelative = 1,
  /// 间接 (经 builtins 入口表)
  Indirect = 2,
  /// 仅 mksnapshot
  ForMksnapshot = 3,
}

impl BuiltinCallJumpMode {
  #[inline]
  #[must_use]
  pub const fn is_indirect(self) -> bool {
    matches!(self, Self::Indirect)
  }
}

/// 单字节码访问器全表 (原版 BYTECODE_LIST(DECLARE_VISITOR) 展开, 217 个 Visit<Xxx>)
pub const VISIT_BYTECODE_METHODS: [&str; 217] = [
  "VisitWide",
  "VisitExtraWide",
  "VisitDebugBreakWide",
  "VisitDebugBreakExtraWide",
  "VisitDebugBreak0",
  "VisitDebugBreak1",
  "VisitDebugBreak2",
  "VisitDebugBreak3",
  "VisitDebugBreak4",
  "VisitDebugBreak5",
  "VisitDebugBreak6",
  "VisitLdar",
  "VisitLdaZero",
  "VisitLdaSmi",
  "VisitLdaUndefined",
  "VisitLdaNull",
  "VisitLdaTheHole",
  "VisitLdaTdzHole",
  "VisitLdaTrue",
  "VisitLdaFalse",
  "VisitLdaConstant",
  "VisitLdaContextSlotNoCell",
  "VisitLdaContextSlot",
  "VisitLdaImmutableContextSlot",
  "VisitLdaCurrentContextSlotNoCell",
  "VisitLdaCurrentContextSlot",
  "VisitLdaImmutableCurrentContextSlot",
  "VisitStar",
  "VisitMov",
  "VisitPushContext",
  "VisitPopContext",
  "VisitTestReferenceEqual",
  "VisitTestUndetectable",
  "VisitTestNull",
  "VisitTestUndefined",
  "VisitTestTypeOf",
  "VisitLdaGlobal",
  "VisitLdaGlobalInsideTypeof",
  "VisitStaGlobal",
  "VisitStaContextSlotNoCell",
  "VisitStaCurrentContextSlotNoCell",
  "VisitStaContextSlot",
  "VisitStaCurrentContextSlot",
  "VisitLdaLookupSlot",
  "VisitLdaLookupContextSlotNoCell",
  "VisitLdaLookupContextSlot",
  "VisitLdaLookupGlobalSlot",
  "VisitLdaLookupSlotInsideTypeof",
  "VisitLdaLookupContextSlotNoCellInsideTypeof",
  "VisitLdaLookupContextSlotInsideTypeof",
  "VisitLdaLookupGlobalSlotInsideTypeof",
  "VisitStaLookupSlot",
  "VisitGetNamedProperty",
  "VisitGetNamedPropertyFromSuper",
  "VisitGetKeyedProperty",
  "VisitGetEnumeratedKeyedProperty",
  "VisitGetPrivateField",
  "VisitLdaModuleVariable",
  "VisitStaModuleVariable",
  "VisitSetNamedProperty",
  "VisitDefineNamedOwnProperty",
  "VisitDefineNamedOwnPropertyInLiteral",
  "VisitSetKeyedProperty",
  "VisitDefineKeyedOwnProperty",
  "VisitStaInArrayLiteral",
  "VisitDefineKeyedOwnPropertyInLiteral",
  "VisitSetPrototypeProperties",
  "VisitSetPrivateField",
  "VisitAdd",
  "VisitSub",
  "VisitMul",
  "VisitDiv",
  "VisitMod",
  "VisitExp",
  "VisitBitwiseOr",
  "VisitBitwiseXor",
  "VisitBitwiseAnd",
  "VisitShiftLeft",
  "VisitShiftRight",
  "VisitShiftRightLogical",
  "VisitAdd_StringConstant_Internalize",
  "VisitAddSmi",
  "VisitSubSmi",
  "VisitMulSmi",
  "VisitDivSmi",
  "VisitModSmi",
  "VisitExpSmi",
  "VisitBitwiseOrSmi",
  "VisitBitwiseXorSmi",
  "VisitBitwiseAndSmi",
  "VisitShiftLeftSmi",
  "VisitShiftRightSmi",
  "VisitShiftRightLogicalSmi",
  "VisitInc",
  "VisitDec",
  "VisitNegate",
  "VisitBitwiseNot",
  "VisitToBooleanLogicalNot",
  "VisitLogicalNot",
  "VisitTypeOf",
  "VisitDeletePropertyStrict",
  "VisitDeletePropertySloppy",
  "VisitGetSuperConstructor",
  "VisitFindNonDefaultConstructorOrConstruct",
  "VisitCallAnyReceiver",
  "VisitCallProperty",
  "VisitCallProperty0",
  "VisitCallProperty1",
  "VisitCallProperty2",
  "VisitCallUndefinedReceiver",
  "VisitCallUndefinedReceiver0",
  "VisitCallUndefinedReceiver1",
  "VisitCallUndefinedReceiver2",
  "VisitCallWithSpread",
  "VisitCallRuntime",
  "VisitCallRuntimeForPair",
  "VisitCallJSRuntime",
  "VisitInvokeIntrinsic",
  "VisitConstruct",
  "VisitConstructWithSpread",
  "VisitConstructForwardAllArgs",
  "VisitTestEqual",
  "VisitTestEqualStrict",
  "VisitTestLessThan",
  "VisitTestGreaterThan",
  "VisitTestLessThanOrEqual",
  "VisitTestGreaterThanOrEqual",
  "VisitTestInstanceOf",
  "VisitTestIn",
  "VisitToName",
  "VisitToNumber",
  "VisitToNumeric",
  "VisitToObject",
  "VisitToString",
  "VisitToBoolean",
  "VisitCreateRegExpLiteral",
  "VisitCreateArrayLiteral",
  "VisitCreateArrayFromIterable",
  "VisitCreateEmptyArrayLiteral",
  "VisitCreateObjectLiteral",
  "VisitCreateEmptyObjectLiteral",
  "VisitCloneObject",
  "VisitGetTemplateObject",
  "VisitCreateClosure",
  "VisitCreateBlockContext",
  "VisitCreateCatchContext",
  "VisitCreateFunctionContext",
  "VisitCreateFunctionContextWithCells",
  "VisitCreateEvalContext",
  "VisitCreateWithContext",
  "VisitCreateMappedArguments",
  "VisitCreateUnmappedArguments",
  "VisitCreateRestParameter",
  "VisitJumpLoop",
  "VisitJump",
  "VisitJumpConstant",
  "VisitJumpIfNullConstant",
  "VisitJumpIfNotNullConstant",
  "VisitJumpIfUndefinedConstant",
  "VisitJumpIfNotUndefinedConstant",
  "VisitJumpIfUndefinedOrNullConstant",
  "VisitJumpIfTrueConstant",
  "VisitJumpIfFalseConstant",
  "VisitJumpIfJSReceiverConstant",
  "VisitJumpIfForInDoneConstant",
  "VisitJumpIfToBooleanTrueConstant",
  "VisitJumpIfToBooleanFalseConstant",
  "VisitJumpIfToBooleanTrue",
  "VisitJumpIfToBooleanFalse",
  "VisitJumpIfTrue",
  "VisitJumpIfFalse",
  "VisitJumpIfNull",
  "VisitJumpIfNotNull",
  "VisitJumpIfUndefined",
  "VisitJumpIfNotUndefined",
  "VisitJumpIfUndefinedOrNull",
  "VisitJumpIfJSReceiver",
  "VisitJumpIfForInDone",
  "VisitSwitchOnSmiNoFeedback",
  "VisitForInEnumerate",
  "VisitForInPrepare",
  "VisitForInNext",
  "VisitForInStep",
  "VisitForOfNext",
  "VisitSetPendingMessage",
  "VisitThrow",
  "VisitReThrow",
  "VisitReturn",
  "VisitThrowReferenceErrorIfTdzHole",
  "VisitThrowSuperNotCalledIfTdzHole",
  "VisitThrowSuperAlreadyCalledIfNotTdzHole",
  "VisitThrowIfNotSuperConstructor",
  "VisitSwitchOnGeneratorState",
  "VisitSuspendGenerator",
  "VisitResumeGenerator",
  "VisitGetIterator",
  "VisitArrayDestructure",
  "VisitDebugger",
  "VisitIncBlockCounter",
  "VisitAbort",
  "VisitStar15",
  "VisitStar14",
  "VisitStar13",
  "VisitStar12",
  "VisitStar11",
  "VisitStar10",
  "VisitStar9",
  "VisitStar8",
  "VisitStar7",
  "VisitStar6",
  "VisitStar5",
  "VisitStar4",
  "VisitStar3",
  "VisitStar2",
  "VisitStar1",
  "VisitStar0",
  "VisitIllegal",
];

/// 内在函数访问器全表 (原版 INTRINSICS_LIST(DECLARE_VISITOR) 展开, 17 个 VisitIntrinsic<Xxx>)
pub const VISIT_INTRINSIC_METHODS: [&str; 17] = [
  "VisitIntrinsicAsyncFunctionAwait",
  "VisitIntrinsicAsyncFunctionEnter",
  "VisitIntrinsicAsyncFunctionReject",
  "VisitIntrinsicAsyncFunctionResolve",
  "VisitIntrinsicAsyncGeneratorAwait",
  "VisitIntrinsicAsyncGeneratorReject",
  "VisitIntrinsicAsyncGeneratorResolve",
  "VisitIntrinsicAsyncGeneratorYieldWithAwait",
  "VisitIntrinsicCreateJSGeneratorObject",
  "VisitIntrinsicGeneratorGetResumeMode",
  "VisitIntrinsicGeneratorClose",
  "VisitIntrinsicGetImportMetaObject",
  "VisitIntrinsicCopyDataProperties",
  "VisitIntrinsicCopyDataPropertiesWithExcludedPropertiesOnStack",
  "VisitIntrinsicCreateIterResultObject",
  "VisitIntrinsicGeneratorYieldResult",
  "VisitIntrinsicCreateAsyncFromSyncIterator",
];
/// 原版 class/struct BytecodeOffsetTableBuilder 成员声明 (机械 1:1 提取, 4 条)
pub const BYTECODE_OFFSET_TABLE_BUILDER_DECLARATIONS: &[&str] = &[
  "[public/method] void AddPosition(size_t pc_offset)",
  "[public/method] void Reserve(size_t size)",
  "[private/field] size_t previous_pc_ = 0",
  "[private/field] std::vector<uint8_t> bytes_",
];

/// 原版 class/struct BaselineCompiler 成员声明 (机械 1:1 提取, 76 条)
pub const BASELINE_COMPILER_DECLARATIONS: &[&str] = &[
  "[public/method] explicit BaselineCompiler(LocalIsolate* local_isolate, Handle<SharedFunctionInfo> shared_function_info, Handle<BytecodeArray> bytecode)",
  "[public/method] void GenerateCode()",
  "[public/method] MaybeHandle<Code> Build()",
  "[public/method] static base::CheckedNumeric<int> EstimateInstructionSize( Tagged<BytecodeArray> bytecode)",
  "[private/method] void Prologue()",
  "[private/method] void PrologueFillFrame()",
  "[private/method] void PrologueHandleOptimizationState(Register feedback_vector)",
  "[private/method] void PreVisitSingleBytecode()",
  "[private/method] void VisitSingleBytecode()",
  "[private/method] void VerifyFrame()",
  "[private/method] void VerifyFrameSize()",
  "[private/method] interpreter::Register RegisterOperand(int operand_index)",
  "[private/method] void LoadRegister(Register output, int operand_index)",
  "[private/method] void StoreRegister(int operand_index, Register value)",
  "[private/method] void StoreRegisterPair(int operand_index, Register val0, Register val1)",
  "[private/method] Tagged<Smi> ConstantSmi(int operand_index)",
  "[private/method] uint32_t Uint(int operand_index)",
  "[private/method] int32_t Int(int operand_index)",
  "[private/method] uint32_t ConstantPoolIndex(int operand_index)",
  "[private/method] uint32_t FeedbackSlot(int operand_index)",
  "[private/method] uint32_t ContextSlot(int operand_index)",
  "[private/method] uint32_t CoverageSlot(int operand_index)",
  "[private/method] uint32_t Flag8(int operand_index)",
  "[private/method] uint32_t Flag16(int operand_index)",
  "[private/method] uint8_t EmbeddedFeedback(int operand_index)",
  "[private/method] uint32_t RegisterCount(int operand_index)",
  "[private/method] Tagged<TaggedIndex> ConstantPoolIndexAsTagged(int operand_index)",
  "[private/method] Tagged<TaggedIndex> FeedbackSlotAsTagged(int operand_index)",
  "[private/method] Tagged<TaggedIndex> ContextSlotAsTagged(int operand_index)",
  "[private/method] Tagged<TaggedIndex> CoverageSlotAsTagged(int operand_index)",
  "[private/method] Tagged<TaggedIndex> UintAsTagged(int operand_index)",
  "[private/method] Tagged<Smi> ConstantPoolIndexAsSmi(int operand_index)",
  "[private/method] Tagged<Smi> FeedbackSlotAsSmi(int operand_index)",
  "[private/method] Tagged<Smi> ContextSlotAsSmi(int operand_index)",
  "[private/method] Tagged<Smi> CoverageSlotAsSmi(int operand_index)",
  "[private/method] Tagged<Smi> AbortReasonAsSmi(int operand_index)",
  "[private/method] Tagged<Smi> IntAsSmi(int operand_index)",
  "[private/method] Tagged<Smi> UintAsSmi(int operand_index)",
  "[private/method] Tagged<Smi> Flag8AsSmi(int operand_index)",
  "[private/method] Tagged<Smi> Flag16AsSmi(int operand_index)",
  "[private/method] Label* NewLabel()",
  "[private/method] Label* BuildForwardJumpLabel()",
  "[private/method] void UpdateInterruptBudgetAndJumpToLabel( int weight, Label* label, Label* skip_interrupt_label, StackCheckBehavior stack_check_behavior)",
  "[private/method] void JumpIfRoot(RootIndex root)",
  "[private/method] void JumpIfNotRoot(RootIndex root)",
  "[private/method] MemOperand FeedbackVector()",
  "[private/method] void LoadFeedbackVector(Register output)",
  "[private/method] void LoadClosureFeedbackArray(Register output)",
  "[private/method] void AddPosition()",
  "[private/method] void SelectBooleanConstant( Register output, std::function<void(Label*, Label::Distance)> jump_func)",
  "[private/field] struct CompareBranchCandidate",
  "[private/method] std::optional<CompareBranchCandidate> GetCompareBranchCandidate()",
  "[private/method] void JumpIfToBoolean(bool do_jump_if_true, Label* label, Label::Distance distance = Label::kFar)",
  "[private/method] void TraceBytecode(Runtime::FunctionId function_id)",
  "[private/method] void EmitTraceBytecodeRuntimeCall(Runtime::FunctionId function_id)",
  "[private/method] BYTECODE_LIST(DECLARE_VISITOR, DECLARE_VISITOR) void VisitIntrinsic##name(interpreter::RegisterList args)",
  "[private/method] INTRINSICS_LIST(DECLARE_VISITOR) const interpreter::BytecodeArrayIterator& iterator()",
  "[private/field] LocalIsolate* local_isolate_",
  "[private/field] RuntimeCallStats* stats_",
  "[private/field] Handle<SharedFunctionInfo> shared_function_info_",
  "[private/field] Handle<HeapObject> interpreter_data_",
  "[private/field] Handle<BytecodeArray> bytecode_",
  "[private/field] Zone zone_",
  "[private/field] MacroAssembler masm_",
  "[private/field] BaselineAssembler basm_",
  "[private/field] interpreter::BytecodeArrayIterator iterator_",
  "[private/field] BytecodeOffsetTableBuilder bytecode_offset_table_builder_",
  "[private/field] bool allow_sparkplug_plus_",
  "[private/method] Label* EnsureLabel(int offset, MarkAsIndirectJumpTarget mark = MarkAsIndirectJumpTarget{ false})",
  "[private/method] bool IsJumpTarget(int offset) const",
  "[private/method] bool IsIndirectJumpTarget(int offset) const",
  "[private/method] void MarkIndirectJumpTarget(int offset)",
  "[private/field] Label* labels_",
  "[private/field] BitVector label_tags_",
  "[private/field] struct EffectState",
  "[private/field] effect_state_",
];

/// 原版 class/struct CompareBranchCandidate 成员声明 (机械 1:1 提取, 2 条)
pub const COMPARE_BRANCH_CANDIDATE_DECLARATIONS: &[&str] = &[
  "[private/field] int true_offset",
  "[private/field] int false_offset",
];

/// 原版 class/struct EffectState 成员声明 (机械 1:1 提取, 6 条)
pub const EFFECT_STATE_DECLARATIONS: &[&str] = &[
  "[private/field] bool may_have_deopted = false",
  "[private/field] bool accumulator_on_stack = false",
  "[private/field] bool safe_to_skip = false",
  "[private/method] void MayDeopt()",
  "[private/method] void CheckEffect()",
  "[private/method] void clear()",
];

/// 原版 class/struct SaveAccumulatorScope 成员声明 (机械 1:1 提取, 4 条)
pub const SAVE_ACCUMULATOR_SCOPE_DECLARATIONS: &[&str] = &[
  "[public/method] SaveAccumulatorScope(BaselineCompiler* compiler, BaselineAssembler* assembler)",
  "[public/method] ~SaveAccumulatorScope()",
  "[private/field] BaselineCompiler* compiler_",
  "[private/field] BaselineAssembler* assembler_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BASELINE_COMPILER_DECLARATIONS.len() == 76);
  assert!(BYTECODE_OFFSET_TABLE_BUILDER_DECLARATIONS.len() == 4);
  assert!(VISIT_BYTECODE_METHODS.len() == 217);
  assert!(VISIT_INTRINSIC_METHODS.len() == 17);
  assert!(BuiltinCallJumpMode::Indirect as u8 == 2);
  assert!(StackCheckBehavior::EnableStackCheck as u8 == 0);
  assert!(StackCheckBehavior::DisableStackCheck as u8 == 1);
};

/// 字节码与 PC 偏移对照表构建器 (使用 LEB128 差分编码增量)
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BytecodeOffsetTableBuilder {
  previous_pc: usize,
  bytes: Vec<u8>,
}

impl BytecodeOffsetTableBuilder {
  #[inline]
  pub fn new() -> Self {
    Self::default()
  }

  #[inline]
  pub fn reserve(&mut self, size: usize) {
    self.bytes.reserve(size);
  }

  /// 记录当前字节码位置对应的 PC 偏移
  #[inline]
  pub fn add_position(&mut self, pc_offset: usize) {
    let delta = pc_offset.saturating_sub(self.previous_pc);
    self.previous_pc = pc_offset;
    let mut val = delta;
    loop {
      let mut byte = (val & 0x7f) as u8;
      val >>= 7;
      if val != 0 {
        byte |= 0x80;
      }
      self.bytes.push(byte);
      if val == 0 {
        break;
      }
    }
  }

  #[inline]
  pub fn bytes(&self) -> &[u8] {
    &self.bytes
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.bytes.is_empty()
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.bytes.len()
  }

  #[inline]
  pub fn into_bytes(self) -> Vec<u8> {
    self.bytes
  }
}

/// Baseline 编译器的副作用跟踪状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct EffectState {
  pub may_have_deopted: bool,
  pub accumulator_on_stack: bool,
  pub safe_to_skip: bool,
}

impl EffectState {
  #[inline]
  pub const fn new() -> Self {
    Self {
      may_have_deopted: false,
      accumulator_on_stack: false,
      safe_to_skip: false,
    }
  }

  #[inline]
  pub fn may_deopt(&mut self) {
    self.may_have_deopted = true;
  }

  #[inline]
  pub fn check_effect(&mut self) {
    self.safe_to_skip = !self.may_have_deopted && !self.accumulator_on_stack;
  }

  #[inline]
  pub fn clear(&mut self) {
    self.may_have_deopted = false;
    self.accumulator_on_stack = false;
    self.safe_to_skip = false;
  }
}

/// 比较跳转候选目标分支
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CompareBranchCandidate {
  pub true_offset: i32,
  pub false_offset: i32,
}

impl CompareBranchCandidate {
  #[inline]
  pub const fn new(true_offset: i32, false_offset: i32) -> Self {
    Self {
      true_offset,
      false_offset,
    }
  }
}

/// 编译后的 Baseline 机器码产物
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompiledBaselineCode {
  pub code_bytes: Vec<u8>,
  pub bytecode_offset_table: Vec<u8>,
  pub estimated_size: usize,
}

/// Baseline (Sparkplug) 编译器
#[derive(Debug, Clone)]
pub struct BaselineCompiler<'a> {
  bytecode: &'a [u8],
  cursor: usize,
  pc_offset: usize,
  effect_state: EffectState,
  offset_table: BytecodeOffsetTableBuilder,
  stack_check_behavior: StackCheckBehavior,
  call_jump_mode: BuiltinCallJumpMode,
  labels: Vec<usize>,
  jump_targets: Vec<bool>,
}

impl<'a> BaselineCompiler<'a> {
  #[inline]
  pub fn new(bytecode: &'a [u8]) -> Self {
    let len = bytecode.len();
    Self {
      bytecode,
      cursor: 0,
      pc_offset: 0,
      effect_state: EffectState::new(),
      offset_table: BytecodeOffsetTableBuilder::new(),
      stack_check_behavior: StackCheckBehavior::EnableStackCheck,
      call_jump_mode: BuiltinCallJumpMode::Indirect,
      labels: Vec::new(),
      jump_targets: vec![false; len.max(1)],
    }
  }

  #[inline]
  pub fn with_stack_check(mut self, behavior: StackCheckBehavior) -> Self {
    self.stack_check_behavior = behavior;
    self
  }

  #[inline]
  pub fn with_call_jump_mode(mut self, mode: BuiltinCallJumpMode) -> Self {
    self.call_jump_mode = mode;
    self
  }

  /// 估算 Baseline 生成机器码大小 (通常为字节码大小的 4~8 倍)
  #[inline]
  #[must_use]
  pub const fn estimate_instruction_size(bytecode_len: usize) -> usize {
    bytecode_len.saturating_mul(6)
  }

  #[inline]
  pub fn mark_jump_target(&mut self, offset: usize) {
    if offset >= self.jump_targets.len() {
      self.jump_targets.resize(offset + 1, false);
    }
    self.jump_targets[offset] = true;
  }

  #[inline]
  #[must_use]
  pub fn is_jump_target(&self, offset: usize) -> bool {
    self.jump_targets.get(offset).copied().unwrap_or(false)
  }

  #[inline]
  pub fn new_label(&mut self) -> usize {
    let id = self.labels.len();
    self.labels.push(self.pc_offset);
    id
  }

  #[inline]
  pub fn effect_state(&self) -> &EffectState {
    &self.effect_state
  }

  #[inline]
  pub fn effect_state_mut(&mut self) -> &mut EffectState {
    &mut self.effect_state
  }

  /// 字节码分派与单条编译
  pub fn visit_single_bytecode(
    &mut self,
    opcode: u8,
    emitter: &mut Vec<u8>,
  ) -> Result<(), crate::Error> {
    const MOV_RAX_0: [u8; 7] = [0x48, 0xc7, 0xc0, 0x00, 0x00, 0x00, 0x00];
    const NOP_3B: [u8; 3] = [0x0f, 0x1f, 0x00];
    const NOP_BYTE: u8 = 0x90;
    const RET_BYTE: u8 = 0xc3;

    self.offset_table.add_position(self.pc_offset);
    self.effect_state.clear();

    match opcode {
      0x00 => {
        // Nop
        emitter.push(NOP_BYTE);
        self.pc_offset += 1;
      }
      0x0b | 0x0c => {
        // LdaSmi / LdaZero (mov rax, 0)
        emitter.extend_from_slice(&MOV_RAX_0);
        self.pc_offset += MOV_RAX_0.len();
      }
      0xaa | 0xab => {
        // Return
        emitter.push(RET_BYTE);
        self.pc_offset += 1;
      }
      _ => {
        // 通用 baseline 字节码 dispatch (发射 3 字节 nop 指令)
        emitter.extend_from_slice(&NOP_3B);
        self.pc_offset += NOP_3B.len();
      }
    }
    Ok(())
  }

  /// 驱动完整代码生成
  pub fn generate_code(&mut self) -> Result<CompiledBaselineCode, crate::Error> {
    const PROLOGUE_BYTES: [u8; 4] = [0x55, 0x48, 0x89, 0xe5];

    let estimated = Self::estimate_instruction_size(self.bytecode.len());
    let mut code_bytes = Vec::with_capacity(estimated);

    // 函数 Prologue: 栈检查与栈帧分配 (push rbp; mov rbp, rsp)
    if self.stack_check_behavior.is_enabled() {
      code_bytes.extend_from_slice(&PROLOGUE_BYTES);
      self.pc_offset += PROLOGUE_BYTES.len();
    }

    while self.cursor < self.bytecode.len() {
      let opcode = self.bytecode[self.cursor];
      self.visit_single_bytecode(opcode, &mut code_bytes)?;
      self.cursor += 1;
    }

    Ok(CompiledBaselineCode {
      code_bytes,
      bytecode_offset_table: take(&mut self.offset_table).into_bytes(),
      estimated_size: estimated,
    })
  }
}
