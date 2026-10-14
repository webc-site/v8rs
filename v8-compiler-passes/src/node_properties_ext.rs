//! NodeProperties 增补面 — 冻结层 v8-compiler-graph 推迟的少数纯图编辑原语
//!
//! 图 crate 已声明推迟 MergeControlToEnd 族 (依赖 CommonOperatorBuilder,
//! 属算子层); 归约管线需要该原语, 此处按原版 1:1 落位并锚注来源。

use v8_compiler_graph::{Node, NodeProperties};
use v8_compiler_ops::CommonOperatorBuilder;

use crate::{
  error::{Error, Result},
  graph_handle::GraphHandle,
};

/// v8: src/compiler/node-properties.cc NodeProperties::MergeControlToEnd
/// 原文: end->AppendInput(zone, node); end->set_op(common->End(end->InputCount()))
pub(crate) fn merge_control_to_end<'h, 'z, 'a>(
  handle: &'h GraphHandle<'h, 'z, 'a>,
  common: &CommonOperatorBuilder<'z, 'a>,
  node: &Node,
) -> Result<()> {
  let end = handle
    .end()
    .ok_or(Error::GraphUninitialized { which: "end" })?;
  let zone = handle.zone();
  end.append_input(zone, node)?;
  let count = end.input_count() as u32;
  let op = common.end(count)?;
  NodeProperties::change_op(end, op);
  Ok(())
}
