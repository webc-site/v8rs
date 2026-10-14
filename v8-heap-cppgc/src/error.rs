use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  /// v8: src/heap/cppgc-internal/gc-info-table.cc:150 CHECK_LT(new_index, GCInfoTable::kMaxIndex)
  /// — 原版致命臂 (Oilpan: GCInfoTable overflow.), Rust 侧禁 panic 改强类型错误。
  #[error("Oilpan: GCInfoTable overflow.")]
  GcInfoTableOverflow,
}

pub type Result<T> = result::Result<T, Error>;
