//! GraphReducer — 迭代图归约机 (Reducer 体系 + revisit/stack 驱动)
//!
//! v8: src/compiler/graph-reducer.h + graph-reducer.cc 1:1。
//! 栈驱动自底向上归约: ReduceTop 递归输入, Reducer 依次改写,
//! 替换/就地更新经 Editor 语义 (本实现的 inherent 方法面) 回写图。
//! 推迟面: TickCounter 节流, JSHeapBroker trace 域, ObserveNodeManager
//! 观察钩子, dead 节点 Type::None 落型 (见 readme/zh.md 台账)。

use std::{
  collections::VecDeque,
  mem::take,
  ptr::eq,
  sync::atomic::{AtomicBool, Ordering},
};

use v8_compiler_graph::{Graph, IrOpcode, Mark, Node, NodeId, NodeProperties};
use v8_compiler_ops::Verifier;

use crate::{
  error::{Error, Result},
  graph_handle::GraphHandle,
  node_marker::{MarkerState, NodeMarker},
  node_properties_ext::merge_control_to_end,
};

/// --trace-turbo-reduction 开关 (原版 v8_flags.trace_turbo_reduction)
static TRACE_TURBO_REDUCTION: AtomicBool = AtomicBool::new(false);

/// v8: src/flags/compiler.h trace_turbo_reduction 写入侧入口
pub fn set_trace_turbo_reduction(on: bool) {
  TRACE_TURBO_REDUCTION.store(on, Ordering::Relaxed);
}

/// v8: src/flags/compiler.h trace_turbo_reduction 读取侧入口
pub fn trace_turbo_reduction() -> bool {
  TRACE_TURBO_REDUCTION.load(Ordering::Relaxed)
}

/// v8: src/compiler/graph-reducer.h Decision
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Decision {
  /// kUnknown
  Unknown,
  /// kTrue
  True,
  /// kFalse
  False,
}

/// v8: src/compiler/graph-reducer.h Reduction
/// 归约结果: replacement 为空即 NoChange
#[derive(Clone, Copy, Debug)]
pub struct Reduction<'z> {
  replacement: Option<&'z Node>,
}

impl<'z> Reduction<'z> {
  /// v8: src/compiler/graph-reducer.h Reduction::Reduction (默认 nullptr)
  pub const fn new(replacement: Option<&'z Node>) -> Self {
    Self { replacement }
  }
  /// v8: src/compiler/graph-reducer.h Reduction::replacement
  pub const fn replacement(&self) -> Option<&'z Node> {
    self.replacement
  }
  /// v8: src/compiler/graph-reducer.h Reduction::Changed
  pub const fn is_changed(&self) -> bool {
    self.replacement.is_some()
  }
  /// v8: src/compiler/graph-reducer.h Reduction::FollowedBy
  /// 原文: if (next.Changed()) return next; return *this;
  pub const fn followed_by(&self, next: Reduction<'z>) -> Reduction<'z> {
    if next.is_changed() { next } else { *self }
  }

  /// v8: src/compiler/graph-reducer.h Reducer::NoChange
  pub const fn no_change() -> Self {
    Self::new(None)
  }
  /// v8: src/compiler/graph-reducer.h Reducer::Replace
  pub const fn replace(node: &'z Node) -> Self {
    Self::new(Some(node))
  }
  /// v8: src/compiler/graph-reducer.h Reducer::Changed (与 Replace 同构)
  pub const fn changed(node: &'z Node) -> Self {
    Self::new(Some(node))
  }
}

/// v8: src/compiler/graph-reducer.cc GraphReducer::State
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum State {
  /// kUnvisited
  Unvisited = 0,
  /// kRevisit
  Revisit = 1,
  /// kOnStack
  OnStack = 2,
  /// kVisited
  Visited = 3,
}

impl MarkerState for State {
  fn from_mark(mark: Mark) -> Self {
    // 值域由 NodeMarker 区间分配保证; 越界仅可能来自别名误用 (原版 static_cast UB)
    match mark {
      0 => Self::Unvisited,
      1 => Self::Revisit,
      2 => Self::OnStack,
      _ => Self::Visited,
    }
  }
  fn to_mark(self) -> Mark {
    self as Mark
  }
}

/// v8: src/compiler/graph-reducer.h GraphReducer::NodeState
struct NodeState<'z> {
  node: &'z Node,
  input_index: usize,
}

/// v8: src/compiler/graph-reducer.h Reducer
/// 注: 原版虚基类 + AdvancedReducer::Editor* 自指环在 Rust 侧以
/// reduce(gr, node) 形参传递具体 GraphReducer 承接 (窄化适配)
pub trait Reducer<'h, 'z, 'a> {
  /// v8: src/compiler/graph-reducer.h Reducer::reducer_name
  fn reducer_name(&self) -> &'static str;
  /// v8: src/compiler/graph-reducer.cc Reducer::Reduce (去观察器尾参, 见台账)
  fn reduce(
    &mut self,
    reducer: &mut GraphReducer<'h, 'z, 'a>,
    node: &'z Node,
  ) -> Result<Reduction<'z>>;
  /// v8: src/compiler/graph-reducer.cc Reducer::Finalize
  fn finalize(&mut self) {}
}

