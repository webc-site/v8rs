//! crate 错误集中定义

use std::{fmt, result};

use thiserror::Error;

pub use crate::regexp_error_generated::RegexpError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum Error {
  /// 解析失败: 错误类别 + 检出位置 (原版 CompileData.error / error_pos)
  #[error("regexp parse failed at {pos}: {kind}")]
  Parse {
    kind: RegexpError,
    /// 错误检出的 0-based 位置
    pos: usize,
  },
}

impl Error {
  /// 解析错误构造 (原版 ReportError)
  #[inline]
  #[must_use]
  pub const fn parse(kind: RegexpError, pos: usize) -> Self {
    Self::Parse { kind, pos }
  }

  /// 错误类别
  #[inline]
  #[must_use]
  pub const fn kind(&self) -> RegexpError {
    match *self {
      Self::Parse { kind, .. } => kind,
    }
  }

  /// 检出位置
  #[inline]
  #[must_use]
  pub const fn pos(&self) -> usize {
    match *self {
      Self::Parse { pos, .. } => pos,
    }
  }
}

/// 错误消息透传 (原版 ErrorString)
impl fmt::Display for RegexpError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(self.message())
  }
}
