// 由 tool/src/wasm/interpreter/wasm-interpreter.h.js 从 v8/src/wasm/interpreter/wasm-interpreter.h 生成; 修订就地改, 复跑覆盖
// Wasm 解释器 (Drumbrake) 常量与寄存器优化表

/// 栈布局常量 (原版 WasmInterpreterThread 侧)
pub mod stack_limits {
  pub const INITIAL_STACK_SIZE: u32 = 1048576;
  pub const STACK_SIZE_INCREMENT: u32 = 1048576;
  pub const MAX_STACK_SIZE: u32 = 33554432;
  /// 尾部守卫区宽 = 增量 (原版 kInterpreterStackGuardSize)
  pub const INTERPRETER_STACK_GUARD_SIZE: u32 = 1048576;
  /// 预留总宽 = 上限 + 守卫 (原版 kInterpreterStackReservationSize)
  pub const INTERPRETER_STACK_RESERVATION_SIZE: u32 = 34603008;
}

/// 解释器线程状态 (对应原版 WasmInterpreterThread::State)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InterpreterState {
  /// STOPPED
  Stopped = 0,
  /// RUNNING
  Running = 1,
  /// FINISHED
  Finished = 2,
  /// TRAPPED
  Trapped = 3,
  /// EH_UNWINDING
  EhUnwinding = 4,
}

/// 异常处理结果 (对应原版 ExceptionHandlingResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExceptionHandlingResult {
  /// HANDLED
  Handled = 0,
  /// UNWOUND
  Unwound = 1,
}

/// 外部调用结果 (对应原版 ExternalCallResult)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExternalCallResult {
  /// EXTERNAL_RETURNED — 正常返回
  ExternalReturned = 0,
  /// EXTERNAL_EXCEPTION — 抛出异常
  ExternalException = 1,
}

/// catch-all tag 索引哨兵 (原版 WasmEHData::kCatchAllTagIndex)
pub const CATCH_ALL_TAG_INDEX: i32 = -1;

/// 委托给调用者的块索引哨兵 (原版 kDelegateToCallerIndex; 0 恒为主块 id)
pub const DELEGATE_TO_CALLER_INDEX: i32 = 0;

/// 覆盖率采样百分比上限 (原版 kMaxPercentValue)
pub const MAX_PERCENT_VALUE: i32 = 100000;

/// 操作数模式 (对应原版 OperatorMode: 寄存器/栈顶来源组合)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OperatorMode {
  /// kR2R
  R2R = 0,
  /// kR2S
  R2S = 1,
  /// kS2R
  S2R = 2,
  /// kS2S
  S2S = 3,
}
/// 模式总数 (原版 kOperatorModeCount)
pub const OPERATOR_MODE_COUNT: usize = 4;

/// 寄存器优化模式 (对应原版 enum class RegMode)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RegMode {
  /// kNoReg
  NoReg = 0,
  /// kI32Reg
  I32Reg = 1,
  /// kI64Reg
  I64Reg = 2,
  /// kF32Reg
  F32Reg = 3,
  /// kF64Reg
  F64Reg = 4,
  /// kAnyReg
  AnyReg = 5,
}

/// 寄存器模式变换对 (原版 RegModeTransform: from/to)
pub type RegModeTransform = (RegMode, RegMode);

