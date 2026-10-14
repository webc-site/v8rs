use std::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum Error {
  /// 索引越界
  #[error("索引越界: index {index} >= length {length}")]
  OutOfBounds { index: usize, length: usize },

  /// 容量不足
  #[error("容量不足: requested {requested} > capacity {capacity}")]
  CapacityExceeded { requested: usize, capacity: usize },

  /// 原版 CHECK 级不变式违约 (RightTrim 前置等)
  #[error("不变式违约: {0}")]
  Invariant(&'static str),

  /// 语义分支依赖尚未落地的子系统 (登记于各模块头注释)
  #[error("语义暂未支持: {0}")]
  Unsupported(&'static str),
}

pub type Result<T> = result::Result<T, Error>;
