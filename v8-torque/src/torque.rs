// 由 tool/src/torque/misc.js 从 v8/src/torque/torque.cc 生成; 修订就地改, 复跑覆盖
// 主入口 (命令行) 骨架

use crate::{
  error::{Error, Result},
  torque_compiler::TorqueCompilerOptions,
};

/// 原版 main/WrappedMain → run(argv)
/// argv[0] 之后为参数; 返回进程退出码 (原版 0/1)
pub fn run(argv: &[String], options: TorqueCompilerOptions) -> Result<i32> {
  // TODO(填体): 参数解析 (--output/-o, --v8-root, --language-server 等) 对齐 torque.cc
  let _ = (argv, options);
  Err(Error::Unimplemented)
}

// ── 原版 src/torque/torque.cc ──
// 主入口签名清单
// L12: std::string ErrorPrefixFor(TorqueMessage::Kind kind)
// L22: int WrappedMain(int argc, const char** argv)
// L104: int main(int argc, const char** argv)
