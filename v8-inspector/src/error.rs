// Copyright 2026 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

//! 强类型错误系统：v8-inspector 域错误定义

use std::{error, fmt, result};

/// Inspector 运行时与协议错误枚举
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
  /// 无效的 RemoteObjectId 格式
  InvalidRemoteObjectId(String),
  /// 无效的 RemoteCallFrameId 格式
  InvalidCallFrameId(String),
  /// 无效的 DebuggerId 格式
  InvalidDebuggerId(String),
  /// 无效的断点类型代码
  InvalidBreakpointType(u8),
  /// 无效的 ConsoleAPIType
  InvalidConsoleAPIType(String),
  /// 无效的 V8MessageOrigin
  InvalidMessageOrigin(String),
  /// 无效的内部值类型
  InvalidValueType(String),
  /// 无效的丢弃原因
  InvalidDiscardReason(String),
  /// 序列化错误
  SerializationError(String),
  /// 反序列化错误
  DeserializationError(String),
  /// 目标实体未找到
  NotFound(String),
  /// 参数非法
  InvalidParameter(String),
  /// 深度超限
  MaxDepthExceeded(i32),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::InvalidRemoteObjectId(msg) => write!(f, "Invalid remote object id: {msg}"),
      Self::InvalidCallFrameId(msg) => write!(f, "Invalid call frame id: {msg}"),
      Self::InvalidDebuggerId(msg) => write!(f, "Invalid debugger id: {msg}"),
      Self::InvalidBreakpointType(tag) => write!(f, "Invalid breakpoint type tag: {tag}"),
      Self::InvalidConsoleAPIType(name) => write!(f, "Invalid console API type: {name}"),
      Self::InvalidMessageOrigin(orig) => write!(f, "Invalid message origin: {orig}"),
      Self::InvalidValueType(val) => write!(f, "Invalid internal value type: {val}"),
      Self::InvalidDiscardReason(r) => write!(f, "Invalid discard reason: {r}"),
      Self::SerializationError(err) => write!(f, "Serialization error: {err}"),
      Self::DeserializationError(err) => write!(f, "Deserialization error: {err}"),
      Self::NotFound(name) => write!(f, "Entity not found: {name}"),
      Self::InvalidParameter(param) => write!(f, "Invalid parameter: {param}"),
      Self::MaxDepthExceeded(depth) => write!(f, "Max custom preview depth exceeded: {depth}"),
    }
  }
}

impl error::Error for Error {}

/// 统一结果类型别名
pub type Result<T> = result::Result<T, Error>;
