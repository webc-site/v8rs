// 由 tool/src/builtins/iterator-helpers.tq.js 从 v8/src/builtins/iterator-helpers.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 47 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "iterator", "namespace iterator"),
  (
    "macro",
    "IsIteratorHelperExhausted",
    "macro IsIteratorHelperExhausted(helper: JSIteratorHelper): bool",
  ),
  (
    "macro",
    "MarkIteratorHelperAsExhausted",
    "macro MarkIteratorHelperAsExhausted(helper: JSIteratorHelper): void",
  ),
  (
    "macro",
    "IsIteratorHelperExecuting",
    "macro IsIteratorHelperExecuting(helper: JSIteratorHelper): bool",
  ),
  (
    "macro",
    "ThrowIfIteratorHelperExecuting",
    "macro ThrowIfIteratorHelperExecuting( implicit context: Context)(helper: JSIteratorHelper): void",
  ),
  (
    "macro",
    "MarkIteratorHelperAsExecuting",
    "macro MarkIteratorHelperAsExecuting(helper: JSIteratorHelper): void",
  ),
  (
    "macro",
    "IteratorHelperIsSuspendedStart",
    "macro IteratorHelperIsSuspendedStart(helper: JSIteratorHelper): bool",
  ),
  (
    "macro",
    "MarkIteratorHelperAsFinishedExecuting",
    "macro MarkIteratorHelperAsFinishedExecuting(helper: JSIteratorHelper): void",
  ),
  (
    "macro",
    "GetIteratorDirect",
    "macro GetIteratorDirect( implicit context: Context)(obj: JSReceiver): IteratorRecord",
  ),
  (
    "builtin",
    "IteratorHelperPrototypeNext",
    "builtin IteratorHelperPrototypeNext( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "IteratorHelperPrototypeReturn",
    "builtin IteratorHelperPrototypeReturn( js-implicit context: NativeContext, receiver: JSAny)(): JSObject",
  ),
  (
    "macro",
    "NewJSIteratorMapHelper",
    "macro NewJSIteratorMapHelper( implicit context: Context)(underlying: IteratorRecord, mapper: Callable): JSIteratorMapHelper",
  ),
  (
    "builtin",
    "IteratorPrototypeMap",
    "builtin IteratorPrototypeMap( js-implicit context: NativeContext, receiver: JSAny)( mapper: JSAny): JSIteratorMapHelper",
  ),
  (
    "builtin",
    "IteratorMapHelperNext",
    "builtin IteratorMapHelperNext( implicit context: Context)(helper: JSIteratorMapHelper): JSAny",
  ),
  (
    "macro",
    "NewJSIteratorFilterHelper",
    "macro NewJSIteratorFilterHelper( implicit context: Context)(underlying: IteratorRecord, predicate: Callable): JSIteratorFilterHelper",
  ),
  (
    "builtin",
    "IteratorPrototypeFilter",
    "builtin IteratorPrototypeFilter( js-implicit context: NativeContext, receiver: JSAny)( predicate: JSAny): JSIteratorFilterHelper",
  ),
  (
    "builtin",
    "IteratorFilterHelperNext",
    "builtin IteratorFilterHelperNext( implicit context: Context)(helper: JSIteratorFilterHelper): JSAny",
  ),
  (
    "macro",
    "NewJSIteratorTakeHelper",
    "macro NewJSIteratorTakeHelper( implicit context: Context)(underlying: IteratorRecord, limit: Number): JSIteratorTakeHelper",
  ),
  (
    "builtin",
    "IteratorPrototypeTake",
    "builtin IteratorPrototypeTake( js-implicit context: NativeContext, receiver: JSAny)( limit: JSAny): JSIteratorTakeHelper",
  ),
  (
    "builtin",
    "IteratorTakeHelperNext",
    "builtin IteratorTakeHelperNext( implicit context: Context)(helper: JSIteratorTakeHelper): JSAny",
  ),
  (
    "macro",
    "NewJSIteratorDropHelper",
    "macro NewJSIteratorDropHelper( implicit context: Context)(underlying: IteratorRecord, limit: Number): JSIteratorDropHelper",
  ),
  (
    "builtin",
    "IteratorPrototypeDrop",
    "builtin IteratorPrototypeDrop( js-implicit context: NativeContext, receiver: JSAny)( limit: JSAny): JSIteratorDropHelper",
  ),
  (
    "builtin",
    "IteratorDropHelperNext",
    "builtin IteratorDropHelperNext( implicit context: Context)(helper: JSIteratorDropHelper): JSAny",
  ),
  (
    "const",
    "kFlatMapMethodName",
    "const kFlatMapMethodName: constexpr string = 'Iterator.prototype.flatMap'",
  ),
  (
    "macro",
    "NewJSIteratorFlatMapHelper",
    "macro NewJSIteratorFlatMapHelper( implicit context: Context)(underlying: IteratorRecord, mapper: Callable): JSIteratorFlatMapHelper",
  ),
  (
    "builtin",
    "IteratorPrototypeFlatMap",
    "builtin IteratorPrototypeFlatMap( js-implicit context: NativeContext, receiver: JSAny)( mapper: JSAny): JSIteratorFlatMapHelper",
  ),
  (
    "builtin",
    "IteratorFlatMapHelperNext",
    "builtin IteratorFlatMapHelperNext( implicit context: Context)(helper: JSIteratorFlatMapHelper): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeReduce",
    "builtin IteratorPrototypeReduce( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeToArray",
    "builtin IteratorPrototypeToArray( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeForEach",
    "builtin IteratorPrototypeForEach( js-implicit context: NativeContext, receiver: JSAny)(fn: JSAny): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeSome",
    "builtin IteratorPrototypeSome( js-implicit context: NativeContext, receiver: JSAny)( predicate: JSAny): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeEvery",
    "builtin IteratorPrototypeEvery( js-implicit context: NativeContext, receiver: JSAny)( predicate: JSAny): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeFind",
    "builtin IteratorPrototypeFind( js-implicit context: NativeContext, receiver: JSAny)( predicate: JSAny): JSAny",
  ),
  (
    "const",
    "kConcatMethodName",
    "const kConcatMethodName: constexpr string = 'Iterator.concat'",
  ),
  (
    "macro",
    "NewJSIteratorConcatHelper",
    "macro NewJSIteratorConcatHelper( implicit context: Context)(iterables: FixedArray): JSIteratorConcatHelper",
  ),
  (
    "builtin",
    "IteratorConcat",
    "builtin IteratorConcat( js-implicit context: NativeContext, receiver: JSAny)( ...items): JSIteratorConcatHelper",
  ),
  (
    "builtin",
    "IteratorConcatHelperNext",
    "builtin IteratorConcatHelperNext( implicit context: Context)(helper: JSIteratorConcatHelper): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeJoin",
    "builtin IteratorPrototypeJoin( js-implicit context: NativeContext, receiver: JSAny)( separator: JSAny): JSAny",
  ),
  (
    "macro",
    "IteratorZipCloseAll",
    "macro IteratorZipCloseAll( implicit context: Context)(iterators: FixedArray, propagate: constexpr bool): void",
  ),
  (
    "macro",
    "NewJSIteratorZipHelper",
    "macro NewJSIteratorZipHelper( implicit context: Context)(iterators: FixedArray, padding: FixedArray, zipMode: JSIteratorZipHelperMode): JSIteratorZipHelper",
  ),
  (
    "builtin",
    "IteratorZip",
    "builtin IteratorZip( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "macro",
    "IteratorZipHelperNextCommon",
    "macro IteratorZipHelperNextCommon( implicit context: Context)(helper: JSIteratorZipHelper, methodName: constexpr string): FixedArray labels Done, DoneWithException(JSAny, JSMessageObject|TheHole)",
  ),
  (
    "builtin",
    "IteratorZipHelperNext",
    "builtin IteratorZipHelperNext( implicit context: Context)(helper: JSIteratorZipHelper): JSAny",
  ),
  (
    "macro",
    "NewJSIteratorZipKeyedHelper",
    "macro NewJSIteratorZipKeyedHelper( implicit context: Context)(iterators: FixedArray, padding: FixedArray, zipMode: JSIteratorZipHelperMode, keys: FixedArray): JSIteratorZipKeyedHelper",
  ),
  (
    "builtin",
    "IteratorZipKeyed",
    "builtin IteratorZipKeyed( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
  (
    "builtin",
    "IteratorZipKeyedHelperNext",
    "builtin IteratorZipKeyedHelperNext( implicit context: Context)(helper: JSIteratorZipKeyedHelper): JSAny",
  ),
  (
    "builtin",
    "IteratorPrototypeIncludes",
    "builtin IteratorPrototypeIncludes( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const FLAT_MAP_METHOD_NAME: &'static str = "Iterator.prototype.flatMap"; // 原版: kFlatMapMethodName
pub const CONCAT_METHOD_NAME: &'static str = "Iterator.concat"; // 原版: kConcatMethodName

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 47);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 18);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 26);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
};
