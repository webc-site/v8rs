//! 错误体系定义 (v8-compiler-ops)

use core::result;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error(transparent)]
  Graph(#[from] v8_compiler_graph::Error),

  #[error(transparent)]
  Zone(#[from] v8_zone::Error),

  #[error("节点 #{node_id} ({mnemonic}) 输入边数量不匹配: 期望 {expected}, 实际 {actual}")]
  InputCountMismatch {
    node_id: u32,
    expected: usize,
    actual: usize,
    mnemonic: &'static str,
  },

  #[error(
    "节点 #{node_id} ({mnemonic}) 无 Control 输出，但被用户节点 #{user_id} 用作 Control 输入"
  )]
  InvalidControlUse {
    node_id: u32,
    user_id: u32,
    mnemonic: &'static str,
  },

  #[error("节点 #{node_id} ({mnemonic}) 无 Effect 输出，但被用户节点 #{user_id} 用作 Effect 输入")]
  InvalidEffectUse {
    node_id: u32,
    user_id: u32,
    mnemonic: &'static str,
  },

  #[error(
    "节点 #{node_id} ({mnemonic}) 不是 FrameState，但被用户节点 #{user_id} 用作 FrameState 输入"
  )]
  InvalidFrameStateUse {
    node_id: u32,
    user_id: u32,
    mnemonic: &'static str,
  },

  #[error(
    "节点 #{node_id} 的 FrameState 输入 #{input_id} ({input_mnemonic}) 不是有效的 FrameState/Start/Dead/DeadValue"
  )]
  InvalidFrameStateInput {
    node_id: u32,
    input_id: u32,
    input_mnemonic: &'static str,
  },

  #[error(
    "节点 #{node_id} 的第 {index} 个 Effect 输入 #{input_id} ({input_mnemonic}) 无 Effect 输出且非 Dead"
  )]
  InvalidEffectInput {
    node_id: u32,
    index: usize,
    input_id: u32,
    input_mnemonic: &'static str,
  },

  #[error(
    "节点 #{node_id} 的第 {index} 个 Control 输入 #{input_id} ({input_mnemonic}) 无 Control 输出且非 Dead"
  )]
  InvalidControlInput {
    node_id: u32,
    index: usize,
    input_id: u32,
    input_mnemonic: &'static str,
  },

  #[error("替换边输入失败: 替换节点 #{node_id} ({mnemonic}) 无法满足 {kind} 边契约")]
  EdgeReplacementMismatch {
    node_id: u32,
    mnemonic: &'static str,
    kind: &'static str,
  },
}

pub type Result<T> = result::Result<T, Error>;
