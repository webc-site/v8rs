// 由 tool/src/builtins/regexp-replace.tq.js 从 v8/src/builtins/regexp-replace.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 10 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "regexp", "namespace regexp"),
  (
    "builtin",
    "SubString",
    "builtin SubString(implicit context: Context)(String, Smi, Smi): String",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::RegExpReplaceGlobalSimpleString",
    "macro RegExpBuiltinsAssembler::RegExpReplaceGlobalSimpleString( implicit context: Context)(JSRegExp, String, RegExpData, String): String",
  ),
  (
    "macro",
    "RegExpReplaceCallableNoExplicitCaptures",
    "macro RegExpReplaceCallableNoExplicitCaptures( implicit context: Context)(matchesElements: FixedArray, matchesCapacity: intptr, string: String, replaceFn: Callable): intptr",
  ),
  (
    "macro",
    "RegExpReplaceCallableWithExplicitCaptures",
    "macro RegExpReplaceCallableWithExplicitCaptures( implicit context: Context)(matchesElements: FixedArray, matchesCapacity: intptr, replaceFn: Callable): intptr",
  ),
  (
    "macro",
    "RegExpReplaceFastGlobalCallable",
    "macro RegExpReplaceFastGlobalCallable( implicit context: Context)(regexp: FastJSRegExp, string: String, replaceFn: Callable): String",
  ),
  (
    "macro",
    "RegExpReplaceFastString",
    "macro RegExpReplaceFastString( implicit context: Context)(regexp: JSRegExp, string: String, replaceString: String): String",
  ),
  (
    "builtin",
    "RegExpReplace",
    "builtin RegExpReplace( implicit context: Context)(regexp: FastJSRegExp, string: String, replaceValue: JSAny): String",
  ),
  (
    "const",
    "kRegExpReplaceCalledOnSlowRegExp",
    "const kRegExpReplaceCalledOnSlowRegExp: constexpr int31 generates 'v8::Isolate::kRegExpReplaceCalledOnSlowRegExp'",
  ),
  (
    "builtin",
    "RegExpPrototypeReplace",
    "builtin RegExpPrototypeReplace( js-implicit context: NativeContext, receiver: JSAny)(...arguments): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[(
  "kRegExpReplaceCalledOnSlowRegExp",
  "v8::Isolate::kRegExpReplaceCalledOnSlowRegExp",
)];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 10);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
};
