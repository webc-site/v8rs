// 由 tool/src/torque/ls.js 从 v8/src/torque/ls/torque-language-server.cc 生成; 修订就地改, 复跑覆盖
// LS 主循环

use crate::error::{Error, Result};

/// 原版 RunLanguageServer: 读入文件清单 + 消息循环
pub fn run_language_server(files: Vec<String>) -> Result<i32> {
  // TODO(填体): Logger::Enable / CompilerHandler 循环
  let _ = files;
  Err(Error::Unimplemented)
}
// ── 原版 src/torque/ls/torque-language-server.cc ──
// L20: int WrappedMain(int argc, const char** argv)
// L50: int main(int argc, const char** argv)
