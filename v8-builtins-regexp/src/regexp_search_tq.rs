// 由 tool/src/builtins/regexp-search.tq.js 从 v8/src/builtins/regexp-search.tq 生成; 修订就地改, 复跑覆盖
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
    "RegExpPrototypeSearchBodyFast",
    "macro RegExpPrototypeSearchBodyFast( implicit context: Context)(regexp: JSRegExp, string: String): JSAny",
  ),
  (
    "macro",
    "RegExpBuiltinsAssembler::BranchIfRegExpResult",
    "macro RegExpBuiltinsAssembler::BranchIfRegExpResult( implicit context: Context)(Object): never labels IsUnmodified, IsModified",
  ),
  (
    "macro",
    "IsRegExpResult",
    "macro IsRegExpResult(implicit context: Context)(execResult: HeapObject): bool",
  ),
  (
    "macro",
    "RegExpPrototypeSearchBodySlow",
    "macro RegExpPrototypeSearchBodySlow( implicit context: Context)(regexp: JSReceiver, string: String): JSAny",
  ),
  (
    "builtin",
    "RegExpSearchFast",
    "builtin RegExpSearchFast( implicit context: Context)(receiver: JSRegExp, string: String): JSAny",
  ),
  (
    "builtin",
    "RegExpPrototypeSearch",
    "builtin RegExpPrototypeSearch( js-implicit context: NativeContext, receiver: JSAny)( string: JSAny): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
};
