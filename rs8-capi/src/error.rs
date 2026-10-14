// rs8-capi/src/error.rs
//! rs8-capi 错误类型定义

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("C API 句柄空指针")]
  NullHandle,

  #[error("C API 参数无效: {0}")]
  InvalidArgument(&'static str),

  #[error("C API 引擎操作失败")]
  OperationFailed,
}
