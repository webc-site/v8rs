// 由 tool/src/builtins/iterator.tq.js 从 v8/src/builtins/iterator.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 52 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "iterator", "namespace iterator"),
  ("struct", "IteratorRecord", "struct IteratorRecord"),
  (
    "const",
    "kIteratorRecordFieldCount",
    "const kIteratorRecordFieldCount: constexpr int31 = 2",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::FastIterableToList",
    "macro IteratorBuiltinsAssembler::FastIterableToList( implicit context: Context)(JSAny): JSArray labels Slow",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::GetIteratorMethod",
    "macro IteratorBuiltinsAssembler::GetIteratorMethod( implicit context: Context)(JSAny): JSAny",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::GetIterator",
    "macro IteratorBuiltinsAssembler::GetIterator( implicit context: Context)(JSAny): IteratorRecord",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::GetIterator",
    "macro IteratorBuiltinsAssembler::GetIterator( implicit context: Context)(JSAny, JSAny): IteratorRecord",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::IteratorStep",
    "macro IteratorBuiltinsAssembler::IteratorStep( implicit context: Context)(IteratorRecord): JSReceiver labels Done",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::IteratorStep",
    "macro IteratorBuiltinsAssembler::IteratorStep( implicit context: Context)(IteratorRecord, Map): JSReceiver labels Done",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::IteratorComplete",
    "macro IteratorBuiltinsAssembler::IteratorComplete( implicit context: Context)(JSReceiver): void labels Done",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::IteratorComplete",
    "macro IteratorBuiltinsAssembler::IteratorComplete( implicit context: Context)(JSReceiver, Map): void labels Done",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::IteratorValue",
    "macro IteratorBuiltinsAssembler::IteratorValue( implicit context: Context)(JSReceiver): JSAny",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::IteratorValue",
    "macro IteratorBuiltinsAssembler::IteratorValue( implicit context: Context)(JSReceiver, Map): JSAny",
  ),
  (
    "macro",
    "IteratorStepValue",
    "macro IteratorStepValue( implicit context: Context)(iterated: IteratorRecord, fastIteratorResultMap: Map): JSAny labels Done, DoneWithException(JSAny, JSMessageObject|TheHole)",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::IterableToList",
    "macro IteratorBuiltinsAssembler::IterableToList( implicit context: Context)(JSAny, JSAny): JSArray",
  ),
  (
    "macro",
    "IteratorBuiltinsAssembler::StringListFromIterable",
    "macro IteratorBuiltinsAssembler::StringListFromIterable( implicit context: Context)(JSAny): JSArray",
  ),
  (
    "builtin",
    "IterableToListWithSymbolLookup",
    "builtin IterableToListWithSymbolLookup( implicit context: Context)(JSAny): JSArray",
  ),
  (
    "builtin",
    "IterableToFixedArrayWithSymbolLookupSlow",
    "builtin IterableToFixedArrayWithSymbolLookupSlow( implicit context: Context)(JSAny): FixedArray",
  ),
  (
    "builtin",
    "GetIteratorWithFeedback",
    "builtin GetIteratorWithFeedback( context: Context, receiver: JSAny, loadSlot: TaggedIndex, callSlot: TaggedIndex, maybeFeedbackVector: Undefined|FeedbackVector): JSAny",
  ),
  (
    "macro",
    "LoadContextFromBaseline",
    "macro LoadContextFromBaseline(): Context",
  ),
  (
    "macro",
    "LoadFeedbackVectorFromBaseline",
    "macro LoadFeedbackVectorFromBaseline(): FeedbackVector",
  ),
  (
    "builtin",
    "GetIteratorBaseline",
    "builtin GetIteratorBaseline( receiver: JSAny, loadSlot: TaggedIndex, callSlot: TaggedIndex): JSAny",
  ),
  (
    "macro",
    "CreateAsyncFromSyncIterator",
    "macro CreateAsyncFromSyncIterator(Context, JSAny): JSAny",
  ),
  (
    "builtin",
    "CreateAsyncFromSyncIteratorBaseline",
    "builtin CreateAsyncFromSyncIteratorBaseline(syncIterator: JSAny): JSAny",
  ),
  (
    "macro",
    "ForOfNextHelper",
    "macro ForOfNextHelper( Context, Object, Object, Undefined|FeedbackVector, uintptr): JSAny|Hole",
  ),
  (
    "builtin",
    "ForOfNext",
    "builtin ForOfNext( context: Context, iterator: JSAny, next: JSFunction, feedbackVector: Undefined|FeedbackVector, callSlot: Smi): Object",
  ),
  (
    "builtin",
    "ForOfNextBaseline",
    "builtin ForOfNextBaseline( context: Context, iterator: JSAny, next: JSFunction, callSlot: Smi): Object",
  ),
  (
    "macro",
    "GetIteratorRecordAfterCreateAsyncFromSyncIterator",
    "macro GetIteratorRecordAfterCreateAsyncFromSyncIterator( context: Context, asyncIterator: IteratorRecord): IteratorRecord",
  ),
  (
    "macro",
    "GetLazyReceiver",
    "macro GetLazyReceiver(receiver: JSAny): JSAny",
  ),
  (
    "builtin",
    "CallIteratorWithFeedback",
    "builtin CallIteratorWithFeedback( context: Context, receiver: JSAny, iteratorMethod: JSAny, callSlot: Smi, feedback: Undefined|FeedbackVector): JSAny",
  ),
  (
    "macro",
    "IteratorCloseOnException",
    "macro IteratorCloseOnException( implicit context: Context)(iteratorObject: JSReceiver): void",
  ),
  (
    "macro",
    "IteratorClose",
    "macro IteratorClose( implicit context: Context)(iterator: IteratorRecord): void",
  ),
  (
    "builtin",
    "IteratorClose",
    "builtin IteratorClose(context: Context, iterator: JSReceiver): Undefined",
  ),
  (
    "builtin",
    "IteratorConstructor",
    "builtin IteratorConstructor( js-implicit context: NativeContext, receiver: JSAny, newTarget: JSAny, target: JSFunction)(): JSObject",
  ),
  (
    "macro",
    "SetterThatIgnoresPrototypeProperties",
    "macro SetterThatIgnoresPrototypeProperties( implicit context: Context)(receiver: JSAny, home: JSObject, key: JSAny, value: JSAny, methodName: constexpr string): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeGetToStringTag",
    "builtin IteratorPrototypeGetToStringTag( js-implicit context: NativeContext)(): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeSetToStringTag",
    "builtin IteratorPrototypeSetToStringTag( js-implicit context: NativeContext, receiver: JSAny)(value: JSAny): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeGetConstructor",
    "builtin IteratorPrototypeGetConstructor( js-implicit context: NativeContext)(): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeSetConstructor",
    "builtin IteratorPrototypeSetConstructor( js-implicit context: NativeContext, receiver: JSAny)(value: JSAny): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeDispose",
    "builtin IteratorPrototypeDispose( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "macro",
    "AllocateRootFunctionWithContext",
    "macro AllocateRootFunctionWithContext( constexpr intptr, FunctionContext, NativeContext): JSFunction",
  ),
  (
    "macro",
    "AsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFunConstant",
    "macro AsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFunConstant(): SharedFunctionInfo",
  ),
  (
    "const",
    "kAsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFun",
    "const kAsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFun: constexpr intptr generates 'RootIndex::kAsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFun'",
  ),
  (
    "type",
    "AsyncIteratorPrototypeAsyncDisposeResolveContext",
    "type AsyncIteratorPrototypeAsyncDisposeResolveContext extends FunctionContext",
  ),
  (
    "const",
    "kAsyncIteratorPrototypeAsyncDisposeResolveContextLength",
    "const kAsyncIteratorPrototypeAsyncDisposeResolveContextLength: constexpr intptr = ContextSlot::MIN_CONTEXT_SLOTS",
  ),
  (
    "builtin",
    "AsyncIteratorPrototypeAsyncDisposeResolveClosure",
    "builtin AsyncIteratorPrototypeAsyncDisposeResolveClosure( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "AsyncIteratorPrototypeAsyncDispose",
    "builtin AsyncIteratorPrototypeAsyncDispose( js-implicit context: Context, receiver: JSAny)(): JSAny",
  ),
  (
    "macro",
    "StoreRegisterToRawAddr",
    "macro StoreRegisterToRawAddr(firstRegAddr: RawPtr, i: intptr, val: Object): void",
  ),
  (
    "builtin",
    "ArrayDestructure",
    "builtin ArrayDestructure( context: Context, receiver: JSAny, count: int32, firstRegAddr: RawPtr): JSAny",
  ),
  (
    "const",
    "kRegisterFileFromFp",
    "const kRegisterFileFromFp: constexpr intptr generates 'InterpreterFrameConstants::kRegisterFileFromFp'",
  ),
  (
    "macro",
    "StoreRegisterToParentFrame",
    "macro StoreRegisterToParentFrame( callerFp: Frame, firstReg: intptr, i: intptr, val: Object): void",
  ),
  (
    "builtin",
    "ArrayDestructureLazyDeoptContinuation",
    "builtin ArrayDestructureLazyDeoptContinuation( context: Context, iteratorArg: JSReceiver|Hole, nextMethodArg: JSAny|Hole, index: Smi, firstReg: Smi, count: Smi, deoptValue: JSAny|Hole): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kAsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFun",
    "RootIndex::kAsyncIteratorPrototypeAsyncDisposeResolveClosureSharedFun",
  ),
  (
    "kRegisterFileFromFp",
    "InterpreterFrameConstants::kRegisterFileFromFp",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const ITERATOR_RECORD_FIELD_COUNT: i64 = 2; // 原版: kIteratorRecordFieldCount

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 52);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 26);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 19);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 1);
};
