// 由 tool/src/builtins/proxy.tq.js 从 v8/src/builtins/proxy.tq 生成; 修订就地改, 复跑覆盖
// Torque 声明骨架 — 声明清单/常量直译/C++ 符号引用, 函数体不展开
//! 任务映射: v8-builtins-proxy-reflect; 聚合基线脚本另产同源模块 (并存不覆盖)

/// Torque 声明条目 (kind: namespace/type/const/macro/builtin/struct/enum/class/import)
#[derive(Debug, Clone, Copy)]
pub struct TqDecl {
  pub kind: &'static str,
  pub name: &'static str,
  /// 原版签名 (空白规整; 含 labels/js-implicit 等修饰)
  pub sig: &'static str,
}

/// 原版顶层声明清单 (共 9 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "proxy", "namespace proxy"),
  (
    "macro",
    "ProxiesCodeStubAssembler::AllocateProxy",
    "macro ProxiesCodeStubAssembler::AllocateProxy( implicit context: Context)(JSReceiver, JSReceiver, int32): JSProxy",
  ),
  (
    "macro",
    "ProxiesCodeStubAssembler::CheckGetSetTrapResult",
    "macro ProxiesCodeStubAssembler::CheckGetSetTrapResult( implicit context: Context)(JSReceiver, JSProxy, Name, Object, constexpr int31): void",
  ),
  (
    "macro",
    "ProxiesCodeStubAssembler::CheckDeleteTrapResult",
    "macro ProxiesCodeStubAssembler::CheckDeleteTrapResult( implicit context: Context)(JSReceiver, JSProxy, Name): void",
  ),
  (
    "macro",
    "ProxiesCodeStubAssembler::CheckHasTrapResult",
    "macro ProxiesCodeStubAssembler::CheckHasTrapResult( implicit context: Context)(JSReceiver, JSProxy, Name): void",
  ),
  (
    "const",
    "kProxyGet",
    "const kProxyGet: constexpr int31 generates 'JSProxy::AccessKind::kGet'",
  ),
  (
    "const",
    "kProxySet",
    "const kProxySet: constexpr int31 generates 'JSProxy::AccessKind::kSet'",
  ),
  (
    "type",
    "ProxyRevokeFunctionContext",
    "type ProxyRevokeFunctionContext extends FunctionContext",
  ),
  (
    "enum",
    "ProxyRevokeFunctionContextSlot",
    "enum ProxyRevokeFunctionContextSlot extends intptr constexpr 'ProxiesCodeStubAssembler::ProxyRevokeFunctionContextSlot'",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[
  ("kProxyGet", "JSProxy::AccessKind::kGet"),
  ("kProxySet", "JSProxy::AccessKind::kSet"),
];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 9);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 2);
  assert!(DECLS.iter().filter(|d| d.kind == "type").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "enum").count() == 1);
};
