// 由 tool/src/builtins/builtins-handler-gen.js 从 v8/src/builtins/builtins-handler-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 23 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "LoadIC_StringLength",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "LoadIC_StringWrapperLength",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ElementsTransitionAndStore_InBounds",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ElementsTransitionAndStore_NoTransitionGrowAndHandleCOW",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ElementsTransitionAndStore_NoTransitionIgnoreTypedArrayOOB",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ElementsTransitionAndStore_NoTransitionHandleCOW",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreFastElementIC_InBounds",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreFastElementIC_InBounds_Generic",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreFastElementIC_InBounds_ISX",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreFastElementIC_InBounds",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreFastElementIC_NoTransitionGrowAndHandleCOW",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreFastElementIC_NoTransitionIgnoreTypedArrayOOB",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreFastElementIC_NoTransitionHandleCOW",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "LoadIC_FunctionPrototype",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StoreGlobalIC_Slow",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "KeyedLoadIC_SloppyArguments",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "KeyedStoreIC_SloppyArguments_InBounds",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "KeyedStoreIC_SloppyArguments_NoTransitionGrowAndHandleCOW",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "KeyedStoreIC_SloppyArguments_NoTransitionIgnoreTypedArrayOOB",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "KeyedStoreIC_SloppyArguments_NoTransitionHandleCOW",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "LoadIndexedInterceptorIC",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "KeyedHasIC_SloppyArguments",
    assembler: "HandlerBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "HasIndexedInterceptorIC",
    assembler: "CodeStubAssembler",
  },
];

/// 平台汇编入口 Builtins::Generate_<名> 清单 (共 4 条)
pub const ASM_GENERATE_ENTRIES: &[&str] = &[
  "KeyedStoreIC_Megamorphic",
  "DefineKeyedOwnIC_Megamorphic",
  "StoreIC_NoFeedback",
  "DefineNamedOwnIC_NoFeedback",
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("HandlerBuiltinsAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 23);
  assert!(ASM_GENERATE_ENTRIES.len() == 4);
};
