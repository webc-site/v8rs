use std::result;

use thiserror::Error;

/// zone 转写错误集中定义; 原版 CHECK/Fatal 路径在此映射为 Err
#[derive(Error, Debug)]
pub enum Error {
  /// 段分配失败或请求超限 (原版 V8::FatalProcessOutOfMemory)
  #[error("zone out of memory: requested {requested} bytes")]
  OutOfMemory { requested: usize },
  /// length * sizeof(T) 溢出 (原版 CHECK_LT)
  #[error("zone allocation size overflow")]
  SizeOverflow,
  /// alignof(T) > kAlignmentInBytes (原版 static_assert)
  #[error("type alignment {alignment} exceeds zone alignment limit")]
  UnsupportedAlignment { alignment: usize },
  /// snapshot 归属了错误的 zone (原版 CHECK_NOT_NULL)
  #[error("zone snapshot restore target mismatch")]
  WrongZone,
}

pub type Result<T> = result::Result<T, Error>;
