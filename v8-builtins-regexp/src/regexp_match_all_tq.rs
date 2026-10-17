// 由 tool/src/builtins/regexp-match-all.tq.js 从 v8/src/builtins/regexp-match-all.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 7 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "regexp", "namespace regexp"),
  (
    "macro",
    "RegExpMatchAllAssembler::CreateRegExpStringIterator",
    "macro RegExpMatchAllAssembler::CreateRegExpStringIterator( NativeContext, JSAny, String, bool, bool): JSAny",
  ),
  (
    "const",
    "kRegExpMatcherFlagsMismatch",
    "const kRegExpMatcherFlagsMismatch: constexpr UseCounterFeature generates 'v8::Isolate::kRegExpMatcherFlagsMismatch'",
  ),
  (
    "const",
    "kRegExpCustomSpecies",
    "const kRegExpCustomSpecies: constexpr UseCounterFeature generates 'v8::Isolate::kRegExpCustomSpecies'",
  ),
  (
    "macro",
    "RegExpPrototypeMatchAllImpl",
    "macro RegExpPrototypeMatchAllImpl( implicit context: Context)(nativeContext: NativeContext, receiver: JSAny, string: JSAny): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeMatchAll",
    "builtin RegExpPrototypeMatchAll( js-implicit context: NativeContext, receiver: JSAny)( string: JSAny): JSAny",
  ),
  (
    "builtin",
    "RegExpStringIteratorPrototypeNext",
    "builtin RegExpStringIteratorPrototypeNext( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kRegExpMatcherFlagsMismatch",
    "v8::Isolate::kRegExpMatcherFlagsMismatch",
  ),
  ("kRegExpCustomSpecies", "v8::Isolate::kRegExpCustomSpecies"),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
};
