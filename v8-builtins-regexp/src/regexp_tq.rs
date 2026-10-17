// 由 tool/src/builtins/regexp.tq.js 从 v8/src/builtins/regexp.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-regexp; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 56 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "regexp", "namespace regexp"),
  (
    "macro",
    "RegExpBuiltinsAssembler::BranchIfFastRegExpForMatch",
    "macro RegExpBuiltinsAssembler::BranchIfFastRegExpForMatch( implicit context: Context)(HeapObject): never labels IsFast, IsSlow",
  ),
  (
    "macro",
    "IsFastRegExpForMatch",
    "macro IsFastRegExpForMatch(implicit context: Context)(o: HeapObject): bool",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::BranchIfFastRegExpForSearch",
    "macro RegExpBuiltinsAssembler::BranchIfFastRegExpForSearch( implicit context: Context)(HeapObject): never labels IsFast, IsSlow",
  ),
  (
    "macro",
    "IsFastRegExpForSearch",
    "macro IsFastRegExpForSearch(implicit context: Context)(o: HeapObject): bool",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::BranchIfFastRegExp_Strict",
    "macro RegExpBuiltinsAssembler::BranchIfFastRegExp_Strict( implicit context: Context)(HeapObject): never labels IsFast, IsSlow",
  ),
  (
    "macro",
    "IsFastRegExpStrict",
    "macro IsFastRegExpStrict(implicit context: Context)(o: HeapObject): bool",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::BranchIfFastRegExp_Permissive",
    "macro RegExpBuiltinsAssembler::BranchIfFastRegExp_Permissive( implicit context: Context)(HeapObject): never labels IsFast, IsSlow",
  ),
  (
    "macro",
    "IsFastRegExpPermissive",
    "macro IsFastRegExpPermissive(implicit context: Context)(o: HeapObject): bool",
  ),
  (
    "macro",
    "RegExpExec",
    "macro RegExpExec( implicit context: Context)(receiver: JSReceiver, string: String): JSAny",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::ConstructNewResultFromMatchInfo",
    "macro RegExpBuiltinsAssembler::ConstructNewResultFromMatchInfo( implicit context: Context)(JSRegExp, RegExpMatchInfo, String, Number): JSRegExpResult|JSRegExpResultWithIndices",
  ),
  (
    "const",
    "kGlobalOrSticky",
    "const kGlobalOrSticky: constexpr int31 generates 'JSRegExp::kGlobal | JSRegExp::kSticky'",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::RegExpExecInternal_Single",
    "macro RegExpBuiltinsAssembler::RegExpExecInternal_Single( implicit context: Context)(JSRegExp, String, Number): RegExpMatchInfo labels IfDidNotMatch",
  ),
  (
    "macro",
    "RegExpPrototypeExecBodyWithoutResult",
    "macro RegExpPrototypeExecBodyWithoutResult( implicit context: Context)(regexp: JSRegExp, string: String, regexpLastIndex: Number, isFastPath: constexpr bool): RegExpMatchInfo labels IfDidNotMatch",
  ),
  (
    "macro",
    "RegExpPrototypeExecBodyWithoutResultFast",
    "macro RegExpPrototypeExecBodyWithoutResultFast( implicit context: Context)(regexp: JSRegExp, string: String): RegExpMatchInfo labels IfDidNotMatch",
  ),
  (
    "macro",
    "RegExpPrototypeExecBodyWithoutResultFast",
    "macro RegExpPrototypeExecBodyWithoutResultFast( implicit context: Context)(regexp: JSRegExp, string: String, lastIndex: Number): RegExpMatchInfo labels IfDidNotMatch",
  ),
  (
    "macro",
    "RegExpPrototypeExecBody",
    "macro RegExpPrototypeExecBody( implicit context: Context)(receiver: JSReceiver, string: String, isFastPath: constexpr bool): JSAny",
  ),
  (
    "macro",
    "LoadRegExpFunction",
    "macro LoadRegExpFunction(nativeContext: NativeContext): JSFunction",
  ),
  (
    "macro",
    "HasInitialRegExpMap",
    "macro HasInitialRegExpMap(implicit context: Context)(o: HeapObject): bool",
  ),
  (
    "macro",
    "IsReceiverInitialRegExpPrototype",
    "macro IsReceiverInitialRegExpPrototype( implicit context: Context)(receiver: Object): bool",
  ),
  ("enum", "Flag", "enum Flag constexpr 'JSRegExp::Flag'"),
  (
    "const",
    "kNoCounterFlagGetter",
    "const kNoCounterFlagGetter: constexpr int31 = -1",
  ),
  (
    "const",
    "kRegExpPrototypeStickyGetter",
    "const kRegExpPrototypeStickyGetter: constexpr int31 generates 'v8::Isolate::kRegExpPrototypeStickyGetter'",
  ),
  (
    "const",
    "kRegExpPrototypeUnicodeGetter",
    "const kRegExpPrototypeUnicodeGetter: constexpr int31 generates 'v8::Isolate::kRegExpPrototypeUnicodeGetter'",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::FastFlagGetter",
    "macro RegExpBuiltinsAssembler::FastFlagGetter(JSRegExp, constexpr Flag): bool",
  ),
  (
    "macro",
    "FlagGetter",
    "macro FlagGetter( implicit context: Context)(receiver: Object, flag: constexpr Flag, counter: constexpr int31, methodName: constexpr string): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeGlobalGetter",
    "builtin RegExpPrototypeGlobalGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeIgnoreCaseGetter",
    "builtin RegExpPrototypeIgnoreCaseGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeMultilineGetter",
    "builtin RegExpPrototypeMultilineGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeHasIndicesGetter",
    "builtin RegExpPrototypeHasIndicesGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeLinearGetter",
    "builtin RegExpPrototypeLinearGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeDotAllGetter",
    "builtin RegExpPrototypeDotAllGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeStickyGetter",
    "builtin RegExpPrototypeStickyGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeUnicodeGetter",
    "builtin RegExpPrototypeUnicodeGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeUnicodeSetsGetter",
    "builtin RegExpPrototypeUnicodeSetsGetter( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::FlagsGetter",
    "macro RegExpBuiltinsAssembler::FlagsGetter( implicit context: Context)(JSAny, constexpr bool): String",
  ),
  (
    "macro",
    "FastFlagsGetter",
    "macro FastFlagsGetter( implicit context: Context)(receiver: FastJSRegExp): String",
  ),
  (
    "macro",
    "SlowFlagsGetter",
    "macro SlowFlagsGetter( implicit context: Context)(receiver: JSAny): String",
  ),
  (
    "builtin",
    "RegExpPrototypeFlagsGetter",
    "builtin RegExpPrototypeFlagsGetter( js-implicit context: NativeContext, receiver: JSAny)(): String",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::SlowLoadLastIndex",
    "macro RegExpBuiltinsAssembler::SlowLoadLastIndex( implicit context: Context)(JSAny): JSAny",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::SlowStoreLastIndex",
    "macro RegExpBuiltinsAssembler::SlowStoreLastIndex( implicit context: Context)(JSAny, JSAny): void",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::FastLoadLastIndex",
    "macro RegExpBuiltinsAssembler::FastLoadLastIndex(JSRegExp): Smi",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::FastStoreLastIndex",
    "macro RegExpBuiltinsAssembler::FastStoreLastIndex(JSRegExp, Smi): void",
  ),
  (
    "macro",
    "LoadLastIndex",
    "macro LoadLastIndex( implicit context: Context)(regexp: JSAny, isFastPath: constexpr bool): JSAny",
  ),
  (
    "macro",
    "LoadLastIndexAsLength",
    "macro LoadLastIndexAsLength( implicit context: Context)(regexp: JSRegExp, isFastPath: constexpr bool): Number",
  ),
  (
    "macro",
    "StoreLastIndex",
    "macro StoreLastIndex( implicit context: Context)(regexp: JSAny, value: Number, isFastPath: constexpr bool): void",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::AdvanceStringIndex",
    "macro RegExpBuiltinsAssembler::AdvanceStringIndex( String, Number, bool, constexpr bool): Number",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::AdvanceStringIndexFast",
    "macro RegExpBuiltinsAssembler::AdvanceStringIndexFast( String, Smi, bool): Smi",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::AdvanceStringIndexSlow",
    "macro RegExpBuiltinsAssembler::AdvanceStringIndexSlow( String, Number, bool): Number",
  ),
  (
    "const",
    "kRegExpMatchIsTrueishOnNonJSRegExp",
    "const kRegExpMatchIsTrueishOnNonJSRegExp: constexpr UseCounterFeature generates 'v8::Isolate::kRegExpMatchIsTrueishOnNonJSRegExp'",
  ),
  (
    "const",
    "kRegExpMatchIsFalseishOnJSRegExp",
    "const kRegExpMatchIsFalseishOnJSRegExp: constexpr UseCounterFeature generates 'v8::Isolate::kRegExpMatchIsFalseishOnJSRegExp'",
  ),
  (
    "const",
    "kRegExpExecCalledOnSlowRegExp",
    "const kRegExpExecCalledOnSlowRegExp: constexpr UseCounterFeature generates 'v8::Isolate::kRegExpExecCalledOnSlowRegExp'",
  ),
  (
    "macro",
    "IsRegExp",
    "macro IsRegExp(implicit context: Context)(obj: JSAny): bool",
  ),
  (
    "macro",
    "RegExpCreate",
    "macro RegExpCreate( implicit context: Context)(nativeContext: NativeContext, maybeString: JSAny, flags: String): JSAny",
  ),
  (
    "macro",
    "ClearTrustedPointerField",
    "macro ClearTrustedPointerField(HeapObject, constexpr int31): void",
  ),
  (
    "macro",
    "RegExpCreate",
    "macro RegExpCreate( implicit context: Context)(initialMap: Map, maybeString: JSAny, flags: String): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kGlobalOrSticky", "JSRegExp::kGlobal | JSRegExp::kSticky"),
  (
    "kRegExpPrototypeStickyGetter",
    "v8::Isolate::kRegExpPrototypeStickyGetter",
  ),
  (
    "kRegExpPrototypeUnicodeGetter",
    "v8::Isolate::kRegExpPrototypeUnicodeGetter",
  ),
  (
    "kRegExpMatchIsTrueishOnNonJSRegExp",
    "v8::Isolate::kRegExpMatchIsTrueishOnNonJSRegExp",
  ),
  (
    "kRegExpMatchIsFalseishOnJSRegExp",
    "v8::Isolate::kRegExpMatchIsFalseishOnJSRegExp",
  ),
  (
    "kRegExpExecCalledOnSlowRegExp",
    "v8::Isolate::kRegExpExecCalledOnSlowRegExp",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const NO_COUNTER_FLAG_GETTER: i64 = -1; // 原版: kNoCounterFlagGetter

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 56);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 37);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "enum").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 10);
};
