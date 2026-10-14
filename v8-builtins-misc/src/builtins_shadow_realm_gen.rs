// 由 tool/src/builtins/builtins-shadow-realm-gen.js 从 v8/src/builtins/builtins-shadow-realm-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 5 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ShadowRealmGetWrappedValue",
    assembler: "ShadowRealmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallWrappedFunction",
    assembler: "ShadowRealmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ShadowRealmPrototypeImportValue",
    assembler: "ShadowRealmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ShadowRealmImportValueFulfilled",
    assembler: "ShadowRealmBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ShadowRealmImportValueRejected",
    assembler: "ShadowRealmBuiltinsAssembler",
  },
];

/// 原版 enum ImportValueFulfilledFunctionContextSlot 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImportValueFulfilledFunctionContextSlot {
  /// kEvalContextSlot = Context::MIN_CONTEXT_SLOTS
  EvalContextSlot,
  /// kSpecifierSlot
  SpecifierSlot,
  /// kExportNameSlot
  ExportNameSlot,
  /// kContextLength
  ContextLength,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("ShadowRealmBuiltinsAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 5);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
  assert!(CLASSES.len() == 1);
};
