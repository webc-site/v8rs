use std::result;

use thiserror::Error;

/// v8-handles 错误集
#[derive(Error, Debug)]
pub enum Error {
  /// 可空句柄为空, 无法转换为 Handle (原 MaybeHandle::ToHandleChecked 的 CHECK)
  #[error("MaybeHandle 为空, 无法转换为 Handle")]
  NullHandle,
  /// 共享对象 id 越界 (原 SharedObjectConveyorHandles::GetPersisted 的 DCHECK)
  #[error("共享对象 id 越界: {0}")]
  InvalidObjectId(u32),
}

pub type Result<T> = result::Result<T, Error>;
