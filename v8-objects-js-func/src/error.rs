// v8-objects-js-func/src/error.rs
// 对应原版错误定义

use std::result;

use thiserror::Error;

/// JSFunction 族错误枚举
#[derive(Error, Debug)]
pub enum Error {
  /// 索引越界
  #[error("Index out of bounds: index {index}, length {len}")]
  IndexOutOfBounds { index: usize, len: usize },

  /// 无效容量
  #[error("Invalid capacity: {0}")]
  InvalidCapacity(usize),

  /// 上下文在链中未找到
  #[error("Context not found in hierarchy")]
  ContextNotFound,

  /// 没有 prototype 槽位
  #[error("Object has no prototype slot")]
  NoPrototypeSlot,

  /// 核心对象库错误透明转发
  #[error(transparent)]
  Core(#[from] v8_objects_core::Error),

  /// JS 核心对象库错误透明转发
  #[error(transparent)]
  JsCore(#[from] v8_objects_js_core::Error),

  /// 数组对象库错误透明转发
  #[error(transparent)]
  Arrays(#[from] v8_objects_arrays::Error),
}

pub type Result<T> = result::Result<T, Error>;
