use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("索引越界: 索引 {index}, 长度/容量 {length}")]
  OutOfBounds { index: usize, length: usize },

  #[error("容量不足或无效: 容量 {0}")]
  InvalidCapacity(usize),

  #[error("分配内存耗尽")]
  OutOfMemory,

  #[error("描述符未找到")]
  NotFound,

  #[error("哈希溢出: 值为 {0}")]
  HashOverflow(u32),

  #[error("过渡树冲突或重复过渡")]
  DuplicateTransition,

  #[error("不支持的过渡状态")]
  UnsupportedTransition,

  #[error(transparent)]
  Core(#[from] v8_objects_core::Error),

  #[error(transparent)]
  Arrays(#[from] v8_objects_arrays::Error),

  #[error(transparent)]
  Zone(#[from] v8_zone::Error),
}

pub type Result<T> = result::Result<T, Error>;
