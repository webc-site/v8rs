// 由 tool/src/compiler/node-properties.js 从 v8/src/compiler/node-properties.h + node-properties.cc 生成; 修订就地改, 复跑覆盖

//! NodeProperties — 节点各类输入的门面 (布局计算 / 访问 / 边类别 / 替换)
//!
//! 输入布局 (node-properties.h:29): value | context | frame_state | effect | control
//! 原版 CHECK 越界 → Err(Error::InputIndexOutOfRange);
//! 推迟面: 投影族 (Operator1 参数)、MergeControlToEnd 族 (CommonOperatorBuilder)、
//! heap-broker 推理族、Type 族 — README 登记。

use std::ptr::eq;

use v8_base::hashing::hash_combine;

use crate::{
  Error,
  node::{Edge, Node},
  opcodes::IrOpcode,
  operator::{Operator, Properties},
  operator_properties::OperatorProperties,
};

/// 原版 NodeProperties 静态类
pub struct NodeProperties;

impl NodeProperties {
  // —— 输入布局 ——

  /// FirstValueIndex — 依据节点输入操作数边界与操作符规范，计算值输入起始索引
  pub fn first_value_index(node: &Node) -> usize {
    let value_count = node.op().value_input_count() as usize;
    let start_index = 0usize;
    debug_assert!(start_index <= node.input_count());
    let _ = value_count;
    start_index
  }

  /// FirstContextIndex = PastValueIndex
  pub fn first_context_index(node: &Node) -> usize {
    Self::past_value_index(node)
  }

  /// FirstFrameStateIndex = PastContextIndex
  pub fn first_frame_state_index(node: &Node) -> Result<usize, Error> {
    Self::past_context_index(node)
  }

  /// FirstEffectIndex = PastFrameStateIndex
  pub fn first_effect_index(node: &Node) -> Result<usize, Error> {
    Self::past_frame_state_index(node)
  }

  /// FirstControlIndex = PastEffectIndex
  pub fn first_control_index(node: &Node) -> Result<usize, Error> {
    Self::past_effect_index(node)
  }

  /// PastValueIndex = FirstValueIndex + ValueInputCount
  pub fn past_value_index(node: &Node) -> usize {
    Self::first_value_index(node) + node.op().value_input_count() as usize
  }

  /// PastContextIndex = FirstContext + ContextInputCount
  pub fn past_context_index(node: &Node) -> Result<usize, Error> {
    Ok(
      Self::first_context_index(node)
        + OperatorProperties::get_context_input_count(node.op()) as usize,
    )
  }

  /// PastFrameStateIndex = FirstFrameState + FrameStateInputCount
  pub fn past_frame_state_index(node: &Node) -> Result<usize, Error> {
    Ok(
      Self::first_frame_state_index(node)?
        + OperatorProperties::get_frame_state_input_count(node.op())? as usize,
    )
  }

  /// PastEffectIndex = FirstEffect + EffectInputCount
  pub fn past_effect_index(node: &Node) -> Result<usize, Error> {
    Ok(Self::first_effect_index(node)? + node.op().effect_input_count() as usize)
  }

  /// PastControlIndex = FirstControl + ControlInputCount
  pub fn past_control_index(node: &Node) -> Result<usize, Error> {
    Ok(Self::first_control_index(node)? + node.op().control_input_count() as usize)
  }

  // —— 输入访问器 ——

  /// GetValueInput — 值输入 (原版 CHECK 越界 → Err)
  pub fn get_value_input(node: &Node, index: usize) -> Result<&Node, Error> {
    let count = node.op().value_input_count() as usize;
    if index >= count {
      return Err(Error::InputIndexOutOfRange {
        index,
        count,
        what: "value",
      });
    }
    node
      .input_at(Self::first_value_index(node) + index)
      .ok_or(Error::NullInput {
        id: node.id(),
        mnemonic: node.op().mnemonic(),
        index,
      })
  }

  /// GetContextInput — context 输入 (JS opcode 恒有)
  pub fn get_context_input(node: &Node) -> Result<&Node, Error> {
    if !OperatorProperties::has_context_input(node.op()) {
      return Err(Error::NoContextInput {
        opcode: node.opcode(),
      });
    }
    node
      .input_at(Self::first_context_index(node))
      .ok_or(Error::NullInput {
        id: node.id(),
        mnemonic: node.op().mnemonic(),
        index: Self::first_context_index(node),
      })
  }

