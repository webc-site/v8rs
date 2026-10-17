// 由 tool/src/sandbox/js-dispatch-table.h.js 从 v8/src/sandbox/js-dispatch-table-inl.h 生成; 修订就地改, 复跑覆盖
// JSDispatchTable 内联访问骨架

// 布局常量实值来自 src/common/globals.h (双目标分支); 表基类由
// tool/src/sandbox/external-entity-table.js 覆盖。

/// 原 JSDispatchHandle = StrongAlias<uint32_t>
pub type JSDispatchHandle = u32;

/// 原 kNullJSDispatchHandle
pub const NULL_JS_DISPATCH_HANDLE: JSDispatchHandle = 0;

/// 表项大小双分支:
/// - x64 + LA57 (五级页表): 32 字节 (log2 = 5)
/// - 其他目标 (arm64 / x64 非 LA57): 16 字节 (log2 = 4)
pub mod layout {
  pub const ENTRY_SIZE_LA57: usize = 32;
  pub const ENTRY_SIZE_LOG2_LA57: u32 = 5;
  pub const ENTRY_SIZE: usize = 16;
  pub const ENTRY_SIZE_LOG2: u32 = 4;
}

/// 原 JSDispatchTable 访问器面 (inl 侧 14 处句柄入参方法族):
// - GetCode
// - SetCodeNoWriteBarrier
// - SetCode
// - SetCodeKeepTieringRequest
// - SetTieringRequest
// - IsTieringRequested
// - ResetTieringRequest
// - GetEntrypoint
// - GetCodeAddress
// - GetParameterCount
// - Mark
// - IsMarked
// - VerifyEntry

/// 原 JSDispatchEntry 方法注册表 (inl 侧 14 处定义):
// - MakeJSDispatchEntry
// - GetEntrypoint
// - GetCodePointer
// - GetCode
// - GetParameterCount
// - SetCodeAndEntrypointPointer
// - SetEntrypointPointer
// - MakeFreelistEntry
// - IsFreelistEntry
// - GetNextFreelistEntryIndex
// - Mark
// - Unmark
// - IsMarked

/// 原 MakeJSDispatchEntry 载荷编码双分支:
/// - x64+LA57: payload = parameter_count (object 存独立 code_object_ 字)
/// - 其他目标: payload = ((object - kObjectPointerOffset) << kObjectPointerShift)
///             | (parameter_count & kParameterCountMask)
/// 语义实现登记跳过。
pub struct JSDispatchEntry;

// TODO(dev): 按 encoded_word_ 布局补 GetCodePointer/SetCodeAndEntrypointPointer。

// —— 提取不变式断言 ——
const _: () = {
  assert!(layout::ENTRY_SIZE_LA57 == 32);
  assert!(layout::ENTRY_SIZE == 16);
  assert!(layout::ENTRY_SIZE_LOG2_LA57 == 5);
  assert!(layout::ENTRY_SIZE_LOG2 == 4);
};
