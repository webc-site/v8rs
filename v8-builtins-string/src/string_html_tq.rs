// 由 tool/src/builtins/string-html.tq.js 从 v8/src/builtins/string-html.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-string; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 15 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "string", "namespace string"),
  (
    "builtin",
    "CreateHTML",
    "builtin CreateHTML( implicit context: Context)(receiver: JSAny, methodName: String, tagName: String, attr: String, attrValue: JSAny): String",
  ),
  (
    "builtin",
    "StringPrototypeAnchor",
    "builtin StringPrototypeAnchor( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeBig",
    "builtin StringPrototypeBig( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeBlink",
    "builtin StringPrototypeBlink( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeBold",
    "builtin StringPrototypeBold( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeFontcolor",
    "builtin StringPrototypeFontcolor( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeFontsize",
    "builtin StringPrototypeFontsize( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeFixed",
    "builtin StringPrototypeFixed( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeItalics",
    "builtin StringPrototypeItalics( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeLink",
    "builtin StringPrototypeLink( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeSmall",
    "builtin StringPrototypeSmall( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeStrike",
    "builtin StringPrototypeStrike( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeSub",
    "builtin StringPrototypeSub( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypeSup",
    "builtin StringPrototypeSup( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 15);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 14);
};
