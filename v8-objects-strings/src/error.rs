use std::result;

use thiserror::Error;

/// v8-objects-strings 错误族
#[derive(Error, Debug)]
pub enum Error {
  /// 字符串未展平 (Flatten 依赖 factory/heap, 归 v8-heap-*; 调用方需先展平)
  #[error("字符串未展平: {0}")]
  NotFlat(&'static str),
  /// uncached external string 的资源协议 (resource->data()/cached_data()) 依赖
  /// v8-api/sandbox 外部指针表, 未在本层落地
  #[error("uncached external string 资源访问未落地")]
  UncachedExternalUnsupported,
  /// 类型/派发不匹配 (cast 失败 / 非法字符串派发)
  #[error("字符串对象派发失败: {0}")]
  InvalidDispatch(&'static str),
  /// 其余未承接语义 (跨 crate 推迟项)
  #[error("未承接: {0}")]
  Unsupported(&'static str),
}

/// 双参形态 `Result<T, E>` 亦兼容 (周边槽位既有用法)。
pub type Result<T, E = Error> = result::Result<T, E>;
