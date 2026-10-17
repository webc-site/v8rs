//! Verifier — IR 图节点与边类型约束校验器
//!
//! 对应原版 verifier.h / verifier.cc 的核心子集：
//! - 校验输入边数量是否完全契合 Operator 规范
//! - 校验 Effect / Control / FrameState 输入边的生产者是否具备对应输出
//! - 校验无 Control / Effect / FrameState 产出的节点未被非法用作对应输入
//! - 校验边替换 (VerifyEdgeInputReplacement) 的类型兼容性
//! - 校验整图连通节点的拓扑契约 (VerifyGraph)

use gxhash::HashSet;
use v8_compiler_graph::{Edge, Graph, IrOpcode, Node, NodeProperties, OperatorProperties};

use crate::{Error, Result};

/// Verifier 校验器
pub struct Verifier;

impl Verifier {
  /// 校验单个节点的输入与输出约束 (原版 Verifier::VerifyNode)
  pub fn verify_node(node: &Node) -> Result<()> {
    // 1. 校验输入边数量是否符合 OperatorProperties 计算出的总输入数
    let expected = OperatorProperties::get_total_input_count(node.op())? as usize;
    let actual = node.input_count();
    if expected != actual {
      return Err(Error::InputCountMismatch {
        node_id: node.id(),
        expected,
        actual,
        mnemonic: node.op().mnemonic(),
      });
    }

    // 2. 检查输出边使用情况 (若无 Control/Effect 输出或不是 FrameState，则不能被用作对应边)
    let check_no_control = node.op().control_output_count() == 0;
    let check_no_effect = node.op().effect_output_count() == 0;
    let check_no_frame_state = node.opcode() != IrOpcode::FrameState;

    if check_no_control || check_no_effect || check_no_frame_state {
      for edge in node.use_edges() {
        let user = edge.from();
        if NodeProperties::is_control_edge(&edge)? && check_no_control {
          return Err(Error::InvalidControlUse {
            node_id: node.id(),
            user_id: user.id(),
            mnemonic: node.op().mnemonic(),
          });
        }
        if NodeProperties::is_effect_edge(&edge)? && check_no_effect {
          return Err(Error::InvalidEffectUse {
            node_id: node.id(),
            user_id: user.id(),
            mnemonic: node.op().mnemonic(),
          });
        }
        if NodeProperties::is_frame_state_edge(&edge)? && check_no_frame_state {
          return Err(Error::InvalidFrameStateUse {
            node_id: node.id(),
            user_id: user.id(),
            mnemonic: node.op().mnemonic(),
          });
        }
      }
    }

    // 3. 校验 FrameState 输入
    if OperatorProperties::get_frame_state_input_count(node.op())? > 0 {
      let input = NodeProperties::get_frame_state_input(node)?;
      let opc = input.opcode();
      let valid = opc == IrOpcode::FrameState
        || opc == IrOpcode::Start
        || opc == IrOpcode::Dead
        || opc == IrOpcode::DeadValue;
      if !valid {
        return Err(Error::InvalidFrameStateInput {
          node_id: node.id(),
          input_id: input.id(),
          input_mnemonic: input.op().mnemonic(),
        });
      }
    }

    // 4. 校验 Effect 输入
    let effect_count = node.op().effect_input_count() as usize;
    for i in 0..effect_count {
      let input = NodeProperties::get_effect_input(node, i)?;
      let valid = input.op().effect_output_count() > 0 || input.opcode() == IrOpcode::Dead;
      if !valid {
        return Err(Error::InvalidEffectInput {
          node_id: node.id(),
          index: i,
          input_id: input.id(),
          input_mnemonic: input.op().mnemonic(),
        });
      }
    }

    // 5. 校验 Control 输入
    let control_count = node.op().control_input_count() as usize;
    for i in 0..control_count {
      let input = NodeProperties::get_control_input(node, i)?;
      let valid = input.op().control_output_count() > 0 || input.opcode() == IrOpcode::Dead;
      if !valid {
        return Err(Error::InvalidControlInput {
          node_id: node.id(),
          index: i,
          input_id: input.id(),
          input_mnemonic: input.op().mnemonic(),
        });
      }
    }

    Ok(())
  }

  /// 校验待替换的输入节点是否满足边类型要求 (原版 Verifier::VerifyEdgeInputReplacement)
  pub fn verify_edge_input_replacement(edge: &Edge, replacement: &Node) -> Result<()> {
    if NodeProperties::is_control_edge(edge)?
      && replacement.op().control_output_count() == 0
      && replacement.opcode() != IrOpcode::Dead
    {
      return Err(Error::EdgeReplacementMismatch {
        node_id: replacement.id(),
        mnemonic: replacement.op().mnemonic(),
        kind: "Control",
      });
    }

    if NodeProperties::is_effect_edge(edge)?
      && replacement.op().effect_output_count() == 0
      && replacement.opcode() != IrOpcode::Dead
    {
      return Err(Error::EdgeReplacementMismatch {
        node_id: replacement.id(),
        mnemonic: replacement.op().mnemonic(),
        kind: "Effect",
      });
    }

    if NodeProperties::is_frame_state_edge(edge)? {
      let opc = replacement.opcode();
      let valid =
        opc == IrOpcode::FrameState || opc == IrOpcode::Dead || opc == IrOpcode::DeadValue;
      if !valid {
        return Err(Error::EdgeReplacementMismatch {
          node_id: replacement.id(),
          mnemonic: replacement.op().mnemonic(),
          kind: "FrameState",
        });
      }
    }

    Ok(())
  }

  /// 校验整张图的连通可达节点契约 (原版 Verifier::Run 核心子集)
  pub fn verify_graph(graph: &Graph) -> Result<()> {
    let mut visited: HashSet<u32> = HashSet::default();
    let mut queue = Vec::new();

    if let Some(end) = graph.end() {
      queue.push(end);
      visited.insert(end.id());
    }
    if let Some(start) = graph.start()
      && visited.insert(start.id())
    {
      queue.push(start);
    }

    // 广度优先遍历图中的连通节点
    let mut head = 0;
    while head < queue.len() {
      let curr = queue[head];
      head += 1;

      Self::verify_node(curr)?;

      // 将输入节点加入待访问队列
      for input in curr.inputs().flatten() {
        if visited.insert(input.id()) {
          queue.push(input);
        }
      }

      // 将使用节点加入待访问队列
      for edge in curr.use_edges() {
        let user = edge.from();
        if visited.insert(user.id()) {
          queue.push(user);
        }
      }
    }

    Ok(())
  }
}
