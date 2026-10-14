// v8-objects-rest/src/error.rs

use std::result;

use thiserror::Error;

/// v8-objects-rest 统一错误类型
#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  /// 槽位或索引越界
  #[error("索引越界: index {index} >= length {length}")]
  OutOfBounds { index: usize, length: usize },

  /// 非法或未知的 FeedbackSlotKind
  #[error("非法 FeedbackSlotKind 值: {0}")]
  InvalidFeedbackSlotKind(u8),

  /// 容量超出限制
  #[error("容量不足: requested {requested} > capacity {capacity}")]
  CapacityExceeded { requested: usize, capacity: usize },

  /// 不变式违约
  #[error("不变式违约: {0}")]
  Invariant(&'static str),

  /// 尚未落地的子系统特性
  #[error("特性暂未支持: {0}")]
  Unsupported(&'static str),

  /// 底层 v8-objects-core 错误透传 (访客/实例类型等原语面)
  ///
  /// 存 Display 文本而非内核错误本身, 以保留本枚举的 PartialEq/Eq
  /// (v8_objects_core::Error 无结构相等).
  #[error("底层错误: {0}")]
  Core(String),
}

impl From<v8_objects_core::Error> for Error {
  #[inline]
  fn from(e: v8_objects_core::Error) -> Self {
    Error::Core(e.to_string())
  }
}

pub type Result<T> = result::Result<T, Error>;
