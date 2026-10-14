use std::result;

use thiserror::Error;
use v8_common::VariableMode;

#[derive(Error, Debug)]
pub enum Error {
  /// 同作用域词法重复声明 (原版 DeclareVariableName 返回 nullptr 的路径)
  #[error("重复声明 `{name}`: {mode:?} 与已有 {existing_mode:?} 冲突")]
  Redeclaration {
    name: String,
    mode: VariableMode,
    existing_mode: VariableMode,
  },
  /// zone 分配失败透明转发
  #[error(transparent)]
  Zone(#[from] v8_zone::Error),
  /// zone 分配路径在无 zone 后端的工厂上调用
  #[error("工厂未挂接 zone 后端 (with_zone 构造)")]
  NoZoneBackend,
}

pub type Result<T> = result::Result<T, Error>;
