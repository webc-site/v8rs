// 由 tool/src/snapshot/builtins-effects-dummy.js 从 v8/src/snapshot/builtins-effects-dummy.cc 生成; 修订就地改, 复跑覆盖
// mksnapshot 专用内建副作用桩

/// mksnapshot 链接版 BuiltinCanAllocate (原版 builtins-effects-dummy.cc):
/// mksnapshot 期间保守假设所有内建均可分配内存, 恒返回 true。
/// 正式版由 mksnapshot 生成至 gen 产物 builtins-effects.cc,
/// V8/D8 链接生成版, 本桩仅 mksnapshot 链接。
#[must_use]
pub const fn builtin_can_allocate(_builtin: u32) -> bool {
  true
}
