// v8-compiler-graph 错误面 — 原版 CHECK/FATAL/UNREACHABLE 的 Result 映射
use std::result;

use thiserror::Error;

use crate::{node::NodeId, opcodes::IrOpcode};

#[derive(Error, Debug)]
pub enum Error {
  /// 原版 Node::New FATAL: 输入为 null (Rust 侧仅 Clone/Insert 族内部路径可触)
  #[error("节点 #{id} {mnemonic}[{index}] 输入为 null")]
  NullInput {
    id: NodeId,
    mnemonic: &'static str,
    index: usize,
  },

  /// 原版 CHECK(IdField::is_valid(id)) — id 超 24 位域
  #[error("节点 id {id} 超出 24 位域 (2^24-1)")]
  NodeIdOverflow { id: NodeId },

  /// 原版 CHECK_IMPLIES(InputIndexField::is_valid) — 输入数超 31 位域
  #[error("输入数 {count} 超出 Use::InputIndexField 31 位域")]
  InputTooMany { count: usize },

  /// 原版 operator.cc CheckRange — 输入/输出计数越界
  #[error("operator 计数越界: {what}={val}")]
  CountOutOfRange { what: &'static str, val: u64 },

  /// opcode 超 IrOpcode 域 (原版 DCHECK; Rust 侧 Operator::new 即校验)
  #[error("opcode {opcode} 越界 (> LAST={last})")]
  OpcodeOutOfRange { opcode: u16, last: u16 },

  /// 原版 node-properties CHECK_LT — 输入索引越界
  #[error("{what} 输入索引 {index} 越界 (count={count})")]
  InputIndexOutOfRange {
    index: usize,
    count: usize,
    what: &'static str,
  },

  /// 原 CHECK(HasContextInput) — 非 JS opcode 无 context 输入
  #[error("opcode {opcode:?} 无 context 输入")]
  NoContextInput { opcode: IrOpcode },

  /// 原 CHECK(HasFrameStateInput)
  #[error("opcode {opcode:?} 无 FrameState 输入")]
  NoFrameStateInput { opcode: IrOpcode },

  /// 原 CHECK_GT(ControlOutputCount, 0)
  #[error("opcode {opcode:?} 无控制输出")]
  NoControlOutput { opcode: IrOpcode },

  /// 参数化分支依赖 v8-compiler-ops 层 (operator 参数 / Runtime 表)
  #[error("依赖 operator 参数层, 推迟实现: {what}")]
  NeedsOpParam { what: &'static str },

  /// 原版 UNREACHABLE() — 输入落入理论不可达分支
  #[error("不可达分支: {what}")]
  Unreachable { what: &'static str },

  /// 原 Truncation::Generalize FATAL — 不相容截断组合
  #[error("Tried to combine incompatible truncations")]
  IncompatibleTruncation,

  /// v8-zone 分配失败透传
  #[error(transparent)]
  Zone(#[from] v8_zone::Error),
}

pub type Result<T> = result::Result<T, Error>;
