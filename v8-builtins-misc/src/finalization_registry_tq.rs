// 由 tool/src/builtins/finalization-registry.tq.js 从 v8/src/builtins/finalization-registry.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 10 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "weakref", "namespace weakref"),
  (
    "const",
    "cleanupCallback",
    "const cleanupCallback = Cast<Callable>(arguments[0]) otherwise ThrowTypeError(MessageTemplate::kWeakRefsCleanupMustBeCallable)",
  ),
  (
    "const",
    "map",
    "const map = GetDerivedMap(target, UnsafeCast<JSReceiver>(newTarget))",
  ),
  (
    "const",
    "finalizationRegistry",
    "const finalizationRegistry = UnsafeCast<JSFinalizationRegistry>( AllocateFastOrSlowJSObjectFromMap(map))",
  ),
  (
    "const",
    "finalizationRegistry",
    "const finalizationRegistry = Cast<JSFinalizationRegistry>(receiver) otherwise ThrowTypeError( MessageTemplate::kIncompatibleMethodReceiver, 'FinalizationRegistry.prototype.register', receiver)",
  ),
  (
    "const",
    "target",
    "const target = UnsafeCast<(JSReceiver | Symbol)>(arguments[0])",
  ),
  ("const", "heldValue", "const heldValue = arguments[1]"),
  (
    "const",
    "unregisterTokenRaw",
    "const unregisterTokenRaw = arguments[2]",
  ),
  ("const", "cell", "const cell = new WeakCell"),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 10);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 8);
};
