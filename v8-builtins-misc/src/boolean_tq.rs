// 由 tool/src/builtins/boolean.tq.js 从 v8/src/builtins/boolean.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 5 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "boolean", "namespace boolean"),
  (
    "macro",
    "ThisBooleanValue",
    "macro ThisBooleanValue( implicit context: Context)(receiver: JSAny, method: constexpr string): Boolean",
  ),
  (
    "builtin",
    "BooleanConstructor",
    "builtin BooleanConstructor( js-implicit context: NativeContext, receiver: JSAny, newTarget: JSAny, target: JSFunction)(...arguments): JSAny",
  ),
  (
    "builtin",
    "BooleanPrototypeToString",
    "builtin BooleanPrototypeToString( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
  (
    "builtin",
    "BooleanPrototypeValueOf",
    "builtin BooleanPrototypeValueOf( js-implicit context: NativeContext, receiver: JSAny)(): JSAny",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 5);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 3);
};
