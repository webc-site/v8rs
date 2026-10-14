// 由 tool/src/builtins/string-iterator.tq.js 从 v8/src/builtins/string-iterator.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 4 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "string", "namespace string"),
  (
    "macro",
    "NewJSStringIterator",
    "macro NewJSStringIterator( implicit context: Context)(string: String, nextIndex: Smi): JSStringIterator",
  ),
  (
    "builtin",
    "StringPrototypeIterator",
    "builtin StringPrototypeIterator( js-implicit context: NativeContext, receiver: JSAny)(): JSStringIterator",
  ),
  (
    "builtin",
    "StringIteratorPrototypeNext",
    "builtin StringIteratorPrototypeNext( js-implicit context: NativeContext, receiver: JSAny)(): JSObject",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 2);
};
