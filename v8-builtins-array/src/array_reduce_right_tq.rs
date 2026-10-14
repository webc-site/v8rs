// 由 tool/src/builtins/array-reduce-right.tq.js 从 v8/src/builtins/array-reduce-right.tq 生成; 修订就地改, 复跑覆盖
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
    "ArrayReduceRightPreLoopEagerDeoptContinuation",
    "builtin ArrayReduceRightPreLoopEagerDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, length: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayReduceRightLoopEagerDeoptContinuation",
    "builtin ArrayReduceRightLoopEagerDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, initialK: JSAny, length: JSAny, accumulator: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayReduceRightLoopLazyDeoptContinuation",
    "builtin ArrayReduceRightLoopLazyDeoptContinuation( js-implicit context: NativeContext, receiver: JSAny)(callback: JSAny, initialK: JSAny, length: JSAny, result: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayReduceRightLoopContinuation",
    "builtin ArrayReduceRightLoopContinuation( implicit context: Context)(_receiver: JSReceiver, callbackfn: Callable, initialAccumulator: JSAny|TheHole, o: JSReceiver, initialK: Number, _length: Number): JSAny",
  ),
  (
    "macro",
    "FastArrayReduceRight",
    "macro FastArrayReduceRight( implicit context: Context)(o: JSReceiver, len: Number, callbackfn: Callable, initialAccumulator: JSAny|TheHole): JSAny labels Bailout(Number, JSAny|TheHole)",
  ),
  (
    "builtin",
    "ArrayReduceRight",
    "builtin ArrayReduceRight( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
};
