// 由 tool/src/objects/dependent-code.h.js 从 v8/src/objects/dependent-code.{h,cc} 生成; 修订就地改, 复跑覆盖
// DependentCode 依赖组位标志与条目布局
//! DependentCode: {Code, DependencyGroups} 弱数组; 组位标志 11 组。

/// 依赖组位标志 (原版 enum DependencyGroup; 末位须容于 Smi 31 位)。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum DependencyGroup {
  /// kTransitionGroup → "transition"
  Transitiongroup = 1 << 0,
  /// kPrototypeCheckGroup → "prototype-check"
  Prototypecheckgroup = 1 << 1,
  /// kPropertyCellChangedGroup → "property-cell-changed"
  Propertycellchangedgroup = 1 << 2,
  /// kFieldTypeGroup → "field-type"
  Fieldtypegroup = 1 << 3,
  /// kFieldConstGroup → "field-const"
  Fieldconstgroup = 1 << 4,
  /// kFieldRepresentationGroup → "field-representation"
  Fieldrepresentationgroup = 1 << 5,
  /// kInitialMapChangedGroup → "initial-map-changed"
  Initialmapchangedgroup = 1 << 6,
  /// kAllocationSiteTenuringChangedGroup → "allocation-site-tenuring-changed"
  Allocationsitetenuringchangedgroup = 1 << 7,
  /// kAllocationSiteTransitionChangedGroup → "allocation-site-transition-changed"
  Allocationsitetransitionchangedgroup = 1 << 8,
  /// kContextCellChangedGroup → "script-context-slot-property-changed"
  Contextcellchangedgroup = 1 << 9,
  /// kEmptyContextExtensionGroup → "empty-context-extension"
  Emptycontextextensiongroup = 1 << 10,
}

/// 组位掩码总数 (原版注释: 最后一位必须容于 Smi)。
pub const DEPENDENCY_GROUP_BITS: u32 = 11;

/// 每条目槽位数 (code 弱引用 + groups Smi)。
pub const SLOTS_PER_ENTRY: usize = 2;
/// 条目内 code 槽偏移。
pub const CODE_SLOT_OFFSET: usize = 0;
/// 条目内 groups 槽偏移。
pub const GROUPS_SLOT_OFFSET: usize = 1;

/// 空依赖代码表根索引名 (原版 kEmptyDependentCode = RootIndex::EmptyWeakArrayList)。
pub const EMPTY_DEPENDENT_CODE_ROOT: &str = "kEmptyWeakArrayList";

/// 条目数 → 槽位数 (原版 LengthFor)。
#[inline]
#[must_use]
pub const fn length_for(number_of_entries: usize) -> usize {
  number_of_entries * SLOTS_PER_ENTRY
}

// —— 提取不变式断言 ——
const _: () = {
  assert!(DEPENDENCY_GROUP_BITS == 11);
  assert!(DependencyGroup::TransitionGroup as u32 == 1);
  assert!(DependencyGroup::EmptyContextExtensionGroup as u32 == 1 << 10);
  assert!(SLOTS_PER_ENTRY == 2 && CODE_SLOT_OFFSET == 0 && GROUPS_SLOT_OFFSET == 1);
};
