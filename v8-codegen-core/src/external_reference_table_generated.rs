// 由 tool/src/codegen/external-reference-table.h.js 从 v8/src/codegen/external-reference-table.{h,cc} 生成; 修订就地改, 复跑覆盖
// 外部引用表布局骨架

// 布局计数 (原 static constexpr int k*; 表达式保持原样, 求值依赖同 crate 产物):
/// 原 kSpecialReferenceCount
pub const SPECIAL_REFERENCE_COUNT: i32 = 1;
/// 原 kBuiltinsReferenceCount
pub const BUILTINS_REFERENCE_COUNT_EXPR: &str =
  "#define COUNT_C_BUILTIN(...) +1 BUILTIN_LIST_C(COUNT_C_BUILTIN)";
/// 原 kIsolateFieldReferenceCount = kNumIsolateFieldIds (isolate-data-fields.h)
pub const ISOLATE_FIELD_REFERENCE_COUNT_EXPR: &str = "kNumIsolateFieldIds";
/// 原 kStubCacheReferenceCount = 6 * 3 (3 组 stub cache)
pub const STUB_CACHE_REFERENCE_COUNT: i32 = 6 * 3;

/// 表项初始化状态 (原 enum InitializationState : uint32_t)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum InitializationState {
  /// kUninitialized
  Uninitialized = 0,
  /// kInitializedIsolateIndependent
  InitializedIsolateIndependent = 1,
  /// kInitialized
  Initialized = 2,
}

/// 原类面方法 (Add*/Copy*/Get* 族, 16 条):
// - address
// - InitializeOncePerIsolateGroup
// - InitIsolateIndependent
// - Init
// - AddIsolateIndependent
// - AddIsolateIndependentReferences
// - AddBuiltins
// - AddRuntimeFunctions
// - AddAccessors
// - Add
// - CopyIsolateIndependentReferences
// - AddIsolateDependentReferences
// - AddIsolateFields
// - AddStubCache
// - GetStatsCounterAddress
// - AddNativeCodeStatsCounters

/// 原 ref_addr_[kSize] / ref_name_[kSize] 双平行数组
pub struct ExternalReferenceTable;

// TODO(dev): 补表构建 (isolate 独立/依赖分相填充)。

// —— 提取不变式断言 ——
const _: () = {
  assert!(SPECIAL_REFERENCE_COUNT == 1);
  assert!(STUB_CACHE_REFERENCE_COUNT == 18);
  assert!(InitializationState::Uninitialized as u32 == 0);
};