  /// GetFrameStateInput — FrameState 输入
  pub fn get_frame_state_input(node: &Node) -> Result<&Node, Error> {
    if !OperatorProperties::has_frame_state_input(node.op())? {
      return Err(Error::NoFrameStateInput {
        opcode: node.opcode(),
      });
    }
    let index = Self::first_frame_state_index(node)?;
    node.input_at(index).ok_or(Error::NullInput {
      id: node.id(),
      mnemonic: node.op().mnemonic(),
      index,
    })
  }

  /// GetEffectInput(index = 0)
  pub fn get_effect_input(node: &Node, index: usize) -> Result<&Node, Error> {
    let count = node.op().effect_input_count() as usize;
    if index >= count {
      return Err(Error::InputIndexOutOfRange {
        index,
        count,
        what: "effect",
      });
    }
    let index = Self::first_effect_index(node)? + index;
    node.input_at(index).ok_or(Error::NullInput {
      id: node.id(),
      mnemonic: node.op().mnemonic(),
      index,
    })
  }

  /// GetControlInput(index = 0)
  pub fn get_control_input(node: &Node, index: usize) -> Result<&Node, Error> {
    let count = node.op().control_input_count() as usize;
    if index >= count {
      return Err(Error::InputIndexOutOfRange {
        index,
        count,
        what: "control",
      });
    }
    let index = Self::first_control_index(node)? + index;
    node.input_at(index).ok_or(Error::NullInput {
      id: node.id(),
      mnemonic: node.op().mnemonic(),
      index,
    })
  }

  // —— 边类别 ——

  /// IsValueEdge
  pub fn is_value_edge(edge: &Edge) -> bool {
    let node = edge.from();
    Self::is_input_range(
      edge,
      Self::first_value_index(node),
      node.op().value_input_count() as usize,
    )
  }

  /// IsContextEdge
  pub fn is_context_edge(edge: &Edge) -> bool {
    let node = edge.from();
    Self::is_input_range(
      edge,
      Self::first_context_index(node),
      OperatorProperties::get_context_input_count(node.op()) as usize,
    )
  }

  /// IsFrameStateEdge
  pub fn is_frame_state_edge(edge: &Edge) -> Result<bool, Error> {
    let node = edge.from();
    Ok(Self::is_input_range(
      edge,
      Self::first_frame_state_index(node)?,
      OperatorProperties::get_frame_state_input_count(node.op())? as usize,
    ))
  }

  /// IsEffectEdge
  pub fn is_effect_edge(edge: &Edge) -> Result<bool, Error> {
    let node = edge.from();
    Ok(Self::is_input_range(
      edge,
      Self::first_effect_index(node)?,
      node.op().effect_input_count() as usize,
    ))
  }

  /// IsControlEdge
  pub fn is_control_edge(edge: &Edge) -> Result<bool, Error> {
    let node = edge.from();
    Ok(Self::is_input_range(
      edge,
      Self::first_control_index(node)?,
      node.op().control_input_count() as usize,
    ))
  }

  /// IsInputRange — [first, first+num) 且 num>0
  fn is_input_range(edge: &Edge, first: usize, num: usize) -> bool {
    if num == 0 {
      return false;
    }
    let index = edge.index();
    first <= index && index < first + num
  }

  // —— 谓词 ——

  /// IsCommon
  pub fn is_common(node: &Node) -> bool {
    node.opcode().is_common()
  }

  /// IsControl
  pub fn is_control(node: &Node) -> bool {
    node.opcode().is_control()
  }

  /// IsConstant
  pub fn is_constant(node: &Node) -> bool {
    node.opcode().is_constant()
  }

  /// IsPhi
  pub fn is_phi(node: &Node) -> bool {
    node.opcode().is_phi()
  }

  /// IsSimd128Operation (wasm 开构建)
  pub fn is_simd128_operation(node: &Node) -> bool {
    node.opcode().is_simd128()
  }

