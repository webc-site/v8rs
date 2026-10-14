// 由 tool/src/builtins/builtins-date-gen.js 从 v8/src/builtins/builtins-date-gen.cc 生成; 修订就地改, 复跑覆盖
// C++ 规则骨架 — 内建登记/枚举直译/类清单, 函数体不展开
//! 任务映射: v8-builtins-misc; 聚合基线脚本另产同源模块 (并存不覆盖)

/// 内建实现种类
#[derive(Debug, Clone, Copy)]
pub struct CcBuiltin {
  pub macro_name: &'static str,
  pub name: &'static str,
  pub assembler: &'static str,
}

/// 内建登记清单 (共 20 条)
pub const CC_BUILTINS: &[CcBuiltin] = &[
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetDate",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetDay",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetFullYear",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetHours",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetMilliseconds",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetMinutes",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetMonth",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetSeconds",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetTime",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetTimezoneOffset",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCDate",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCDay",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCFullYear",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCHours",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCMilliseconds",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCMinutes",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCMonth",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeGetUTCSeconds",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeValueOf",
    assembler: "DateBuiltinsAssembler",
  },
  CcBuiltin {
    macro_name: "TF_BUILTIN",
    name: "DatePrototypeToPrimitive",
    assembler: "CodeStubAssembler",
  },
];

/// 类骨架 (名, 基类)
pub const CLASSES: &[(&str, &str)] = &[("DateBuiltinsAssembler", "CodeStubAssembler")];

// —— 对数断言 ——
const _: () = {
  assert!(CC_BUILTINS.len() == 20);
  assert!(ASM_GENERATE_ENTRIES.len() == 0);
};
