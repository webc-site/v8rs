// 由 tool/src/builtins/array-from-async.tq.js 从 v8/src/builtins/array-from-async.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 32 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "array", "namespace array"),
  (
    "enum",
    "ArrayFromAsyncLabels",
    "enum ArrayFromAsyncLabels extends uint31 constexpr 'ArrayBuiltins::ArrayFromAsyncLabels'",
  ),
  (
    "macro",
    "ArrayFromAsyncAwaitPoint",
    "macro ArrayFromAsyncAwaitPoint<T : type extends FunctionContext>( implicit context: Context)(resolveContext: T, stepSlot: Slot<T, Smi>, promiseFunSlot: Slot<T, JSReceiver>, resolveSlot: Slot<T, Undefined|JSFunction>, rejectSlot: Slot<T, Und…",
  ),
  (
    "macro",
    "RejectArrayFromAsyncPromise",
    "macro RejectArrayFromAsyncPromise<T : type extends FunctionContext>( implicit context: Context)(resolveContext: T, errorSlot: Slot<T, JSAny>, promiseSlot: Slot<T, JSPromise>): JSAny",
  ),
  (
    "struct",
    "ArrayFromAsyncIterableResumeState",
    "struct ArrayFromAsyncIterableResumeState",
  ),
  (
    "type",
    "ArrayFromAsyncIterableResolveContext",
    "type ArrayFromAsyncIterableResolveContext extends FunctionContext",
  ),
  (
    "enum",
    "ArrayFromAsyncIterableResolveContextSlots",
    "enum ArrayFromAsyncIterableResolveContextSlots extends intptr constexpr 'ArrayBuiltins::ArrayFromAsyncIterableResolveContextSlots'",
  ),
  (
    "macro",
    "AllocateRootFunctionWithContext",
    "macro AllocateRootFunctionWithContext( constexpr intptr, FunctionContext, NativeContext): JSFunction",
  ),
  (
    "const",
    "kArrayFromAsyncIterableOnFulfilledSharedFun",
    "const kArrayFromAsyncIterableOnFulfilledSharedFun: constexpr intptr generates 'RootIndex::ArrayFromAsyncIterableOnFulfilledSharedFun'",
  ),
  (
    "const",
    "kArrayFromAsyncIterableOnRejectedSharedFun",
    "const kArrayFromAsyncIterableOnRejectedSharedFun: constexpr intptr generates 'RootIndex::ArrayFromAsyncIterableOnRejectedSharedFun'",
  ),
  (
    "macro",
    "CreateArrayFromAsyncIterableResolveContext",
    "macro CreateArrayFromAsyncIterableResolveContext( implicit context: Context)(resumeState: ArrayFromAsyncIterableResumeState, promise: JSPromise, promiseFun: JSReceiver, iterator: JSReceiver, next: JSAny, arr: JSReceiver, error: JSAny, mapfn…",
  ),
  (
    "macro",
    "GetIteratorRecordFromArrayFromAsyncIterableResolveContext",
    "macro GetIteratorRecordFromArrayFromAsyncIterableResolveContext( context: ArrayFromAsyncIterableResolveContext): iterator::IteratorRecord",
  ),
  (
    "macro",
    "CreateArrayFromIterableAsynchronously",
    "macro CreateArrayFromIterableAsynchronously( context: ArrayFromAsyncIterableResolveContext): JSAny",
  ),
  (
    "macro",
    "ArrayFromAsyncIterableAwaitPoint",
    "macro ArrayFromAsyncIterableAwaitPoint( context: ArrayFromAsyncIterableResolveContext, step: ArrayFromAsyncLabels, value: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFromAsyncIterableOnFulfilled",
    "builtin ArrayFromAsyncIterableOnFulfilled( js-implicit context: Context, receiver: JSAny, target: JSFunction)( result: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFromAsyncIterableOnRejected",
    "builtin ArrayFromAsyncIterableOnRejected( js-implicit context: Context, receiver: JSAny, target: JSFunction)( result: JSAny): JSAny",
  ),
  (
    "macro",
    "ArrayFromAsyncAsyncIteratorCloseOnException",
    "macro ArrayFromAsyncAsyncIteratorCloseOnException( implicit context: Context)( iterator: iterator::IteratorRecord): void labels RejectPromise",
  ),
  (
    "macro",
    "ArrayFromAsyncIterableOnFulfilledSharedFunConstant",
    "macro ArrayFromAsyncIterableOnFulfilledSharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "macro",
    "ArrayFromAsyncIterableOnRejectedSharedFunConstant",
    "macro ArrayFromAsyncIterableOnRejectedSharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "struct",
    "ArrayFromAsyncArrayLikeResumeState",
    "struct ArrayFromAsyncArrayLikeResumeState",
  ),
  (
    "type",
    "ArrayFromAsyncArrayLikeResolveContext",
    "type ArrayFromAsyncArrayLikeResolveContext extends FunctionContext",
  ),
  (
    "enum",
    "ArrayFromAsyncArrayLikeResolveContextSlots",
    "enum ArrayFromAsyncArrayLikeResolveContextSlots extends intptr constexpr 'ArrayBuiltins::ArrayFromAsyncArrayLikeResolveContextSlots'",
  ),
  (
    "const",
    "kArrayFromAsyncArrayLikeOnFulfilledSharedFun",
    "const kArrayFromAsyncArrayLikeOnFulfilledSharedFun: constexpr intptr generates 'RootIndex::ArrayFromAsyncArrayLikeOnFulfilledSharedFun'",
  ),
  (
    "const",
    "kArrayFromAsyncArrayLikeOnRejectedSharedFun",
    "const kArrayFromAsyncArrayLikeOnRejectedSharedFun: constexpr intptr generates 'RootIndex::ArrayFromAsyncArrayLikeOnRejectedSharedFun'",
  ),
  (
    "macro",
    "CreateArrayFromAsyncArrayLikeResolveContext",
    "macro CreateArrayFromAsyncArrayLikeResolveContext( implicit context: Context)(resumeState: ArrayFromAsyncArrayLikeResumeState, promise: JSPromise, promiseFun: JSReceiver, arrayLike: JSReceiver, arr: JSReceiver, error: JSAny, mapfn: JSAny, t…",
  ),
  (
    "macro",
    "CreateArrayFromArrayLikeAsynchronously",
    "macro CreateArrayFromArrayLikeAsynchronously( context: ArrayFromAsyncArrayLikeResolveContext): JSAny",
  ),
  (
    "macro",
    "ArrayFromAsyncArrayLikeAwaitPoint",
    "macro ArrayFromAsyncArrayLikeAwaitPoint( context: ArrayFromAsyncArrayLikeResolveContext, step: ArrayFromAsyncLabels, value: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFromAsyncArrayLikeOnFulfilled",
    "builtin ArrayFromAsyncArrayLikeOnFulfilled( js-implicit context: Context, receiver: JSAny, target: JSFunction)( result: JSAny): JSAny",
  ),
  (
    "builtin",
    "ArrayFromAsyncArrayLikeOnRejected",
    "builtin ArrayFromAsyncArrayLikeOnRejected( js-implicit context: Context, receiver: JSAny, target: JSFunction)( result: JSAny): JSAny",
  ),
  (
    "macro",
    "ArrayFromAsyncArrayLikeOnFulfilledSharedFunConstant",
    "macro ArrayFromAsyncArrayLikeOnFulfilledSharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "macro",
    "ArrayFromAsyncArrayLikeOnRejectedSharedFunConstant",
    "macro ArrayFromAsyncArrayLikeOnRejectedSharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "builtin",
    "ArrayFromAsync",
    "builtin ArrayFromAsync( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kArrayFromAsyncIterableOnFulfilledSharedFun",
    "RootIndex::ArrayFromAsyncIterableOnFulfilledSharedFun",
  ),
  (
    "kArrayFromAsyncIterableOnRejectedSharedFun",
    "RootIndex::ArrayFromAsyncIterableOnRejectedSharedFun",
  ),
  (
    "kArrayFromAsyncArrayLikeOnFulfilledSharedFun",
    "RootIndex::ArrayFromAsyncArrayLikeOnFulfilledSharedFun",
  ),
  (
    "kArrayFromAsyncArrayLikeOnRejectedSharedFun",
    "RootIndex::ArrayFromAsyncArrayLikeOnRejectedSharedFun",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 32);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "enum").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 15);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 5);
};
