// 由 tool/src/baseline/baseline-assembler.h.js 从 v8/src/baseline/baseline-assembler.{h,-inl.h} 生成; 修订就地改, 复跑覆盖

/// 指针压缩模式 (原版 enum class CompressionMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
#[repr(u8)]
pub enum CompressionMode {
  /// 默认 (按构建配置)
  Default = 0,
  /// 强制解压
  ForceDecompression = 1,
}

impl CompressionMode {
  #[inline]
  #[must_use]
  pub const fn is_default(self) -> bool {
    matches!(self, Self::Default)
  }

  #[inline]
  #[must_use]
  pub const fn is_force_decompression(self) -> bool {
    matches!(self, Self::ForceDecompression)
  }
}

/// 其他架构 inl 文件跳过登记 (目标仅 x64 + arm64)
pub const SKIP_REGISTRATION: [&str; 14] = [
  "src/baseline/arm/baseline-assembler-arm-inl.h",
  "src/baseline/arm/baseline-compiler-arm-inl.h",
  "src/baseline/ia32/baseline-assembler-ia32-inl.h",
  "src/baseline/ia32/baseline-compiler-ia32-inl.h",
  "src/baseline/loong64/baseline-assembler-loong64-inl.h",
  "src/baseline/loong64/baseline-compiler-loong64-inl.h",
  "src/baseline/mips64/baseline-assembler-mips64-inl.h",
  "src/baseline/mips64/baseline-compiler-mips64-inl.h",
  "src/baseline/ppc/baseline-assembler-ppc-inl.h",
  "src/baseline/ppc/baseline-compiler-ppc-inl.h",
  "src/baseline/riscv/baseline-assembler-riscv-inl.h",
  "src/baseline/riscv/baseline-compiler-riscv-inl.h",
  "src/baseline/s390/baseline-assembler-s390-inl.h",
  "src/baseline/s390/baseline-compiler-s390-inl.h",
];

