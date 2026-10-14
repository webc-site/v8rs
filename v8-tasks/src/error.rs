use std::result;

use thiserror::Error;

/// crate 级错误；对应原版 `CHECK` 不可达分支的可错误化改写。
#[derive(Error, Debug)]
pub enum Error {
  /// 对应原 `CHECK_NE(kInvalidTaskId, id)`：以 0 号 id 调用 `TryAbort`。
  #[error("invalid task id (kInvalidTaskId)")]
  InvalidTaskId,
  /// 对应原 `CHECK_NE(kInvalidTaskId, id)`（id 计数器回绕）：不支持 id 溢出。
  #[error("task id counter overflow")]
  TaskIdOverflow,
}

pub type Result<T> = result::Result<T, Error>;
