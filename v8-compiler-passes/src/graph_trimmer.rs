//! GraphTrimmer — 死节点修剪 (end 可达性标记)
//!
//! v8: src/compiler/graph-trimmer.h + graph-trimmer.cc 1:1。
//! 从 end (可加自定义根) 沿 inputs 传递闭包标活, 活节点的死使用者
//! 边置空。is_live 经 NodeMarker 平移 bool。

use std::sync::atomic::{AtomicBool, Ordering};

use v8_compiler_graph::{Mark, Node};

use crate::{
  error::{Error, Result},
  graph_handle::GraphHandle,
  node_marker::{MarkerState, NodeMarker},
};

/// --trace-turbo-trimming 开关 (原版 v8_flags.trace_turbo_trimming)
static TRACE_TURBO_TRIMMING: AtomicBool = AtomicBool::new(false);

/// v8: src/flags/compiler.h trace_turbo_trimming 写入侧入口
pub fn set_trace_turbo_trimming(on: bool) {
  TRACE_TURBO_TRIMMING.store(on, Ordering::Relaxed);
}

/// v8: src/flags/compiler.h trace_turbo_trimming 读取侧入口
pub fn trace_turbo_trimming() -> bool {
  TRACE_TURBO_TRIMMING.load(Ordering::Relaxed)
}

/// v8: src/compiler/graph-trimmer.h NodeMarker<bool> 的两态平移
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum LiveState {
  /// false
  NotLive = 0,
  /// true
  Live = 1,
}

impl MarkerState for LiveState {
  fn from_mark(mark: Mark) -> Self {
    if mark == 0 { Self::NotLive } else { Self::Live }
  }
  fn to_mark(self) -> Mark {
    self as Mark
  }
}

/// v8: src/compiler/graph-trimmer.h GraphTrimmer
pub struct GraphTrimmer<'z> {
  /// 原 is_live_ — NodeMarker<bool> (两态)
  is_live: NodeMarker<LiveState>,
  /// 原 live_ — NodeVector (兼作遍历队列)
  pub live: Vec<&'z Node>,
}

impl<'z> GraphTrimmer<'z> {
  /// v8: src/compiler/graph-trimmer.cc GraphTrimmer::GraphTrimmer
  /// 原文: is_live_(graph, 2); live_.reserve(graph->NodeCount())
  pub fn new(handle: &GraphHandle<'_, 'z, '_>) -> Self {
    // 原版 live_.reserve(graph->NodeCount())
    let live = Vec::with_capacity(handle.node_count());
    Self {
      is_live: NodeMarker::new(handle, 2),
      live,
    }
  }

  /// v8: src/compiler/graph-trimmer.h GraphTrimmer::IsLive
  pub fn is_live(&self, node: &Node) -> bool {
    self.is_live.get(node) == LiveState::Live
  }

  /// v8: src/compiler/graph-trimmer.h GraphTrimmer::MarkAsLive
  /// 原文: DCHECK(!node->IsDead()); 未标则置位并入队
  pub fn mark_as_live(&mut self, node: &'z Node) {
    debug_assert!(!node.is_dead());
    if !self.is_live(node) {
      self.is_live.set(node, LiveState::Live);
      self.live.push(node);
    }
  }

  /// v8: src/compiler/graph-trimmer.cc GraphTrimmer::TrimGraph (无参版)
  /// 原文: 标 end → 队列闭包标 inputs → 活节点的死使用者边置空
  pub fn trim_graph(&mut self, handle: &GraphHandle<'_, 'z, '_>) -> Result<()> {
    // end 标活
    let end = handle
      .end()
      .ok_or(Error::GraphUninitialized { which: "end" })?;
    self.mark_as_live(end);
    // 计算活节点传递闭包 (队列边推边扫)
    let mut i = 0usize;
    while i < self.live.len() {
      let live = self.live[i];
      for input in live.inputs().flatten() {
        self.mark_as_live(input);
      }
      i += 1;
    }
    // 活 → 死 的边清除
    for &live in &self.live {
      debug_assert!(self.is_live(live));
      for edge in live.use_edges() {
        let user = edge.from();
        if !self.is_live(user) {
          if trace_turbo_trimming() {
            log::info!("DeadLink: {}({}) -> {}", user.id(), edge.index(), live.id());
          }
          edge.update_to(None);
        }
      }
    }
    Ok(())
  }

  /// v8: src/compiler/graph-trimmer.h GraphTrimmer::TrimGraph(begin, end)
  /// 原文: 序列内非死节点先 MarkAsLive, 再走无参 TrimGraph
  pub fn trim_graph_with_roots(
    &mut self,
    handle: &GraphHandle<'_, 'z, '_>,
    roots: &[&'z Node],
  ) -> Result<()> {
    for node in roots {
      if !node.is_dead() {
        self.mark_as_live(node);
      }
    }
    self.trim_graph(handle)
  }
}
