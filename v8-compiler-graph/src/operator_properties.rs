// 由 tool/src/compiler/operator-properties.js 从 v8/src/compiler/operator-properties.h + operator-properties.cc 生成; 修订就地改, 复跑覆盖

//! OperatorProperties — operator 输入布局谓词 (context/frame-state/总输入数)
//!
//! 谓词的 opcode 面在 [`IrOpcode`](crate::opcodes::IrOpcode) 位表
//! (needs_exact_context 分区 69/30/2/23、frame_state 92+1 的对数见 opcodes.rs 头注);
//! 此处组合 operator 实例计数。参数化分支以 `Err(NeedsOpParam)` 推迟:
//! - JSCallRuntime: Runtime::NeedsExactContext / Linkage::NeedsFrameStateInput (运行时表)
//! - JSCreateArguments: CreateArgumentsTypeOf (operator 参数)

use crate::{
  Error,
  opcodes::{ExactContext, IrOpcode},
  operator::Operator,
};

/// 原版 OperatorProperties 静态类 (Rust 关联函数聚合)
pub struct OperatorProperties;

impl OperatorProperties {
  /// HasContextInput — JS opcode 恒有 context 输入 (IrOpcode::IsJsOpcode)
  pub fn has_context_input(op: &Operator) -> bool {
    Self::opcode(op).is_some_and(|v| v.has_context_input())
  }

  /// GetContextInputCount — 1 或 0
  pub fn get_context_input_count(op: &Operator) -> u32 {
    Self::has_context_input(op) as u32
  }

  /// NeedsExactContext — 是否必须精确 context (不能传任意 context)
  pub fn needs_exact_context(op: &Operator) -> Result<bool, crate::Error> {
    match Self::opcode_or_err(op)?.exact_context() {
      ExactContext::Exact => Ok(true),
      ExactContext::AnyContext => Ok(false),
      ExactContext::NeedsParam => Err(Error::NeedsOpParam {
        what: "NeedsExactContext: JSCallRuntime(Runtime 表)/JSCreateArguments(参数)",
      }),
      ExactContext::Unreachable | ExactContext::NotJs => Err(Error::Unreachable {
        what: "NeedsExactContext: 未登记 JS/非 JS opcode",
      }),
    }
  }

  /// HasFrameStateInput — 是否需要 FrameState 输入
  ///
  /// JSCallRuntime 需查 CallRuntime 参数 (Linkage::NeedsFrameStateInput), 推迟
  pub fn has_frame_state_input(op: &Operator) -> Result<bool, crate::Error> {
    let opcode = Self::opcode_or_err(op)?;
    if opcode == IrOpcode::JSCallRuntime {
      return Err(Error::NeedsOpParam {
        what: "HasFrameStateInput: JSCallRuntime 参数",
      });
    }
    Ok(opcode.has_frame_state_input())
  }

  /// GetFrameStateInputCount — 1 或 0
  pub fn get_frame_state_input_count(op: &Operator) -> Result<u32, crate::Error> {
    Ok(Self::has_frame_state_input(op)? as u32)
  }

  /// GetTotalInputCount — value + context + frame_state + effect + control
  pub fn get_total_input_count(op: &Operator) -> Result<u32, crate::Error> {
    Ok(
      op.value_input_count()
        + Self::get_context_input_count(op)
        + Self::get_frame_state_input_count(op)?
        + op.effect_input_count()
        + op.control_input_count(),
    )
  }

  /// IsBasicBlockBegin — 基本块起始 opcode (Start/End/Dead/Loop/Merge/If 族)
  pub fn is_basic_block_begin(op: &Operator) -> bool {
    Self::opcode(op).is_some_and(|v| v.is_basic_block_begin())
  }

  /// opcode 解码 (域内返回 None 交给调用方区分错误路径)
  fn opcode(op: &Operator) -> Option<IrOpcode> {
    IrOpcode::from_repr(op.opcode())
  }

  fn opcode_or_err(op: &Operator) -> Result<IrOpcode, crate::Error> {
    Self::opcode(op).ok_or(Error::OpcodeOutOfRange {
      opcode: op.opcode(),
      last: IrOpcode::LAST_U16,
    })
  }
}
