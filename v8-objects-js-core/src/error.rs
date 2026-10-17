use std::result;

use thiserror::Error;

/// JS 对象相关操作的错误类型
#[derive(Error, Debug)]
pub enum Error {
  /// 索引越界
  #[error("索引越界: 下标 {index}, 长度 {len}")]
  IndexOutOfBounds { index: usize, len: usize },

  /// 无效容量
  #[error("无效容量: {0}")]
  InvalidCapacity(usize),

  /// 非快属性模式对象
  #[error("对象非快属性模式")]
  NotFastObject,

  /// 未找到属性
  #[error("未找到属性")]
  PropertyNotFound,

  /// 无效参数对象
  #[error("无效参数对象")]
  InvalidArguments,

  /// 无效属性表示
  #[error("无效属性表示")]
  InvalidRepresentation,

  /// 核心对象错误
  #[error(transparent)]
  ObjectsCore(#[from] v8_objects_core::Error),

  /// 映射字典错误
  #[error(transparent)]
  ObjectsMaps(#[from] v8_objects_maps::Error),

  /// 数组对象错误
  #[error(transparent)]
  ObjectsArrays(#[from] v8_objects_arrays::Error),
}

pub type Result<T> = result::Result<T, Error>;
