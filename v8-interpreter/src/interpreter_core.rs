// 由 tool/src/interpreter/interpreter.h.js 从 v8/src/interpreter/interpreter.{h,cc} 生成; 修订就地改, 复跑覆盖

use crate::{
  bytecode_operands::{ImplicitRegisterUse, OperandScale},
  bytecodes::{BYTECODE_COUNT, Bytecode},
};

/// 宽变体数 (原版 kNumberOfWideVariants = kOperandScaleCount)
pub const NUMBER_OF_WIDE_VARIANTS: usize = 3;
/// 每刻度分发表项数 (原版 kEntriesPerOperandScale = 1 << 8)
pub const ENTRIES_PER_OPERAND_SCALE: usize = 256;
/// 分发表总容量 (原版 kDispatchTableSize = 3 * 256)
pub const DISPATCH_TABLE_SIZE: usize = 768;
/// 字节码总数 (原版 kNumberOfBytecodes = kLast + 1)
pub const NUMBER_OF_BYTECODES: usize = BYTECODE_COUNT;

/// 分发表索引 = 字节码值 + 刻度索引 * 256 (原版 GetDispatchTableIndex)
#[inline]
#[must_use]
pub const fn dispatch_table_index(bytecode: Bytecode, operand_scale: OperandScale) -> usize {
  bytecode as usize + operand_scale.as_index() * ENTRIES_PER_OPERAND_SCALE
}

/// handler 元数据 (原版 struct BytecodeHandlerData 字段面)
#[derive(Debug, Clone, Copy)]
pub struct BytecodeHandlerDataSkeleton {
  pub bytecode: Bytecode,
  pub operand_scale: OperandScale,
  /// 默认 kNone (原版 implicit_register_use = ImplicitRegisterUse::kNone)
  pub implicit_register_use: ImplicitRegisterUse,
  pub made_call: bool,
  pub reloaded_frame_ptr: bool,
  /// 默认 true
  pub bytecode_array_valid: bool,
}

/// 原版 class/struct BytecodeHandlerData 成员声明 (机械 1:1 提取, 7 条)
pub const BYTECODE_HANDLER_DATA_DECLARATIONS: &[&str] = &[
  "[private/method] BytecodeHandlerData(Bytecode bytecode, OperandScale operand_scale) : bytecode(bytecode), operand_scale(operand_scale)",
  "[private/field] Bytecode bytecode",
  "[private/field] OperandScale operand_scale",
  "[private/field] ImplicitRegisterUse implicit_register_use = ImplicitRegisterUse::kNone",
  "[private/field] bool made_call = false",
  "[private/field] bool reloaded_frame_ptr = false",
  "[private/field] bool bytecode_array_valid = true",
];

/// 原版 class/struct Interpreter 成员声明 (机械 1:1 提取, 20 条)
pub const INTERPRETER_DECLARATIONS: &[&str] = &[
  "[public/method] explicit Interpreter(Isolate* isolate)",
  "[public/method] virtual ~Interpreter() = default",
  "[public/method] Interpreter(const Interpreter&) = delete",
  "[public/method] Interpreter& operator=(const Interpreter&) = delete",
  "[public/method] static std::unique_ptr<UnoptimizedCompilationJob> NewCompilationJob( ParseInfo* parse_info, FunctionLiteral* literal, Handle<Script> script, AccountingAllocator* allocator, std::vector<FunctionLiteral*>* eager_inner_literals, LocalIsolate* local_isolate)",
  "[public/method] static std::unique_ptr<UnoptimizedCompilationJob> NewSourcePositionCollectionJob(ParseInfo* parse_info, FunctionLiteral* literal, Handle<BytecodeArray> existing_bytecode, AccountingAllocator* allocator, LocalIsolate* local_isolate)",
  "[public/method] V8_EXPORT_PRIVATE Tagged<Code> GetBytecodeHandler(Bytecode bytecode, OperandScale operand_scale)",
  "[public/method] void SetBytecodeHandler(Bytecode bytecode, OperandScale operand_scale, Tagged<Code> handler)",
  "[public/method] void ForEachBytecode(const std::function<void(Bytecode, OperandScale)>& f)",
  "[public/method] void Initialize()",
  "[public/method] bool IsDispatchTableInitialized() const",
  "[public/method] Address dispatch_table_address()",
  "[public/method] Address address_of_interpreter_entry_trampoline_instruction_start() const",
  "[private/method] static size_t GetDispatchTableIndex(Bytecode bytecode, OperandScale operand_scale)",
  "[private/const] static const int kNumberOfWideVariants = BytecodeOperands::kOperandScaleCount",
  "[private/const] static const int kDispatchTableSize = kNumberOfWideVariants * (kMaxUInt8 + 1)",
  "[private/const] static const int kNumberOfBytecodes = static_cast<int>(Bytecode::kLast) + 1",
  "[private/field] Isolate* isolate_",
  "[private/field] Address dispatch_table_[kDispatchTableSize]",
  "[private/field] Address interpreter_entry_trampoline_instruction_start_",
];

// —— 提取不变式断言 ——
const _: () = {
  assert!(INTERPRETER_DECLARATIONS.len() == 20);
  assert!(BYTECODE_HANDLER_DATA_DECLARATIONS.len() == 7);
  assert!(DISPATCH_TABLE_SIZE == 768);
  assert!(NUMBER_OF_BYTECODES == 217);
  assert!(dispatch_table_index(Bytecode::Wide, OperandScale::Single) == 0);
  assert!(dispatch_table_index(Bytecode::Wide, OperandScale::Double) == 256);
};
