#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod error;
mod instruction_handlers_generated;
mod wasm_interpreter_consts;
mod wasm_interpreter_inl;

pub use error::{Error, Result};
pub use instruction_handlers_generated::{InstrHandlerSize, InstructionHandler};
pub use wasm_interpreter_consts::{
  BRANCH_ON_CAST_DATA_TARGET_TYPE_BIT_SIZE, CATCH_ALL_TAG_INDEX, CODE_OFFSET_SIZE,
  DELEGATE_TO_CALLER_INDEX, ExceptionHandlingResult, ExternalCallResult, FILE_CHUNKS_COUNT,
  INSTRUCTION_TABLE_MASK, INSTRUCTION_TABLE_SIZE, INVALID_CODE_OFFSET, InterpreterState,
  MAX_FILE_SIZE, MAX_PERCENT_VALUE, OPERATOR_MODE_COUNT, OperatorMode, REG_MODES, RegMode,
  RegModeTransform, SLOT_SIZE, WRITE_COUNT_CHECK_INTERVAL, stack_limits,
};
pub use wasm_interpreter_inl::{
  INLINE_FNS, K_REF_ALIGN_MASK, K_SLOT_SIZE, SlotValueKind, js_to_wasm_wrapper_packed_array_size,
  value_size_in_slots,
};
