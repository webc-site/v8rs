// 由 tool/src/interpreter/interpreter-assembler.h.js 从 v8/src/interpreter/interpreter-assembler.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 栈检查开关 (原版 enum StackCheckBehavior)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum StackCheckBehavior {
  /// 启用栈检查
  EnableStackCheck = 0,
  /// 禁用栈检查
  DisableStackCheck = 1,
}

/// OSR 参数 (原版 enum OnStackReplacementParams)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OnStackReplacementParams {
  /// baseline code 已缓存
  BaselineCodeIsCached = 0,
  /// 默认
  Default = 1,
}

/// 首字节码偏移 = BytecodeArray::kHeaderSize - kHeapObjectTag (原版 kFirstBytecodeOffset)
/// 字节码数组头大小与堆对象 tag 属 objects crate, 公式原样保留, 数值由参数注入
#[inline]
#[must_use]
pub const fn first_bytecode_offset(bytecode_array_header_size: i32, heap_object_tag: i32) -> i32 {
  bytecode_array_header_size - heap_object_tag
}

/// 原版 class/struct InterpreterAssembler 成员声明 (机械 1:1 提取, 143 条)
pub const INTERPRETER_ASSEMBLER_DECLARATIONS: &[&str] = &[
  "[public/method] InterpreterAssembler(compiler::CodeAssemblerState* state, Bytecode bytecode, OperandScale operand_scale)",
  "[public/method] ~InterpreterAssembler()",
  "[public/method] InterpreterAssembler(const InterpreterAssembler&) = delete",
  "[public/method] InterpreterAssembler& operator=(const InterpreterAssembler&) = delete",
  "[public/method] TNode<Uint32T> BytecodeOperandCount(int operand_index)",
  "[public/method] TNode<Uint32T> BytecodeOperandFlag8(int operand_index)",
  "[public/method] TNode<Uint32T> BytecodeOperandFlag16(int operand_index)",
  "[public/method] TNode<UintPtrT> BytecodeOperandConstantPoolIndex(int operand_index)",
  "[public/method] TNode<UintPtrT> BytecodeOperandFeedbackSlot(int operand_index)",
  "[public/method] TNode<UintPtrT> BytecodeOperandContextSlot(int operand_index)",
  "[public/method] TNode<UintPtrT> BytecodeOperandCoverageSlot(int operand_index)",
  "[public/method] TNode<Smi> BytecodeOperandConstantPoolIndexSmi(int operand_index)",
  "[public/method] TNode<Smi> BytecodeOperandFeedbackSlotSmi(int operand_index)",
  "[public/method] TNode<Smi> BytecodeOperandContextSlotSmi(int operand_index)",
  "[public/method] TNode<Smi> BytecodeOperandCoverageSlotSmi(int operand_index)",
  "[public/method] TNode<TaggedIndex> BytecodeOperandConstantPoolIndexTaggedIndex( int operand_index)",
  "[public/method] TNode<TaggedIndex> BytecodeOperandFeedbackSlotTaggedIndex(int operand_index)",
  "[public/method] TNode<TaggedIndex> BytecodeOperandContextSlotTaggedIndex(int operand_index)",
  "[public/method] TNode<TaggedIndex> BytecodeOperandCoverageSlotTaggedIndex(int operand_index)",
  "[public/method] TNode<Uint32T> BytecodeOperandUImm(int operand_index)",
  "[public/method] TNode<UintPtrT> BytecodeOperandUImmWord(int operand_index)",
  "[public/method] TNode<Smi> BytecodeOperandUImmSmi(int operand_index)",
  "[public/method] TNode<Int32T> BytecodeOperandImm(int operand_index)",
  "[public/method] TNode<IntPtrT> BytecodeOperandImmIntPtr(int operand_index)",
  "[public/method] TNode<Smi> BytecodeOperandImmSmi(int operand_index)",
  "[public/method] TNode<Uint32T> BytecodeOperandRuntimeId(int operand_index)",
  "[public/method] TNode<UintPtrT> BytecodeOperandNativeContextIndex(int operand_index)",
  "[public/method] TNode<Uint32T> BytecodeOperandIntrinsicId(int operand_index)",
  "[public/method] TNode<Uint32T> BytecodeOperandAbortReason(int operand_index)",
  "[public/method] TNode<Uint32T> BytecodeOperandEmbeddedFeedback(int operand_index)",
  "[public/method] TNode<IntPtrT> BytecodeOperandOffset(int operand_index)",
  "[public/method] TNode<Object> GetAccumulator()",
  "[public/method] void SetAccumulator(TNode<Object> value)",
  "[public/method] void ClobberAccumulator(TNode<Object> clobber_value)",
  "[public/method] TNode<Context> GetContext()",
  "[public/method] void SetContext(TNode<Context> value)",
  "[public/method] TNode<Context> GetContextAtDepth(TNode<Context> context, TNode<Uint32T> depth)",
  "[public/field] class RegListNodePair",
  "[public/method] TNode<FixedArray> ExportParametersAndRegisterFile( TNode<FixedArray> array, const RegListNodePair& registers)",
  "[public/method] TNode<FixedArray> ImportRegisterFile(TNode<FixedArray> array, const RegListNodePair& registers)",
  "[public/method] TNode<Object> LoadRegister(Register reg)",
  "[public/method] TNode<IntPtrT> LoadAndUntagRegister(Register reg)",
  "[public/method] TNode<Object> LoadRegisterAtOperandIndex(int operand_index)",
  "[public/method] std::pair<TNode<Object>, TNode<Object>> LoadRegisterPairAtOperandIndex( int operand_index)",
  "[public/method] void StoreRegister(TNode<Object> value, Register reg)",
  "[public/method] void StoreRegisterAtOperandIndex(TNode<Object> value, int operand_index)",
  "[public/method] void StoreRegisterPairAtOperandIndex(TNode<Object> value1, TNode<Object> value2, int operand_index)",
  "[public/method] void StoreRegisterTripleAtOperandIndex(TNode<Object> value1, TNode<Object> value2, TNode<Object> value3, int operand_index)",
  "[public/method] RegListNodePair GetRegisterListAtOperandIndex(int operand_index)",
  "[public/method] TNode<Object> LoadRegisterFromRegisterList(const RegListNodePair& reg_list, int index)",
  "[public/method] void StoreRegisterFromRegisterList(TNode<Object> value, const RegListNodePair& reg_list, TNode<IntPtrT> index)",
  "[public/method] TNode<IntPtrT> RegisterLocationInRegisterList(const RegListNodePair& reg_list, int index)",
  "[public/method] TNode<Object> LoadConstantPoolEntryAtOperandIndex(int operand_index)",
  "[public/method] TNode<IntPtrT> LoadAndUntagConstantPoolEntryAtOperandIndex(int operand_index)",
  "[public/method] TNode<Object> LoadConstantPoolEntry(TNode<WordT> index)",
  "[public/method] TNode<IntPtrT> LoadAndUntagConstantPoolEntry(TNode<WordT> index)",
  "[public/method] TNode<JSFunction> LoadFunctionClosure()",
  "[public/method] TNode<Union<FeedbackVector, Undefined>> LoadFeedbackVector()",
  "[public/method] auto LoadFeedbackVectorOrUndefinedIfJitless()",
  "[public/const] static constexpr UpdateFeedbackMode DefaultUpdateFeedbackMode()",
  "[public/method] void CallJSAndDispatch(TNode<JSAny> function, TNode<Context> context, const RegListNodePair& args, ConvertReceiverMode receiver_mode)",
  "[public/method] void CallJSWithSpreadAndDispatch(TNode<JSAny> function, TNode<Context> context, const RegListNodePair& args, TNode<UintPtrT> slot_id)",
  "[public/method] TNode<Object> Construct( TNode<JSAny> target, TNode<Context> context, TNode<JSAny> new_target, const RegListNodePair& args, TNode<UintPtrT> slot_id, TNode<Union<FeedbackVector, Undefined>> maybe_feedback_vector)",
  "[public/method] TNode<Object> ConstructWithSpread(TNode<JSAny> target, TNode<Context> context, TNode<JSAny> new_target, const RegListNodePair& args, TNode<UintPtrT> slot_id)",
  "[public/method] TNode<Object> ConstructForwardAllArgs(TNode<JSAny> target, TNode<Context> context, TNode<JSAny> new_target, TNode<TaggedIndex> slot_id)",
  "[public/method] void Jump(TNode<IntPtrT> jump_offset)",
  "[public/method] void JumpBackward(TNode<IntPtrT> jump_offset)",
  "[public/method] void JumpIfTaggedEqual(TNode<Object> lhs, TNode<Object> rhs, TNode<IntPtrT> jump_offset)",
  "[public/method] void JumpIfTaggedEqual(TNode<Object> lhs, TNode<Object> rhs, int operand_index)",
  "[public/method] void JumpIfTaggedEqualConstant(TNode<Object> lhs, TNode<Object> rhs, int operand_index)",
  "[public/method] void JumpIfTaggedNotEqual(TNode<Object> lhs, TNode<Object> rhs, TNode<IntPtrT> jump_offset)",
  "[public/method] void JumpIfTaggedNotEqual(TNode<Object> lhs, TNode<Object> rhs, int operand_index)",
  "[public/method] void JumpIfTaggedNotEqualConstant(TNode<Object> lhs, TNode<Object> rhs, int operand_index)",
  "[public/method] void UpdateInterruptBudgetOnReturn()",
  "[public/method] TNode<Int32T> UpdateInterruptBudget(TNode<Int32T> weight)",
  "[public/method] void DecreaseInterruptBudget(TNode<Int32T> weight, StackCheckBehavior stack_check_behavior)",
  "[public/method] TNode<Int8T> LoadOsrState(TNode<FeedbackVector> feedback_vector)",
  "[public/method] void Dispatch()",
  "[public/method] void DispatchWide(OperandScale operand_scale)",
  "[public/method] void DispatchToBytecode(TNode<WordT> target_bytecode, TNode<IntPtrT> new_bytecode_offset)",
  "[public/method] void DispatchToBytecodeWithOptionalStarLookahead( TNode<WordT> target_bytecode)",
  "[public/method] void Abort(AbortReason abort_reason)",
  "[public/method] void AbortIfWordNotEqual(TNode<WordT> lhs, TNode<WordT> rhs, AbortReason abort_reason)",
  "[public/method] void AbortIfRegisterCountInvalid(TNode<FixedArray> parameters_and_registers, TNode<IntPtrT> parameter_count, TNode<UintPtrT> register_count)",
  "[public/method] void OnStackReplacement(TNode<Context> context, TNode<FeedbackVector> feedback_vector, TNode<IntPtrT> relative_jump, TNode<Int32T> loop_depth, TNode<IntPtrT> feedback_slot, TNode<Int8T> osr_state, OnStackReplacementParams params)",
  "[public/const] static constexpr int kFirstBytecodeOffset = BytecodeArray::kHeaderSize - kHeapObjectTag",
  "[public/method] TNode<IntPtrT> BytecodeOffset()",
  "[protected/method] Bytecode bytecode() const",
  "[protected/method] static bool TargetSupportsUnalignedAccess()",
  "[protected/method] void ToNumberOrNumeric(Object::Conversion mode)",
  "[protected/method] void StoreRegisterForShortStar(TNode<Object> value, TNode<WordT> opcode)",
  "[protected/method] TNode<WordT> LoadBytecode(TNode<IntPtrT> bytecode_offset)",
  "[protected/method] TNode<IntPtrT> LoadParameterCountWithoutReceiver()",
  "[protected/method] TNode<BytecodeArray> BytecodeArrayTaggedPointer()",
  "[private/method] TNode<ExternalReference> DispatchTablePointer()",
  "[private/method] TNode<Object> GetAccumulatorUnchecked()",
  "[private/method] TNode<RawPtrT> GetInterpretedFramePointer()",
  "[private/method] TNode<IntPtrT> RegisterLocation(Register reg)",
  "[private/method] TNode<IntPtrT> RegisterLocation(TNode<IntPtrT> reg_index)",
  "[private/method] TNode<IntPtrT> NextRegister(TNode<IntPtrT> reg_index)",
  "[private/method] TNode<Object> LoadRegister(TNode<IntPtrT> reg_index)",
  "[private/method] void StoreRegister(TNode<Object> value, TNode<IntPtrT> reg_index)",
  "[private/method] void CallPrologue()",
  "[private/method] void CallEpilogue()",
  "[private/method] void TraceBytecode(Runtime::FunctionId function_id)",
  "[private/method] TNode<IntPtrT> RegisterFrameOffset(TNode<IntPtrT> index)",
  "[private/method] TNode<IntPtrT> OperandOffset(int operand_index)",
  "[private/method] TNode<Word32T> BytecodeOperandReadUnaligned(int relative_offset, MachineType result_type)",
  "[private/method] TNode<Uint8T> BytecodeOperandUnsignedByte(int operand_index)",
  "[private/method] TNode<Int8T> BytecodeOperandSignedByte(int operand_index)",
  "[private/method] TNode<Uint16T> BytecodeOperandUnsignedShort(int operand_index)",
  "[private/method] TNode<Int16T> BytecodeOperandSignedShort(int operand_index)",
  "[private/method] TNode<Uint32T> BytecodeOperandUnsignedQuad(int operand_index)",
  "[private/method] TNode<Int32T> BytecodeOperandSignedQuad(int operand_index)",
  "[private/method] TNode<Int32T> BytecodeSignedOperand(int operand_index, OperandSize operand_size)",
  "[private/method] TNode<Uint32T> BytecodeUnsignedOperand(int operand_index, OperandSize operand_size)",
  "[private/method] TNode<IntPtrT> BytecodeOperandReg(int operand_index)",
  "[private/method] TNode<UintPtrT> BytecodeOperandConstantPoolIdx(int operand_index)",
  "[private/method] void JumpToOffset(TNode<IntPtrT> new_bytecode_offset)",
  "[private/method] void JumpConditional(TNode<BoolT> condition, TNode<IntPtrT> jump_offset)",
  "[private/method] void JumpConditionalByImmediateOperand(TNode<BoolT> condition, int operand_index)",
  "[private/method] void JumpConditionalByConstantOperand(TNode<BoolT> condition, int operand_index)",
  "[private/method] void SaveBytecodeOffset()",
  "[private/method] TNode<IntPtrT> ReloadBytecodeOffset()",
  "[private/method] TNode<IntPtrT> Advance()",
  "[private/method] TNode<IntPtrT> Advance(int delta)",
  "[private/method] TNode<IntPtrT> Advance(TNode<IntPtrT> delta)",
  "[private/method] void StarDispatchLookahead(TNode<WordT> target_bytecode)",
  "[private/method] void InlineShortStar(TNode<WordT> target_bytecode)",
  "[private/method] void DispatchToBytecodeHandlerEntry(TNode<RawPtrT> handler_entry, TNode<IntPtrT> bytecode_offset)",
  "[private/method] int CurrentBytecodeSize() const",
  "[private/method] OperandScale operand_scale() const",
  "[private/field] Bytecode bytecode_",
  "[private/field] OperandScale operand_scale_",
  "[private/field] CodeStubAssembler::TVariable<RawPtrT> interpreted_frame_pointer_",
  "[private/field] CodeStubAssembler::TVariable<BytecodeArray> bytecode_array_",
  "[private/field] CodeStubAssembler::TVariable<IntPtrT> bytecode_offset_",
  "[private/field] CodeStubAssembler::TVariable<ExternalReference> dispatch_table_",
  "[private/field] CodeStubAssembler::TVariable<Object> accumulator_",
  "[private/field] ImplicitRegisterUse implicit_register_use_",
  "[private/field] bool made_call_",
  "[private/field] bool reloaded_frame_ptr_",
  "[private/field] bool bytecode_array_valid_",
];

/// 原版 class/struct RegListNodePair 成员声明 (机械 1:1 提取, 5 条)
pub const REG_LIST_NODE_PAIR_DECLARATIONS: &[&str] = &[
  "[public/method] RegListNodePair(TNode<IntPtrT> base_reg_location, TNode<Word32T> reg_count) : base_reg_location_(base_reg_location), reg_count_(reg_count)",
  "[public/method] TNode<Word32T> reg_count() const",
  "[public/method] TNode<IntPtrT> base_reg_location() const",
  "[private/field] TNode<IntPtrT> base_reg_location_",
  "[private/field] TNode<Word32T> reg_count_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(INTERPRETER_ASSEMBLER_DECLARATIONS.len() == 143);
  assert!(REG_LIST_NODE_PAIR_DECLARATIONS.len() == 5);
  assert!(first_bytecode_offset(40, 1) == 39);
};
