// 由 tool/src/builtins/array-map.tq.js 从 v8/src/builtins/array-map.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-collections; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 9 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "array", "namespace array"),
  (
    "builtin",
    "ArrayMapPreLoopLazyDeoptContinuation",
    "builtin ArrayMapPreLoopLazyDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, thisArg: JSAny, length: JSAny, result: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayMapLoopEagerDeoptContinuation",
    "builtin ArrayMapLoopEagerDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, thisArg: JSAny, array: JSAny, initialK: JSAny, length: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayMapLoopLazyDeoptContinuation",
    "builtin ArrayMapLoopLazyDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, thisArg: JSAny, array: JSAny, initialK: JSAny, length: JSAny, result: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayMapLoopContinuation",
    "builtin ArrayMapLoopContinuation( implicit context: Context)(_receiver: JSReceiver, callbackfn: Callable, thisArg: JSAny, array: JSReceiver, o: JSReceiver, initialK: Number, length: Number): JSAny",
  ),
  ("struct", "Vector", "struct Vector"),
  (
    "macro",
    "NewVector",
    "macro NewVector(implicit context: Context)(length: Smi): Vector",
  ),
  (
    "macro",
    "FastArrayMap",
    "macro FastArrayMap( implicit context: Context)(fastO: FastJSArrayForRead, len: Smi, callbackfn: Callable, thisArg: JSAny): JSArray labels Bailout(JSArray, Smi)",
  ),
  (
    "builtin",
    "ArrayMap",
    "builtin ArrayMap( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 9);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 2);
};
