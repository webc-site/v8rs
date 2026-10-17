// 由 tool/src/builtins/builtins-proxy-gen.js 从 v8/src/builtins/builtins-proxy-gen.cc + src/builtins/builtins-proxy-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-proxy-gen.cc, src/builtins/builtins-proxy-gen.h)
//! 任务映射: v8-builtins-proxy-reflect; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 4 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallProxy",
    assembler: "ProxiesCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ConstructProxy",
    assembler: "ProxiesCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ProxyGetPropertyFastPath",
    assembler: "ProxiesCodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ProxyGetPropertyTrapResultLazyDeoptContinuation",
    assembler: "ProxiesCodeStubAssembler",
  },
];

/// 原版 enum ProxyRevokeFunctionContextSlot 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProxyRevokeFunctionContextSlot {
  /// kProxySlot = Context::MIN_CONTEXT_SLOTS
  ProxySlot,
  /// kProxyContextLength
  ProxyContextLength,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("ProxiesCodeStubAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 4);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 1);
};
