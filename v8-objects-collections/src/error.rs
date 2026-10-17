use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  /// 索引越界
  #[error("索引越界: index {index} >= length {length}")]
  OutOfBounds { index: usize, length: usize },

  /// 容量无效或超出限制
  #[error("容量不足或无效: {0}")]
  InvalidCapacity(usize),

  /// 条目无效或越界
  #[error("条目无效或未找到: {0}")]
  InvalidEntry(usize),

  /// 哈希表已满 (无法进一步容纳新元素)
  #[error("哈希表已满")]
  TableFull,

  /// 不变式违约
  #[error("不变式违约: {0}")]
  Invariant(&'static str),

  /// 内存分配失败
  #[error("内存分配失败")]
  AllocationFailed,

  /// 数组操作错误透传
  #[error(transparent)]
  Arrays(#[from] v8_objects_arrays::Error),
}

pub type Result<T> = result::Result<T, Error>;
