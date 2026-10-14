// 由 tool/src/builtins/builtins-call-gen.js 从 v8/src/builtins/builtins-call-gen.cc + src/builtins/builtins-call-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-call-gen.cc, src/builtins/builtins-call-gen.h)
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 19 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsNullOrUndefined_Baseline_Compact",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsNullOrUndefined_Baseline",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsNotNullOrUndefined_Baseline_Compact",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsNotNullOrUndefined_Baseline",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsAny_Baseline_Compact",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsAny_Baseline",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsNullOrUndefined_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsNotNullOrUndefined_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Call_ReceiverIsAny_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallWithArrayLike",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallWithArrayLike_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallWithSpread",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallWithSpread_Baseline",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallWithSpread_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallFunctionTemplate_Generic",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallFunctionTemplate_CheckAccess",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallFunctionTemplate_CheckCompatibleReceiver",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "CallFunctionTemplate_CheckAccessAndCompatibleReceiver",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "HandleApiCallOrConstruct",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
];

/// 平台汇编入口 Builtins::Generate_<名> 清单 (共 16 条)
pub const ASM_GENERATE_ENTRIES: &[&str] = &[
  "CallFunction_ReceiverIsNullOrUndefined",
  "CallFunction_ReceiverIsNotNullOrUndefined",
  "CallFunction_ReceiverIsAny",
  "CallBoundFunction",
  "Call_ReceiverIsNullOrUndefined",
  "Call_ReceiverIsNotNullOrUndefined",
  "Call_ReceiverIsAny",
  "CallVarargs",
  "CallForwardVarargs",
  "CallFunctionForwardVarargs",
  "CallApiCallbackGeneric",
  "CallApiCallbackOptimizedNoProfiling",
  "CallApiCallbackOptimized",
  "CallApiGetter",
  "CallNamedInterceptorGetter",
  "CallNamedInterceptorSetter",
];

/// 原版 enum CallFunctionTemplateMode : uint8_t 直译 (变体不带值, 语义由 Developer 修订)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CallFunctionTemplateMode {
  /// kGeneric
  Generic,
  /// kCheckAccess
  CheckAccess,
  /// kCheckCompatibleReceiver
  CheckCompatibleReceiver,
  /// kCheckAccessAndCompatibleReceiver
  CheckAccessAndCompatibleReceiver,
}

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("CallOrConstructBuiltinsAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 19);
  assert!(ASM_GENERATE_ENTRIES.len() == 16);
  assert!(CLASSES.len() == 1);
};
