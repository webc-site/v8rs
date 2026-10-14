// 由 tool/src/builtins/promise-finally.tq.js 从 v8/src/builtins/promise-finally.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-promise; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 22 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "promise", "namespace promise"),
  (
    "type",
    "PromiseValueThunkOrReasonContext",
    "type PromiseValueThunkOrReasonContext extends FunctionContext",
  ),
  (
    "enum",
    "PromiseValueThunkOrReasonContextSlot",
    "enum PromiseValueThunkOrReasonContextSlot extends intptr constexpr 'PromiseBuiltins::PromiseValueThunkOrReasonContextSlot'",
  ),
  (
    "type",
    "PromiseFinallyContext",
    "type PromiseFinallyContext extends FunctionContext",
  ),
  (
    "enum",
    "PromiseFinallyContextSlot",
    "enum PromiseFinallyContextSlot extends intptr constexpr 'PromiseBuiltins::PromiseFinallyContextSlot'",
  ),
  (
    "builtin",
    "PromiseValueThunkFinally",
    "builtin PromiseValueThunkFinally( js-implicit context: Context, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "PromiseThrowerFinally",
    "builtin PromiseThrowerFinally( js-implicit context: Context, receiver: JSAny)(): never",
  ),
  (
    "const",
    "kPromiseThrowerFinallySharedFun",
    "const kPromiseThrowerFinallySharedFun: constexpr intptr generates 'RootIndex::PromiseThrowerFinallySharedFun'",
  ),
  (
    "macro",
    "CreateThrowerFunction",
    "macro CreateThrowerFunction( implicit context: Context)(nativeContext: NativeContext, reason: JSAny): JSFunction",
  ),
  (
    "builtin",
    "PromiseCatchFinally",
    "builtin PromiseCatchFinally( js-implicit context: Context, receiver: JSAny)(reason: JSAny): JSAny",
  ),
  (
    "const",
    "kPromiseValueThunkFinallySharedFun",
    "const kPromiseValueThunkFinallySharedFun: constexpr intptr generates 'RootIndex::PromiseValueThunkFinallySharedFun'",
  ),
  (
    "macro",
    "CreateValueThunkFunction",
    "macro CreateValueThunkFunction( implicit context: Context)(nativeContext: NativeContext, value: JSAny): JSFunction",
  ),
  (
    "builtin",
    "PromiseThenFinally",
    "builtin PromiseThenFinally( js-implicit context: Context, receiver: JSAny)(value: JSAny): JSAny",
  ),
  (
    "struct",
    "PromiseFinallyFunctions",
    "struct PromiseFinallyFunctions",
  ),
  (
    "const",
    "kPromiseThenFinallySharedFun",
    "const kPromiseThenFinallySharedFun: constexpr intptr generates 'RootIndex::PromiseThenFinallySharedFun'",
  ),
  (
    "const",
    "kPromiseCatchFinallySharedFun",
    "const kPromiseCatchFinallySharedFun: constexpr intptr generates 'RootIndex::PromiseCatchFinallySharedFun'",
  ),
  (
    "macro",
    "CreatePromiseFinallyFunctions",
    "macro CreatePromiseFinallyFunctions( implicit context: Context)(nativeContext: NativeContext, onFinally: Callable, constructor: Constructor): PromiseFinallyFunctions",
  ),
  (
    "builtin",
    "PromisePrototypeFinally",
    "builtin PromisePrototypeFinally( js-implicit context: Context, receiver: JSAny)(onFinally: JSAny): JSAny",
  ),
  (
    "macro",
    "PromiseCatchFinallySharedFunConstant",
    "macro PromiseCatchFinallySharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "macro",
    "PromiseThenFinallySharedFunConstant",
    "macro PromiseThenFinallySharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "macro",
    "PromiseThrowerFinallySharedFunConstant",
    "macro PromiseThrowerFinallySharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "macro",
    "PromiseValueThunkFinallySharedFunConstant",
    "macro PromiseValueThunkFinallySharedFunConstant(): SharedFunctionInfo",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kPromiseThrowerFinallySharedFun",
    "RootIndex::PromiseThrowerFinallySharedFun",
  ),
  (
    "kPromiseValueThunkFinallySharedFun",
    "RootIndex::PromiseValueThunkFinallySharedFun",
  ),
  (
    "kPromiseThenFinallySharedFun",
    "RootIndex::PromiseThenFinallySharedFun",
  ),
  (
    "kPromiseCatchFinallySharedFun",
    "RootIndex::PromiseCatchFinallySharedFun",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 22);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "enum").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 1);
};
