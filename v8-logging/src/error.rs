//! 统一错误类型(禁 panic;所有可失败操作返回 `crate::Result`)。

use std::{io, result};

/// v8-logging 的错误集合。
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// 打开/写入日志文件失败(对应 C++ 中 `base::OS::FOpen`/`OpenTemporaryFile` 失败路径)。
  #[error("io error: {0}")]
  Io(#[from] io::Error),

  /// 输出目标被拒绝或不可用(如临时文件编号耗尽)。
  #[error("output unavailable: {0}")]
  OutputUnavailable(String),
}

pub type Result<T> = result::Result<T, Error>;
