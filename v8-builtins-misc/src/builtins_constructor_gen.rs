// 由 tool/src/builtins/builtins-constructor-gen.js 从 v8/src/builtins/builtins-constructor-gen.cc + src/builtins/builtins-constructor-gen.h 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 同名族合并脚本 (src/builtins/builtins-constructor-gen.cc, src/builtins/builtins-constructor-gen.h)
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 10 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Construct_Baseline",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Construct_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ConstructWithArrayLike",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ConstructWithSpread",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ConstructWithSpread_Baseline",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ConstructWithSpread_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ConstructForwardAllArgs_Baseline",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ConstructForwardAllArgs_WithFeedback",
    assembler: "CallOrConstructBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "FastNewClosure",
    assembler: "ConstructorBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "FastNewObject",
    assembler: "ConstructorBuiltinsAssembler",
  },
];

/// 平台汇编入口 Builtins::Generate_<名> 清单 (共 5 条)
pub const ASM_GENERATE_ENTRIES: &[&str] = &[
  "ConstructVarargs",
  "ConstructForwardVarargs",
  "ConstructFunctionForwardVarargs",
  "InterpreterForwardAllArgsThenConstruct",
  "ConstructForwardAllArgs",
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("ConstructorBuiltinsAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 10);
  assert!(ASM_GENERATE_ENTRIES.len() == 5);
};
