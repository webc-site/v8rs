use std::result;

use thiserror::Error;

/// Profiler 错误枚举
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
  /// 达到并发 Profile 上限 (100)
  #[error("Maximum number of simultaneous profiles reached (max: {0})")]
  MaxSimultaneousProfilesExceeded(i32),

  /// 未找到指定的 Profile 节点或对象
  #[error("Profile item not found: {0}")]
  NotFound(String),

  /// 无效的操作状态或采样缓冲区满
  #[error("Invalid sampler state or buffer full: {0}")]
  InvalidState(String),

  /// IO 错误
  #[error("IO error: {0}")]
  Io(String),
}

/// Profiler 操作结果类型
pub type Result<T> = result::Result<T, Error>;
