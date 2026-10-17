// 由 tool/src/builtins/object.tq.js 从 v8/src/builtins/object.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 16 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "runtime", "namespace runtime"),
  ("namespace", "object", "namespace object"),
  (
    "const",
    "objectJSReceiver",
    "const objectJSReceiver = Cast<JSReceiver>(object) otherwise return False",
  ),
  (
    "const",
    "objectJSProxy",
    "const objectJSProxy = Cast<JSProxy>(objectJSReceiver) otherwise return runtime::ObjectIsExtensible(objectJSReceiver)",
  ),
  (
    "const",
    "objectJSReceiver",
    "const objectJSReceiver = Cast<JSReceiver>(object) otherwise return object",
  ),
  (
    "const",
    "objectJSProxy",
    "const objectJSProxy = Cast<JSProxy>(objectJSReceiver) otherwise return runtime::JSReceiverPreventExtensionsThrow(objectJSReceiver)",
  ),
  (
    "const",
    "objectJSReceiver",
    "const objectJSReceiver = Cast<JSReceiver>(object) otherwise return False",
  ),
  (
    "const",
    "objectJSProxy",
    "const objectJSProxy = Cast<JSProxy>(objectJSReceiver) otherwise return runtime::JSReceiverPreventExtensionsDontThrow( objectJSReceiver)",
  ),
  (
    "const",
    "objectJSReceiver",
    "const objectJSReceiver: JSReceiver = ToObject_Inline(context, object)",
  ),
  (
    "const",
    "objectJSProxy",
    "const objectJSProxy = Cast<JSProxy>(object) otherwise return runtime::JSReceiverGetPrototypeOf(object)",
  ),
  (
    "const",
    "objectJSReceiver",
    "const objectJSReceiver = Cast<JSReceiver>(object) otherwise return object",
  ),
  (
    "const",
    "objectJSProxy",
    "const objectJSProxy = Cast<JSProxy>(objectJSReceiver) otherwise return runtime::JSReceiverSetPrototypeOfThrow( objectJSReceiver, proto)",
  ),
  (
    "const",
    "objectJSReceiver",
    "const objectJSReceiver = Cast<JSReceiver>(object) otherwise return False",
  ),
  (
    "const",
    "objectJSProxy",
    "const objectJSProxy = Cast<JSProxy>(objectJSReceiver) otherwise return runtime::JSReceiverSetPrototypeOfDontThrow( objectJSReceiver, proto)",
  ),
  (
    "const",
    "method",
    "const method = GetProperty(receiver, 'toString')",
  ),
  (
    "const",
    "result",
    "const result: PropertyDescriptorObject = AllocatePropertyDescriptorObject()",
  ),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 16);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 14);
};
