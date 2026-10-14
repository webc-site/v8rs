//! js_graph — JS 图门面
//!
//! 1:1 对应 v8/src/compiler/js-graph.{h,cc}。承接 Graph 与各算子构建器门面。

use v8_compiler_graph::Graph;
use v8_compiler_ops::CommonOperatorBuilder;
use v8_zone::Zone;

/// JS 图门面
/// v8: src/compiler/js-graph.h JSGraph
pub struct JSGraph<'z, 'a> {
  pub graph: Graph<'z, 'a>,
  pub common: CommonOperatorBuilder<'z, 'a>,
}

impl<'z, 'a> JSGraph<'z, 'a> {
  pub fn new(zone: &'z Zone<'a>) -> Self {
    Self {
      graph: Graph::new(zone),
      common: CommonOperatorBuilder::new(zone),
    }
  }
}
