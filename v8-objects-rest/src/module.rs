// 由 tool/src/objects/module.h.js 从 v8/src/objects/module.{h,tq} 生成; 修订就地改, 复跑覆盖
// Module 状态机枚举与命名空间布局索引
//! Module: 模块生命周期状态与命名空间对象布局索引。

/// 模块生命周期状态 (原版 Module::Status, 顺序敏感; 共 8 态)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Status {
  /// kUnlinked
  Unlinked = 0,
  /// kPreLinking
  Prelinking = 1,
  /// kLinking
  Linking = 2,
  /// kLinked
  Linked = 3,
  /// kEvaluating
  Evaluating = 4,
  /// kEvaluatingAsync
  Evaluatingasync = 5,
  /// kEvaluated
  Evaluated = 6,
  /// kErrored
  Errored = 7,
}

impl Status {
  /// 状态总数
  pub const COUNT: usize = 8;

  /// 数值反查
  #[inline]
  #[must_use]
  pub const fn from_repr(v: u8) -> Option<Self> {
    match v {
      0 => Some(Self::Unlinked),
      1 => Some(Self::Prelinking),
      2 => Some(Self::Linking),
      3 => Some(Self::Linked),
      4 => Some(Self::Evaluating),
      5 => Some(Self::Evaluatingasync),
      6 => Some(Self::Evaluated),
      7 => Some(Self::Errored),
      _ => None,
    }
  }
}

/// JSModuleNamespace in-object 字段索引 (同时是描述符索引; 原版匿名 enum)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum NamespaceField {
  /// `toStringTag` 字段。
  ToStringTag = 0,
}
/// in-object 字段总数 (原版 kInObjectFieldCount)。
pub const NAMESPACE_IN_OBJECT_FIELD_COUNT: usize = 1;

/// JSModuleNamespace::kSize 公式 (原版: kHeaderSize + kTaggedSize * kInObjectFieldCount)。
/// kHeaderSize = sizeof(JSModuleNamespace), 由 Developer 就地落值。
#[inline]
#[must_use]
pub const fn namespace_size(header_size: usize, tagged_size: usize) -> usize {
  header_size + tagged_size * NAMESPACE_IN_OBJECT_FIELD_COUNT
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(Status::COUNT == 8);
  assert!(Status::Unlinked as u8 == 0);
  assert!(Status::Errored as u8 == 7);
  assert!(NAMESPACE_IN_OBJECT_FIELD_COUNT == 1);
};
