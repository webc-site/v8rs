#![cfg_attr(docsrs, feature(doc_cfg))]
//! v8-compiler-graph — TurboFan IR 基础层 (第一梯队)
//!
//! 文件映射 (原版 → 本 crate):
//! - opcodes.h/cc + operator-properties 谓词表 → opcodes (X-macro 机械展开)
//! - operator.h/cc → operator
//! - operator-properties.h/cc → operator_properties
//! - node.h/cc → node (zone bump + Use 前缀布局 1:1)
//! - node-properties.h/cc → node_properties
//! - turbofan-graph.h/cc → graph
//! - use-info.h + representation-change.cc(Truncation 格) → use_info
//! - codegen/machine-type.h 最小子集 → machine_type
//! - bytecode-graph-builder.h/cc 建图纯面 → build_graph
//!
//! 完整映射与推迟登记见 README.md。

mod build_graph;
mod error;
mod graph;
mod machine_type;
mod node;
mod node_properties;
mod opcodes;
mod operator;
mod operator_properties;
mod use_info;

pub use build_graph::{
  InputBuffer, K_INPUT_BUFFER_SIZE_INCREMENT, create_end, create_start, new_phi,
};
pub use error::{Error, Result};
pub use graph::{Graph, GraphDecorator, SubgraphScope};
pub use machine_type::{MachineRepresentation, MachineSemantic, MachineType};
pub use node::{
  Edge, InputEdges, Inputs, K_MAX_INLINE_CAPACITY, K_OUTLINE_MARKER, Mark, Node, NodeId, UseEdges,
  Uses,
};
pub use node_properties::NodeProperties;
pub use opcodes::{
  ExactContext, IrOpcode, OP_ATOMIC, OP_BASIC_BLOCK_BEGIN, OP_COMMON, OP_COMPARISON, OP_CONSTANT,
  OP_CONTEXT_CHAIN_EXTENDING, OP_CONTROL, OP_EXACT_CONTEXT_LAZY, OP_EXACT_CONTEXT_PARAM,
  OP_EXACT_CONTEXT_TRUE, OP_FEEDBACK, OP_FRAME_STATE_INPUT, OP_GRAPH_TERMINATOR, OP_IF_PROJECTION,
  OP_INLINEE, OP_JS, OP_MACHINE, OP_MACHINE_CONSTANT, OP_MERGE, OP_PHI, OP_SIMD128, OP_SIMPLIFIED,
  OPCODE_PROPS,
};
pub use operator::{Operator, Operator1, OperatorCounts, Properties};
pub use operator_properties::OperatorProperties;
pub use use_info::{
  CheckForMinusZeroMode, FeedbackSource, IdentifyZeros, Truncation, TypeCheckKind, UseInfo,
};
