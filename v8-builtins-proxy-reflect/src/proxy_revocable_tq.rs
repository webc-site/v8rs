// 由 tool/src/builtins/proxy-revocable.tq.js 从 v8/src/builtins/proxy-revocable.tq 生成; 修订就地改, 复跑覆盖
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

/// 原版顶层声明清单 (共 4 条)
pub const DECLS: &[TqDecl] = &[
  ("namespace", "proxy", "namespace proxy"),
  (
    "macro",
    "ProxiesCodeStubAssembler::AllocateProxyRevokeFunction",
    "macro ProxiesCodeStubAssembler::AllocateProxyRevokeFunction( implicit context: Context)(JSProxy): JSFunction",
  ),
  (
    "const",
    "kIsRevocable",
    "const kIsRevocable: constexpr int32 generates 'JSProxy::kIsRevocableBit'",
  ),
  (
    "builtin",
    "ProxyRevocable",
    "builtin ProxyRevocable( js-implicit context: NativeContext)(target: JSAny, handler: JSAny): JSProxyRevocableResult",
  ),
];

/// constexpr ... generates 'C++ 符号' 引用登记 (机械登记, 不猜值)
pub const CPP_GENERATES: &[(&str, &str)] = &[("kIsRevocable", "JSProxy::kIsRevocableBit")];

// —— 对数断言 ——
const _: () = {
  assert!(DECLS.len() == 4);
  assert!(DECLS.iter().filter(|d| d.kind == "namespace").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "macro").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "const").count() == 1);
  assert!(DECLS.iter().filter(|d| d.kind == "builtin").count() == 1);
};
