// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/message-pipe.h 生成; 修订就地改, 复跑覆盖
// LS 消息管道 (stdin/stdout LSP 帧)

use crate::{
  error::{Error, Result},
  ls::json::JsonValue,
};

/// 原版 ReadMessage: 从 stdin 读 Content-Length 帧并解析
pub fn read_message() -> Result<JsonValue> {
  // TODO(填体): Content-Length 帧协议
  Err(Error::Unimplemented)
}

/// 原版 WriteMessage: 序列化并写 stdout
pub fn write_message(message: JsonValue) -> Result<()> {
  // TODO(填体)
  let _ = message;
  Err(Error::Unimplemented)
}
