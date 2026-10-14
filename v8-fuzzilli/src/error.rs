use std::result;

use thiserror::Error;

/// Fuzzilli 错误枚举
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  /// 边索引超出最大限制
  #[error("Edge index {0} exceeds maximum capacity")]
  InvalidEdgeIndex(u32),

  /// 剩余边容量不足以分配 builtins 覆盖率
  #[error("Insufficient edges for builtins: requested {requested}, available {available}")]
  InsufficientEdges {
    /// 请求的边数
    requested: u32,
    /// 剩余可用边数
    available: u32,
  },

  /// Builtins 覆盖率重复初始化
  #[error("Builtins edge coverage already initialized")]
  BuiltinsAlreadyInitialized,

  /// 标志区域数量超出最大限制
  #[error("Too many bool flag regions: current {current}, maximum {max}")]
  TooManyRegions {
    /// 当前区域数
    current: usize,
    /// 最大允许区域数
    max: usize,
  },

  /// 无效的 Fuzzilli 操作名
  #[error("Unknown or invalid Fuzzilli operation: {0}")]
  InvalidOperation(String),

  /// 无效的崩溃类别参数
  #[error("Invalid crash argument: {0}")]
  InvalidCrashKind(i32),

  /// IO 错误
  #[error("IO error: {0}")]
  Io(String),
}

/// Fuzzilli 操作结果类型
pub type Result<T> = result::Result<T, Error>;
