//! WebAssembly Interpreter (DrumBrake) 集成测试
//! 对应原版 v8/src/wasm/interpreter/* 规范

use aok::{OK, Void};
use v8_wasm_interp::{
  BRANCH_ON_CAST_DATA_TARGET_TYPE_BIT_SIZE, CATCH_ALL_TAG_INDEX, CODE_OFFSET_SIZE,
  DELEGATE_TO_CALLER_INDEX, ExceptionHandlingResult, ExternalCallResult, FILE_CHUNKS_COUNT,
  INLINE_FNS, INSTRUCTION_TABLE_MASK, INSTRUCTION_TABLE_SIZE, INVALID_CODE_OFFSET,
  InstrHandlerSize, InstructionHandler, InterpreterState, K_REF_ALIGN_MASK, K_SLOT_SIZE,
  MAX_FILE_SIZE, MAX_PERCENT_VALUE, OPERATOR_MODE_COUNT, OperatorMode, REG_MODES, RegMode,
  SLOT_SIZE, SlotValueKind, WRITE_COUNT_CHECK_INTERVAL, js_to_wasm_wrapper_packed_array_size,
  stack_limits, value_size_in_slots,
};

#[ctor::ctor(unsafe)]
fn _log_init() {
  log_init::init();
}

#[test]
fn test_stack_limits_and_constants() -> Void {
  assert_eq!(stack_limits::INITIAL_STACK_SIZE, 1024 * 1024);
  assert_eq!(stack_limits::STACK_SIZE_INCREMENT, 1024 * 1024);
  assert_eq!(stack_limits::MAX_STACK_SIZE, 32 * 1024 * 1024);
  assert_eq!(stack_limits::INTERPRETER_STACK_GUARD_SIZE, 1024 * 1024);
  assert_eq!(
    stack_limits::INTERPRETER_STACK_RESERVATION_SIZE,
    stack_limits::MAX_STACK_SIZE + stack_limits::INTERPRETER_STACK_GUARD_SIZE
  );

  assert_eq!(CATCH_ALL_TAG_INDEX, -1);
  assert_eq!(DELEGATE_TO_CALLER_INDEX, 0);
  assert_eq!(MAX_PERCENT_VALUE, 100_000);
  assert_eq!(OPERATOR_MODE_COUNT, 4);
  assert_eq!(SLOT_SIZE, 4);
  assert_eq!(CODE_OFFSET_SIZE, 4);
  assert_eq!(INSTRUCTION_TABLE_SIZE, 4096);
  assert_eq!(INSTRUCTION_TABLE_MASK, 4095);
  assert_eq!(BRANCH_ON_CAST_DATA_TARGET_TYPE_BIT_SIZE, 30);
  assert_eq!(INVALID_CODE_OFFSET, u32::MAX);
  assert_eq!(WRITE_COUNT_CHECK_INTERVAL, 1000);
  assert_eq!(FILE_CHUNKS_COUNT, 10);
  assert_eq!(MAX_FILE_SIZE, 100 * 1024 * 1024);
  assert_eq!(K_REF_ALIGN_MASK, 0x04);
  assert_eq!(K_SLOT_SIZE, 4);
  assert_eq!(REG_MODES.len(), 256);
  assert_eq!(INLINE_FNS.len(), 57);
  OK
}

#[test]
fn test_interpreter_enums() -> Void {
  // InterpreterState
  assert_eq!(InterpreterState::Stopped as u8, 0);
  assert_eq!(InterpreterState::Running as u8, 1);
  assert_eq!(InterpreterState::Finished as u8, 2);
  assert_eq!(InterpreterState::Trapped as u8, 3);
  assert_eq!(InterpreterState::EhUnwinding as u8, 4);

  // ExceptionHandlingResult
  assert_eq!(ExceptionHandlingResult::Handled as u8, 0);
  assert_eq!(ExceptionHandlingResult::Unwound as u8, 1);

  // ExternalCallResult
  assert_eq!(ExternalCallResult::ExternalReturned as u8, 0);
  assert_eq!(ExternalCallResult::ExternalException as u8, 1);

  // OperatorMode
  assert_eq!(OperatorMode::R2R as u8, 0);
  assert_eq!(OperatorMode::R2S as u8, 1);
  assert_eq!(OperatorMode::S2R as u8, 2);
  assert_eq!(OperatorMode::S2S as u8, 3);

  // RegMode
  assert_eq!(RegMode::NoReg as u8, 0);
  assert_eq!(RegMode::I32Reg as u8, 1);
  assert_eq!(RegMode::I64Reg as u8, 2);
  assert_eq!(RegMode::F32Reg as u8, 3);
  assert_eq!(RegMode::F64Reg as u8, 4);
  assert_eq!(RegMode::AnyReg as u8, 5);

  // SlotValueKind
  assert_eq!(SlotValueKind::I32 as u8, 0);
  assert_eq!(SlotValueKind::I64 as u8, 1);
  assert_eq!(SlotValueKind::F32 as u8, 2);
  assert_eq!(SlotValueKind::F64 as u8, 3);
  assert_eq!(SlotValueKind::S128 as u8, 4);
  assert_eq!(SlotValueKind::Ref as u8, 5);

  // InstrHandlerSize
  assert_eq!(InstrHandlerSize::Large as u8, 0);
  assert_eq!(InstrHandlerSize::Small as u8, 1);
  OK
}

#[test]
fn test_slot_sizes_and_wrapper_packing() -> Void {
  assert_eq!(value_size_in_slots(SlotValueKind::I32), 1);
  assert_eq!(value_size_in_slots(SlotValueKind::I64), 2);
  assert_eq!(value_size_in_slots(SlotValueKind::F32), 1);
  assert_eq!(value_size_in_slots(SlotValueKind::F64), 2);
  assert_eq!(value_size_in_slots(SlotValueKind::S128), 4);
  assert_eq!(value_size_in_slots(SlotValueKind::Ref), 2);

  // 1. [i32, ref]: i32 占用 4B, ref 补齐到 8B 边界 (+4B) 写入 8B, 总计 16B
  let size1 = js_to_wasm_wrapper_packed_array_size(
    [SlotValueKind::I32, SlotValueKind::Ref].into_iter(),
    [].into_iter(),
  );
  assert_eq!(size1, 16);

  // 2. [i32] vs [f64]: args 4B, rets 8B, 较大者为 8B, 已经是 8 字节对齐
  let size2 = js_to_wasm_wrapper_packed_array_size(
    [SlotValueKind::I32].into_iter(),
    [SlotValueKind::F64].into_iter(),
  );
  assert_eq!(size2, 8);

  // 3. [s128] vs [i32, i64]: args 16B, rets 4+8=12B, 较大者 16B 对齐后 16B
  let size3 = js_to_wasm_wrapper_packed_array_size(
    [SlotValueKind::S128].into_iter(),
    [SlotValueKind::I32, SlotValueKind::I64].into_iter(),
  );
  assert_eq!(size3, 16);
  OK
}

#[test]
fn test_instruction_handler_decoding() -> Void {
  assert_eq!(InstructionHandler::COUNT, 1690);

  // 首尾项与中间项
  let first = InstructionHandler::from_repr(0);
  assert_eq!(first, Some(InstructionHandler::R2rI32LoadMem8S));

  let last = InstructionHandler::from_repr(1689);
  assert_eq!(last, Some(InstructionHandler::S2sI64AtomicStore32U));

  // 越界哨兵
  assert_eq!(InstructionHandler::from_repr(1690), None);
  assert_eq!(InstructionHandler::from_repr(u16::MAX), None);
  OK
}
