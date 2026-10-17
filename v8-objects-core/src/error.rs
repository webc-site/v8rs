use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("非法实例类型: {0}")]
  InvalidInstanceType(u16),

  #[error("Setter 副作用类型非法: 不能为 HasNoSideEffect")]
  InvalidSetterSideEffectType,

  /// 语义分支依赖尚未落地的子系统 (登记于各模块头注释)
  #[error("语义暂未支持: {0}")]
  Unsupported(&'static str),

  #[error(transparent)]
  Zone(#[from] v8_zone::Error),
}

pub type Result<T> = result::Result<T, Error>;