/// 逐 opcode 寄存器优化表 (原版 kRegModes[256], 索引即 wasm opcode)
#[rustfmt::skip]
pub static REG_MODES: [RegModeTransform; 256] = [
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::I32Reg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::I32Reg, RegMode::NoReg), (RegMode::I32Reg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::AnyReg, RegMode::NoReg), (RegMode::I32Reg, RegMode::AnyReg),
  (RegMode::I32Reg, RegMode::AnyReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::AnyReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::AnyReg),
  (RegMode::AnyReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::F32Reg), (RegMode::I32Reg, RegMode::F64Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::I64Reg),
  (RegMode::I32Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::NoReg), (RegMode::I64Reg, RegMode::NoReg),
  (RegMode::F32Reg, RegMode::NoReg), (RegMode::F64Reg, RegMode::NoReg), (RegMode::I32Reg, RegMode::NoReg), (RegMode::I32Reg, RegMode::NoReg),
  (RegMode::I64Reg, RegMode::NoReg), (RegMode::I64Reg, RegMode::NoReg), (RegMode::I64Reg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg),
  (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg),
  (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::F32Reg, RegMode::I32Reg),
  (RegMode::F32Reg, RegMode::I32Reg), (RegMode::F32Reg, RegMode::I32Reg), (RegMode::F32Reg, RegMode::I32Reg), (RegMode::F32Reg, RegMode::I32Reg),
  (RegMode::F32Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I32Reg),
  (RegMode::F64Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I32Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg), (RegMode::I64Reg, RegMode::I32Reg),
  (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg),
  (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg),
  (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg),
  (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::I64Reg, RegMode::I64Reg), (RegMode::F32Reg, RegMode::F32Reg),
  (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg),
  (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg),
  (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F32Reg, RegMode::F32Reg),
  (RegMode::F32Reg, RegMode::F32Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg),
  (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg),
  (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg),
  (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::F64Reg, RegMode::F64Reg), (RegMode::I64Reg, RegMode::I32Reg),
  (RegMode::F32Reg, RegMode::I32Reg), (RegMode::F32Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I32Reg),
  (RegMode::I32Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::I64Reg), (RegMode::F32Reg, RegMode::I64Reg), (RegMode::F32Reg, RegMode::I64Reg),
  (RegMode::F64Reg, RegMode::I64Reg), (RegMode::F64Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::F32Reg), (RegMode::I32Reg, RegMode::F32Reg),
  (RegMode::I64Reg, RegMode::F32Reg), (RegMode::I64Reg, RegMode::F32Reg), (RegMode::F64Reg, RegMode::F32Reg), (RegMode::I32Reg, RegMode::F64Reg),
  (RegMode::I32Reg, RegMode::F64Reg), (RegMode::I64Reg, RegMode::F64Reg), (RegMode::I64Reg, RegMode::F64Reg), (RegMode::F32Reg, RegMode::F64Reg),
  (RegMode::F32Reg, RegMode::I32Reg), (RegMode::F64Reg, RegMode::I64Reg), (RegMode::I32Reg, RegMode::F32Reg), (RegMode::I64Reg, RegMode::F64Reg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
  (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg), (RegMode::NoReg, RegMode::NoReg),
];

/// 栈槽宽 (原版 kSlotSize = sizeof(int32_t))
pub const SLOT_SIZE: usize = 4;

/// 代码偏移宽 (原版 kCodeOffsetSize = sizeof(int32_t))
pub const CODE_OFFSET_SIZE: usize = 4;

/// 指令表大小 (原版 kInstructionTableSize)
pub const INSTRUCTION_TABLE_SIZE: u32 = 4096;

/// 指令表掩码 (原版 kInstructionTableMask)
pub const INSTRUCTION_TABLE_MASK: u32 = 4095;

/// cast data 目标类型位宽 (原版 kBranchOnCastDataTargetTypeBitSize)
pub const BRANCH_ON_CAST_DATA_TARGET_TYPE_BIT_SIZE: u32 = 30;

/// 无效代码偏移哨兵 (原版 kInvalidCodeOffset = (CodeOffset)-1)
pub const INVALID_CODE_OFFSET: u32 = u32::MAX;

/// tracing 落盘写次数检查间隔 (原版 kWriteCountCheckInterval)
pub const WRITE_COUNT_CHECK_INTERVAL: i64 = 1000;

/// tracing 文件块数 (原版 kFileChunksCount)
pub const FILE_CHUNKS_COUNT: i32 = 10;

/// tracing 单文件上限 (原版 kMaxFileSize)
pub const MAX_FILE_SIZE: i64 = 100 * 1024 * 1024;

// —— 提取不变式断言 ——
const _: () = {
  assert!(stack_limits::MAX_STACK_SIZE == 32 * 1024 * 1024);
  assert!(stack_limits::INTERPRETER_STACK_RESERVATION_SIZE > stack_limits::MAX_STACK_SIZE);
  assert!(REG_MODES.len() == 256);
  assert!(INSTRUCTION_TABLE_MASK == 4095);
  assert!(OPERATOR_MODE_COUNT == 4);
};
