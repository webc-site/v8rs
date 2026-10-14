// v8-objects-js-async/src/error.rs
// 统一错误定义

use std::result;

use thiserror::Error;

/// v8-objects-js-async 模块错误枚举
#[derive(Error, Debug)]
pub enum Error {
  /// 索引越界
  #[error("索引越界: index {index}, length {len}")]
  IndexOutOfBounds { index: usize, len: usize },

  /// 无效容量
  #[error("无效容量: {0}")]
  InvalidCapacity(usize),

  /// 无效 Promise 状态码
  #[error("无效 Promise 状态码: {0}")]
  InvalidPromiseState(u32),

  /// 无效 Generator 恢复模式
  #[error("无效 Generator 恢复模式: {0}")]
  InvalidResumeMode(i32),

  /// Promise 非处于 Pending 状态
  #[error("Promise 已处于结算状态，非 Pending")]
  PromiseNotPending,

  /// Promise 仍在 Pending 状态
  #[error("Promise 仍在 Pending 状态，无法提取结算结果")]
  PromiseStillPending,

  /// 原版不变式违约
  #[error("不变式违约: {0}")]
  Invariant(&'static str),

  /// ECMA402 DefaultNumberOption/GetNumberOption: 属性值越界或 NaN
  /// (原版 MessageTemplate::kPropertyValueOutOfRange, option-utils.cc)
  #[error("属性值超出范围: {0}")]
  PropertyValueOutOfRange(&'static str),

  /// ECMA402 GetOption: 字符串值不在枚举表中
  /// (原版 MessageTemplate::kValueOutOfRange, option-utils.h)
  #[error("选项值超出枚举范围: {0}")]
  ValueOutOfRange(&'static str),

  /// GetOptionsObject: options 非 undefined 且非 Object
  /// (原版 MessageTemplate::kInvalidArgument + TypeError, option-utils.cc)
  #[error("非法参数 (options 需为 Object)")]
  InvalidArgumentType,

  /// 结构化克隆头部版本高于 kLatestVersion
  /// (原版 MessageTemplate::kDataCloneDeserializationVersionError, value-serializer.cc:1429)
  #[error("DataClone 反序列化版本错误")]
  DataCloneVersionError,

  /// ValueSerializer 缓冲区扩容失败 (原版 out_of_memory_ + Nothing 路径)
  #[error("序列化缓冲区内存不足")]
  SerializerOutOfMemory,

  /// 核心对象库错误透明转发
  #[error(transparent)]
  Core(#[from] v8_objects_core::Error),

  /// JS 核心对象库错误透明转发
  #[error(transparent)]
  JsCore(#[from] v8_objects_js_core::Error),

  /// 数组对象库错误透明转发
  #[error(transparent)]
  Arrays(#[from] v8_objects_arrays::Error),

  /// JS 函数对象库错误透明转发
  #[error(transparent)]
  JsFunc(#[from] v8_objects_js_func::Error),
}

pub type Result<T> = result::Result<T, Error>;
