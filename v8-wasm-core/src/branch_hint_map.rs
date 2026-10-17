// 由 tool/src/wasm/branch-hint-map.h.js 从 v8/src/wasm/branch-hint-map.h 生成; 修订就地改, 复跑覆盖
// 分支提示映射 (BranchHint 来自 src/common/globals.h, 已在 common 侧覆盖)

use std::collections::HashMap;

/// 分支提示种类 (对应 enum class BranchHint : uint8_t { kNone, kTrue, kFalse }, 源 src/common/globals.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum BranchHint {
  #[default]
  None = 0,
  True = 1,
  False = 2,
}

/// 偏移 → 分支提示 (对应 class BranchHintMap, 骨架)
#[derive(Debug, Clone, Default)]
pub struct BranchHintMap {
  map_: HashMap<u32, BranchHint>,
}

impl BranchHintMap {
  /// 对应 insert(offset, hint)
  pub fn insert(&mut self, offset: u32, hint: BranchHint) {
    self.map_.insert(offset, hint);
  }

  /// 对应 GetHintFor(offset); 无记录返回 kNone
  #[must_use]
  pub fn get_hint_for(&self, offset: u32) -> BranchHint {
    self.map_.get(&offset).copied().unwrap_or(BranchHint::None)
  }

  /// 对应 NumHintsForTesting()
  #[must_use]
  pub fn num_hints_for_testing(&self) -> usize {
    self.map_.len()
  }
}

/// 函数偏移 → 提示映射 (对应 using BranchHintInfo)
pub type BranchHintInfo = HashMap<u32, BranchHintMap>;

// —— 提取不变式断言 ——
const _: () = {
  assert!(BranchHint::None as u8 == 0);
  assert!(BranchHint::True as u8 == 1);
  assert!(BranchHint::False as u8 == 2);
};
