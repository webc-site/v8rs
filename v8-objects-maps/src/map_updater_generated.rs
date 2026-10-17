// 由 tool/src/objects/map-updater.h.js 从 v8/src/objects/map-updater.{h,cc} 生成; 修订就地改, 复跑覆盖
// MapUpdater 状态机 (maps 已有手写同名模块 → _generated 落点)
//! MapUpdater 内部状态机。
//! 本文件为生成基线; 语义修订归 `crate::map_updater` (手写模块, 本文件禁手改混淆)。

/// 更新流程状态 (原版 enum State, private; 共 5 态)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum State {
  /// kInitialized
  Initialized = 0,
  /// kAtRootMap
  Atrootmap = 1,
  /// kAtTargetMap
  Attargetmap = 2,
  /// kAtIntegrityLevelSource
  Atintegritylevelsource = 3,
  /// kEnd
  End = 4,
}

/// 实例类型变更场景 (原版 enum class InstanceTypeChange)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum InstanceTypeChange {
  /// kTypedArrayDetaching
  TypedArrayDetaching = 0,
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(State::Initialized as u8 == 0);
  assert!(State::End as u8 == 4);
};

const STATE_COUNT: usize = 5;
const _: () = { assert!(STATE_COUNT == 5); };