  /// IsExceptionalCall — 有 IfException 控制使用则 Some(该投影)
  pub fn find_exceptional_call(node: &Node) -> Result<Option<&Node>, Error> {
    if node.op().has_property(Properties::NO_THROW) {
      return Ok(None);
    }
    for edge in node.use_edges() {
      if !Self::is_control_edge(&edge)? {
        continue;
      }
      if edge.from().opcode() == IrOpcode::IfException {
        return Ok(Some(edge.from()));
      }
    }
    Ok(None)
  }

  /// FindSuccessfulControlProjection — IfSuccess 投影或自身
  pub fn find_successful_control_projection(node: &Node) -> Result<&Node, Error> {
    if node.op().control_output_count() == 0 {
      return Err(Error::NoControlOutput {
        opcode: node.opcode(),
      });
    }
    if node.op().has_property(Properties::NO_THROW) {
      return Ok(node);
    }
    for edge in node.use_edges() {
      if !Self::is_control_edge(&edge)? {
        continue;
      }
      if edge.from().opcode() == IrOpcode::IfSuccess {
        return Ok(edge.from());
      }
    }
    Ok(node)
  }

  /// IsValueIdentity — TypeGuard 直通值输入
  pub fn is_value_identity(node: &Node) -> Result<Option<&Node>, Error> {
    if node.opcode() == IrOpcode::TypeGuard {
      return Ok(Some(Self::get_value_input(node, 0)?));
    }
    Ok(None)
  }

  // —— 替换器 ——

  /// ReplaceValueInput
  pub fn replace_value_input(node: &Node, value: &Node, index: usize) -> Result<(), Error> {
    let count = node.op().value_input_count() as usize;
    if index >= count {
      return Err(Error::InputIndexOutOfRange {
        index,
        count,
        what: "value",
      });
    }
    node.replace_input(Self::first_value_index(node) + index, Some(value));
    Ok(())
  }

  /// ReplaceValueInputs — 全部值输入并为单个 value
  pub fn replace_value_inputs(node: &Node, value: &Node) -> Result<(), Error> {
    let mut value_input_count = node.op().value_input_count() as usize;
    if value_input_count == 0 {
      return Err(Error::InputIndexOutOfRange {
        index: 0,
        count: 0,
        what: "value",
      });
    }
    node.replace_input(0, Some(value));
    value_input_count -= 1;
    while value_input_count > 0 {
      node.remove_input(value_input_count);
      value_input_count -= 1;
    }
    Ok(())
  }

  /// ReplaceContextInput
  pub fn replace_context_input(node: &Node, context: &Node) -> Result<(), Error> {
    if !OperatorProperties::has_context_input(node.op()) {
      return Err(Error::NoContextInput {
        opcode: node.opcode(),
      });
    }
    node.replace_input(Self::first_context_index(node), Some(context));
    Ok(())
  }

  /// ReplaceControlInput(index = 0)
  pub fn replace_control_input(node: &Node, control: &Node, index: usize) -> Result<(), Error> {
    let count = node.op().control_input_count() as usize;
    if index >= count {
      return Err(Error::InputIndexOutOfRange {
        index,
        count,
        what: "control",
      });
    }
    node.replace_input(Self::first_control_index(node)? + index, Some(control));
    Ok(())
  }

  /// ReplaceEffectInput(index = 0)
  pub fn replace_effect_input(node: &Node, effect: &Node, index: usize) -> Result<(), Error> {
    let count = node.op().effect_input_count() as usize;
    if index >= count {
      return Err(Error::InputIndexOutOfRange {
        index,
        count,
        what: "effect",
      });
    }
    node.replace_input(Self::first_effect_index(node)? + index, Some(effect));
    Ok(())
  }

  /// ReplaceFrameStateInput
  pub fn replace_frame_state_input(node: &Node, frame_state: &Node) -> Result<(), Error> {
    if !OperatorProperties::has_frame_state_input(node.op())? {
      return Err(Error::NoFrameStateInput {
        opcode: node.opcode(),
      });
    }
    node.replace_input(Self::first_frame_state_index(node)?, Some(frame_state));
    Ok(())
  }

  /// RemoveNonValueInputs — 裁到值输入数
  pub fn remove_non_value_inputs(node: &Node) {
    node.trim_input_count(node.op().value_input_count() as usize);
  }

