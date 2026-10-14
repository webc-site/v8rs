// 由 tool/src/builtins/array-find.tq.js 从 v8/src/builtins/array-find.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-array; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 7 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "array", "namespace array"),
  (
    "builtin",
    "ArrayFindLoopEagerDeoptContinuation",
    "builtin ArrayFindLoopEagerDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, thisArg: JSAny, initialK: JSAny, length: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFindLoopLazyDeoptContinuation",
    "builtin ArrayFindLoopLazyDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(_callback: JSAny, _thisArg: JSAny, _initialK: JSAny, _length: JSAny, _result: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFindLoopAfterCallbackLazyDeoptContinuation",
    "builtin ArrayFindLoopAfterCallbackLazyDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, thisArg: JSAny, initialK: JSAny, length: JSAny, foundValue: JSAny, isFound: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFindLoopContinuation",
    "builtin ArrayFindLoopContinuation( implicit context: Context)(_receiver: JSReceiver, callbackfn: Callable, thisArg: JSAny, o: JSReceiver, initialK: Number, length: Number): JSAny",
  ),
  (
    "macro",
    "FastArrayFind",
    "macro FastArrayFind( implicit context: Context)(o: JSReceiver, len: Number, callbackfn: Callable, thisArg: JSAny): JSAny labels Bailout(Smi)",
  ),
  (
    "builtin",
    "ArrayPrototypeFind",
    "builtin ArrayPrototypeFind( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
};