/// v8: src/compiler/graph-reducer.h GraphReducer
/// 原版 NON_EXPORTED_BASE(AdvancedReducer::Editor) 的 Editor 面以
/// inherent 方法曝光 (replace/replace_max/revisit/replace_with_value)
pub struct GraphReducer<'h, 'z, 'a> {
  /// 原 graph_ (句柄化)
  pub handle: &'h GraphHandle<'h, 'z, 'a>,
  /// 原 dead_ (构造默认参 nullptr)
  pub dead: Option<&'z Node>,
  /// 原 state_ — NodeMarker<State>
  state: NodeMarker<State>,
  /// 原 reducers_ — ZoneVector<Reducer*> 的所有化
  reducers: Vec<Box<dyn Reducer<'h, 'z, 'a> + 'h>>,
  /// 原 revisit_ — ZoneQueue
  revisit: VecDeque<&'z Node>,
  /// 原 stack_ — ZoneStack<NodeState>
  stack: Vec<NodeState<'z>>,
}

impl<'h, 'z, 'a> GraphReducer<'h, 'z, 'a> {
  /// v8: src/compiler/graph-reducer.cc GraphReducer::GraphReducer
  /// 推迟: tick_counter/broker/observe 尾参; `NodeProperties::SetType(dead, Type::None())`
  /// 依赖 turbofan-types 层 (见台账)
  pub fn new(handle: &'h GraphHandle<'h, 'z, 'a>, dead: Option<&'z Node>) -> Self {
    Self {
      handle,
      dead,
      // 原版 state_(graph, 4)
      state: NodeMarker::new(handle, 4),
      reducers: Vec::new(),
      revisit: VecDeque::new(),
      stack: Vec::new(),
    }
  }

