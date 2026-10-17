// 由 tool/src/builtins/weak-ref.tq.js 从 v8/src/builtins/weak-ref.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-collections; 聚合基线脚本另产同源模块 (并存不覆盖)

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
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "weakref", "namespace weakref"),
  (
    "const",
    "weakTarget",
    "const weakTarget = UnsafeCast<(JSReceiver | Symbol)>(arguments[0])",
  ),
  (
    "const",
    "map",
    "const map = GetDerivedMap(target, UnsafeCast<JSReceiver>(newTarget))",
  ),
  (
    "const",
    "weakRef",
    "const weakRef = UnsafeCast<JSWeakRef>(AllocateFastOrSlowJSObjectFromMap(map))",
  ),
  (
    "const",
    "weakRef",
    "const weakRef = Cast<JSWeakRef>(receiver) otherwise ThrowTypeError( MessageTemplate::kIncompatibleMethodReceiver, 'WeakRef.prototype.deref', receiver)",
  ),
  ("const", "target", "const target = weakRef.target"),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 7);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 5);
};
