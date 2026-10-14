use core::result;

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum Error {
  #[error("未知或无效的栈帧类型代码: {0}")]
  InvalidFrameType(u8),

  #[error("栈帧指针越界或未对齐 (fp: {fp:#x}, sp: {sp:#x})")]
  UnalignedOrOutOfBoundsFrame { fp: usize, sp: usize },
}

pub type Result<T> = result::Result<T, Error>;
