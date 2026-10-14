//! MaglevInterpreterFrameState / MergePointInterpreterFrameState
//! (maglev-interpreter-frame-state.h/.cc) — 合并点帧状态数据面接缝。
//!
//! 破环: 原版 `New`/`NewForLoop`/`NewForCatchBlock`/`NewForPeel`/`Merge`/`InitializeLoop`/
//! `MergeThrow` 依赖 `MaglevCompilationUnit` / `Graph` IR 宿主 / `BytecodeLivenessState` /
//! `LoopInfo` / `KnownNodeAspects` 等反馈向量与 Isolate 面 → 深水, 逐原函数名登记
//! readme/zh.md 台账。本模块承接前端 Graph `BasicBlockRec::state` 所需的引用类型与
//! 无宿主依赖的纯数据面 (枚举/槽位/计数)。

use crate::graph::NodeRef;

/// v8: maglev-interpreter-frame-state.h:293-298 enum class BasicBlockType
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BasicBlockType {
  /// v8: :294 kDefault
  Default,
  /// v8: :295 kLoopHeader
  LoopHeader,
  /// v8: :296 kExceptionHandlerStart
  ExceptionHandlerStart,
  /// v8: :297 kUnusedExceptionHandlerStart
  UnusedExceptionHandlerStart,
}

/// v8: maglev-interpreter-frame-state.h ContextScopeInfo (optional<pair<Context,ScopeInfo>>)
/// Graph 侧以 `Option<u32>` 接缝 (context_may_alias), 此处承接结构化最小面。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContextScopeInfo {
  /// v8: 首元 Context 节点位
  pub context: NodeRef,
  /// v8: 次元 ScopeInfo 位 (原版 std::optional 承载)
  pub scope_info: u32,
}

/// v8: maglev-interpreter-frame-state.h:291 MergePointInterpreterFrameState* 的 arena 接缝。
/// 原版为 zone 分配指针 (承载完整 merge 状态机), 此处为索引化引用位,
/// 实体在 builder 席 (依赖 Isolate/反馈向量, 台账登记)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MergePointFrameRef(pub u32);

/// v8: maglev-interpreter-frame-state.cc:MergeDead 计数面 (纯数据, 无宿主)
/// 承接 predecessor 收支的标量核, 深水 Merge 体在 builder 席。
#[derive(Debug, Clone, Copy, Default)]
pub struct MergePointCounters {
  /// v8: predecessor_count_
  pub predecessor_count: i32,
  /// v8: predecessors_so_far_
  pub predecessors_so_far: i32,
}

impl MergePointCounters {
  /// v8: cc MergeDead 的 predecessor 收支 (num 计入 so_far)
  /// DCHECK_GE/GT 语义在 debug 构建以 debug_assert 承接。
  pub fn merge_dead(&mut self, num: i32) {
    debug_assert!(self.predecessor_count >= num, "MergeDead: count < num");
    debug_assert!(
      self.predecessors_so_far < self.predecessor_count,
      "MergeDead: 已合并完"
    );
    self.predecessor_count -= num;
    self.predecessors_so_far += num;
  }
}
