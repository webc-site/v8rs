// 由 tool/src/builtins/string-pad.tq.js 从 v8/src/builtins/string-pad.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 7 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "string", "namespace string"),
  (
    "builtin",
    "StringSubstring",
    "builtin StringSubstring( implicit context: Context)(String, intptr, intptr): String",
  ),
  (
    "const",
    "kStringPadStart",
    "const kStringPadStart: constexpr int31 = 0",
  ),
  (
    "const",
    "kStringPadEnd",
    "const kStringPadEnd: constexpr int31 = 1",
  ),
  (
    "macro",
    "StringPad",
    "macro StringPad( implicit context: Context)(receiver: JSAny, arguments: Arguments, methodName: constexpr string, variant: constexpr int31): String",
  ),
  (
    "builtin",
    "StringPrototypePadStart",
    "builtin StringPrototypePadStart( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
  (
    "builtin",
    "StringPrototypePadEnd",
    "builtin StringPrototypePadEnd( js-implicit context: NativeContext, receiver: JSAny)( ...arguments): String",
  ),
];

/// 字面量常量直译 (原版 const <名>: <类型> = <字面量>)
pub const STRING_PAD_START: i64 = 0; // 原版: kStringPadStart
pub const STRING_PAD_END: i64 = 1; // 原版: kStringPadEnd

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
};
