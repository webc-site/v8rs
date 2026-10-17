//! 错误面 ≡ 字节码解码/反汇编的 DCHECK/UNREACHABLE 收敛 (regexp-bytecodes-inl.h /
//! regexp-bytecodes.cc); regexp-error.h:52 `enum class Error` 的 parser 诊断码面
//! (原 ReportFailure 通道) 与本面不相交, 登记推迟待 parser wave 落地。

use std::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  /// 非法 opcode 字节 (原版 Bytecodes::FromPtr 的 DCHECK(IsValid) 面)
  #[error("invalid regexp bytecode: {byte}")]
  InvalidBytecode { byte: u8 },
  /// 字节码区截断: offset 处剩余不足一条指令
  #[error("regexp bytecode stream truncated at offset {offset}: need {need}, have {have}")]
  Truncated {
    offset: usize,
    need: usize,
    have: usize,
  },
  /// 原版 operator<< 的枚举 UNREACHABLE 分支 (regexp-bytecodes.cc:21-52):
  /// 位模式不在枚举字母表内, Rust 侧收敛为错误而非 abort
  #[error("unknown regexp enum encoding: {kind} = {value}")]
  UnknownEnumEncoding { kind: &'static str, value: u8 },
}

pub type Result<T> = result::Result<T, Error>;
