// 由 tool/src/builtins/promise-abstract-operations.tq.js 从 v8/src/builtins/promise-abstract-operations.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 25 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "promise", "namespace promise"),
  (
    "const",
    "handlerContext",
    "const handlerContext: Context = ExtractHandlerContext(primaryHandler, secondaryHandler)",
  ),
  (
    "const",
    "reactions",
    "const reactions = UnsafeCast<(Zero | PromiseReaction)>(promise.reactions_or_result)",
  ),
  (
    "const",
    "promiseHookFlags",
    "const promiseHookFlags = PromiseHookFlags()",
  ),
  (
    "const",
    "reactions",
    "const reactions = UnsafeCast<(Zero | PromiseReaction)>(promise.reactions_or_result)",
  ),
  (
    "const",
    "executorContext",
    "const executorContext = %RawDownCast<PromiseCapabilitiesExecutorContext>( AllocateSyntheticFunctionContext( nativeContext, FunctionContextSlot::kCapabilitiesContextLength))",
  ),
  (
    "const",
    "promiseContext",
    "const promiseContext = CreatePromiseResolvingFunctionsContext( promise, debugEvent, nativeContext)",
  ),
  (
    "const",
    "resolve",
    "const resolve: JSFunction = AllocateRootFunctionWithContext( kPromiseCapabilityDefaultResolveSharedFun, promiseContext, nativeContext)",
  ),
  (
    "const",
    "reject",
    "const reject: JSFunction = AllocateRootFunctionWithContext( kPromiseCapabilityDefaultRejectSharedFun, promiseContext, nativeContext)",
  ),
  (
    "const",
    "nativeContext",
    "const nativeContext = LoadNativeContext(context)",
  ),
  (
    "const",
    "context",
    "const context = %RawDownCast<PromiseResolvingFunctionContext>(context)",
  ),
  (
    "const",
    "promiseOrEmpty",
    "const promiseOrEmpty = *ContextSlot( context, PromiseResolvingFunctionContextSlot::kPromiseIfNotResolvedSlot)",
  ),
  (
    "const",
    "debugEvent",
    "const debugEvent = *ContextSlot( context, PromiseResolvingFunctionContextSlot::kDebugEventSlot)",
  ),
  (
    "const",
    "context",
    "const context = %RawDownCast<PromiseResolvingFunctionContext>(context)",
  ),
  (
    "const",
    "promiseOrEmpty",
    "const promiseOrEmpty = *ContextSlot( context, PromiseResolvingFunctionContextSlot::kPromiseIfNotResolvedSlot)",
  ),
  (
    "const",
    "jsPromise",
    "const jsPromise = Cast<JSPromise>(receiver) otherwise unreachable",
  ),
  (
    "const",
    "callableOnFulfilled",
    "const callableOnFulfilled = Cast<(Callable | Undefined)>(onFulfilled) otherwise unreachable",
  ),
  (
    "const",
    "callableOnRejected",
    "const callableOnRejected = Cast<(Callable | Undefined)>(onRejected) otherwise unreachable",
  ),
  (
    "const",
    "resultPromise",
    "const resultPromise = Cast<(JSPromise | Undefined)>(resultCapability) otherwise unreachable",
  ),
  (
    "const",
    "receiver",
    "const receiver = Cast<JSReceiver>(receiver) otherwise ThrowTypeError(MessageTemplate::kCalledOnNonObject, 'PromiseReject')",
  ),
  (
    "const",
    "promiseFun",
    "const promiseFun = *NativeContextSlot(ContextSlot::PROMISE_FUNCTION_INDEX)",
  ),
  (
    "const",
    "context",
    "const context = %RawDownCast<PromiseCapabilitiesExecutorContext>(context)",
  ),
  (
    "const",
    "capability",
    "const capability: PromiseCapability = *ContextSlot(context, FunctionContextSlot::kCapabilitySlot)",
  ),
  (
    "const",
    "promiseFun",
    "const promiseFun = *NativeContextSlot(nativeContext, ContextSlot::PROMISE_FUNCTION_INDEX)",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 25);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 23);
};
