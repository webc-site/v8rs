// 由 tool/src/builtins/array-filter.tq.js 从 v8/src/builtins/array-filter.tq 生成; 修订就地改, 复跑覆盖
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
    "ArrayFilterLoopEagerDeoptContinuation",
    "builtin ArrayFilterLoopEagerDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, thisArg: JSAny, array: JSAny, initialK: JSAny, length: JSAny, initialTo: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFilterLoopLazyDeoptContinuation",
    "builtin ArrayFilterLoopLazyDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, thisArg: JSAny, array: JSAny, initialK: JSAny, length: JSAny, valueK: JSAny, initialTo: JSAny, result: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFilterLoopContinuation",
    "builtin ArrayFilterLoopContinuation( implicit context: Context)(_receiver: JSReceiver, callbackfn: Callable, thisArg: JSAny, array: JSReceiver, o: JSReceiver, initialK: Number, length: Number, initialTo: Number): JSAny",
  ),
  (
    "macro",
    "FastArrayFilter",
    "macro FastArrayFilter( implicit context: Context)(fastO: FastJSArray, len: Smi, callbackfn: Callable, thisArg: JSAny, output: FastJSArray): void labels Bailout(Number, Number)",
  ),
  (
    "macro",
    "FastFilterSpeciesCreate",
    "macro FastFilterSpeciesCreate( implicit context: Context)(receiver: JSReceiver): JSReceiver labels Slow",
  ),
  (
    "builtin",
    "ArrayFilter",
    "builtin ArrayFilter( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 2);
};
