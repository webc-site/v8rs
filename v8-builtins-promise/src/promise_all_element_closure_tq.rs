// 由 tool/src/builtins/promise-all-element-closure.tq.js 从 v8/src/builtins/promise-all-element-closure.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-promise; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 13 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "promise", "namespace promise"),
  (
    "struct",
    "PromiseAllWrapResultAsFulfilledFunctor",
    "struct PromiseAllWrapResultAsFulfilledFunctor",
  ),
  (
    "struct",
    "PromiseAllSettledWrapResultAsFulfilledFunctor",
    "struct PromiseAllSettledWrapResultAsFulfilledFunctor",
  ),
  (
    "struct",
    "PromiseAllSettledWrapResultAsRejectedFunctor",
    "struct PromiseAllSettledWrapResultAsRejectedFunctor",
  ),
  (
    "macro",
    "LoadJSReceiverIdentityHash",
    "macro LoadJSReceiverIdentityHash(JSReceiver): uint32 labels IfNoHash",
  ),
  (
    "type",
    "PromiseAllResolveElementContext",
    "type PromiseAllResolveElementContext extends FunctionContext",
  ),
  (
    "enum",
    "PromiseAllResolveElementContextSlots",
    "enum PromiseAllResolveElementContextSlots extends intptr constexpr 'PromiseBuiltins::PromiseAllResolveElementContextSlots'",
  ),
  (
    "const",
    "kPropertyArrayNoHashSentinel",
    "const kPropertyArrayNoHashSentinel: constexpr int31 generates 'PropertyArray::kNoHashSentinel'",
  ),
  (
    "const",
    "kPropertyArrayHashFieldMax",
    "const kPropertyArrayHashFieldMax: constexpr int31 generates 'PropertyArray::HashField::kMax'",
  ),
  (
    "macro",
    "PromiseAllResolveElementClosure",
    "macro PromiseAllResolveElementClosure<F: type>( implicit context: PromiseAllResolveElementContext)(value: JSAny, function: JSFunction, wrapResultFunctor: F): JSAny",
  ),
  (
    "builtin",
    "PromiseAllResolveElementClosure",
    "builtin PromiseAllResolveElementClosure( js-implicit context: Context, receiver: JSAny, target: JSFunction)( value: JSAny): JSAny",
  ),
  (
    "builtin",
    "PromiseAllSettledResolveElementClosure",
    "builtin PromiseAllSettledResolveElementClosure( js-implicit context: Context, receiver: JSAny, target: JSFunction)( value: JSAny): JSAny",
  ),
  (
    "builtin",
    "PromiseAllSettledRejectElementClosure",
    "builtin PromiseAllSettledRejectElementClosure( js-implicit context: Context, receiver: JSAny, target: JSFunction)( value: JSAny): JSAny",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  (
    "kPropertyArrayNoHashSentinel",
    "PropertyArray::kNoHashSentinel",
  ),
  (
    "kPropertyArrayHashFieldMax",
    "PropertyArray::HashField::kMax",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 13);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "struct").count() == 3);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "enum").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 3);
};
