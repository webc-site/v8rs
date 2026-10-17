//! 错误集中定义 (仓库规范: src/error.rs)

use std::result;

use thiserror::Error;

/// 本 crate 错误面: 依赖错误透明转发 + 归约管线特有缺失边输入
#[derive(Error, Debug)]
pub enum Error {
  /// 冻结图底层错误透明转发
  #[error(transparent)]
  Graph(#[from] v8_compiler_graph::Error),
  /// 算子层错误透明转发
  #[error(transparent)]
  Ops(#[from] v8_compiler_ops::Error),
  /// zone 容器错误透明转发
  #[error(transparent)]
  Zone(#[from] v8_zone::Error),
  /// ReplaceWithValue 路径缺少必要边输入 (原版 DCHECK_NOT_NULL 面)
  #[error("missing {kind} replacement for node {}", node_id)]
  MissingEdgeReplacement { node_id: u32, kind: &'static str },
  /// 图端点未初始化 (原版对空 start/end 的裸指针 UB 面显式化)
  #[error("graph {which} not initialized")]
  GraphUninitialized { which: &'static str },
}

pub type Result<T> = result::Result<T, Error>;
