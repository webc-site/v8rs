#![cfg_attr(docsrs, feature(doc_cfg))]
//! v8-compiler-passes — TurboFan 顶层优化管线族 (reducer/typer/pipeline/lowering…)
//!
//! 席位 W9CP (Wave 9), 原版面: v8/src/compiler 顶层 197 文件 (target/wave9/W9CP.txt)。
//! 文件映射 (原版 → 本 crate):
//! - graph-reducer.h/cc + node-marker.h/cc → graph_reducer + node_marker
//! - graph-trimmer.h/cc → graph_trimmer
//! - turbofan-graph.h 别名语义窄化承接 → graph_handle (Rust 适配层, 非原版文件)
//! - node-properties.cc MergeControlToEnd (图 crate 推迟面) → node_properties_ext
//! - 其余族 (typer/reducer/lowering/verifier/pipeline/…) 见 readme/zh.md 映射与推迟台账。
//!
//! 冻结依赖: v8-compiler-graph (IR 基础层), v8-compiler-ops (算子层), v8-zone。

mod error;
mod graph_handle;
mod graph_reducer;
mod graph_trimmer;
mod node_marker;
mod node_properties_ext;

pub use error::{Error, Result};
pub use graph_handle::GraphHandle;
pub use graph_reducer::{
  Decision, GraphReducer, Reducer, Reduction, State, set_trace_turbo_reduction,
  trace_turbo_reduction,
};
pub use graph_trimmer::{GraphTrimmer, LiveState, set_trace_turbo_trimming, trace_turbo_trimming};
pub use node_marker::{MarkerState, NodeMarker, NodeMarkerBase};

/// 桥接: 从编译器前端的 Reduction 转换为 GraphReducer 的 Reduction (真实消费 reduction 节点)
pub fn from_frontend_reduction<'z>(red: v8_compiler_frontend::Reduction<'z>) -> Reduction<'z> {
  Reduction::new(red.replacement)
}
