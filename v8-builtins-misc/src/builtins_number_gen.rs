// 由 tool/src/builtins/builtins-number-gen.js 从 v8/src/builtins/builtins-number-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 43 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "BuiltinPrefix",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AddLhsIsStringConstantInternalizeWithVector",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AddLhsIsStringConstantInternalizeTrampoline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AddRhsIsStringConstantInternalizeWithVector",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "AddRhsIsStringConstantInternalizeTrampoline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Equal_WithEmbeddedFeedback",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StrictEqual_WithEmbeddedFeedback",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Equal_Generic_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StrictEqual_Generic_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Negate_Number_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "EqualityType",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "StrictEqual_Any_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Equal_Any_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Name",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Add_String_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "ExponentiateAndTryPatchCode",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Exponentiate_None_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "Exponentiate_Number_Baseline",
    assembler: "CodeStubAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "OpName",
    assembler: "",
  },
];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 43);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
