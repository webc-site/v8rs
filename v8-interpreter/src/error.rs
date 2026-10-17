use std::result;

use thiserror::Error;

/// 解释器错误定义
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  #[error("invalid bytecode flag: {0}")]
  InvalidFlag(u8),

  #[error("invalid operand count: expected {expected}, got {got}")]
  InvalidOperandCount { expected: usize, got: usize },

  #[error("operand index {index} out of range (count {count})")]
  OperandIndexOutOfRange { index: usize, count: usize },

  #[error("invalid register count: expected at most {expected}, got {got}")]
  InvalidRegisterCount { expected: usize, got: usize },

  #[error("register list is empty")]
  EmptyRegisterList,

  #[error("unexpected end of bytecode input")]
  UnexpectedEndOfInput,

  #[error("invalid jump target offset {0}")]
  InvalidJumpTarget(isize),

  #[error("invalid raw bytecode: {0}")]
  InvalidBytecode(u8),

  #[error("invalid bytecode offset: {0}")]
  InvalidBytecodeOffset(usize),

  #[error("unsupported AST node: {0}")]
  UnsupportedAstNode(&'static str),

  #[error("register index out of bounds: {0}")]
  RegisterOutOfBounds(i32),

  #[error("constant pool index out of bounds: {0}")]
  ConstantPoolOutOfBounds(usize),

  /// 名字槽位 (LdaGlobal/StaGlobal/Get|SetNamedProperty/DefineNamedOwnPropertyInLiteral) 的
  /// 池索引有效但落到的不是 `String` 变体 —— 与越界是两类缺陷, 混报会误导排查 (#26)
  #[error("constant pool entry {idx} is not a name slot candidate (got {kind})")]
  ConstantPoolTypeMismatch { idx: usize, kind: &'static str },

  #[error("unbound label")]
  UnboundLabel,

  #[error("execution error: {0}")]
  ExecutionError(String),

  #[error(transparent)]
  Core(#[from] v8_codegen_core::Error),
}

pub type Result<T> = result::Result<T, Error>;