  /// v8: src/compiler/graph-reducer.h GraphReducer::graph
  pub fn graph(&self) -> &'h Graph<'z, 'a> {
    self.handle.graph()
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::AddReducer
  /// 原版收裸指针 (调用方保活); Rust 侧转移所有权进归约机
  pub fn add_reducer(&mut self, reducer: Box<dyn Reducer<'h, 'z, 'a> + 'h>) {
    self.reducers.push(reducer);
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReduceNode
  pub fn reduce_node(&mut self, node: &'z Node) -> Result<()> {
    debug_assert!(self.stack.is_empty() && self.revisit.is_empty());
    self.push(node);
    loop {
      if !self.stack.is_empty() {
        // 处理栈顶节点, 可能继续下压或弹出
        self.reduce_top()?;
      } else if let Some(node) = self.revisit.pop_front() {
        // 栈空时消费回访队列 (状态可能在排队期间变化)
        if self.state.get(node) == State::Revisit {
          self.push(node);
        }
      } else {
        // 运行全部终结器
        for reducer in &mut self.reducers {
          reducer.finalize();
        }
        // 终结器可能新入队, 空则收敛
        if self.revisit.is_empty() {
          break;
        }
      }
    }
    debug_assert!(self.revisit.is_empty() && self.stack.is_empty());
    Ok(())
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReduceGraph
  /// 原文: ReduceNode(graph()->end())
  pub fn reduce_graph(&mut self) -> Result<()> {
    let end = self
      .handle
      .end()
      .ok_or(Error::GraphUninitialized { which: "end" })?;
    self.reduce_node(end)
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::Reduce (单节点逐 reducer)
  /// 分派期间 reducers 暂移出 self (借用分离), 结束归位
  fn reduce(&mut self, node: &'z Node) -> Result<Reduction<'z>> {
    let mut reducers = take(&mut self.reducers);
    let result = self.reduce_dispatch(&mut reducers, node);
    self.reducers = reducers;
    result
  }

  fn reduce_dispatch(
    &mut self,
    reducers: &mut [Box<dyn Reducer<'h, 'z, 'a> + 'h>],
    node: &'z Node,
  ) -> Result<Reduction<'z>> {
    // 原版: skip 哨兵 = reducers_.end() (无跳过位)
    let mut skip: Option<usize> = None;
    let mut i = 0usize;
    while i < reducers.len() {
      if Some(i) != skip {
        // 推迟: tick_counter_->TickAndMaybeEnterSafepoint() (见台账)
        let reduction = reducers[i].reduce(self, node)?;
        if reduction.is_changed() {
          if ptr_eq(reduction.replacement(), Some(node)) {
            // 就地更新: 其余 reducer 需对该节点重跑
            if trace_turbo_reduction() {
              log::info!(
                "- In-place update of #{} by reducer {}",
                node.id(),
                reducers[i].reducer_name()
              );
            }
            skip = Some(i);
            i = 0;
            continue;
          } else {
            // 节点被替换
            if trace_turbo_reduction() {
              let repl = reduction.replacement();
              log::info!(
                "- Replacement of #{} with #{} by reducer {}",
                node.id(),
                repl.map(Node::id).unwrap_or_default(),
                reducers[i].reducer_name()
              );
            }
            return Ok(reduction);
          }
        }
      }
      i += 1;
    }
    if skip.is_none() {
      // 任何 reducer 都无改动
      return Ok(Reduction::no_change());
    }
    // 至少一个 reducer 做了就地归约
    Ok(Reduction::changed(node))
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReduceTop
  fn reduce_top(&mut self) -> Result<()> {
    debug_assert!(!self.stack.is_empty());
    let entry = self.stack.len() - 1;
    let node = self.stack[entry].node;
    debug_assert_eq!(self.state.get(node), State::OnStack);

    if node.is_dead() {
      // 归约期间被杀
      self.pop();
      return Ok(());
    }

    let input_count = node.input_count();
    let start = if self.stack[entry].input_index < input_count {
      self.stack[entry].input_index
    } else {
      0
    };

    // 必要时递归输入 (先 start..count, 再 0..start)
    for i in start..input_count {
      if self.recurse_input(node, entry, i)? {
        return Ok(());
      }
    }
    for i in 0..start {
      if self.recurse_input(node, entry, i)? {
        return Ok(());
      }
    }

    // 记住归约前的最大节点 id
    let max_id = self.handle.node_count() as NodeId - 1;

    // 全部输入已访问或在栈上, 应用归约
    let reduction = self.reduce(node)?;
    if !reduction.is_changed() {
      self.pop();
      return Ok(());
    }

    // Changed ⇒ Some (构造不变量; None 分支为 debug 兜底)
    let Some(replacement) = reduction.replacement() else {
      debug_assert!(false, "Changed 必然携带 replacement");
      self.pop();
      return Ok(());
    };

    if ptr_eq(Some(replacement), Some(node)) {
      // 就地更新: 所有使用者回访
      for user in node.uses() {
        debug_assert!(!ptr_eq(Some(user), Some(node)) || self.state.get(node) != State::Visited);
        self.revisit(user);
      }
      // 可能需要再递归新输入
      let input_count = node.input_count();
      for i in 0..input_count {
        if self.recurse_input(node, entry, i)? {
          return Ok(());
        }
      }
    }

    // 归约后弹出
    self.pop();

    if !ptr_eq(Some(replacement), Some(node)) {
      self.replace_max(node, replacement, max_id)?;
    }
    Ok(())
  }

  /// ReduceTop 的输入递归原语: 下压一个未访问输入则返回 true 并要求外层返回
  fn recurse_input(&mut self, node: &'z Node, entry: usize, i: usize) -> Result<bool> {
    let Some(input) = node.input_at(i) else {
      // 良性图不变量: 输入非空; None 为别名破坏 (原版 DCHECK 域)
      debug_assert!(false, "输入槽为空");
      return Ok(false);
    };
    if !ptr_eq(Some(input), Some(node)) && self.recurse(input) {
      self.stack[entry].input_index = i + 1;
      return Ok(true);
    }
    Ok(false)
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::Replace
  pub fn replace(&mut self, node: &'z Node, replacement: &'z Node) -> Result<()> {
    self.replace_max(node, replacement, NodeId::MAX)
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::Replace (max_id 版)
  pub fn replace_max(
    &mut self,
    node: &'z Node,
    replacement: &'z Node,
    max_id: NodeId,
  ) -> Result<()> {
    if ptr_eq(Some(node), Some(replacement)) {
      return Ok(());
    }
    if ptr_eq(self.handle.start(), Some(node)) {
      self.handle.set_start(replacement);
    }
    if ptr_eq(self.handle.end(), Some(node)) {
      self.handle.set_end(replacement);
    }
    if replacement.id() <= max_id {
      // replacement 是旧节点: 摘除 node 全部使用, 假定 replacement 已归约完
      for edge in node.use_edges() {
        let user = edge.from();
        // 原版为 DCHECK 面; 转写为常态校验并以 Result 上抛
        Verifier::verify_edge_input_replacement(&edge, replacement)?;
        edge.update_to(Some(replacement));
        // 自引用使用者不回访
        if !ptr_eq(Some(user), Some(node)) {
          self.revisit(user);
        }
      }
      node.kill();
    } else {
      // 仅替换旧使用者, 允许本次归约新建的节点继续引用 node
      for edge in node.use_edges() {
        let user = edge.from();
        if user.id() <= max_id {
          edge.update_to(Some(replacement));
          if !ptr_eq(Some(user), Some(node)) {
            self.revisit(user);
          }
        }
      }
      // 不再被使用则摘除 node
      if node.use_count() == 0 {
        node.kill();
      }
      // 有替换则归约后递归 replacement
      self.recurse(replacement);
    }
    Ok(())
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::ReplaceWithValue
  pub fn replace_with_value(
    &mut self,
    node: &'z Node,
    value: &'z Node,
    mut effect: Option<&'z Node>,
    mut control: Option<&'z Node>,
  ) -> Result<()> {
    if effect.is_none() && node.op().effect_input_count() > 0 {
      effect = Some(NodeProperties::get_effect_input(node, 0)?);
    }
    if control.is_none() && node.op().control_input_count() > 0 {
      control = Some(NodeProperties::get_control_input(node, 0)?);
    }

    // 值/效应/控制三类边分拆改写
    for edge in node.use_edges() {
      let user = edge.from();
      debug_assert!(!user.is_dead());
      if NodeProperties::is_control_edge(&edge)? {
        if user.opcode() == IrOpcode::IfSuccess {
          let control = control.ok_or(Error::MissingEdgeReplacement {
            node_id: user.id(),
            kind: "Control",
          })?;
          self.replace(user, control)?;
        } else if user.opcode() == IrOpcode::IfException {
          let dead = self.dead.ok_or(Error::MissingEdgeReplacement {
            node_id: user.id(),
            kind: "Dead",
          })?;
          edge.update_to(Some(dead));
          self.revisit(user);
        } else {
          let control = control.ok_or(Error::MissingEdgeReplacement {
            node_id: user.id(),
            kind: "Control",
          })?;
          edge.update_to(Some(control));
          self.revisit(user);
        }
      } else if NodeProperties::is_effect_edge(&edge)? {
        let effect = effect.ok_or(Error::MissingEdgeReplacement {
          node_id: user.id(),
          kind: "Effect",
        })?;
        edge.update_to(Some(effect));
        self.revisit(user);
      } else {
        edge.update_to(Some(value));
        self.revisit(user);
      }
    }
    Ok(())
  }

  /// v8: src/compiler/graph-reducer.h AdvancedReducer::RelaxEffectsAndControls
  /// 原文: ReplaceWithValue(node, node, nullptr, nullptr)
  pub fn relax_effects_and_controls(&mut self, node: &'z Node) -> Result<()> {
    self.replace_with_value(node, node, None, None)
  }

  /// v8: src/compiler/graph-reducer.h AdvancedReducer::RelaxControls
  /// 原文: ReplaceWithValue(node, node, node, control)
  pub fn relax_controls(&mut self, node: &'z Node, control: Option<&'z Node>) -> Result<()> {
    self.replace_with_value(node, node, Some(node), control)
  }

  /// v8: src/compiler/graph-reducer.h AdvancedReducer::MergeControlToEnd
  /// 原文: NodeProperties::MergeControlToEnd + Revisit(graph->end())
  pub fn merge_control_to_end(
    &mut self,
    common: &v8_compiler_ops::CommonOperatorBuilder<'z, 'a>,
    node: &'z Node,
  ) -> Result<()> {
    merge_control_to_end(self.handle, common, node)?;
    let end = self
      .handle
      .end()
      .ok_or(Error::GraphUninitialized { which: "end" })?;
    self.revisit(end);
    Ok(())
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::Pop
  fn pop(&mut self) {
    debug_assert!(!self.stack.is_empty());
    if let Some(ns) = self.stack.pop() {
      self.state.set(ns.node, State::Visited);
    }
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::Push
  fn push(&mut self, node: &'z Node) {
    debug_assert_ne!(self.state.get(node), State::OnStack);
    self.state.set(node, State::OnStack);
    self.stack.push(NodeState {
      node,
      input_index: 0,
    });
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::Recurse
  /// 原文: state > kRevisit → false; 否则入栈 true
  fn recurse(&mut self, node: &'z Node) -> bool {
    if self.state.get(node) > State::Revisit {
      return false;
    }
    self.push(node);
    true
  }

  /// v8: src/compiler/graph-reducer.cc GraphReducer::Revisit
  /// 原文: kVisited → kRevisit 入队
  pub fn revisit(&mut self, node: &'z Node) {
    if self.state.get(node) == State::Visited {
      self.state.set(node, State::Revisit);
      self.revisit.push_back(node);
    }
  }
}

/// 节点指针同一性 (冻结层 Node 未定义 PartialEq, 引用即身份)
pub(crate) fn ptr_eq(a: Option<&Node>, b: Option<&Node>) -> bool {
  match (a, b) {
    (Some(x), Some(y)) => eq(x, y),
    (None, None) => true,
    _ => false,
  }
}
