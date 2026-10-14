// 由 tool/src/builtins/promise-all.tq.js 从 v8/src/builtins/promise-all.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 18 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "promise", "namespace promise"),
  (
    "const",
    "kPromiseBuiltinsPromiseContextLength",
    "const kPromiseBuiltinsPromiseContextLength: constexpr int31 generates 'PromiseBuiltins::kPromiseContextLength'",
  ),
  (
    "macro",
    "CreatePromiseAllResolveElementContext",
    "macro CreatePromiseAllResolveElementContext( implicit context: Context)(capability: PromiseCapability, nativeContext: NativeContext): PromiseAllResolveElementContext",
  ),
  (
    "macro",
    "CreatePromiseAllResolveElementFunction",
    "macro CreatePromiseAllResolveElementFunction( implicit context: Context)( resolveElementContext: PromiseAllResolveElementContext, index: Smi, resolveFunction: constexpr intptr): JSFunction",
  ),
  (
    "macro",
    "CreatePromiseResolvingFunctionsContext",
    "macro CreatePromiseResolvingFunctionsContext( implicit context: Context)(promise: JSPromise, debugEvent: Boolean, nativeContext: NativeContext): PromiseResolvingFunctionContext",
  ),
  (
    "macro",
    "IsPromiseThenLookupChainIntact",
    "macro IsPromiseThenLookupChainIntact( implicit context: Context)(nativeContext: NativeContext, receiverMap: Map): bool",
  ),
  (
    "const",
    "kPromiseAllResolveElementClosureSharedFun",
    "const kPromiseAllResolveElementClosureSharedFun: constexpr intptr generates 'static_cast<intptr_t>(RootIndex::PromiseAllResolveElementClosureSharedFun)'",
  ),
  (
    "struct",
    "PromiseAllResolveElementFunctor",
    "struct PromiseAllResolveElementFunctor",
  ),
  (
    "struct",
    "PromiseAllRejectElementFunctor",
    "struct PromiseAllRejectElementFunctor",
  ),
  (
    "const",
    "kPromiseAllSettledResolveElementClosureSharedFun",
    "const kPromiseAllSettledResolveElementClosureSharedFun: constexpr intptr generates 'static_cast<intptr_t>(RootIndex::PromiseAllSettledResolveElementClosureSharedFun)'",
  ),
  (
    "struct",
    "PromiseAllSettledResolveElementFunctor",
    "struct PromiseAllSettledResolveElementFunctor",
  ),
  (
    "const",
    "kPromiseAllSettledRejectElementClosureSharedFun",
    "const kPromiseAllSettledRejectElementClosureSharedFun: constexpr intptr generates 'static_cast<intptr_t>(RootIndex::PromiseAllSettledRejectElementClosureSharedFun)'",
  ),
  (
    "struct",
    "PromiseAllSettledRejectElementFunctor",
    "struct PromiseAllSettledRejectElementFunctor",
  ),
  (
    "macro",
    "PerformPromiseAll",
    "macro PerformPromiseAll<F1: type, F2: type>( implicit context: Context)(nativeContext: NativeContext, iter: iterator::IteratorRecord, constructor: Constructor, capability: PromiseCapability, promiseResolveFunction: JSAny, createResolveEleme…",
  ),
  (
    "macro",
    "GeneratePromiseAll",
    "macro GeneratePromiseAll<F1: type, F2: type>( implicit context: Context)(receiver: JSAny, iterable: JSAny, createResolveElementFunctor: F1, createRejectElementFunctor: F2, message: constexpr string): JSAny",
  ),
  (
    "builtin",
    "PromiseAll",
    "builtin PromiseAll( js-implicit context: Context, receiver: JSAny)(iterable: JSAny): JSAny",
  ),
  (
    "builtin",
    "PromiseAllSettled",
    "builtin PromiseAllSettled( js-implicit context: Context, receiver: JSAny)(iterable: JSAny): JSAny",
  ),
  (
    "macro",
    "MakeFixedArrayCOW",
    "macro MakeFixedArrayCOW(FixedArray): void",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kPromiseBuiltinsPromiseContextLength",
    "PromiseBuiltins::kPromiseContextLength",
  ),
  (
    "kPromiseAllResolveElementClosureSharedFun",
    "static_cast<intptr_t>(RootIndex::PromiseAllResolveElementClosureSharedFun)",
  ),
  (
    "kPromiseAllSettledResolveElementClosureSharedFun",
    "static_cast<intptr_t>(RootIndex::PromiseAllSettledResolveElementClosureSharedFun)",
  ),
  (
    "kPromiseAllSettledRejectElementClosureSharedFun",
    "static_cast<intptr_t>(RootIndex::PromiseAllSettledRejectElementClosureSharedFun)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 18);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
};