/// 原版 class/struct BaselineAssembler 成员声明 (机械 1:1 提取, 99 条)
pub const BASELINE_ASSEMBLER_DECLARATIONS: &[&str] = &[
  "[public/field] class ScratchRegisterScope",
  "[public/method] explicit BaselineAssembler(MacroAssembler* masm) : masm_(masm)",
  "[public/method] inline static MemOperand RegisterFrameOperand( interpreter::Register interpreter_register)",
  "[public/method] inline void RegisterFrameAddress(interpreter::Register interpreter_register, Register rscratch)",
  "[public/method] inline MemOperand ContextOperand()",
  "[public/method] inline MemOperand FunctionOperand()",
  "[public/method] inline MemOperand FeedbackVectorOperand()",
  "[public/method] inline MemOperand FeedbackCellOperand()",
  "[public/method] inline void GetCode(LocalIsolate* isolate, CodeDesc* desc)",
  "[public/method] inline int pc_offset() const",
  "[public/method] inline void CodeEntry() const",
  "[public/method] inline void ExceptionHandler() const",
  "[public/method] V8_INLINE void RecordComment(const char* string)",
  "[public/method] inline void Trap()",
  "[public/method] inline void DebugBreak()",
  "[public/method] inline void AssertInSandboxedExecutionMode()",
  "[public/method] inline void Bind(Label* label)",
  "[public/method] inline void JumpTarget()",
  "[public/method] inline void Jump(Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfRoot(Register value, RootIndex index, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfNotRoot(Register value, RootIndex index, Label* target, Label ::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfSmi(Register value, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfNotSmi(Register value, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void TestAndBranch(Register value, int mask, Condition cc, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIf(Condition cc, Register lhs, const Operand& rhs, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfJSAnyIsPrimitive(Register heap_object, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfObjectType(Condition cc, Register object, InstanceType instance_type, Register map, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfObjectTypeFast(Condition cc, Register object, InstanceType instance_type, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfInstanceType(Condition cc, Register map, InstanceType instance_type, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfPointer(Condition cc, Register value, MemOperand operand, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline Condition CheckSmi(Register value)",
  "[public/method] inline void JumpIfSmi(Condition cc, Register value, Tagged<Smi> smi, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfSmi(Condition cc, Register lhs, Register rhs, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfImmediate(Condition cc, Register left, int right, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfTagged(Condition cc, Register value, MemOperand operand, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfTagged(Condition cc, MemOperand operand, Register value, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfStaticRootToBoolean(Register value, Label* true_target, Label::Distance true_distance, Label* false_target, Label::Distance false_distance)",
  "[public/method] inline void JumpIfByte(Condition cc, Register value, int32_t byte, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void LoadMap(Register output, Register value)",
  "[public/method] inline void LoadRoot(Register output, RootIndex index)",
  "[public/method] inline void LoadNativeContextSlot(Register output, uint32_t index)",
  "[public/method] inline void Move(Register output, Register source)",
  "[public/method] inline void Move(Register output, MemOperand operand)",
  "[public/method] inline void Move(Register output, Tagged<Smi> value)",
  "[public/method] inline void Move(Register output, Tagged<TaggedIndex> value)",
  "[public/method] inline void Move(Register output, interpreter::Register source)",
  "[public/method] inline void Move(interpreter::Register output, Register source)",
  "[public/method] inline void Move(Register output, RootIndex source)",
  "[public/method] inline void Move(MemOperand output, Register source)",
  "[public/method] inline void Move(Register output, ExternalReference reference)",
  "[public/method] inline void Move(Register output, Handle<HeapObject> value)",
  "[public/method] inline void Move(Register output, int32_t immediate)",
  "[public/method] inline void MoveMaybeSmi(Register output, Register source)",
  "[public/method] inline void MoveSmi(Register output, Register source)",
  "[public/method] inline void CallBuiltin(Builtin builtin)",
  "[public/method] inline void TailCallBuiltin(Builtin builtin)",
  "[public/method] inline void CallRuntime(Runtime::FunctionId function, int nargs)",
  "[public/method] inline void LoadTaggedField(Register output, Register source, int offset)",
  "[public/method] inline void LoadTaggedSignedField(Register output, Register source, int offset)",
  "[public/method] inline void LoadTaggedSignedFieldAndUntag(Register output, Register source, int offset)",
  "[public/method] inline void LoadWord16FieldZeroExtend(Register output, Register source, int offset)",
  "[public/method] inline void LoadWord8Field(Register output, Register source, int offset)",
  "[public/method] inline void StoreTaggedSignedField(Register target, int offset, Tagged<Smi> value)",
  "[public/method] inline void StoreTaggedFieldWithWriteBarrier(Register target, int offset, Register value)",
  "[public/method] inline void StoreTaggedFieldNoWriteBarrier(Register target, int offset, Register value)",
  "[public/method] inline void LoadFixedArrayElement(Register output, Register array, int32_t index)",
  "[public/method] inline void LoadPrototype(Register prototype, Register object)",
  "[public/method] inline void LoadTaggedField(TaggedRegister output, Register source, int offset)",
  "[public/method] inline void LoadTaggedField(TaggedRegister output, TaggedRegister source, int offset)",
  "[public/method] inline void LoadTaggedField(Register output, TaggedRegister source, int offset)",
  "[public/method] inline void LoadFixedArrayElement(Register output, TaggedRegister array, int32_t index)",
  "[public/method] inline void LoadFixedArrayElement(TaggedRegister output, TaggedRegister array, int32_t index)",
  "[public/method] inline void TryLoadOptimizedOsrCode(Register scratch_and_result, Register feedback_vector, FeedbackSlot slot, Label* on_result, Label::Distance distance)",
  "[public/method] inline void AddToInterruptBudgetAndJumpIfNotExceeded( int32_t weight, Label* skip_interrupt_label)",
  "[public/method] inline void AddToInterruptBudgetAndJumpIfNotExceeded( Register weight, Label* skip_interrupt_label)",
  "[public/method] inline void LdaContextSlotNoCell( Register context, uint32_t index, uint32_t depth, CompressionMode compression_mode = CompressionMode::kDefault, Register output = kInterpreterAccumulatorRegister)",
  "[public/method] inline void StaContextSlotNoCell(Register context, Register value, uint32_t index, uint32_t depth)",
  "[public/method] inline void LdaModuleVariable(Register context, int cell_index, uint32_t depth)",
  "[public/method] inline void StaModuleVariable(Register context, Register value, int cell_index, uint32_t depth)",
  "[public/method] inline void IncrementSmi(MemOperand lhs)",
  "[public/method] inline void SmiUntag(Register value)",
  "[public/method] inline void SmiUntag(Register output, Register value)",
  "[public/method] inline void Word32And(Register output, Register lhs, int rhs)",
  "[public/method] inline void JumpIfNotBothSmi(Register lhs, Register rhs, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void JumpIfTagged(Condition cc, Register lhs, Register rhs, Label* target, Label::Distance distance = Label::kFar)",
  "[public/method] inline void SmiAddConstantAndJumpIfOverflow( Register value, int constant, Label* overflow, Label::Distance distance = Label::kFar)",
  "[public/method] inline void Switch(Register reg, int case_value_base, Label** labels, int num_labels)",
  "[public/method] inline void LoadRegister(Register output, interpreter::Register source)",
  "[public/method] inline void StoreRegister(interpreter::Register output, Register value)",
  "[public/method] inline void LoadFunction(Register output)",
  "[public/method] inline void LoadContext(Register output)",
  "[public/method] inline void StoreContext(Register context)",
  "[public/method] inline void LoadFeedbackCell(Register output)",
  "[public/method] inline void AssertFeedbackCell(Register object)",
  "[public/method] inline void MaybeEmitPlaceHolderForDeopt()",
  "[public/method] inline static void EmitReturn(MacroAssembler* masm)",
  "[public/method] MacroAssembler* masm()",
  "[private/field] MacroAssembler* masm_",
  "[private/field] ScratchRegisterScope* scratch_register_scope_ = nullptr",
];

/// 原版 class/struct EnsureAccumulatorPreservedScope 成员声明 (机械 1:1 提取, 5 条)
pub const ENSURE_ACCUMULATOR_PRESERVED_SCOPE_DECLARATIONS: &[&str] = &[
  "[public/method] inline explicit EnsureAccumulatorPreservedScope(BaselineAssembler* assembler)",
  "[public/method] inline ~EnsureAccumulatorPreservedScope()",
  "[private/method] inline void AssertEqualToAccumulator(Register reg)",
  "[private/field] BaselineAssembler* assembler_",
  "[private/field] Assembler::CodeComment comment_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BASELINE_ASSEMBLER_DECLARATIONS.len() == 99);
  assert!(ENSURE_ACCUMULATOR_PRESERVED_SCOPE_DECLARATIONS.len() == 5);
  assert!(SKIP_REGISTRATION.len() == 14);
};