  /// RemoveValueInputs — 逐个移除值输入
  pub fn remove_value_inputs(node: &Node) {
    let mut value_input_count = node.op().value_input_count() as usize;
    while value_input_count > 0 {
      value_input_count -= 1;
      node.remove_input(value_input_count);
    }
  }

  /// ReplaceUses — 按边类别整体替换 (value/effect/success/exception)
  ///
  /// None 占位 = 原版 DCHECK_NOT_NULL 之外的合法 null 边 (UpdateTo(nullptr))
  pub fn replace_uses(
    node: &Node,
    value: Option<&Node>,
    effect: Option<&Node>,
    success: Option<&Node>,
    exception: Option<&Node>,
  ) -> Result<(), Error> {
    for edge in node.use_edges() {
      if Self::is_control_edge(&edge)? {
        match edge.from().opcode() {
          IrOpcode::IfSuccess => edge.update_to(success),
          IrOpcode::IfException => edge.update_to(exception),
          _ => edge.update_to(success),
        }
      } else if Self::is_effect_edge(&edge)? {
        edge.update_to(effect);
      } else {
        edge.update_to(value);
      }
    }
    Ok(())
  }

  /// ChangeOp — 安全换 operator (Verifier 校验推迟; 当前与 unchecked 等价)
  pub fn change_op(node: &Node, new_op: &Operator) {
    node.set_op(new_op);
  }

  /// ChangeOpUnchecked
  pub fn change_op_unchecked(node: &Node, new_op: &Operator) {
    node.set_op(new_op);
  }

  // —— 工具 ——

  /// FindFrameStateBefore — 沿 effect 链回溯到 Checkpoint 取 FrameState
  pub fn find_frame_state_before<'n>(
    node: &'n Node,
    unreachable_sentinel: &'n Node,
  ) -> Result<&'n Node, Error> {
    let mut effect = Self::get_effect_input(node, 0)?;
    loop {
      if effect.opcode() == IrOpcode::Checkpoint {
        break;
      }
      if effect.opcode() == IrOpcode::Dead || effect.opcode() == IrOpcode::Unreachable {
        return Ok(unreachable_sentinel);
      }
      debug_assert!(effect.op().has_property(Properties::NO_WRITE));
      debug_assert_eq!(1, effect.op().effect_input_count());
      effect = Self::get_effect_input(effect, 0)?;
    }
    Self::get_frame_state_input(effect)
  }

  /// GetOuterContext — 沿 context 链上行消耗 depth
  pub fn get_outer_context<'n>(node: &'n Node, depth: &mut usize) -> Result<&'n Node, Error> {
    let mut context = Self::get_context_input(node)?;
    while *depth > 0 && context.opcode().is_context_chain_extending() {
      context = Self::get_context_input(context)?;
      *depth -= 1;
    }
    Ok(context)
  }

  /// IsSame — 透明穿透 CheckHeapObject / TypeGuard 比较
  pub fn is_same(mut a: &Node, mut b: &Node) -> Result<bool, Error> {
    loop {
      if a.opcode() == IrOpcode::CheckHeapObject || a.opcode() == IrOpcode::TypeGuard {
        a = Self::get_value_input(a, 0)?;
        continue;
      }
      if b.opcode() == IrOpcode::CheckHeapObject || b.opcode() == IrOpcode::TypeGuard {
        b = Self::get_value_input(b, 0)?;
        continue;
      }
      return Ok(eq(a, b));
    }
  }

  /// HashCode — hash_combine(op, input_count, input ids...)
  pub fn hash_code(node: &Node) -> usize {
    let mut h = hash_combine(node.op().hash_code(), node.input_count());
    for input in node.inputs().flatten() {
      h = hash_combine(h, input.id() as usize);
    }
    h
  }

  /// Equals — operator 等价 + 输入数相等 + 输入 id 逐位相等
  pub fn equals(a: &Node, b: &Node) -> bool {
    if !a.op().equals(b.op()) {
      return false;
    }
    if a.input_count() != b.input_count() {
      return false;
    }
    let a_inputs = a.inputs();
    let b_inputs = b.inputs();
    for (x, y) in a_inputs.zip(b_inputs) {
      let (Some(x), Some(y)) = (x, y) else {
        return false;
      };
      if x.id() != y.id() {
        return false;
      }
    }
    true
  }
}
