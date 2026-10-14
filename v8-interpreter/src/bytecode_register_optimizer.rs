// 由 tool/src/interpreter/bytecode-register-optimizer.h.js 从 v8/src/interpreter/bytecode-register-optimizer.{h,cc} 生成; 修订就地改, 复跑覆盖

/// 无效等价类 id (原版 kInvalidEquivalenceId = kMaxUInt32)
pub const INVALID_EQUIVALENCE_ID: u32 = u32::MAX;

/// 寄存器变量持有确性 (原版 enum class VariableHintMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VariableHintMode {
  /// 确定持有变量
  DefinitelyHasVariable = 0,
  /// 可能持有变量
  MightHaveVariable = 1,
}

/// 字节码写出回调 trait 骨架 (原版 class BytecodeWriter 纯虚三发射)
pub trait BytecodeWriter {
  fn emit_ldar(&mut self, input: i32);
  fn emit_star(&mut self, output: i32);
  fn emit_mov(&mut self, input: i32, output: i32);
}

/// 原版 class/struct BytecodeRegisterOptimizer 成员声明 (机械 1:1 提取, 59 条)
pub const BYTECODE_REGISTER_OPTIMIZER_DECLARATIONS: &[&str] = &[
  "[public/field] class BytecodeWriter",
  "[public/method] BytecodeRegisterOptimizer(Zone* zone, BytecodeRegisterAllocator* register_allocator, int fixed_registers_count, int parameter_count, BytecodeWriter* bytecode_writer)",
  "[public/method] ~BytecodeRegisterOptimizer() override = default",
  "[public/method] BytecodeRegisterOptimizer(const BytecodeRegisterOptimizer&) = delete",
  "[public/method] BytecodeRegisterOptimizer& operator=(const BytecodeRegisterOptimizer&) = delete",
  "[public/method] void DoLdar(Register input)",
  "[public/method] void DoStar(Register output)",
  "[public/method] void DoMov(Register input, Register output)",
  "[public/method] void Flush()",
  "[public/method] bool EnsureAllRegistersAreFlushed() const",
  "[public/method] void PrepareOutputRegister(Register reg)",
  "[public/method] void PrepareOutputRegisterList(RegisterList reg_list)",
  "[public/method] Register GetInputRegister(Register reg)",
  "[public/method] RegisterList GetInputRegisterList(RegisterList reg_list)",
  "[public/method] void SetVariableInRegister(Variable* var, Register reg)",
  "[public/method] Variable* GetPotentialVariableInRegister(Register reg)",
  "[public/method] Variable* GetPotentialVariableInAccumulator()",
  "[public/method] bool IsVariableInRegister(Variable* var, Register reg)",
  "[public/method] TypeHint GetTypeHint(Register reg)",
  "[public/method] void SetTypeHintForAccumulator(TypeHint hint)",
  "[public/method] void ResetTypeHintForAccumulator()",
  "[public/method] bool IsAccumulatorReset()",
  "[public/method] int maxiumum_register_index() const",
  "[private/const] static const uint32_t kInvalidEquivalenceId",
  "[private/field] class RegisterInfo",
  "[private/method] void RegisterAllocateEvent(Register reg) override",
  "[private/method] void RegisterListAllocateEvent(RegisterList reg_list) override",
  "[private/method] void RegisterListFreeEvent(RegisterList reg) override",
  "[private/method] void RegisterFreeEvent(Register reg) override",
  "[private/method] void RegisterTransfer(RegisterInfo* input, RegisterInfo* output)",
  "[private/method] void OutputRegisterTransfer(RegisterInfo* input, RegisterInfo* output)",
  "[private/method] void CreateMaterializedEquivalent(RegisterInfo* info)",
  "[private/method] RegisterInfo* GetMaterializedEquivalentNotAccumulator(RegisterInfo* info)",
  "[private/method] void Materialize(RegisterInfo* info)",
  "[private/method] void AddToEquivalenceSet(RegisterInfo* set_member, RegisterInfo* non_set_member)",
  "[private/method] void PushToRegistersNeedingFlush(RegisterInfo* reg)",
  "[private/method] RegisterInfo* GetRegisterInfo(Register reg)",
  "[private/method] RegisterInfo* GetOrCreateRegisterInfo(Register reg)",
  "[private/method] RegisterInfo* NewRegisterInfo(Register reg)",
  "[private/method] void GrowRegisterMap(Register reg)",
  "[private/method] bool RegisterIsTemporary(Register reg) const",
  "[private/method] bool RegisterIsObservable(Register reg) const",
  "[private/method] static Register OperandToRegister(uint32_t operand)",
  "[private/method] size_t GetRegisterInfoTableIndex(Register reg) const",
  "[private/method] Register RegisterFromRegisterInfoTableIndex(size_t index) const",
  "[private/method] uint32_t NextEquivalenceId()",
  "[private/method] void AllocateRegister(RegisterInfo* info)",
  "[private/method] Zone* zone()",
  "[private/field] const Register accumulator_",
  "[private/field] RegisterInfo* accumulator_info_",
  "[private/field] const Register temporary_base_",
  "[private/field] int max_register_index_",
  "[private/field] ZoneVector<RegisterInfo*> register_info_table_",
  "[private/field] int register_info_table_offset_",
  "[private/field] ZoneDeque<RegisterInfo*> registers_needing_flushed_",
  "[private/field] uint32_t equivalence_id_",
  "[private/field] BytecodeWriter* bytecode_writer_",
  "[private/field] bool flush_required_",
  "[private/field] Zone* zone_",
];

/// 原版 class/struct BytecodeWriter 成员声明 (机械 1:1 提取, 7 条)
pub const BYTECODE_WRITER_DECLARATIONS: &[&str] = &[
  "[public/method] BytecodeWriter() = default",
  "[public/method] virtual ~BytecodeWriter() = default",
  "[public/method] BytecodeWriter(const BytecodeWriter&) = delete",
  "[public/method] BytecodeWriter& operator=(const BytecodeWriter&) = delete",
  "[public/method] virtual void EmitLdar(Register input) = 0",
  "[public/method] virtual void EmitStar(Register output) = 0",
  "[public/method] virtual void EmitMov(Register input, Register output) = 0",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(BYTECODE_REGISTER_OPTIMIZER_DECLARATIONS.len() == 59);
  assert!(BYTECODE_WRITER_DECLARATIONS.len() == 7);
  assert!(INVALID_EQUIVALENCE_ID == u32::MAX);
};
